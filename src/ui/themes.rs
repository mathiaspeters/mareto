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

impl Into<iced::theme::Button> for InactiveToggleButton {
    fn into(self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(self))
    }
}

pub struct ErrorTextColor;

impl Into<iced::theme::Text> for ErrorTextColor {
    fn into(self) -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgb(0.9, 0.2, 0.2))
    }
}
