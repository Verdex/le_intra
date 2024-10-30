
use super::PMAst;

use proc_macro::*;

pub (crate) struct Error(pub (crate) Span, pub (crate) Vec<Box<str>>);
pub (crate) type PResult<'a, T> = Result<(&'a [TokenTree], T, Span), Error>;


pub (crate) fn parse_pattern_matcher(input : &[TokenTree], prev : Span) -> Result<PMAst, Error> {
    let next = input;

    let (next, func_name, prev) = parse_return_bracket(next, prev)?;
    let (next, _, prev) = parse_comma(next, prev)?;


    Ok(PMAst { func_name })
}

fn parse_ident<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, &'a TokenTree> {
    match input {
        [t @ TokenTree::Ident(_), rest @ ..] => Ok((rest, t, t.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected <ident>".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}

fn parse_arrow<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, ()> {
    match input {
        [TokenTree::Punct(p1), TokenTree::Punct(p2), rest @ ..] if p1.as_char() == '=' && p2.as_char() == '>' 
            => Ok((rest, (), p2.span())),
        [TokenTree::Punct(p), x, ..] if p.as_char() == '=' 
            => Err(Error(x.span(), vec!["expected '=>'".into()])),
        [x, ..] => Err(Error(x.span(), vec!["expected '=>'".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}

fn parse_comma<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, ()> {
    match input {
        [TokenTree::Punct(p), rest @ ..] if p.as_char() == ',' => Ok((rest, (), p.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected ','".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}

fn parse_semicolon<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, ()> {
    match input {
        [TokenTree::Punct(p), rest @ ..] if p.as_char() == ';' => Ok((rest, (), p.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected ';'".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}

fn parse_return_bracket<'a>( input : &'a [TokenTree], prev : Span ) -> PResult<'a, Box<str>> {
    match input {
        [TokenTree::Group(g), rest @ ..] if g.delimiter() == Delimiter::Brace
            => Ok((rest, g.stream().to_string().into(), g.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected '{ return_expr }'".into()])),
        [] => Err(Error(prev, vec!["unexpected end of stream".into()])),
    }
}

fn parse_type_bracket<'a>( input : &'a [TokenTree], prev : Span ) -> PResult<'a, TokenTree> {
    match input {
        [TokenTree::Group(g), rest @ ..] if g.delimiter() == Delimiter::Bracket
            => Ok((rest, g.stream().to_string().into(), g.span())),
        [x, ..] => Err(Error(x.span(), vec!["expected '[ pattern ]'".into()])),
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

pattern_matcher!( pattern => { } );



fn name<T>(input : &T) -> impl Iterator<Item = ?> {
}


*/