use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, none_of, space1},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Bold(Vec<Token>),
    Italic(Vec<Token>),
    InlineCode(Vec<String>),
    Regular(String),
    Link { label: Vec<Token>, url: String },
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_token(s).map(|(_, v)| v).map_err(|e| {
            println!("{}", e);
        })
    }
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    let (input, token) = alt((
        parse_inline_code,
        parse_bold,
        // parse_italic,
        // parse_link,
        parse_regular,
    ))(input)?;

    Ok((input, token))
}

fn parse_bold(input: &str) -> IResult<&str, Token> {
    let (input, words) = delimited(tag("**"), take_until("**"), tag("**"))(input)?;
    let (_, tokens) = parse_tokens_split_with_space(words)?;
    Ok((input, Token::Bold(tokens)))
}

fn parse_regular(input: &str) -> IResult<&str, Token> {
    let (input, word) = parse_word(input)?;
    Ok((input, Token::Regular(word)))
}

fn parse_tokens_split_with_space(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, tokens) = separated_list0(space_or_line_ending, parse_token)(input)?;
    Ok((input, tokens))
}

fn parse_inline_code(input: &str) -> IResult<&str, Token> {
    let (input, words) = delimited(tag("`"), take_until("`"), tag("`"))(input)?;
    let (_, words) = parse_words_split_with_space(words)?;
    Ok((input, Token::InlineCode(words)))
}

fn parse_words_split_with_space(input: &str) -> IResult<&str, Vec<String>> {
    let (input, words) = separated_list0(space_or_line_ending, parse_word)(input)?;
    let words = words.into_iter().filter(|w| !w.is_empty()).collect();
    Ok((input, words))
}

fn space_or_line_ending(input: &str) -> IResult<&str, &str> {
    alt((space1, line_ending))(input)
}

fn parse_word(input: &str) -> IResult<&str, String> {
    let (input, word) = many0(none_of("\n \t \r"))(input)?;
    Ok((input, word.into_iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn parse_inline_code() {
        let token = "`hello world`".parse::<Token>().unwrap();

        assert_eq!(
            token,
            Token::InlineCode(vec!["hello".to_string(), "world".to_string()])
        );
    }

    #[test]
    fn parse_bold() {
        let token = "**hello world**".parse::<Token>().unwrap();

        assert_eq!(
            token,
            Token::Bold(vec![
                Token::Regular("hello".to_string()),
                Token::Regular("world".to_string())
            ])
        );
    }

    #[test]
    fn parse_bold_inline() {
        let token = "**`bold inline code`**".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            Bold(vec![InlineCode(vec![
                "bold".to_string(),
                "inline".to_string(),
                "code".to_string()
            ])])
        );
    }
}
