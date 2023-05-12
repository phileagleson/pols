use tree_sitter::Parser;

pub fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_poweron::language())
        .unwrap();
    parser
}

#[test]
fn test_parser() {
    let mut parser = get_parser();
    let source_code = "TARGET=ACCOUNT";
    let tree = parser.parse(source_code, None).unwrap();
    assert_eq!(
        tree.root_node().to_sexp(),
        "(source_file (target_division (record_type)))"
    );
}
