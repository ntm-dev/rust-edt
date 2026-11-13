use gpui::FontWeight;

use crate::prelude::*;

/// A small, pill-shaped badge that displays a numeric count.
///
/// The count is capped at 99 and displayed as "99+" beyond that.
#[derive(IntoElement, RegisterComponent)]
pub struct CountBadge {
    count: usize,
}

impl CountBadge {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl RenderOnce for CountBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = self.count.to_string();
        let colors = cx.theme().colors();
        let bg = colors.tab_active_foreground;
        let fg = colors.status_bar_background;

        h_flex()
            .absolute()
            .bottom_0()
            .right_0()
            .px(px(4.))
            .h(px(16.))
            .line_height(px(16.))
            .min_w(px(16.))
            .rounded_full()
            .justify_center()
            .text_center()
            .bg(bg)
            .child(
                Label::new(label)
                    .size(LabelSize::Custom(rems_from_px(9_f32)))
                    .weight(FontWeight::NORMAL)
                    .color(Color::Custom(fg)),
            )
    }
}

impl Component for CountBadge {
    fn scope() -> ComponentScope {
        ComponentScope::Status
    }

    fn description() -> &'static str {
        "A small, pill-shaped badge that displays a numeric count."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let container = || {
            div()
                .relative()
                .size_8()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
        };

        v_flex()
            .gap_6()
            .child(example_group_with_title(
                "Count Badge",
                vec![
                    single_example(
                        "Basic Count",
                        container().child(CountBadge::new(3)).into_any_element(),
                    ),
                    single_example(
                        "Capped Count",
                        container().child(CountBadge::new(150)).into_any_element(),
                    ),
                ],
            ))
            .into_any_element()
    }
}
