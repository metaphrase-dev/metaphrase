use super::*;

pub struct CurlyApostropheFilter {}

impl LinterFilter for CurlyApostropheFilter {
    fn message(&self) -> &'static str {
        "Please use curly apostrophes."
    }

    fn regex_pattern(&self) -> &'static str {
        r"[']"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curly_apostrophe_filter_with_straight_apostrophe() {
        let filter = CurlyApostropheFilter {};

        let result = filter.check("It's me, Mario!");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use curly apostrophes.", warnings[0].message);
        assert_eq!(2, warnings[0].start);
        assert_eq!(3, warnings[0].end);
    }

    #[test]
    fn test_curly_apostrophe_filter_with_curly_apostrophe() {
        let filter = CurlyApostropheFilter {};

        let result = filter.check("It’s me, Mario!");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}

