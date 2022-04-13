use peg::parser;

parser!( grammar iec61131() for str {
    rule _() = [' ' | '\t' | '\n' | '\r']*
        rule value_seperator() = _ "," _

        rule multi_line_comment() -> Comment = ml:$("(*" (!"*)" [_])* "*)") { Comment::Multi(ml.to_owned()) }
        rule single_line_comment() -> Comment = sl:$("//" (!"\n" [_])*) { Comment::Single(sl.to_owned()) }
    pub rule comment() -> Comment = multi_line_comment() / single_line_comment()
});

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Comment {
    Multi(String),
    Single(String),
}

#[cfg(test)]
mod test {
    use super::*;

    mod comment {
        use super::*;

        #[test]
        fn test_multi_line_comment() {
            assert_eq!(
                iec61131::comment("(* This is a multi\nline\ncomment *)").unwrap(),
                Comment::Multi("(* This is a multi\nline\ncomment *)".into())
            )
        }

        #[test]
        fn test_single_line_comment() {
            assert_eq!(
                iec61131::comment("// this is a single line comment").unwrap(),
                Comment::Single("// this is a single line comment".into())
            )
        }
    }
}
