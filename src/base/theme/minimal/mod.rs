use crate::prelude::*;

pub struct MinimalTheme;

impl Theme for MinimalTheme {
    fn name(&self) -> &'static str {
        "minimal"
    }

    fn fullname(&self) -> String {
        "Minimal".to_string()
    }
}
