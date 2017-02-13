use super::*;

pub struct SpaceBeforeDoublePonctuationFilter {}

impl LinterFilter for SpaceBeforeDoublePonctuationFilter {
    fn locales(&self) -> Vec<&'static str> {
        vec!["fr"]
    }

    fn message(&self) -> &'static str {
        "Please use a non-breaking space before “double” ponctuation marks: `;`, `:`, `!`, `?`."
    }

    fn regex_pattern(&self) -> &'static str {
        r"[\w ][;:!?]"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_with_no_warnings() {
        let filter = SpaceBeforeDoublePonctuationFilter {};

        let result = filter.check("Ah ! Non ! C’est un peu court, jeune homme !");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_with_two_warnings() {
        let filter = SpaceBeforeDoublePonctuationFilter {};

        let result = filter.check("Ah! Non! C’est un peu court, jeune homme !");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(2, warnings.len());

        assert_eq!(
            "Please use a non-breaking space before “double” ponctuation marks: `;`, `:`, `!`, `?`.",
            warnings[0].message
        );
        assert_eq!(1, warnings[0].start);
        assert_eq!(3, warnings[0].end);

        assert_eq!(
            "Please use a non-breaking space before “double” ponctuation marks: `;`, `:`, `!`, `?`.",
            warnings[1].message
        );
        assert_eq!(6, warnings[1].start);
        assert_eq!(8, warnings[1].end);
    }

    #[test]
    fn test_filter_with_a_missing_non_breaking_space() {
        let filter = SpaceBeforeDoublePonctuationFilter {};

        // The space before the last `!` is not a non-breaking space
        let result = filter.check("Ah ! Non ! C’est un peu court, jeune homme !");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!(
            "Please use a non-breaking space before “double” ponctuation marks: `;`, `:`, `!`, `?`.",
            warnings[0].message
        );
        assert_eq!(46, warnings[0].start);
        assert_eq!(48, warnings[0].end);
    }
}

