# Oak Changelog

## v0.2.0 (Parser & Interpreter Integration Improvements)

### Added
- Introduced a new `Parser` struct to handle Oak language parsing with a token stream (`VecDeque<Token>`).
- Added support for Oak's block structure: `BEGIN PROJ`, `END PROJ`, `BEGIN SECTION`, `END SECTION`.
- Added support for `var` declarations and `:=` assignment operator.
- Added support for `ret` as a special function call (e.g., `ret print result`).
- Added support for string and number literals, and variable identifiers in expressions.
- Added support for function calls and assignments in the parser.
- Added error handling in the parser using `Box<dyn Error>` for flexibility.

### Changed
- Updated the tokenizer to recognize Oak-specific tokens: `BEGIN PROJ`, `END PROJ`, `BEGIN SECTION`, `END SECTION`, `var`, `ret`, and `:=`.
- Updated the parser to return all statements from projects and sections, not just the first statement.
- Updated the runtime to walk and interpret all AST nodes, not just the first, so all statements in a script are executed in order.
- Improved function call parsing so that `ret print result` is parsed as a function call to `print` with `result` as an argument.

### Fixed
- Fixed an issue where only the first statement in a project or section was executed.
- Fixed an issue where assignments and function calls were not parsed or executed correctly according to Oak syntax.

### Notes
- The parser and interpreter now fully support the Oak language syntax as demonstrated in `test.oak` and `examples/math_demo.oak`.
- All changes follow the Oak language's intended structure and semantics. 
