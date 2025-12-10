use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CseqTokenizer<'a> {
    pub seq: &'a str,
    pub method: &'a str,
}

impl<'a> Tokenize<'a> for CseqTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::take_until, character::complete::space1, combinator::rest,
            error::Error as NomError, Parser,
        };

        let (_, (seq, _, method)) =
            (take_until::<_, _, NomError<&str>>(" "), space1, rest).parse(part)
                .map_err(|_| Error::tokenizer(("cseq header", part)))?;

        Ok(Self { seq, method })
    }
}
