# Syn AST

- https://docs.rs/syn/latest/syn/index.html#structs
- https://docs.rs/syn/latest/syn/index.html#enums

# Joern CPG

- https://cpg.joern.io

# Syn AST -> Joern CPG

## Auto gen

- `FILE`
- `SOURCE_FILE`
- `NAMESPACE`

## Manual gen

- ... -> `CODE`, `FULL_NAME`, `INDEX`, `IS_EXTERNAL`, `NAME`
- ... -> `META_DATA`, `LANGUAGE`, `ROOT`, `OVERLAYS`
- ... -> `FILENAME` field
- ... -> `METHOD_REF`, `TYPE_REF`

## Enum

- `AttrStyle` ->
- `BinOp` ->
- `Dataderive` ->
- `Expr` ->
- `FieldMutability` ->
- `Fields` ->
- `FnArg` ->
- `ForeignItem` ->
- `GenericArgument` ->
- `GenericParam` ->
- `ImplItem` ->
- `ImplRestriction` ->
- `Item` ->
- `Lit` ->
- `MacroDelimiter` ->
- `Member` ->
- `Meta` ->
- `Pat` ->
- `PathArguments` ->
- `RangeLimits` ->
- `ReturnType` ->
- `StaticMutability` ->
- `Stmt` ->
- `TraitBoundModifier` ->
- `TraitItem` ->
- `Type` ->
- `TypeParamBound` ->
- `UnOp` ->
- `UseTree` ->
- `Visibility` ->
- `WherePredicate` ->

## Struct

- `Abi` -> `NewNamespaceBlock` | `NewIdentifier`
- `AngleBracketedGenericArguments` -> `NewTypeArgument` | `NewTypeParameter`
- `Arm` -> `NewControlStructure`
- `AssocConst` -> `NewTypeArgument`
- `AssocType` -> `NewTypeArgument`
- `Attribute` -> `NewAnnotation`
- `BareFnArg` ->
- `BareVariadic` ->
- `Block` -> `NewBlock`
- `BoundLifetimes` -> `NewTypeParameter`
- `ConstParam` -> `NewTypeParameter`
- `Constraint` -> `NewTypeArgument`
- `ExprArray` -> `NewArrayInitializer`
- `ExprAssign` ->
- `ExprAsync` -> `NewBlock`
- `ExprAwait` -> `NewCall`
- `ExprBinary` ->
- `ExprBlock` -> `NewBlock`
- `ExprBreak` -> `NewControlStructure`
- `ExprCall` -> `NewCall`
- `ExprCast` -> `NewCall`
- `ExprClosure` -> `NewMethod`
- `ExprConst` -> `NewLocal`
- `ExprContinue` -> `NewControlStructure`
- `ExprField` -> `NewFieldIdentifier`
- `ExprForLoop` -> `NewControlStructure`
- `ExprGroup` ->
- `ExprIf` -> `NewControlStructure`
- `ExprIndex` -> `NewFieldIdentifier`
- `ExprInfer` -> `NewLocal`
- `ExprLet` -> `NewLocal`
- `ExprLit` -> `NewLiteral`
- `ExprLoop` -> `NewControlStructure`
- `ExprMacro` -> `NewCall`
- `ExprMatch` -> `NewControlStructure`
- `ExprMethodCall` -> `NewCall`
- `ExprParen` ->
- `ExprPath` -> `NewTypeRef` | `NewMethodRef`
- `ExprRange` -> `NewArrayInitializer`
- `ExprReference` -> `NewTypeRef`
- `ExprRepeat` -> `NewArrayInitializer`
- `ExprReturn` -> `NewReturn`
- `ExprStruct` -> `NewLocal`
- `ExprTry` -> `NewControlStructure`
- `ExprTryBlock` -> `NewBlock`
- `ExprTuple` -> `NewArrayInitializer`
- `ExprUnary` ->
- `ExprUnsafe` -> `NewBlock`
- `ExprWhile` -> `NewControlStructure`
- `ExprYield` -> `NewReturn`
- `Field` -> `NewMember`
- `FieldPat` -> `NewMember`
- `FieldValue` -> `NewMember`
- `FieldsNamed` ->
- `FieldsUnnamed` ->
- `File` -> `NewFile`
- `ForeignItemFn` -> `NewMember` && `NewMethod`
- `ForeignItemMacro` -> `NewMember` && `NewCall`
- `ForeignItemStatic` -> `NewMember` && `NewLocal`
- `ForeignItemType` -> `NewMember`&&`NewTypeDecl`
- `Generics` -> `NewTypeParameter`
- `Ident` -> `NewIdentifier`
- `ImplItemConst` -> `NewMember` && `NewLocal`
- `ImplItemFn` -> `NewMember` && `NewMethod`
- `ImplItemMacro` -> `NewMember` && `NewCall`
- `ImplItemType` -> `NewMember`&&`NewTypeDecl`
- `Index` -> `NewFieldIdentifier`
- `ItemConst` -> `NewLocal`
- `ItemEnum` -> `NewTypeDecl`
- `ItemExternCrate` -> `NewImport`
- `ItemFn` -> `NewMethod`
- `ItemForeignMod` -> `NewNamespaceBlock`
- `ItemImpl` -> `NewTypeDecl`
- `ItemMacro` -> `NewCall`
- `ItemMod` -> `NewImport` | `NewNamespaceBlock`
- `ItemStatic` -> `NewLocal`
- `ItemStruct` -> `NewTypeDecl`
- `ItemTrait` -> `NewTypeDecl`
- `ItemTraitAlias` -> `NewTypeDecl`
- `ItemType` -> `NewTypeDecl`
- `ItemUnion` -> `NewTypeDecl`
- `ItemUse` -> `NewImport`
- `Label` -> `NewJumpLabel`
- `Lifetime` -> `NewTypeArgument` | `NewTypeParameter`
- `LifetimeParam` -> `NewTypeParameter`
- `LitBool` -> `NewLiteral`
- `LitByte` -> `NewLiteral`
- `LitByteStr` -> `NewLiteral`
- `LitCStr` -> `NewLiteral`
- `LitChar` -> `NewLiteral`
- `LitFloat` -> `NewLiteral`
- `LitInt` -> `NewLiteral`
- `LitStr` -> `NewLiteral`
- `Local` -> `NewLocal`
- `LocalInit` ->
- `Macro` -> `NewCall`
- `MetaList` -> `NewAnnotationParameter`
- `MetaNameValue` -> `NewAnnotationParameterAssign`
- `ParenthesizedGenericArguments` -> `NewTypeArgument` | `NewTypeParameter`
- `PatConst` ->
- `PatIdent` ->
- `PatLit` ->
- `PatMacro` ->
- `PatOr` ->
- `PatParen` ->
- `PatPath` ->
- `PatRange` ->
- `PatReference` ->
- `PatRest` ->
- `PatSlice` ->
- `PatStruct` ->
- `PatTuple` ->
- `PatTupleStruct` ->
- `PatType` -> `NewMethodParameterIn`
- `PatWild` ->
- `Path` -> `NewTypeRef` | `NewMethodRef`
- `PathSegment` ->
- `PredicateLifetime` -> `NewTypeParameter`
- `PredicateType` -> `NewTypeParameter`
- `QSelf` -> `NewTypeParameter`
- `Receiver` -> `NewMethodParameterIn`
- `Signature` ->
- `StmtMacro` -> `NewCall`
- `TraitBound` -> `NewTypeParameter`
- `TraitItemConst` -> `NewMember` && `NewLocal`
- `TraitItemFn` -> `NewMember` && `NewMethod`
- `TraitItemMacro` -> `NewMember` && `NewCall`
- `TraitItemType` ->`NewMember` && `NewTypeDecl`
- `TypeArray` -> `NewTypeRef`
- `TypeBareFn` -> `NewTypeRef`
- `TypeGenerics` -> `NewTypeRef`
- `TypeGroup` -> `NewTypeRef`
- `TypeImplTrait` -> `NewTypeRef`
- `TypeInfer` -> `NewTypeRef`
- `TypeMacro` -> `NewTypeRef`
- `TypeNever` -> `NewTypeRef`
- `TypeParam` -> `NewTypeRef`
- `TypeParen` -> `NewTypeRef`
- `TypePath` -> `NewTypeRef`
- `TypePtr` -> `NewTypeRef`
- `TypeReference` -> `NewTypeRef`
- `TypeSlice` -> `NewTypeRef`
- `TypeTraitObject` -> `NewTypeRef`
- `TypeTuple` -> `NewTypeRef`
- `UseGlob` -> `NewImport`
- `UseGroup` -> `NewImport`
- `UseName` -> `NewImport`
- `UsePath` -> `NewImport`
- `UseRename` -> `NewImport`
- `Variadic` -> `NewMethodParameterIn`
- `Variant` -> `NewMember`
- `VisRestricted` -> `NewModifier`
- `WhereClause` -> `NewTypeParameter`

## CPG Node

- `NewAnnotation`:
  - fullname
- `NewAnnotationLiteral`
- `NewAnnotationParameter`
- `NewAnnotationParameterAssign`
- `NewArrayInitializer`
- `NewBinding`:
  - methodfullname
- `NewBlock`
- `NewCall`
  - methodfullname
  - dispatchType
- `NewClosureBinding`:
  - evaluationStrategy
- `NewComment`: filename
- `NewConfigFile`
- `NewControlStructure`:
  - controlStructureType
- `NewDependency`
- `NewFieldIdentifier`: : No child
- `NewFile`
- `NewFinding`
- `NewIdentifier`
- `NewImport`: No child
- `NewJumpLabel`
- `NewJumpTarget`
- `NewKeyValuePair`
- `NewLiteral`
- `NewLocal`: No child
- `NewLocation`:
  - filename
  - methodfullname
- `NewMember`
- `NewMetaData`
- `NewMethod`:
  - Phải có con là 1 node `NewMethodReturn`, `NewMethodParameterIn`
  - filename
  - fullname
- `NewMethodParameterIn`:
  - evaluationStrategy
  - isVariadic
- `NewMethodParameterOut`:
  - evaluationStrategy
  - isVariadic
- `NewMethodRef`
  - methodfullname
- `NewMethodReturn`:
  - evaluationStrategy
- `NewModifier`:
  - modifierType
- `NewNamespace`
- `NewNamespaceBlock`:
  - filename
  - fullname
- `NewReturn`
- `NewTag`
- `NewTagNodePair`
- `NewTemplateDom`
- `NewType`
  - fullname
  - typeDeclFullName
- `NewTypeArgument`
- `NewTypeDecl`:
  - filename
  - fullName
- `NewTypeParameter`
- `NewTypeRef`: No child
- `NewUnknown`

## CPG Property

- `NAME`
  Name of represented object, e.g., method name (e.g. "run")

- `FULL_NAME`
  This is the fully-qualified name of an entity, e.g., the fully-qualified name of a method or type. The details of what constitutes a fully-qualified name are language specific. This field SHOULD be human readable.

- `CODE`
  This field holds the code snippet that the node represents.

- `METHOD_FULL_NAME`
  The FULL_NAME of a method. Used to link CALL and METHOD nodes. It is required to have exactly one METHOD node for each METHOD_FULL_NAME

- `EVALUATION_STRATEGY`
  For formal method input parameters, output parameters, and return parameters, this field holds the evaluation strategy, which is one of the following: 1) `BY_REFERENCE` indicates that the parameter is passed by reference, 2) `BY_VALUE` indicates that it is passed by value, that is, a copy is made, 3) `BY_SHARING` the parameter is a pointer/reference and it is shared with the caller/callee. While a copy of the pointer is made, a copy of the object that it points to is not made.

- `DISPATCH_TYPE`
  This field holds the dispatch type of a call, which is either `STATIC_DISPATCH` or `DYNAMIC_DISPATCH`. For statically dispatched method calls, the call target is known at compile time while for dynamically dispatched calls, it can only be determined at runtime as it may depend on the type of an object (as is the case for virtual method calls) or calculation of an offset.

- `ORDER`
  This integer indicates the position of the node among its siblings in the AST. The left-most child has an order of 0.

- `CONTROL_STRUCTURE_TYPE`
  The `CONTROL_STRUCTURE_TYPE` field indicates which kind of control structure a `CONTROL_STRUCTURE` node represents. The available types are the following: BREAK, CONTINUE, DO, WHILE, FOR, GOTO, IF, ELSE, TRY, THROW and SWITCH.

- `MODIFIER_TYPE`
  The modifier type is a free-form string. The following are known modifier types: `STATIC`, `PUBLIC`, `PROTECTED`, `PRIVATE`, `ABSTRACT`, `NATIVE`, `CONSTRUCTOR`, `VIRTUAL`

- `TYPE_DECL_FULL_NAME`
  The static type decl of a TYPE. This property is matched against the FULL_NAME of TYPE_DECL nodes. It is required to have exactly one TYPE_DECL for each different TYPE_DECL_FULL_NAME

- `TYPE_FULL_NAME`
  This field contains the fully-qualified static type name of the program construct represented by a node. It is the name of an instantiated type, e.g., `java.util.List<Integer>`, rather than `java.util.List[T]`. If the type cannot be determined, this field should be set to the empty string.

- `IS_VARIADIC`
  Specifies whether a parameter is the variadic argument handling parameter of a variadic method. Only one parameter of a method is allowed to have this property set to true.

- `SIGNATURE`
  The method signature encodes the types of parameters in a string. The string SHOULD be human readable and suitable for differentiating methods with different parameter types sufficiently to allow for resolving of function overloading. The present specification does not enforce a strict format for the signature, that is, it can be chosen by the frontend implementor to fit the source language.

- `FILENAME`
  The path of the source file this node was generated from, relative to the root path in the meta data node. This field must be set but may be set to the value `<unknown>` to indicate that no source file can be associated with the node, e.g., because the node represents an entity known to exist because it is referenced, but for which the file that is is declared in is unknown.

- `ROOT`
  The path to the root directory of the source/binary this CPG is generated from.

- `LANGUAGE`
  This field indicates which CPG language frontend generated the CPG. Frontend developers may freely choose a value that describes their frontend so long as it is not used by an existing frontend. Reserved values are to date: C, LLVM, GHIDRA, PHP.

## Error

- Exception in thread "Writer" java.lang.RuntimeException: Edge with type='ARGUMENT' with direction='IN' not supported by nodeType='ARRAY_INITIALIZER'

- Exception in thread "Writer" java.lang.RuntimeException: Edge with type='CONDITION' with direction='IN' not supported by nodeType='LOCAL'

- Exception in thread "Writer" java.lang.RuntimeException: Edge with type='RECEIVER' with direction='IN' not supported by nodeType='FIELD_IDENTIFIER'

- Exception in thread "Writer" java.lang.RuntimeException: Edge with type='AST' with direction='OUT' not supported by nodeType='TYPE_PARAMETER'

- Exception in thread "Writer" java.lang.RuntimeException: Edge with type='RECEIVER' with direction='IN' not supported by nodeType='LOCAL'

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
