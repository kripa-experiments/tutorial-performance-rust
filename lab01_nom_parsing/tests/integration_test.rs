use lab01_nom_parsing::parse_lisp_string;

#[test]
fn test_parse_lisp_string() {
    let input = "(move robot1 roomA)";
    let result = parse_lisp_string(input);
    println!("Testing '{}': {:?}", input, result);
    assert_eq!(result, Ok(("", vec!["move", "robot1", "roomA"])));
}

#[test]
fn test_parse_spaced_string() {
    let input = "(  spaced   input  )";
    let result = parse_lisp_string(input);
    println!("Testing '{}': {:?}", input, result);
    assert_eq!(result, Ok(("", vec!["spaced", "input"])));
}
