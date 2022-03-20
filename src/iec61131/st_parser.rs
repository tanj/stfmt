pub use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "iec61131.pest"]
pub struct StParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, parses_to};

    #[test]
    fn test_numeric_16_123() {
        parses_to! {
            parser: StParser,
            input: "16#123",
            rule: Rule::numeric,
            tokens: [numeric(0, 6)]
        };
    }

    #[test]
    fn test_numeric_16_with_underscore() {
        parses_to! {
            parser: StParser,
            input: "16#dead_BEEF",
            rule: Rule::numeric,
            tokens: [numeric(0, 12)]
        };
    }

    #[test]
    fn test_numeric_8() {
        parses_to! {
            parser: StParser,
            input: "8#123",
            rule: Rule::numeric,
            tokens: [numeric(0, 5)]
        };
    }

    #[test]
    fn test_numeric_2() {
        parses_to! {
            parser: StParser,
            input: "2#1010_1010_",
            rule: Rule::numeric,
            tokens: [numeric(0, 12)]
        };
    }

    #[test]
    fn test_numeric_10() {
        parses_to! {
            parser: StParser,
            input: "10#123_456",
            rule: Rule::numeric,
            tokens: [numeric(0, 10)]
        };
    }
}
