
#[cfg(test)]
mod test { 
    use le_intra::pattern_matcher;

    #[test]
    fn should() {
        pattern_matcher!([ (a, b) ]; [ (c, d) ] => { (a, b, c, d) });
        pattern_matcher!([ (a, b) ] a, b; [ (c, d) ] c => { (a, b, c, d) });
    }
}