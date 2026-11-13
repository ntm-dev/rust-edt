use gpui::{Entity, Subscription, WeakEntity};
use i18n::t;
use project::Project;
use ui::{PopoverMenu, TintColor, Tooltip, prelude::*};
use workspace::{ItemHandle, StatusItemView, Workspace};

const MAX_BRANCH_NAME_LENGTH: usize = 40;
const MAX_SHORT_SHA_LENGTH: usize = 8;

pub struct GitBranchStatus {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    _git_store_subscription: Subscription,
}

impl GitBranchStatus {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let subscription = cx.subscribe(&git_store, |_, _, _, cx| cx.notify());
        Self {
            workspace: workspace.weak_handle(),
            project,
            _git_store_subscription: subscription,
        }
    }
}

impl Render for GitBranchStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(repository) = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .active_repository()
        else {
            return div().into_any_element();
        };

        let (label, is_detached, is_dirty) = {
            let repo = repository.read(cx);
            let status = repo.status_summary();
            let tracked = status.index + status.worktree;
            let is_dirty = status.conflict > 0
                || tracked.modified > 0
                || tracked.added > 0
                || tracked.deleted > 0
                || status.untracked > 0;
            let name = repo
                .branch
                .as_ref()
                .map(|branch| util::truncate_and_trailoff(branch.name(), MAX_BRANCH_NAME_LENGTH))
                .or_else(|| {
                    repo.head_commit.as_ref().map(|commit| {
                        commit
                            .sha
                            .chars()
                            .take(MAX_SHORT_SHA_LENGTH)
                            .collect::<String>()
                    })
                });
            (name, repo.branch.is_none(), is_dirty)
        };

        let Some(label) = label else {
            return div().into_any_element();
        };
        let tooltip_label = label.clone();
        let display_label = if is_dirty {
            format!("{label}*")
        } else {
            label
        };
        let workspace = self.workspace.clone();

        PopoverMenu::new("status-bar-branch-menu")
            .menu(move |window, cx| {
                git_ui_core::build_branch_picker(
                    workspace.clone(),
                    Some(repository.clone()),
                    window,
                    cx,
                )
            })
            .trigger_with_tooltip(
                Button::new("status-bar-branch-trigger", display_label)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .label_size(LabelSize::Small)
                    .color(Color::Default)
                    .start_icon(
                        Icon::new(IconName::GitBranch)
                            .size(IconSize::Small)
                            .color(Color::Default),
                    ),
                move |_window, cx| {
                    let meta = if is_detached {
                        t!("Detached HEAD: {branch}", branch = tooltip_label)
                    } else {
                        t!("Currently Checked Out: {branch}", branch = tooltip_label)
                    };
                    Tooltip::with_meta(
                        t!("Branch & Stash"),
                        Some(&wu_actions::git::Branch),
                        meta,
                        cx,
                    )
                },
            )
            .anchor(gpui::Anchor::BottomLeft)
            .into_any_element()
    }
}

impl StatusItemView for GitBranchStatus {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}
