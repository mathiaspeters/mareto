use iced::{widget::button, Border, Color, Theme};

pub struct InactiveToggleButton;

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

impl From<InactiveToggleButton> for iced::theme::Button {
    fn from(val: InactiveToggleButton) -> Self {
        iced::theme::Button::Custom(Box::new(val))
    }
}

pub struct ErrorTextColor;

impl From<ErrorTextColor> for iced::theme::Text {
    fn from(_val: ErrorTextColor) -> Self {
        iced::theme::Text::Color(Color::from_rgb(0.9, 0.2, 0.2))
    }
}
