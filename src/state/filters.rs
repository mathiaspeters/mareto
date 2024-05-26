use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub filter_input: FilterState<FilterInput>,
    pub min_depth: FilterState<DepthLimit>,
    pub max_depth: FilterState<DepthLimit>,
    pub show_files: FilterState<bool>,
    pub show_folders: FilterState<bool>,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            filter_input: FilterState::new(Default::default()),
            min_depth: FilterState::new(Default::default()),
            max_depth: FilterState::new(Default::default()),
            show_files: FilterState::new(true),
            show_folders: FilterState::new(true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterState<T> {
    pub state: T,
    pub filtered: Vec<bool>,
}

impl<T> FilterState<T> {
    pub fn new(initial: T) -> Self {
        Self {
            state: initial,
            filtered: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterInput {
    pub input: String,
    pub regex: Option<Result<Regex, (Option<Regex>, String)>>,
    pub use_regex: bool,
    pub case_sensitive: bool,
}

impl FilterInput {
    pub fn update_regex(&mut self) {
        match (self.input.is_empty(), self.use_regex) {
            (false, true) => {
                self.regex = match RegexBuilder::new(self.input.as_str())
                    .case_insensitive(!self.case_sensitive)
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
