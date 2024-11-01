
use super::{ PMAst, Pattern };

use proc_macro::*;

pub struct Error(pub Span, pub Vec<Box<str>>);
pub type PResult<'a, T> = Result<(&'a [TokenTree], T, Span), Error>;


pub fn parse_pattern_matcher(input : &[TokenTree], prev : Span) -> Result<PMAst, Error> {
    let next = input;

    let (next, patterns, prev) = parse_pattern(next, prev)?;
    let (next, _, prev) = parse_arrow(next, prev)?;
    let (next, return_expr, prev) = parse_return_bracket(next, prev)?;

    Ok(PMAst { patterns, return_expr })
}

fn parse_pattern<'a>(mut next : &'a [TokenTree], mut prev : Span) -> PResult<'a, Vec<Pattern>> {
    let mut ps = vec![];

    loop {
        let x = parse_type_bracket(next, prev)?;
        next = x.0;
        let p = x.1;
        prev = x.2;

        let x = parse_ident_list(next, prev)?;
        next = x.0;
        let nexts = x.1;
        prev = x.2;

        let pattern = Pattern { p, nexts };

        ps.push(pattern);

        match parse_semicolon(next, prev) {
            Ok((n, _, p)) => { next = n; prev = p; },
            Err(_) => { break; },
        }
    }

    Ok((next, ps, prev))
}

fn parse_ident_list<'a>(mut next : &'a [TokenTree], mut prev : Span) -> PResult<'a, Vec<Box<str>>> {
    let mut ids = vec![];

    match parse_ident(next, prev) {
        Ok((n, id, p)) => {
            ids.push(id);
            next = n;        
            prev = p;
        },
        Err(_) => { 
            return Ok((next, ids, prev));
        },
    }

    loop {
        match parse_comma(next, prev) {
            Ok((n, _, p)) => {
                next = n;        
                prev = p;
            },
            Err(_) => { break; },
        }

        let x = parse_ident(next, prev)?;
        next = x.0;
        ids.push(x.1);
        prev = x.2;
    }

    Ok((next, ids, prev))
}

fn parse_ident<'a>(input : &'a [TokenTree], prev : Span) -> PResult<'a, Box<str>> {
    match input {
        [t @ TokenTree::Ident(_), rest @ ..] => Ok((rest, t.to_string().into(), t.span())),
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

fn parse_type_bracket<'a>( input : &'a [TokenTree], prev : Span ) -> PResult<'a, Box<str>> {
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