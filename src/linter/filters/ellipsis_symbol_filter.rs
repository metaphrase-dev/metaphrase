use super::*;

pub struct EllipsisSymbolFilter {}

impl LinterFilter for EllipsisSymbolFilter {
    fn message(&self) -> &'static str {
        "Please use the ellipsis symbol (`…`) instead of three dots (`...`)."
    }

    fn regex_pattern(&self) -> &'static str {
        r"[.]{3}"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsis_symbol_filter_with_three_dots() {
        let filter = EllipsisSymbolFilter {};

        let result = filter.check("You know...");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use the ellipsis symbol (`…`) instead of three dots (`...`).", warnings[0].message);
        assert_eq!(8, warnings[0].start);
        assert_eq!(11, warnings[0].end);
    }

    #[test]
    fn test_ellipsis_symbol_filter_with_success() {
        let filter = EllipsisSymbolFilter {};

        let result = filter.check("You know…");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}

