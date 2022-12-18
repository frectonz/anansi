use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, none_of, space1},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Strong(Vec<Token>),
    Emphasis(Vec<Token>),
    StrongEmphasis(Vec<Token>),
    InlineCode(Vec<String>),
    Regular(String),
    Link { url: Url, label: Vec<Token> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Url {
    Inline { url: String, title: Option<String> },
    Reference(String),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_token(s).map(|(_, v)| v).map_err(|_| ())
    }
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    let (input, token) = alt((
        parse_inline_code,
        parse_strong_emphasis,
        parse_strong,
        parse_emphasis,
        parse_link,
        parse_regular,
    ))(input)?;
    Ok((input, token))
}

fn parse_link(input: &str) -> IResult<&str, Token> {
    let (input, label) = delimited(tag("["), take_until("]"), tag("]"))(input)?;
    let (_, label) = parse_tokens_split_with_space(label)?;

    let (input, url) = if input.starts_with('(') {
        let url_end = alt((tag(" "), tag(")")));
        let take_until_url_end = alt((take_until(" "), take_until(")")));
        let (input, url) = delimited(tag("("), take_until_url_end, url_end)(input)?;
        (
            input,
            Url::Inline {
                url: url.to_string(),
                title: None,
            },
        )
    } else {
        let (input, url) = delimited(alt((tag("["), tag(" ["))), take_until("]"), tag("]"))(input)?;
        (input, Url::Reference(url.to_string()))
    };

    let title = if input.is_empty() {
        None
    } else {
        let (input, _) = tag("\"")(input)?;
        let (_, title) = take_until("\"")(input)?;

        Some(title.to_string())
    };

    let url = match url {
        Url::Inline { url, .. } => Url::Inline { url, title },
        url => url,
    };

    Ok((input, Token::Link { url, label }))
}

fn parse_strong_emphasis(input: &str) -> IResult<&str, Token> {
    let (input, words) = delimited(tag("***"), take_until("***"), tag("***"))(input)?;
    let (_, tokens) = parse_tokens_split_with_space(words)?;
    Ok((input, Token::StrongEmphasis(tokens)))
}

fn parse_strong(input: &str) -> IResult<&str, Token> {
    let (input, words) = delimited(tag("**"), take_until("**"), tag("**"))(input)?;
    let (_, tokens) = parse_tokens_split_with_space(words)?;
    Ok((input, Token::Strong(tokens)))
}

fn parse_emphasis(input: &str) -> IResult<&str, Token> {
    let (input, words) = delimited(tag("*"), take_until("*"), tag("*"))(input)?;
    let (_, tokens) = parse_tokens_split_with_space(words)?;
    Ok((input, Token::Emphasis(tokens)))
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
    use super::{Token, Url};

    #[test]
    fn parse_inline_code() {
        let token = "`hello world`".parse::<Token>().unwrap();

        assert_eq!(
            token,
            Token::InlineCode(vec!["hello".to_string(), "world".to_string()])
        );
    }

    #[test]
    fn parse_strong() {
        let token = "**hello world**".parse::<Token>().unwrap();

        assert_eq!(
            token,
            Token::Strong(vec![
                Token::Regular("hello".to_string()),
                Token::Regular("world".to_string())
            ])
        );
    }

    #[test]
    fn parse_strong_inline() {
        let token = "**`bold inline code`**".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            Strong(vec![InlineCode(vec![
                "bold".to_string(),
                "inline".to_string(),
                "code".to_string()
            ])])
        );
    }

    #[test]
    fn parse_emphasis() {
        let token = "*hello world*".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            Emphasis(vec![
                Regular("hello".to_string()),
                Regular("world".to_string()),
            ])
        );
    }

    #[test]
    fn parse_emphasis_inline() {
        let token = "*`italic inline code`*".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            Emphasis(vec![InlineCode(vec![
                "italic".to_string(),
                "inline".to_string(),
                "code".to_string()
            ])])
        );
    }

    #[test]
    fn parse_strong_emphasis() {
        let token = "***hello world***".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            StrongEmphasis(vec![
                Regular("hello".to_string()),
                Regular("world".to_string()),
            ])
        );
    }

    #[test]
    fn parse_strong_emphasis_inline() {
        let token = "***`bold italic inline code`***".parse::<Token>().unwrap();

        use Token::*;
        assert_eq!(
            token,
            StrongEmphasis(vec![InlineCode(vec![
                "bold".to_string(),
                "italic".to_string(),
                "inline".to_string(),
                "code".to_string()
            ])])
        );
    }

    #[test]
    fn parse_link_with_title() {
        let token = "[hello world](https://example.com \"title\")"
            .parse::<Token>()
            .unwrap();

        use Token::*;
        assert_eq!(
            token,
            Token::Link {
                label: vec![Regular("hello".to_string()), Regular("world".to_string())],
                url: Url::Inline {
                    url: "https://example.com".to_string(),
                    title: "title".to_string().into()
                }
            }
        );
    }

    #[test]
    fn parse_link() {
        let token = "[hello world](https://example.com)"
            .parse::<Token>()
            .unwrap();

        use Token::*;
        assert_eq!(
            token,
            Token::Link {
                label: vec![Regular("hello".to_string()), Regular("world".to_string())],
                url: Url::Inline {
                    url: "https://example.com".to_string(),
                    title: None
                }
            }
        );
    }

    #[test]
    fn parse_link_with_reference() {
        let token1 = "[hello world][ref]".parse::<Token>().unwrap();
        let token2 = "[hello world] [ref]".parse::<Token>().unwrap();

        use Token::*;
        let parsed_token = Token::Link {
            label: vec![Regular("hello".to_string()), Regular("world".to_string())],
            url: Url::Reference("ref".to_string()),
        };

        assert_eq!(token1, parsed_token);
        assert_eq!(token2, parsed_token);
    }

    #[test]
    fn parse_bold_link() {
        let token = "[**hello world**](https://example.com)"
            .parse::<Token>()
            .unwrap();

        use Token::*;
        assert_eq!(
            token,
            Token::Link {
                label: vec![Strong(vec![
                    Regular("hello".to_string()),
                    Regular("world".to_string())
                ])],
                url: Url::Inline {
                    url: "https://example.com".to_string(),
                    title: None
                }
            }
        );
    }
}
