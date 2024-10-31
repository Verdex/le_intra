
mod parser;

use proc_macro::*;

use parser::Error;

pub (crate) struct PMAst {
    patterns : Vec<Pattern>,
    return_expr : Box<str>,
}

pub (crate) struct Pattern {
    pub (crate) p : Box<str>,
    pub (crate) nexts : Vec<Box<str>>,
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
        _ => "".parse().unwrap(),
    }
}

