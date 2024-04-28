use std::fmt::Display;

use iced::{widget::combo_box, Theme};

#[derive(Debug, Clone)]
pub struct Options {
    pub filter_input: FilterInput,
    pub min_depth: DepthLimit,
    pub max_depth: DepthLimit,
    pub show_files: bool,
    pub show_folders: bool,
    pub sorting: DropDownState<SortingOption>,
    pub display_type: DropDownState<DisplayType>,
    pub remove_empty: bool,
    pub preview_changes: bool,
    pub theme: DropDownState<Theme>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            filter_input: Default::default(),
            min_depth: Default::default(),
            max_depth: Default::default(),
            show_files: true,
            show_folders: true,
            sorting: DropDownState {
                selected: Some(SortingOption::Ascending),
                options: combo_box::State::new(SortingOption::variants()),
            },
            display_type: DropDownState {
                selected: Some(DisplayType::RelativePath),
                options: combo_box::State::new(DisplayType::variants()),
            },
            remove_empty: false,
            preview_changes: true,
            theme: DropDownState {
                selected: Some(Theme::Light),
                options: combo_box::State::new(vec![Theme::Light, Theme::Dark]),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterInput {
    pub input: String,
    pub use_regex: bool,
    pub case_insensitive: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DepthLimit {
    pub is_active: bool,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct DropDownState<T> {
    pub selected: Option<T>,
    pub options: combo_box::State<T>,
}

macro_rules! drop_down_enum {
    (
        $name:ident,
        $($variant:ident)
        ,
        *
    ) => {
        #[derive(Debug, Copy, Clone)]
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

drop_down_enum!(SortingOption, Ascending, Descending);
drop_down_enum!(DisplayType, AbsolutePath, RelativePath, JustName);
