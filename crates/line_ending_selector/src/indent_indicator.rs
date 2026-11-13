use editor::Editor;
use gpui::{Action as _, App, Entity, FocusHandle, Focusable, SharedString, Subscription};
use i18n::t;
use language::language_settings::LanguageSettings;
use ui::{ContextMenu, PopoverMenu, Tooltip, prelude::*};
use workspace::{StatusItemView, item::ItemHandle};

#[derive(Default)]
pub struct IndentIndicator {
    label: Option<SharedString>,
    focus_handle: Option<FocusHandle>,
    _observe_active_editor: Option<Subscription>,
}

impl IndentIndicator {
    fn update(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.focus_handle = Some(editor.focus_handle(cx));
        self.label = editor.read(cx).active_buffer(cx).map(|buffer| {
            let settings = LanguageSettings::for_buffer(buffer.read(cx), cx);
            let size = settings.tab_size.get();
            if settings.hard_tabs {
                t!("Tab Size: {size}", size = size)
            } else {
                t!("Spaces: {size}", size = size)
            }
        });
        cx.notify();
    }
}

impl Render for IndentIndicator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        div().when_some(self.label.clone(), |el, label| {
            el.child(
                PopoverMenu::new("indent-indicator-menu")
                    .anchor(gpui::Anchor::BottomRight)
                    .trigger_with_tooltip(
                        Button::new("indent-indicator", label)
                            .label_size(LabelSize::Small)
                            .tab_index(0isize),
                        Tooltip::text(t!("Select Indentation")),
                    )
                    .menu(move |window, cx| {
                        let focus_handle = focus_handle.clone();
                        Some(ContextMenu::build(window, cx, move |menu, _, _| {
                            let menu = match focus_handle {
                                Some(focus_handle) => menu.context(focus_handle),
                                None => menu,
                            };
                            menu.action(
                                t!("Convert Indentation to Spaces"),
                                editor::actions::ConvertIndentationToSpaces.boxed_clone(),
                            )
                            .action(
                                t!("Convert Indentation to Tabs"),
                                editor::actions::ConvertIndentationToTabs.boxed_clone(),
                            )
                        }))
                    }),
            )
        })
    }
}

impl StatusItemView for IndentIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe_in(&editor, window, Self::update));
            self.update(editor, window, cx);
        } else {
            self.label = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}
