
use super::{ PMAst, Pattern };

macro_rules! pattern_matcher_format {
    () => { 
"match %input {{ 
    {} => {{ 
        %expr 
    }}, 
}}"
};
}

pub fn generate(input : PMAst) -> Box<str> {

    let mut matches = input.patterns.iter()
        .map(|pat| (format!(pattern_matcher_format!(), pat.p), &pat.nexts))
        .collect::<Vec<_>>();

    let last = matches.pop().unwrap();
    // Note:  Last nexts aren't meaningful
    let mut end = last.0.replace("%expr", &input.return_expr);
    while let Some((code, nexts)) = matches.pop() {
        let inner_expr = nexts.iter().map(|x| end.replace("%input", x) )
                                     .collect::<Vec<_>>()
                                     .join("\n");
        end = code.replace("%expr", &inner_expr); 
    }

    let func = format!("|input| {{ {} }}", end.replace("%input", "input"));

    let indices = input.patterns.iter().map(|x| format!("{}", x.nexts.len())).collect::<Vec<_>>().join(", ");
    let indices = format!("vec![ {} ]", indices);

    format!( "( {}, {} )", func, indices ).into()
}

#[cfg(test)]
mod test {
    use super::*;

    fn pat(p : &str, nexts : &[&str]) -> Pattern {
        Pattern { p: p.into(), nexts: nexts.iter().map(|x| x.to_string().into()).collect() }
    }

    fn pat_mat<const N : usize>(pat : [Pattern; N], ret : &str) -> PMAst {
        PMAst { patterns: pat.into_iter().collect(), return_expr: ret.into() }
    }

    #[test]
    fn blarg() {
        let input = pat_mat([pat("(x, y)", &["x", "y"]), pat("(a, c)", &["a", "c"]), pat("(b, d)", &[])], "final");
        let output = generate(input);
        assert!(false, "{}", output);
    }

}