
mod parser;

use proc_macro::*;

use parser::Error;

pub (crate) enum PMAst {
    Remove
}

#[proc_macro]
pub fn pattern_matcher( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    let ast = parser::parse_pattern_matcher(&input, Span::call_site());
    match ast {
        Err(Error(span, message)) => { 
            let mut error_code = format!("compile_error!(\"{}\")", message.join("\n"))
                .parse::<TokenStream>()
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>();

            for x in &mut error_code {
                x.set_span(span);
            }

            error_code.into_iter().collect()
        },
        _ => todo!(),
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