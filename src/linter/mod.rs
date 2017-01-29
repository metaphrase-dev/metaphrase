extern crate regex;

mod filters;

use errors::*;
use linter::filters::*;

#[derive(Debug, RustcEncodable)]
pub struct LinterWarning {
    pub message: &'static str,
    pub start: usize,
    pub end: usize,
}

pub struct Linter {
    locale: String,
}

impl Linter {
    pub fn new(locale: String) -> Result<Linter, LughError> {
        let linter = Linter { locale: locale };

        Ok(linter)
    }

    pub fn check(&self, text: &str) -> Result<(), Vec<LinterWarning>> {
        let mut warnings = Vec::<LinterWarning>::new();

        let filters: Vec<Box<LinterFilter>> = vec![
            Box::new(CurlyApostropheFilter {}),
            Box::new(EllipsisSymbolFilter {}),
        ];

        let active_filters = filters
            .into_iter()
            .filter( |filter| filter.locales().is_empty() ||
                filter.locales().contains(&self.locale.as_str()) )
            .collect::<Vec<Box<LinterFilter>>>();

        for filter in &active_filters {
            let result = filter.check(text);

            if result.is_err() {
                warnings.append(&mut result.err().unwrap());
            }
        }

        if warnings.len() > 0 {
            Err(warnings)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_with_straight_apostrophe_and_three_dots() {
        let linter = Linter::new("fr".to_string()).unwrap();

        let result = linter.check("It's me...");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(2, warnings.len());

        assert_eq!("Please use curly apostrophes.", warnings[0].message);
        assert_eq!(2, warnings[0].start);
        assert_eq!(3, warnings[0].end);

        assert_eq!("Please use the ellipsis symbol (`…`) instead of three dots (`...`).", warnings[1].message);
        assert_eq!(7, warnings[1].start);
        assert_eq!(10, warnings[1].end);
    }

    #[test]
    fn test_linter_with_curly_apostrophe_and_ellipsis_symbol() {
        let linter = Linter::new("fr".to_string()).unwrap();

        let result = linter.check("It’s me…");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}
