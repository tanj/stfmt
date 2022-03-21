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

    mod wstring_rule {
        use super::*;

        #[test]
        fn test_no_escapes() {
            parses_to! {
                parser: StParser,
                input: "\"Hello, world!\"",
                rule: Rule::w_string,
                tokens: [w_string(0, 15)]
            };
        }

        #[test]
        fn test_escapes() {
            parses_to! {
                parser: StParser,
                input: "\"$0a$0A$n$N$r$R$l$L$p$P$t$T$$$\"\"",
                rule: Rule::w_string,
                tokens: [w_string(0, 32)]
            };
        }
    }

    mod number_rule {
        use super::*;

        #[test]
        fn test_int_neg() {
            parses_to! {
                parser: StParser,
                input: "-123",
                rule: Rule::number,
                tokens: [number(0, 4)]
            };
        }

        #[test]
        fn test_int_pos_with_underscore() {
            parses_to! {
                parser: StParser,
                input: "123_456",
                rule: Rule::number,
                tokens: [number(0, 7)]
            };
        }

        #[test]
        fn test_int_zero() {
            parses_to! {
                parser: StParser,
                input: "0",
                rule: Rule::number,
                tokens: [number(0, 1)]
            };
        }

        #[test]
        fn test_float_neg() {
            parses_to! {
                parser: StParser,
                input: "-123.456",
                rule: Rule::number,
                tokens: [number(0, 8)]
            };
        }

        #[test]
        fn test_float_pos_e() {
            parses_to! {
                parser: StParser,
                input: "123.456e+3",
                rule: Rule::number,
                tokens: [number(0, 10)]
            };
        }

        #[test]
        fn test_float_e_no_sign() {
            parses_to! {
                parser: StParser,
                input: "-123.456e09",
                rule: Rule::number,
                tokens: [number(0, 11)]
            };
        }
    }

    mod various_times {
        use super::*;

        #[test]
        fn test_ltime_all_divs() {
            parses_to! {
                parser: StParser,
                input: "LTIME#1000d15h23m12s34ms2us44ns",
                rule: Rule::time,
                tokens: [time(0, 31)]
            };
        }

        #[test]
        fn test_t() {
            parses_to! {
                parser: StParser,
                input: "T#5S",
                rule: Rule::time,
                tokens: [time(0, 4)]
            };
        }

        #[test]
        fn test_time() {
            parses_to! {
                parser: StParser,
                input: "time#5S",
                rule: Rule::time,
                tokens: [time(0, 7)]
            };
        }

        #[test]
        fn test_date_and_time() {
            parses_to! {
                parser: StParser,
                input: "date_and_time#2022-03-1-14:5:03.123",
                rule: Rule::datetime,
                tokens: [datetime(0, 35)]
            };
        }

        #[test]
        fn test_dt() {
            parses_to! {
                parser: StParser,
                input: "DT#2022-03-22-20:21:22.123456",
                rule: Rule::datetime,
                tokens: [datetime(0, 29)]
            };
        }

        #[test]
        fn test_ldt() {
            parses_to! {
                parser: StParser,
                input: "LDT#2022-03-22-20:21:22.123456",
                rule: Rule::datetime,
                tokens: [datetime(0, 30)]
            };
        }

        #[test]
        fn test_ldate_and_time() {
            parses_to! {
                parser: StParser,
                input: "LDATE_AND_time#2022-03-22-20:21:22.123456",
                rule: Rule::datetime,
                tokens: [datetime(0, 41)]
            };
        }

        #[test]
        fn test_tod() {
            parses_to! {
                parser: StParser,
                input: "tod#20:21:22.123456",
                rule: Rule::tod,
                tokens: [tod(0, 19)]
            };
        }

        #[test]
        fn test_ltod() {
            parses_to! {
                parser: StParser,
                input: "Ltod#20:21:22.123456",
                rule: Rule::tod,
                tokens: [tod(0, 20)]
            };
        }

        #[test]
        fn test_time_of_day() {
            parses_to! {
                parser: StParser,
                input: "time_OF_day#20:21:22.123456",
                rule: Rule::tod,
                tokens: [tod(0, 27)]
            };
        }

        #[test]
        fn test_ltime_of_day() {
            parses_to! {
                parser: StParser,
                input: "LTIME_of_day#20:21:22.123456",
                rule: Rule::tod,
                tokens: [tod(0, 28)]
            };
        }
    }

    mod comment_rule {
        use super::*;

        #[test]
        fn test_comment_single_line_comment() {
            parses_to! {
                parser: StParser,
                input: "// this is a comment",
                rule: Rule::COMMENT,
                tokens: [COMMENT(0, 20, [single_line_comment(0, 20)])]
            };
        }

        #[test]
        fn test_comment_multi_line_comment() {
            parses_to! {
                parser: StParser,
                input: "(* this is a comment * ) \nthat continues on another line *)",
                rule: Rule::COMMENT,
                tokens: [COMMENT(0, 59, [multi_line_comment(0, 59)])]
            };
        }

        #[test]
        fn test_multi_line_comment() {
            parses_to! {
                parser: StParser,
                input: "(* this is a comment * ) \nthat continues on another line *)",
                rule: Rule::multi_line_comment,
                tokens: [multi_line_comment(0, 59)]
            };
        }

        #[test]
        fn test_single_line_comment() {
            parses_to! {
                parser: StParser,
                input: "// this is a comment",
                rule: Rule::single_line_comment,
                tokens: [single_line_comment(0, 20)]
            };
        }

        #[test]
        fn test_comment_in_index() {
            parses_to! {
                parser: StParser,
                input: "[1 (* comment *)]",
                rule: Rule::index,
                tokens: [index(0, 17,[number(1, 2), COMMENT(3, 16, [multi_line_comment(3, 16)])])]
            };
        }
    }
}
