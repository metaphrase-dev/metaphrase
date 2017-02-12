pub mod curly_apostrophe_filter;
pub mod ellipsis_symbol_filter;
pub mod no_space_before_comma_filter;
pub mod space_before_double_ponctuation_filter;

use super::*;

pub use self::curly_apostrophe_filter::CurlyApostropheFilter;
pub use self::ellipsis_symbol_filter::EllipsisSymbolFilter;
pub use self::no_space_before_comma_filter::NoSpaceBeforeCommaFilter;
pub use self::space_before_double_ponctuation_filter::SpaceBeforeDoublePonctuationFilter;

pub trait LinterFilter {
    fn check(&self, text: &str) -> Result<(), Vec<LinterWarning>> {
        use regex::Regex;

        let mut warnings = Vec::<LinterWarning>::new();

        let results = Regex::new(self.regex_pattern()).unwrap();

        for result in results.find_iter(text) {
            warnings.push(
                LinterWarning {
                    message: self.message(),
                    start: result.start(),
                    end: result.end(),
                }
            );
        }

        if warnings.is_empty() {
            Ok(())
        } else {
            Err(warnings)
        }
    }

    fn locales(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn message(&self) -> &'static str;
    fn regex_pattern(&self) -> &'static str;
}
