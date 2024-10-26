
use super::PMAst;

use proc_macro::*;

pub (crate) struct Error(pub (crate) Span, pub (crate) Vec<Box<str>>);
pub (crate) type PResult<'a, T> = Result<(&'a [TokenTree], T, Span), Error>;


pub (crate) fn parse_pattern_matcher(input : &[TokenTree], prev : Span) -> Result<PMAst, Error> {
    let _ = parse_colon(input, prev)?;
    Ok(PMAst::Remove)
}

fn parse_colon<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, ()> {
    match input {
        [TokenTree::Punct(p), rest @ ..] if p.as_char() == ':' => Ok((rest, (), p.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected ':'".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}


//        [t @ TokenTree::Punct(p), rest @ ..] if p.as_char() == ';' => Ok((t, Input::new(rest, p.span()))),