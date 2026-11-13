use std::{collections::BTreeSet, ops::Range, path::Path, time::Duration};

use collections::{HashMap, HashSet};
use editor::{Editor, EditorEvent};
use gpui::{
    App, Context, Div, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Hsla,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Task, UniformListScrollHandle, WeakEntity,
    Window, div, px, relative, rgb, uniform_list,
};
use i18n::t;
use language::Point;
use project::{Project, ProjectPath};
use theme::ActiveTheme;
use ui::{
    ContextMenu, Icon, IconButton, IconName, IconPosition, IconSize, Label, LabelSize, PopoverMenu,
    Tooltip, h_flex, prelude::*, v_flex,
};
use util::ResultExt;
use workspace::{Item, Workspace};

use crate::OpenProblemsTable;

const ROW_HEIGHT: f32 = 24.;
const SEVERITY_COLUMN_WIDTH: f32 = 36.;
const CELL_PADDING: f32 = 10.;
const TREE_INDENT: f32 = 22.;
const REFRESH_DEBOUNCE: Duration = Duration::from_millis(100);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Table,
    Tree,
}

#[derive(Clone)]
struct ProblemRow {
    path: ProjectPath,
    file_label: SharedString,
    position_label: SharedString,
    point: Point,
    severity: lsp::DiagnosticSeverity,
    message: SharedString,
    source: Option<SharedString>,
    code: Option<SharedString>,
    code_url: Option<SharedString>,
    ignored: bool,
}

enum TreeItem {
    File {
        path: ProjectPath,
        label: SharedString,
        count: usize,
        collapsed: bool,
    },
    Row(usize),
}

enum Matcher {
    Text(String),
    Source(String),
    Glob(globset::GlobMatcher),
}

impl Matcher {
    fn matches(&self, row: &ProblemRow) -> bool {
        match self {
            Matcher::Text(text) => {
                row.message.to_lowercase().contains(text)
                    || row.file_label.to_lowercase().contains(text)
                    || row
                        .source
                        .as_ref()
                        .is_some_and(|source| source.to_lowercase().contains(text))
                    || row
                        .code
                        .as_ref()
                        .is_some_and(|code| code.to_lowercase().contains(text))
            }
            Matcher::Source(source) => row
                .source
                .as_ref()
                .is_some_and(|value| value.to_lowercase().contains(source)),
            Matcher::Glob(glob) => glob.is_match(row.file_label.as_ref()),
        }
    }
}

#[derive(Default)]
struct TextFilter {
    include: Vec<Matcher>,
    exclude: Vec<Matcher>,
}

impl TextFilter {
    fn parse(text: &str) -> Self {
        let mut filter = TextFilter::default();
        for token in text.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let (negated, token) = match token.strip_prefix('!') {
                Some(rest) => (true, rest.trim()),
                None => (false, token),
            };
            if token.is_empty() {
                continue;
            }
            let matcher = if let Some(source) = token.strip_prefix("@source:") {
                Matcher::Source(source.to_lowercase())
            } else if token.contains('*') || token.contains('/') {
                match globset::Glob::new(token) {
                    Ok(glob) => Matcher::Glob(glob.compile_matcher()),
                    Err(_) => Matcher::Text(token.to_lowercase()),
                }
            } else {
                Matcher::Text(token.to_lowercase())
            };
            if negated {
                filter.exclude.push(matcher);
            } else {
                filter.include.push(matcher);
            }
        }
        filter
    }

    fn accepts(&self, row: &ProblemRow) -> bool {
        if self.exclude.iter().any(|matcher| matcher.matches(row)) {
            return false;
        }
        self.include.is_empty() || self.include.iter().any(|matcher| matcher.matches(row))
    }
}

pub struct ProblemsTable {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    filter_editor: Entity<Editor>,
    rows: Vec<ProblemRow>,
    visible: Vec<usize>,
    tree_items: Vec<TreeItem>,
    collapsed: HashSet<ProjectPath>,
    view_mode: ViewMode,
    show_errors: bool,
    show_warnings: bool,
    show_infos: bool,
    active_file_only: bool,
    hide_excluded: bool,
    selected: Option<usize>,
    refresh_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<()> for ProblemsTable {}

impl ProblemsTable {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(Self::deploy);
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &OpenProblemsTable,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(existing) = workspace.item_of_type::<ProblemsTable>(cx) {
            workspace.activate_item(&existing, true, true, window, cx);
            return;
        }
        let project = workspace.project().clone();
        let workspace_handle = cx.entity().downgrade();
        let table = cx.new(|cx| ProblemsTable::new(project, workspace_handle, window, cx));
        workspace.add_item_to_active_pane(Box::new(table), None, true, window, cx);
    }

    pub fn build(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(project, workspace, window, cx))
    }

    fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_subscription =
            cx.subscribe_in(&project, window, |this, _, event, window, cx| {
                if matches!(
                    event,
                    project::Event::DiagnosticsUpdated { .. }
                        | project::Event::DiskBasedDiagnosticsFinished { .. }
                ) {
                    this.schedule_refresh(window, cx);
                }
            });
        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(
                &t!("Filter (e.g. text, **/*.ts, !**/node_modules/**)"),
                window,
                cx,
            );
            editor
        });
        let filter_subscription =
            cx.subscribe_in(&filter_editor, window, |_, _, event: &EditorEvent, _, cx| {
                if matches!(event, EditorEvent::BufferEdited) {
                    cx.notify();
                }
            });
        let mut this = Self {
            project,
            workspace,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            filter_editor,
            rows: Vec::new(),
            visible: Vec::new(),
            tree_items: Vec::new(),
            collapsed: HashSet::default(),
            view_mode: ViewMode::Table,
            show_errors: true,
            show_warnings: true,
            show_infos: true,
            active_file_only: false,
            hide_excluded: true,
            selected: None,
            refresh_task: Task::ready(()),
            _subscriptions: vec![project_subscription, filter_subscription],
        };
        this.refresh(window, cx);
        this
    }

    fn schedule_refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.refresh_task = cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(REFRESH_DEBOUNCE).await;
            this.update_in(cx, |this, window, cx| this.refresh(window, cx))
                .log_err();
        });
    }

    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let paths = project
            .read(cx)
            .diagnostic_summaries(false, cx)
            .filter(|(_, _, summary)| summary.error_count + summary.warning_count > 0)
            .map(|(path, _, _)| path)
            .collect::<BTreeSet<_>>();
        let path_style = project.read(cx).path_style(cx);
        let server_names: HashMap<lsp::LanguageServerId, SharedString> = project
            .read(cx)
            .language_server_statuses(cx)
            .map(|(id, status)| (id, status.name.0.clone()))
            .collect();
        let mut server_for_path: HashMap<ProjectPath, SharedString> = HashMap::default();
        for (path, server_id, summary) in project.read(cx).diagnostic_summaries(false, cx) {
            if summary.error_count + summary.warning_count > 0
                && let Some(name) = server_names.get(&server_id)
            {
                server_for_path.entry(path).or_insert_with(|| name.clone());
            }
        }

        self.refresh_task = cx.spawn_in(window, async move |this, cx| {
            let mut rows = Vec::new();
            for path in paths {
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(path.clone(), cx))
                    .await;
                let Ok(buffer) = buffer else {
                    continue;
                };
                let ignored = project.read_with(cx, |project, cx| {
                    project
                        .entry_for_path(&path, cx)
                        .is_some_and(|entry| entry.is_ignored)
                });
                let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
                let file_label: SharedString = path.path.display(path_style).to_string().into();
                for entry in snapshot
                    .diagnostics_in_range::<_, Point>(Point::zero()..snapshot.max_point(), false)
                {
                    let diagnostic = &entry.diagnostic;
                    if !diagnostic.is_primary
                        || diagnostic.severity > lsp::DiagnosticSeverity::INFORMATION
                    {
                        continue;
                    }
                    let point = entry.range.start;
                    let code: Option<SharedString> = match &diagnostic.code {
                        Some(lsp::NumberOrString::Number(number)) => {
                            Some(number.to_string().into())
                        }
                        Some(lsp::NumberOrString::String(string)) => Some(string.clone().into()),
                        None => None,
                    };
                    let code_url = diagnostic
                        .code_description
                        .as_ref()
                        .map(|uri| SharedString::from(uri.to_string()));
                    rows.push(ProblemRow {
                        path: path.clone(),
                        file_label: file_label.clone(),
                        position_label: t!(
                            "[Ln {line}, Col {col}]",
                            line = point.row + 1,
                            col = point.column + 1
                        ),
                        point,
                        severity: diagnostic.severity,
                        message: diagnostic.message.as_str().replace('\n', " ").into(),
                        source: diagnostic
                            .source
                            .clone()
                            .map(SharedString::from)
                            .or_else(|| server_for_path.get(&path).cloned()),
                        code,
                        code_url,
                        ignored,
                    });
                }
            }
            rows.sort_by(|a, b| {
                a.severity
                    .cmp(&b.severity)
                    .then_with(|| a.file_label.cmp(&b.file_label))
                    .then_with(|| a.point.cmp(&b.point))
            });
            this.update(cx, |this, cx| {
                this.rows = rows;
                if this.selected.is_some_and(|ix| ix >= this.rows.len()) {
                    this.selected = None;
                }
                cx.notify();
            })
            .log_err();
        });
    }

    fn recompute(&mut self, cx: &mut Context<Self>) {
        let filter = TextFilter::parse(&self.filter_editor.read(cx).text(cx));
        let active_path = if self.active_file_only {
            self.workspace
                .upgrade()
                .and_then(|workspace| workspace.read(cx).active_item(cx))
                .and_then(|item| item.project_path(cx))
        } else {
            None
        };
        let active_file_only = self.active_file_only;
        self.visible = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                let severity_ok = if row.severity == lsp::DiagnosticSeverity::ERROR {
                    self.show_errors
                } else if row.severity == lsp::DiagnosticSeverity::WARNING {
                    self.show_warnings
                } else {
                    self.show_infos
                };
                severity_ok
                    && (!active_file_only || active_path.as_ref() == Some(&row.path))
                    && !(self.hide_excluded && row.ignored)
                    && filter.accepts(row)
            })
            .map(|(ix, _)| ix)
            .collect();

        self.tree_items.clear();
        if self.view_mode == ViewMode::Tree {
            let mut order = self.visible.clone();
            order.sort_by(|&a, &b| {
                self.rows[a]
                    .file_label
                    .cmp(&self.rows[b].file_label)
                    .then_with(|| self.rows[a].point.cmp(&self.rows[b].point))
            });
            let mut current: Option<ProjectPath> = None;
            for ix in order {
                let row = &self.rows[ix];
                if current.as_ref() != Some(&row.path) {
                    current = Some(row.path.clone());
                    let count = self
                        .visible
                        .iter()
                        .filter(|&&other| self.rows[other].path == row.path)
                        .count();
                    self.tree_items.push(TreeItem::File {
                        path: row.path.clone(),
                        label: row.file_label.clone(),
                        count,
                        collapsed: self.collapsed.contains(&row.path),
                    });
                }
                if !self.collapsed.contains(&row.path) {
                    self.tree_items.push(TreeItem::Row(ix));
                }
            }
        }
    }

    fn filter_is_active(&self, cx: &App) -> bool {
        !self.filter_editor.read(cx).text(cx).trim().is_empty()
            || !self.show_errors
            || !self.show_warnings
            || !self.show_infos
            || self.active_file_only
    }

    fn open_row(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.rows.get(ix).cloned() else {
            return;
        };
        self.selected = Some(ix);
        cx.notify();
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let open = workspace.update(cx, |workspace, cx| {
            workspace.open_path(row.path.clone(), None, true, window, cx)
        });
        cx.spawn_in(window, async move |_, cx| {
            let item = open.await?;
            if let Some(editor) = item.downcast::<Editor>() {
                editor.update_in(cx, |editor, window, cx| {
                    editor.go_to_singleton_buffer_point(row.point, window, cx);
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn toolbar(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.recompute(cx);
        let colors = cx.theme().colors().clone();
        let total = self.rows.len();
        let shown = self.visible.len();
        let filtered = self.filter_is_active(cx) && shown != total;
        let weak = cx.entity().downgrade();
        let view_mode = self.view_mode;
        let (show_errors, show_warnings, show_infos, active_file_only, hide_excluded) = (
            self.show_errors,
            self.show_warnings,
            self.show_infos,
            self.active_file_only,
            self.hide_excluded,
        );

        h_flex()
            .gap_1()
            .items_center()
            .child(
                h_flex()
                    .w(px(240.))
                    .h(px(22.))
                    .px_1()
                    .gap_1()
                    .items_center()
                    .rounded(px(3.))
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.editor_background)
                    .child(div().flex_1().min_w_0().child(self.filter_editor.clone()))
                    .when(filtered, |this| {
                        this.child(
                            Label::new(t!(
                                "Showing {shown} of {total}",
                                shown = shown,
                                total = total
                            ))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                        )
                    })
                    .child(
                        PopoverMenu::new("problems-filter-menu")
                            .trigger(
                                IconButton::new("problems-filter-button", IconName::Filter)
                                    .icon_size(IconSize::Custom(gpui::rems(16. / 13.)))
                                    .width(px(20.))
                                    .height(px(20.).into())
                                    .tooltip(Tooltip::text(t!("More Filters..."))),
                            )
                            .menu(move |window, cx| {
                                let weak = weak.clone();
                                Some(ContextMenu::build(window, cx, move |menu, _, _| {
                                    let toggle = |label: SharedString,
                                                  checked: bool,
                                                  apply: fn(&mut ProblemsTable)| {
                                        let weak = weak.clone();
                                        (label, checked, weak, apply)
                                    };
                                    let entries = [
                                        toggle(t!("Show Errors"), show_errors, |table| {
                                            table.show_errors = !table.show_errors
                                        }),
                                        toggle(t!("Show Warnings"), show_warnings, |table| {
                                            table.show_warnings = !table.show_warnings
                                        }),
                                        toggle(t!("Show Infos"), show_infos, |table| {
                                            table.show_infos = !table.show_infos
                                        }),
                                    ];
                                    let mut menu = menu;
                                    for (label, checked, weak, apply) in entries {
                                        menu = menu.toggleable_entry(
                                            label,
                                            checked,
                                            IconPosition::Start,
                                            None,
                                            move |_, cx| {
                                                weak.update(cx, |table, cx| {
                                                    apply(table);
                                                    cx.notify();
                                                })
                                                .ok();
                                            },
                                        );
                                    }
                                    menu = menu.separator();
                                    for (label, checked, apply) in [
                                        (
                                            t!("Show Active File Only"),
                                            active_file_only,
                                            (|table: &mut ProblemsTable| {
                                                table.active_file_only = !table.active_file_only
                                            }) as fn(&mut ProblemsTable),
                                        ),
                                        (
                                            t!("Hide Excluded Files"),
                                            hide_excluded,
                                            (|table: &mut ProblemsTable| {
                                                table.hide_excluded = !table.hide_excluded
                                            }) as fn(&mut ProblemsTable),
                                        ),
                                    ] {
                                        let weak = weak.clone();
                                        menu = menu.toggleable_entry(
                                            label,
                                            checked,
                                            IconPosition::Start,
                                            None,
                                            move |_, cx| {
                                                weak.update(cx, |table, cx| {
                                                    apply(table);
                                                    cx.notify();
                                                })
                                                .ok();
                                            },
                                        );
                                    }
                                    menu
                                }))
                            }),
                    ),
            )
            .when(view_mode == ViewMode::Tree, |this| {
                this.child(
                    IconButton::new("problems-collapse-all", IconName::CollapseAll)
                        .icon_size(IconSize::Custom(gpui::rems(16. / 13.)))
                        .width(px(22.))
                        .height(px(22.).into())
                        .tooltip(Tooltip::text(t!("Collapse All")))
                        .on_click(cx.listener(|this, _, _, cx| {
                            let paths = this
                                .rows
                                .iter()
                                .map(|row| row.path.clone())
                                .collect::<Vec<_>>();
                            this.collapsed.extend(paths);
                            cx.notify();
                        })),
                )
            })
            .child(
                IconButton::new(
                    "problems-view-mode",
                    if view_mode == ViewMode::Table {
                        IconName::ListTree
                    } else {
                        IconName::ListFlat
                    },
                )
                .icon_size(IconSize::Custom(gpui::rems(16. / 13.)))
                .width(px(22.))
                .height(px(22.).into())
                .tooltip(Tooltip::text(if view_mode == ViewMode::Table {
                    t!("View as Tree")
                } else {
                    t!("View as Table")
                }))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.view_mode = match this.view_mode {
                        ViewMode::Table => ViewMode::Tree,
                        ViewMode::Tree => ViewMode::Table,
                    };
                    cx.notify();
                })),
            )
    }
}

enum Column {
    Severity,
    Code,
    Message,
    File,
    Source,
}

fn column_cell(column: Column) -> Div {
    let mut cell = div()
        .h_full()
        .flex()
        .flex_row()
        .items_center()
        .pl(px(CELL_PADDING))
        .overflow_hidden()
        .whitespace_nowrap()
        .text_ellipsis();
    let weight = match column {
        Column::Severity => {
            return cell.w(px(SEVERITY_COLUMN_WIDTH)).flex_none();
        }
        Column::Code => 1.,
        Column::Message => 4.,
        Column::File => 2.,
        Column::Source => 1.,
    };
    {
        let style = cell.style();
        style.flex_grow = Some(weight);
        style.flex_shrink = Some(1.);
        style.flex_basis = Some(relative(0.).into());
    }
    cell.min_w_0()
}

fn severity_icon(severity: lsp::DiagnosticSeverity) -> Icon {
    let (name, color) = if severity == lsp::DiagnosticSeverity::ERROR {
        (IconName::XCircle, Color::Error)
    } else if severity == lsp::DiagnosticSeverity::WARNING {
        (IconName::Warning, Color::Warning)
    } else {
        (IconName::Info, Color::Info)
    };
    Icon::new(name)
        .size(IconSize::Custom(gpui::rems(16. / 13.)))
        .color(color)
}

impl Render for ProblemsTable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.recompute(cx);
        let colors = cx.theme().colors().clone();
        let text_color = rgb(0xbbbbbb);
        let link_color = rgb(0x3794ff);
        let odd_row_background: Hsla = Hsla::from(rgb(0xbbbbbb)).opacity(0.04);

        let header = h_flex()
            .h(px(ROW_HEIGHT))
            .w_full()
            .flex_none()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(text_color)
            .child(column_cell(Column::Severity))
            .child(column_cell(Column::Code).child(t!("Code")))
            .child(column_cell(Column::Message).child(t!("Message")))
            .child(column_cell(Column::File).child(t!("File")))
            .child(column_cell(Column::Source).child(t!("Source")));

        let is_empty = if self.view_mode == ViewMode::Table {
            self.visible.is_empty()
        } else {
            self.tree_items.is_empty()
        };

        let body = if is_empty {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Label::new(t!("No problems have been detected in the workspace."))
                        .color(Color::Muted),
                )
                .into_any_element()
        } else if self.view_mode == ViewMode::Table {
            uniform_list(
                "problems-table-rows",
                self.visible.len(),
                cx.processor(move |this, range: Range<usize>, _window, cx| {
                    let colors = cx.theme().colors().clone();
                    let mut elements = Vec::with_capacity(range.len());
                    for position in range {
                        let Some(&ix) = this.visible.get(position) else {
                            continue;
                        };
                        let Some(row) = this.rows.get(ix).cloned() else {
                            continue;
                        };
                        let selected = this.selected == Some(ix);
                        let code_cell = column_cell(Column::Code).child(match (&row.source, &row.code)
                        {
                            (Some(source), Some(code)) => {
                                let code_label = div()
                                    .id(("problem-code", ix))
                                    .ml(px(4.))
                                    .flex_none()
                                    .child(SharedString::from(format!("({code})")))
                                    .when_some(row.code_url.clone(), |label, url| {
                                        label
                                            .text_color(link_color)
                                            .underline()
                                            .cursor_pointer()
                                            .on_click(move |_, _, cx: &mut App| {
                                                cx.stop_propagation();
                                                cx.open_url(&url);
                                            })
                                    });
                                h_flex()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .child(div().truncate().child(source.clone()))
                                    .child(code_label)
                                    .into_any_element()
                            }
                            _ => div().child("-").into_any_element(),
                        });
                        elements.push(
                            h_flex()
                                .id(("problem-row", ix))
                                .h(px(ROW_HEIGHT))
                                .w_full()
                                .text_color(text_color)
                                .cursor_pointer()
                                .when(position % 2 == 1, |row| row.bg(odd_row_background))
                                .when(selected, |row| row.bg(colors.element_selected))
                                .hover(|row| row.bg(colors.element_hover))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.open_row(ix, window, cx);
                                }))
                                .child(
                                    column_cell(Column::Severity)
                                        .child(severity_icon(row.severity)),
                                )
                                .child(code_cell)
                                .child(column_cell(Column::Message).child(row.message.clone()))
                                .child(
                                    column_cell(Column::File)
                                        .child(div().truncate().child(row.file_label.clone()))
                                        .child(
                                            div()
                                                .ml(px(6.))
                                                .flex_none()
                                                .opacity(0.7)
                                                .child(row.position_label.clone()),
                                        ),
                                )
                                .child(column_cell(Column::Source).child(
                                    row.source.clone().unwrap_or_else(|| SharedString::from("")),
                                )),
                        );
                    }
                    elements
                }),
            )
            .track_scroll(&self.scroll_handle)
            .flex_1()
            .into_any_element()
        } else {
            uniform_list(
                "problems-tree-rows",
                self.tree_items.len(),
                cx.processor(move |this, range: Range<usize>, _window, cx| {
                    let colors = cx.theme().colors().clone();
                    let mut elements = Vec::with_capacity(range.len());
                    for position in range {
                        match this.tree_items.get(position) {
                            Some(TreeItem::File {
                                path,
                                label,
                                count,
                                collapsed,
                            }) => {
                                let path = path.clone();
                                let name = Path::new(label.as_ref())
                                    .file_name()
                                    .map(|name| name.to_string_lossy().to_string())
                                    .unwrap_or_else(|| label.to_string());
                                let directory = Path::new(label.as_ref())
                                    .parent()
                                    .map(|parent| parent.to_string_lossy().to_string())
                                    .filter(|parent| !parent.is_empty());
                                let badge_background = colors.tab_active_foreground;
                                let badge_foreground = colors.status_bar_background;
                                let count = *count;
                                let collapsed = *collapsed;
                                elements.push(
                                    h_flex()
                                        .id(("problem-file", position))
                                        .h(px(ROW_HEIGHT))
                                        .w_full()
                                        .pl(px(4.))
                                        .gap_1()
                                        .text_color(text_color)
                                        .cursor_pointer()
                                        .hover(|row| row.bg(colors.element_hover))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            if !this.collapsed.remove(&path) {
                                                this.collapsed.insert(path.clone());
                                            }
                                            cx.notify();
                                        }))
                                        .child(
                                            Icon::new(if collapsed {
                                                IconName::ChevronRight
                                            } else {
                                                IconName::ChevronDown
                                            })
                                            .size(IconSize::Small),
                                        )
                                        .child(Icon::new(IconName::File).size(IconSize::Small))
                                        .child(div().flex_none().child(name))
                                        .when_some(directory, |this, directory| {
                                            this.child(
                                                div()
                                                    .truncate()
                                                    .opacity(0.7)
                                                    .child(SharedString::from(directory)),
                                            )
                                        })
                                        .child(
                                            div()
                                                .flex_none()
                                                .px(px(6.))
                                                .h(px(16.))
                                                .rounded_full()
                                                .flex()
                                                .items_center()
                                                .text_size(px(11.))
                                                .bg(badge_background)
                                                .text_color(badge_foreground)
                                                .child(count.to_string()),
                                        ),
                                );
                            }
                            Some(TreeItem::Row(ix)) => {
                                let ix = *ix;
                                let Some(row) = this.rows.get(ix).cloned() else {
                                    continue;
                                };
                                let selected = this.selected == Some(ix);
                                let source_code = match (&row.source, &row.code) {
                                    (Some(source), Some(code)) => {
                                        Some(SharedString::from(format!("{source}({code})")))
                                    }
                                    (Some(source), None) => Some(source.clone()),
                                    _ => None,
                                };
                                elements.push(
                                    h_flex()
                                        .id(("problem-tree-row", position))
                                        .h(px(ROW_HEIGHT))
                                        .w_full()
                                        .pl(px(TREE_INDENT))
                                        .gap_1()
                                        .text_color(text_color)
                                        .cursor_pointer()
                                        .when(selected, |row| row.bg(colors.element_selected))
                                        .hover(|row| row.bg(colors.element_hover))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.open_row(ix, window, cx);
                                        }))
                                        .child(severity_icon(row.severity))
                                        .child(div().truncate().child(row.message.clone()))
                                        .when_some(source_code, |this, source_code| {
                                            this.child(
                                                div()
                                                    .flex_none()
                                                    .opacity(0.7)
                                                    .child(source_code),
                                            )
                                        })
                                        .child(
                                            div()
                                                .flex_none()
                                                .opacity(0.7)
                                                .child(row.position_label.clone()),
                                        ),
                                );
                            }
                            None => {}
                        }
                    }
                    elements
                }),
            )
            .track_scroll(&self.scroll_handle)
            .flex_1()
            .into_any_element()
        };

        v_flex()
            .id("problems-table")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(colors.editor_background)
            .when(self.view_mode == ViewMode::Table, |this| this.child(header))
            .child(body)
    }
}

impl Focusable for ProblemsTable {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProblemsTable {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: &mut dyn FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _: &App) -> SharedString {
        t!("Problems")
    }
}
