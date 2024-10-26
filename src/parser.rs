
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


/*

// intra 1
atom!(x => [ (a, b) ] a, b; [ (c, d) ] c, d; [ x if x % 2 == 0 ] => { ret.push(x); });

pattern_list = \[ rust_pattern [if boolean_expr] \] [ident_next_list] ; *

p.or(p)
p.and(p)
sub_list?
call?

pattern_matcher!(name, input type, pattern, return_statement, return type);



fn name<T>(input : &T) -> impl Iterator<Item = ?> {
}


*/