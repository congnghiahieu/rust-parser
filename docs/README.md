# Overrall view

## Project view

- Anonymous module (Root module)
  - 1st child module level 1
  - 2nd child module level 1
  - 3rd child module level 1
    - 1st child module level 2
    - 2nd child module level 2
    - 3rd child module level 2
      - 1st child module level 3
      - 2nd child module level 3
      - 3rd child module level 3
      - ...
    - ...
  - ...

## Module view

- Module:
  - Shebang
  - Inner attribute
  - Items
    - 1st item
    - 2nd item
    - 3rd item
      - Outer attribute
      - 1st statement
      - 2nd statement
      - 3rd statement
        - 1st expresion
        - 2nd expresion
        - 3rd expresion
        - ...
      - ...
    - ...

## Path

- Simple path (short name) == `item_identifier`
- Full path (full name) == `root_module_identifier/module_level1_identifier/module_level2_identifier/item_identifier`

# Rust reference

- Comments https://doc.rust-lang.org/reference/comments.html

- Tokens https://doc.rust-lang.org/reference/tokens.html

  1. Keywords https://doc.rust-lang.org/reference/keywords.html
  2. Identifiers https://doc.rust-lang.org/reference/identifiers.html
  3. Literals https://doc.rust-lang.org/reference/tokens.html#literals
  4. Lifetimes and loop labels https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels
  5. Punctuation https://doc.rust-lang.org/reference/tokens.html#punctuation
  6. Delimiters https://doc.rust-lang.org/reference/tokens.html#delimiters

- Macros

  1. Macros Invocation https://doc.rust-lang.org/reference/macros.html
  2. Declarative Macros https://doc.rust-lang.org/reference/macros-by-example.html
  3. Procedural Macros https://doc.rust-lang.org/reference/procedural-macros.html

- Attribute

  1.  Inner attribute
  2.  Outer attribute

- Items https://doc.rust-lang.org/reference/items.html

  1. Modules https://doc.rust-lang.org/reference/items/modules.html
  2. Extern crate declarations https://doc.rust-lang.org/reference/items/extern-crates.html
  3. Use declarations https://doc.rust-lang.org/reference/items/use-declarations.html
  4. Functions https://doc.rust-lang.org/reference/items/functions.html
  5. Type aliases https://doc.rust-lang.org/reference/items/type-aliases.html
  6. Structs https://doc.rust-lang.org/reference/items/structs.html
  7. Enumerations https://doc.rust-lang.org/reference/items/enumerations.html
  8. Unions https://doc.rust-lang.org/reference/items/unions.html
  9. Constant https://doc.rust-lang.org/reference/items/constant-items.html
  10. Static https://doc.rust-lang.org/reference/items/static-items.html
  11. Traits https://doc.rust-lang.org/reference/items/traits.html
  12. Implementations https://doc.rust-lang.org/reference/items/implementations.html
  13. External blocks https://doc.rust-lang.org/reference/items/external-blocks.html
  14. Generic parameters https://doc.rust-lang.org/reference/items/generics.html
  15. Associated Items https://doc.rust-lang.org/reference/items/associated-items.html

- Statements https://doc.rust-lang.org/reference/statements.html

  1. Declaration statements https://doc.rust-lang.org/reference/statements.html#declaration-statements
  2. Expression statements https://doc.rust-lang.org/reference/statements.html#expression-statements

- Expressions https://doc.rust-lang.org/reference/expressions.html

  1. Literal expressions https://doc.rust-lang.org/reference/expressions/literal-expr.html
  2. Path expressions https://doc.rust-lang.org/reference/expressions/path-expr.html
  3. Block expressions https://doc.rust-lang.org/reference/expressions/block-expr.html
  4. Operator expressions https://doc.rust-lang.org/reference/expressions/operator-expr.html
  5. Grouped expressions https://doc.rust-lang.org/reference/expressions/grouped-expr.html
  6. Array and array index expressions https://doc.rust-lang.org/reference/expressions/array-expr.html
  7. Tuple and tuple indexing expressions https://doc.rust-lang.org/reference/expressions/tuple-expr.html
  8. Struct expressions https://doc.rust-lang.org/reference/expressions/struct-expr.html
  9. Call expressions https://doc.rust-lang.org/reference/expressions/call-expr.html
  10. Method-call expressions https://doc.rust-lang.org/reference/expressions/method-call-expr.html
  11. Field access expressions https://doc.rust-lang.org/reference/expressions/field-expr.html
  12. Closure expressions https://doc.rust-lang.org/reference/expressions/closure-expr.html
  13. Loops and other breakable expressions https://doc.rust-lang.org/reference/expressions/loop-expr.html
  14. Range expressions https://doc.rust-lang.org/reference/expressions/range-expr.html
  15. if and if let expressions https://doc.rust-lang.org/reference/expressions/if-expr.html
  16. match expressions https://doc.rust-lang.org/reference/expressions/match-expr.html
  17. return expressions https://doc.rust-lang.org/reference/expressions/return-expr.html
  18. Await expressions https://doc.rust-lang.org/reference/expressions/await-expr.html
  19. \_ expressions https://doc.rust-lang.org/reference/expressions/underscore-expr.html

- Patterns https://doc.rust-lang.org/reference/patterns.html

  1. Destructuring https://doc.rust-lang.org/reference/patterns.html#destructuring
  2. Literal patterns https://doc.rust-lang.org/reference/patterns.html#literal-patterns
  3. Identifier patterns https://doc.rust-lang.org/reference/patterns.html#identifier-patterns
  4. Wildcard pattern https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern
  5. Rest patterns https://doc.rust-lang.org/reference/patterns.html#rest-patterns
  6. Range patterns https://doc.rust-lang.org/reference/patterns.html#range-patterns
  7. Reference patterns https://doc.rust-lang.org/reference/patterns.html#reference-patterns
  8. Struct patterns https://doc.rust-lang.org/reference/patterns.html#struct-patterns
  9. Tuple struct patterns https://doc.rust-lang.org/reference/patterns.html#tuple-struct-patterns
  10. Tuple patterns https://doc.rust-lang.org/reference/patterns.html#tuple-patterns
  11. Grouped patterns https://doc.rust-lang.org/reference/patterns.html#grouped-patterns
  12. Slice patterns https://doc.rust-lang.org/reference/patterns.html#slice-patterns
  13. Path patterns https://doc.rust-lang.org/reference/patterns.html#path-patterns
  14. Or-patterns https://doc.rust-lang.org/reference/patterns.html#or-patterns

- Type system https://doc.rust-lang.org/reference/types.html

  1. Boolean type
  2. Numeric types
  3. Textual types
  4. Never type
  5. Tuple types
  6. Array types
  7. Slice types
  8. Struct types
  9. Enumerated types
  10. Union types
  11. Function item types
  12. Closure types
  13. Pointer types
  14. Function pointer types
  15. Trait object types
  16. Impl trait type
  17. Type parameters
  18. Inferred type

- Trait and lifetime bounds https://doc.rust-lang.org/reference/trait-bounds.html

  1. Super trait
  2. where
  3. ?
  4. Associated types
  5. Lifetime bounds https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds
  6. Higher-ranked trait bounds https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds

- Names https://doc.rust-lang.org/reference/names.html

  1. Paths https://doc.rust-lang.org/reference/paths.html
  2. Visibility and Privacy https://doc.rust-lang.org/reference/visibility-and-privacy.html

- Unsafety https://doc.rust-lang.org/reference/unsafety.html

  1. The unsafe keyword https://doc.rust-lang.org/reference/unsafe-keyword.html

- Scoping rules https://doc.rust-lang.org/stable/rust-by-example/scope.html

# Syn AST

- https://docs.rs/syn/latest/syn/index.html#structs
- https://docs.rs/syn/latest/syn/index.html#enums

# Joern CPG

- https://cpg.joern.io
