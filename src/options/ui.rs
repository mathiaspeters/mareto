use crate::mareto::Message;
use iced::{
    widget::{
        button, column, combo_box, container, row, rule::Rule, scrollable, text, text_input,
        toggler,
    },
    Border, Element, Length, Padding, Theme,
};

use super::{data::Options, DepthLimit};

pub fn options_view(options: &Options) -> Element<'_, Message> {
    container(scrollable(
        column![
            row![
                text_input("Filter input", &options.filter_input.input)
                    .on_input(Message::FilterUpdated)
                    .padding(12)
                    .width(Length::Fill),
                toggle_button(
                    "Aa",
                    options.filter_input.case_insensitive,
                    Message::FilterCaseInsensitiveToggled,
                ),
                toggle_button(
                    ".*",
                    !options.filter_input.use_regex,
                    Message::FilterRegexToggled,
                ),
            ],
            Rule::horizontal(1),
            depth_control(
                "Limit minimum depth".to_owned(),
                &options.min_depth,
                Message::MinDepthToggled,
                Message::MinDepthLimitChanged
            ),
            depth_control(
                "Limit maximum depth".to_owned(),
                &options.max_depth,
                Message::MaxDepthToggled,
                Message::MaxDepthLimitChanged
            ),
            Rule::horizontal(1),
            toggler(
                Some("Show files".to_owned()),
                options.show_files,
                Message::ShowFilesToggled
            ),
            toggler(
                Some("Show folders".to_owned()),
                options.show_folders,
                Message::ShowFoldersToggled
            ),
            Rule::horizontal(1),
            combo_box(
                &options.sorting.options,
                "Sort order",
                options.sorting.selected.as_ref(),
                Message::SortOrderSelected
            )
            .padding(12),
            combo_box(
                &options.display_type.options,
                "Display type",
                options.display_type.selected.as_ref(),
                Message::DisplayTypeSelected
            )
            .padding(12),
            Rule::horizontal(1),
            toggler(
                Some("Remove empty folders".to_owned()),
                options.remove_empty,
                Message::RemoveFoldersToggled
            ),
            toggler(
                Some("Preview changes".to_owned()),
                options.preview_changes,
                Message::PreviewChangesToggled
            ),
            Rule::horizontal(1),
            combo_box(
                &options.theme.options,
                "App theme",
                options.theme.selected.as_ref(),
                Message::ThemeSelected
            )
            .padding(12),
        ]
        .spacing(12)
        .padding(Padding::from([0, 20, 12, 0])),
    ))
    .into()
}

fn depth_control<'a, F1, F2>(
    label: String,
    state: &DepthLimit,
    on_toggled: F1,
    on_input: F2,
) -> Element<'a, Message>
where
    F1: 'a + Fn(bool) -> Message,
    F2: 'a + Fn(String) -> Message,
{
    column![
        row![
            text(&label).width(Length::Fill),
            toggler(None, state.is_active, on_toggled).width(Length::Shrink)
        ]
        .width(Length::Fill),
        text_input(
            &label,
            &state.limit.map(|l| l.to_string()).unwrap_or("".to_owned())
        )
        .on_input(on_input)
        .width(Length::Fill)
        .padding(12)
    ]
    .spacing(8)
    .into()
}

fn toggle_button<'a>(
    label: &'a str,
    active: bool,
    on_press: Message,
) -> iced::widget::Button<'a, Message> {
    let mut button = button(label).padding(12).on_press(on_press);
    if !active {
        button = button.style(InactiveToggleButton);
    }
    button
}

struct InactiveToggleButton;

impl button::StyleSheet for InactiveToggleButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let mut border_color = style.palette().text;
        border_color.a = 0.2;
        button::Appearance {
            background: Some(iced::Background::Color(style.palette().background)),
            text_color: style.palette().text,
            border: Border {
                color: border_color,
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Button> for InactiveToggleButton {
    fn into(self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(self))
    }
}
