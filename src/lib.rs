
mod parser;

use proc_macro::*;

#[proc_macro]
pub fn pattern_matcher( input : TokenStream ) -> TokenStream {
    todo!()
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