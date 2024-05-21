use std::fmt::Display;

use iced::Theme;
use regex::{Regex, RegexBuilder};

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

#[derive(Debug, Clone, Default)]
pub struct FilterInput {
    pub input: String,
    pub regex: Option<Result<Regex, (Option<Regex>, String)>>,
    pub use_regex: bool,
    pub case_insensitive: bool,
}

impl FilterInput {
    pub fn update_regex(&mut self) {
        match (self.input.is_empty(), self.use_regex) {
            (false, true) => {
                self.regex = match RegexBuilder::new(self.input.as_str())
                    .case_insensitive(self.case_insensitive)
                    .build()
                {
                    Ok(re) => Some(Ok(re)),
                    Err(err) => {
                        let mut previous_regex = None;
                        std::mem::swap(&mut previous_regex, &mut self.regex);
                        let previous_regex = match previous_regex {
                            Some(Ok(re)) => Some(re),
                            Some(Err((re, _))) => re,
                            _ => None,
                        };
                        let error_message = match err {
                            regex::Error::Syntax(s) => s,
                            regex::Error::CompiledTooBig(_) => "Regex too big".to_owned(),
                            _ => unimplemented!(),
                        };
                        Some(Err((previous_regex, error_message)))
                    }
                }
            }
            _ => self.regex = None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DepthLimit {
    pub is_active: bool,
    pub limit: Option<usize>,
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

drop_down_enum!(SortingOption, NoSorting, SortAscending, SortDescending);
drop_down_enum!(DisplayType, AbsolutePath, RelativePath, JustName);
