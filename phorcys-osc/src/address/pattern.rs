use crate::error::{Error, Result};

use combine::{
    attempt, between, choice, many, many1, one_of, optional, parser::char::char, sep_by,
    EasyParser, ParseError, Parser, Stream,
};
use regex::{escape as regex_escape, Regex};

/// Characters available in OSC method name.
pub const METHOD_CHARS: &str =
    r#"!"$%&'()+-.0123456789:;<=>@ABCDEFGHIJKLMNOPQRSTUVWXYZ\^_`abcdefghijklmnopqrstuvwxyz|~"#;

pub enum AddressPattern {}

impl AddressPattern {
    /// Parses an address pattern string.
    /// If succeeded, pattern expression tree will be returned.
    pub fn parse(pattern: &str) -> Result<Vec<Vec<Expression>>> {
        let expression_tree = match pattern_address().easy_parse(pattern) {
            Ok((exps, "")) => exps,
            Ok((_, left)) => return Err(Error::InvalidPattern(format!("left: {}", left))),
            Err(e) => return Err(Error::InvalidPattern(e.to_string())),
        };

        Ok(expression_tree)
    }

    /// Parses an address pattern string and compiles into `regex::Regex`.
    ///
    /// **Important**: Returned regex will not check about validity of "any character".
    /// For example, `/foo,` is invalid address, but the regex compiled from `/foo?` will accept it.
    /// You should check address validity before matching it!
    pub fn compile_to_regex(pattern: &str) -> Result<Regex> {
        let expression_tree = AddressPattern::parse(pattern)?;

        let mut regex_string = String::new();
        regex_string.push('^');
        for method_part in expression_tree {
            regex_string.push('/');
            for expression in method_part {
                expression.push_regex_part(&mut regex_string);
            }
        }
        regex_string.push('$');

        Regex::new(&regex_string).map_err(|e| Error::InvalidPattern(e.to_string()))
    }
}

/// Expression a part in OSC Address Pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expression {
    /// Literal `ABC`.
    Literal(String),

    /// Selection of literals `{ABC,DEF}`.
    Literals(Vec<String>),

    /// Selection of characters `[a-c]`.
    Chars(bool, Vec<(char, char)>),

    /// Any character `?`.
    AnyChar,

    /// Any string `*`.
    AnyString,
}

impl Expression {
    /// Pushes this expression part as Regex into `String`.
    pub fn push_regex_part(&self, regex_string: &mut String) {
        match self {
            Expression::Literal(s) => regex_string.push_str(&regex_escape(s)),
            Expression::Literals(s) => {
                regex_string.push_str("(?:");
                let pat = s
                    .iter()
                    .map(|l| regex_escape(l))
                    .collect::<Vec<_>>()
                    .join("|");
                regex_string.push_str(&pat);
                regex_string.push(')');
            }
            Expression::Chars(inv, pairs) => {
                regex_string.push('[');
                if *inv {
                    regex_string.push('^');
                }
                for (start, end) in pairs {
                    regex_string.push_str(&regex_escape(&start.to_string()));
                    if start != end {
                        regex_string.push('-');
                        regex_string.push_str(&regex_escape(&end.to_string()));
                    }
                }
                regex_string.push(']');
            }
            Expression::AnyChar => regex_string.push('.'),
            Expression::AnyString => regex_string.push_str("(?:.*)"),
        }
    }
}

/// Parses a valid OSC method character.
fn method_char<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    one_of(METHOD_CHARS.chars())
}

/// Parses literal.
fn pattern_literal<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(method_char()).map(Expression::Literal)
}

/// Parses character selection pattern like `[ABC0-9]`.
fn pattern_select_char<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let char_just = method_char().map(|c| (c, c));
    let char_range = (method_char(), char('-'), method_char()).map(|(start, _, end)| (start, end));
    let pair = (
        char('['),
        optional(char('!')),
        many(choice!(attempt(char_range), attempt(char_just))),
        char(']'),
    );
    pair.map(|(_, bang, ranges, _)| Expression::Chars(bang.is_some(), ranges))
}

/// Parses literal selection pattern like `{abc,def}`.
fn pattern_select_string<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        char('{'),
        char('}'),
        sep_by(many1(method_char()), char(',')),
    )
    .map(Expression::Literals)
}

/// Parses ?.
fn pattern_any_char<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char('?').map(|_| Expression::AnyChar)
}

/// Parses *.
fn pattern_any_string<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char('*').map(|_| Expression::AnyString)
}

/// Parses the whole address pattern.
fn pattern_address<Input>() -> impl Parser<Input, Output = Vec<Vec<Expression>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let method_part = (
        char('/'),
        many1(choice!(
            attempt(pattern_literal()),
            attempt(pattern_select_char()),
            attempt(pattern_select_string()),
            attempt(pattern_any_char()),
            attempt(pattern_any_string())
        )),
    );
    let method_part = method_part.map(|(_, exps)| exps);
    many1(method_part)
}

#[cfg(test)]
mod test {
    use super::{
        pattern_any_char, pattern_any_string, pattern_literal, pattern_select_char,
        pattern_select_string, AddressPattern, Expression,
    };

    use combine::Parser;
    use regex::Regex;

    #[test]
    fn test_literal() {
        assert_eq!(
            pattern_literal().parse("abcabc"),
            Ok((Expression::Literal("abcabc".into()), ""))
        );
        assert_eq!(
            pattern_literal().parse("abc-_abc"),
            Ok((Expression::Literal("abc-_abc".into()), ""))
        );
    }

    #[test]
    fn test_literals() {
        assert_eq!(
            pattern_select_string().parse("{abc}"),
            Ok((Expression::Literals(vec!["abc".into()]), ""))
        );
        assert_eq!(
            pattern_select_string().parse("{abc,def}"),
            Ok((Expression::Literals(vec!["abc".into(), "def".into()]), ""))
        );
    }

    #[test]
    fn test_select_char() {
        assert_eq!(
            pattern_select_char().parse("[ABC]"),
            Ok((
                Expression::Chars(false, vec![('A', 'A'), ('B', 'B'), ('C', 'C')]),
                ""
            ))
        );
        assert_eq!(
            pattern_select_char().parse("[aA-Cabc-]"),
            Ok((
                Expression::Chars(
                    false,
                    vec![
                        ('a', 'a'),
                        ('A', 'C'),
                        ('a', 'a'),
                        ('b', 'b'),
                        ('c', 'c'),
                        ('-', '-')
                    ],
                ),
                ""
            ))
        );
        assert_eq!(
            pattern_select_char().parse("[!A-X]"),
            Ok((Expression::Chars(true, vec![('A', 'X')]), ""))
        );
        assert_eq!(
            pattern_select_char().parse("[!A-Xa-z-]"),
            Ok((
                Expression::Chars(true, vec![('A', 'X'), ('a', 'z'), ('-', '-')]),
                ""
            ))
        );
    }

    #[test]
    fn test_any_char() {
        assert_eq!(pattern_any_char().parse("?"), Ok((Expression::AnyChar, "")));
    }

    #[test]
    fn test_any_string() {
        assert_eq!(
            pattern_any_string().parse("*"),
            Ok((Expression::AnyString, ""))
        );
    }

    #[test]
    fn test_parse() {
        let parsed = AddressPattern::parse("/a").expect("Should be success");
        assert_eq!(parsed[0], vec![Expression::Literal("a".into())]);

        let parsed = AddressPattern::parse("/foo/*").expect("Should be success");
        assert_eq!(parsed[0], vec![Expression::Literal("foo".into())]);
        assert_eq!(parsed[1], vec![Expression::AnyString]);

        let parsed =
            AddressPattern::parse("/foo*/com[A-Za-z]{ine,ination}??").expect("Should be success");
        assert_eq!(
            parsed[0],
            vec![Expression::Literal("foo".into()), Expression::AnyString]
        );
        assert_eq!(
            parsed[1],
            vec![
                Expression::Literal("com".into()),
                Expression::Chars(false, vec![('A', 'Z'), ('a', 'z')]),
                Expression::Literals(vec!["ine".into(), "ination".into()]),
                Expression::AnyChar,
                Expression::AnyChar,
            ]
        );
    }

    #[test]
    fn test_regex_compilation() {
        let parsed = AddressPattern::compile_to_regex("/aaaa").expect("Should be success");
        let expected = Regex::new(r#"^/aaaa$"#).expect("Should be success");
        assert_eq!(parsed.to_string(), expected.to_string());

        let parsed = AddressPattern::compile_to_regex("/foo*/com[A-Za-z]{ine,ination}??")
            .expect("Should be success");
        let expected =
            Regex::new(r#"^/foo(?:.*)/com[A-Za-z](?:ine|ination)..$"#).expect("Should be success");
        assert_eq!(parsed.to_string(), expected.to_string());
    }
}
