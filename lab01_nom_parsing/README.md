# Lab 01: Nom Parsing

## Tool
`nom` (Parser Combinators)

## Description
This lab implements a parser to extract words from a parenthesized Lisp-style string (e.g., `(action arg1 arg2)`).
It demonstrates zero-copy slicing by returning `Vec<&str>`.

## Usage
```rust
use lab01_nom_parsing::parse_lisp_string;

let input = "(move robot1 roomA)";
let result = parse_lisp_string(input);
assert_eq!(result, Ok(("", vec!["move", "robot1", "roomA"])));
```
