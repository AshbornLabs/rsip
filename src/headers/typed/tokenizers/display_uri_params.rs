use crate::{common::uri, headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DisplayUriParamsTokenizer<'a> {
    pub display_name: Option<&'a str>,
    pub uri: uri::Tokenizer<'a, &'a str, char>,
    pub params: Vec<uri::param::Tokenizer<'a, &'a str, char>>,
}

impl<'a> Tokenize<'a> for DisplayUriParamsTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use crate::parser_utils::is_empty_or_fail_with;
        use nom::{
            bytes::complete::{tag, take_until},
            combinator::rest,
            error::Error as NomError,
            multi::many0,
            Parser,
        };

        if part.contains('<') {
            let (_, (display_name, _, uri, _, params)) = (
                take_until::<_, _, NomError<&str>>("<"),
                tag("<"),
                take_until(">"),
                tag(">"),
                rest,
            ).parse(part)
            .map_err(|_| Error::tokenizer(("header parts", part)))?;

            let (rem, params) = many0(uri::param::Tokenizer::tokenize).parse(params)
                .map_err(|_| Error::tokenizer(("params", part)))?;
            is_empty_or_fail_with(rem, ("params tokenizing left trailing input", part))?;

            Ok(Self {
                display_name: crate::utils::opt_trim(display_name),
                uri: uri::Tokenizer::tokenize(uri)
                    .map_err(|_| Error::tokenizer(("URI in addr-spec", part)))?
                    .1,
                params,
            })
        } else {
            let (_, (uri, params)) = (
                uri::Tokenizer::tokenize_without_params,
                many0(uri::param::Tokenizer::tokenize),
            ).parse(part)?;

            Ok(Self {
                display_name: None,
                uri,
                params,
            })
        }
    }
}
