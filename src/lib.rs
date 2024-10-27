
mod parser;

use proc_macro::*;

use parser::Error;

pub (crate) struct PMAst<'a> {
    func_name : &'a TokenTree,
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

