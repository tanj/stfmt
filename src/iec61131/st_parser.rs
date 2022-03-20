pub use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "iec61131.pest"]
pub struct StParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, parses_to};

    mod numeric_rule {
        use super::*;

        #[test]
        fn test_hex_123() {
            parses_to! {
                parser: StParser,
                input: "16#123",
                rule: Rule::numeric,
                tokens: [numeric(0, 6)]
            };
        }

        #[test]
        fn test_hex_with_underscore() {
            parses_to! {
                parser: StParser,
                input: "16#dead_BEEF",
                rule: Rule::numeric,
                tokens: [numeric(0, 12)]
            };
        }

        #[test]
        fn test_octaly() {
            parses_to! {
                parser: StParser,
                input: "8#123",
                rule: Rule::numeric,
                tokens: [numeric(0, 5)]
            };
        }

        #[test]
        fn test_binary() {
            parses_to! {
                parser: StParser,
                input: "2#1010_1010_",
                rule: Rule::numeric,
                tokens: [numeric(0, 12)]
            };
        }

        #[test]
        fn test_decimal() {
            parses_to! {
                parser: StParser,
                input: "10#123_456",
                rule: Rule::numeric,
                tokens: [numeric(0, 10)]
            };
        }
    }

    mod string_rule {
        use super::*;

        #[test]
        fn test_no_escapes() {
            parses_to! {
                parser: StParser,
                input: "'Hello, world!'",
                rule: Rule::string,
                tokens: [string(0, 15)]
            };
        }

        #[test]
        fn test_escapes() {
            parses_to! {
                parser: StParser,
                input: "'$0a$0A$n$N$r$R$l$L$p$P$t$T$$$''",
                rule: Rule::string,
                tokens: [string(0, 32)]
            };
        }
    }
}
