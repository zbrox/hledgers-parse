use nom::{
    bytes::complete::{tag, take_while},
    character::complete::alphanumeric1,
    combinator::opt,
    sequence::terminated,
    IResult,
};

use crate::types::Tag;

use super::utils::is_char_alphanumeric;

pub fn parse_tag(input: &str) -> IResult<&str, Tag> {
    let (tail, name) = terminated(take_while(|c| is_char_alphanumeric(c) || c == '-'), tag(":"))(input)?;
    let (tail, value) = opt(alphanumeric1)(tail)?;

    Ok((
        tail,
        Tag {
            name: name.into(),
            value: value.map(str::to_string),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::{parsers::tags::parse_tag, types::Tag};

    #[test]
    fn test_parse_tag_with_space() {
        assert_eq!(parse_tag("not a tag:"), Err(nom::Err::Error(nom::error::Error { input: " a tag:", code: nom::error::ErrorKind::Tag })))
    }

    #[test]
    fn test_parse_tag_no_value() {
        assert_eq!(
            parse_tag("cash:"),
            Ok((
                "",
                Tag {
                    name: "cash".into(),
                    value: None,
                }
            ))
        )
    }

    #[test]
    fn test_parse_tag_with_value() {
        assert_eq!(
            parse_tag("cash:atm"),
            Ok((
                "",
                Tag {
                    name: "cash".into(),
                    value: Some("atm".into()),
                }
            ))
        )
    }
}
