use super::*;

pub struct NoSpaceBeforeCommaFilter {}

impl LinterFilter for NoSpaceBeforeCommaFilter {
    fn message(&self) -> &'static str {
        "Please don’t use a space before a comma."
    }

    fn regex_pattern(&self) -> &'static str {
        r"\s+,"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_with_no_warnings() {
        let filter = NoSpaceBeforeCommaFilter {};

        let result = filter.check("Lorsqu’on le lui demande, il répond qu’il se nomme Simbad le marin.");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_with_a_warning() {
        let filter = NoSpaceBeforeCommaFilter {};

        let result = filter.check("Lorsqu’on le lui demande , il répond qu’il se nomme Simbad le marin.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please don’t use a space before a comma.", warnings[0].message);
        assert_eq!(26, warnings[0].start);
        assert_eq!(28, warnings[0].end);
    }
}

