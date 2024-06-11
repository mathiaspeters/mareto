use std::fmt::Display;

use iced::Theme;

#[derive(Debug, Clone)]
pub struct Options {
    pub sorting: DropDownState<SortingOption>,
    pub display_type: DropDownState<DisplayType>,
    pub remove_empty: bool,
    pub preview_changes: bool,
    pub theme: DropDownState<Theme>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            sorting: DropDownState {
                selected: Some(SortingOption::NoSorting),
                options: SortingOption::variants(),
            },
            display_type: DropDownState {
                selected: Some(DisplayType::RelativePath),
                options: DisplayType::variants(),
            },
            remove_empty: false,
            preview_changes: true,
            theme: DropDownState {
                selected: Some(Theme::Light),
                options: Theme::ALL.to_vec(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DropDownState<T> {
    pub selected: Option<T>,
    pub options: Vec<T>,
}

macro_rules! drop_down_enum {
    (
        $name:ident,
        $($variant:ident)
        ,
        *
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            fn variants() -> Vec<Self> {
                vec![$(
                    Self::$variant,
                )*]
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant => write!(f, "{}", stringify!($variant).chars().enumerate().flat_map(|(i, c)| if i > 0 && c.is_ascii_uppercase() { vec![' ', c.to_ascii_lowercase()] } else { vec![c] }).collect::<String>()),
                    )*
                }
            }
        }
    };
}

drop_down_enum!(
    SortingOption,
    NoSorting,
    SortAscendingCaseInsensitive,
    SortAscendingCaseSensitive,
    SortDescendingCaseInsensitive,
    SortDescendingCaseSensitive
);
drop_down_enum!(DisplayType, AbsolutePath, RelativePath, JustName);
