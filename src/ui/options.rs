use crate::{
    mareto::Message,
    state::{DepthLimit, FilterOptions, Options},
};
use iced::{
    widget::{
        column, container, pick_list, row, rule::Rule, scrollable, text, text_input, toggler,
    },
    Element, Length, Padding,
};

use super::{components::toggle_button, themes::ErrorTextColor};

pub fn options<'a>(
    options: &'a Options,
    filter_options: &'a FilterOptions,
) -> Element<'a, Message> {
    let regex_error_text = match &filter_options.filter_input.state.regex {
        Some(Err((_, text))) => text,
        _ => "",
    };
    container(scrollable(
        column![
            column![
                row![
                    text_input("Filter input", &filter_options.filter_input.state.input)
                        .on_input(Message::FilterUpdated)
                        .padding(12)
                        .width(Length::Fill),
                    toggle_button(
                        "Aa",
                        filter_options.filter_input.state.case_sensitive,
                        Message::FilterCaseSensitivityToggled,
                    ),
                    toggle_button(
                        ".*",
                        filter_options.filter_input.state.use_regex,
                        Message::FilterRegexToggled,
                    ),
                ],
                text(regex_error_text).style(ErrorTextColor),
            ],
            Rule::horizontal(1),
            depth_control(
                "Limit minimum depth".to_owned(),
                &filter_options.min_depth.state,
                Message::MinDepthToggled,
                Message::MinDepthLimitChanged
            ),
            depth_control(
                "Limit maximum depth".to_owned(),
                &filter_options.max_depth.state,
                Message::MaxDepthToggled,
                Message::MaxDepthLimitChanged
            ),
            Rule::horizontal(1),
            toggler(
                Some("Show files".to_owned()),
                filter_options.show_files.state,
                Message::ShowFilesToggled
            ),
            toggler(
                Some("Show folders".to_owned()),
                filter_options.show_folders.state,
                Message::ShowFoldersToggled
            ),
            Rule::horizontal(1),
            pick_list(
                &options.sorting.options[..],
                options.sorting.selected,
                Message::SortOrderSelected
            )
            .width(Length::Fill)
            .padding(12),
            pick_list(
                &options.display_type.options[..],
                options.display_type.selected,
                Message::DisplayTypeSelected
            )
            .width(Length::Fill)
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
            pick_list(
                &options.theme.options[..],
                options.theme.selected.as_ref(),
                Message::ThemeSelected
            )
            .width(Length::Fill)
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
