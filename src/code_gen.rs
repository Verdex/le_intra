
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

    let x = matches.pop().unwrap();
    // Note:  Last nexts aren't meaningful
    let mut end = x.0;
    while let Some((code, nexts)) = matches.pop() {
        end = nexts.iter().map(|x| code.replace("%expr", &end).replace("%input", x) )
                          .collect::<Vec<_>>()
                          .join("\n");
    }

    end.into()
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
        let input = pat_mat([pat("(x, y)", &["x", "y"]), pat("(a, c)", &["a", "c"]), pat("(b, d)", &[])], "(a, b)");
        let output = generate(input);
        assert!(false, "{}", output);
    }

}