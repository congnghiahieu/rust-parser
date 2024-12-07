# Struct

| AST (Rust)                       | CPG (Joern)                                                |
| -------------------------------- | ---------------------------------------------------------- |
| `Abi`                            | `NewNamespaceBlock`                                        |
| `AngleBracketedGenericArguments` | `NewTypeArgument`                                          |
| `Arm`                            | `NewControlStructure`                                      |
| `AssocConst`                     | `NewTypeArgument`                                          |
| `AssocType`                      | `NewTypeArgument`                                          |
| `Attribute`                      | `NewAnnotation`                                            |
| `BareFnArg`                      | `NewTypeRef`                                               |
| `BareVariadic`                   | `NewMethodParameterIn`                                     |
| `Block`                          | `NewBlock`                                                 |
| `BoundLifetimes`                 | `NewLifetime`                                              |
| `ConstParam`                     | `NewTypeParameter`                                         |
| `Constraint`                     | `NewTypeArgument`                                          |
| `ExprArray`                      | `NewCall`                                                  |
| `ExprAssign`                     | `NewCall`                                                  |
| `ExprAsync`                      | `NewBlock`                                                 |
| `ExprAwait`                      | `NewCall`                                                  |
| `ExprBinary`                     | `NewCall`                                                  |
| `ExprBlock`                      | `NewBlock`                                                 |
| `ExprBreak`                      | `NewControlStructure`                                      |
| `ExprCall`                       | `NewCall`                                                  |
| `ExprCast`                       | `NewCall`                                                  |
| `ExprClosure`                    | `NewMethod`                                                |
| `ExprConst`                      | `NewBlock`                                                 |
| `ExprContinue`                   | `NewControlStructure`                                      |
| `ExprField`                      | `NewCall`                                                  |
| `ExprForLoop`                    | `NewControlStructure`                                      |
| `ExprGroup`                      | `NewUnknown`                                               |
| `ExprIf`                         | `NewControlStructure`                                      |
| `ExprIndex`                      | `NewCall`                                                  |
| `ExprInfer`                      | `NewUnknown`                                               |
| `ExprLet`                        | `NewCall`                                                  |
| `ExprLit`                        | `NewLiteral`                                               |
| `ExprLoop`                       | `NewControlStructure`                                      |
| `ExprMacro`                      | `NewCall`                                                  |
| `ExprMatch`                      | `NewControlStructure`                                      |
| `ExprMethodCall`                 | `NewCall`                                                  |
| `ExprParen`                      | `NewUnknown`                                               |
| `ExprPath`                       | `NewIdentifier` \| `NewTypeRef` \| `NewMethodRef`          |
| `ExprRange`                      | `NewCall`                                                  |
| `ExprReference`                  | `NewCall`                                                  |
| `ExprRepeat`                     | `NewCall`                                                  |
| `ExprReturn`                     | `NewReturn`                                                |
| `ExprStruct`                     | `NewUnknown`                                               |
| `ExprTry`                        | `NewControlStructure`                                      |
| `ExprTryBlock`                   | `NewBlock`                                                 |
| `ExprTuple`                      | `NewCall`                                                  |
| `ExprUnary`                      | `NewCall`                                                  |
| `ExprUnsafe`                     | `NewBlock`                                                 |
| `ExprWhile`                      | `NewControlStructure`                                      |
| `ExprYield`                      | `NewReturn`                                                |
| `Field`                          | `NewMember`                                                |
| `FieldPat`                       | `NewMember`                                                |
| `FieldValue`                     | `NewMember`                                                |
| `FieldsNamed`                    | `NewMember`                                                |
| `FieldsUnnamed`                  | `NewMember`                                                |
| `File`                           | `NewFile`                                                  |
| `ForeignItemFn`                  | `NewMethod`                                                |
| `ForeignItemMacro`               | `NewCall`                                                  |
| `ForeignItemStatic`              | `NewLocal`                                                 |
| `ForeignItemType`                | `NewTypeDecl`                                              |
| `Generics`                       | `NewTypeParameter`                                         |
| `Ident`                          | `NewIdentifier`                                            |
| `ImplItemConst`                  | `NewLocal`                                                 |
| `ImplItemFn`                     | `NewMethod`                                                |
| `ImplItemMacro`                  | `NewCall`                                                  |
| `ImplItemType`                   | `NewTypeDecl`                                              |
| `Index`                          | `NewFieldIdentifier`                                       |
| `ItemConst`                      | `NewLocal`                                                 |
| `ItemEnum`                       | `NewTypeDecl`                                              |
| `ItemExternCrate`                | `NewImport`                                                |
| `ItemFn`                         | `NewMethod`                                                |
| `ItemForeignMod`                 | `NewUnknown`                                               |
| `ItemImpl`                       | `NewTypeDecl`                                              |
| `ItemMacro`                      | `NewCall`                                                  |
| `ItemMod`                        | `NewImport` \| `NewNamespaceBlock`                         |
| `ItemStatic`                     | `NewLocal`                                                 |
| `ItemStruct`                     | `NewTypeDecl`                                              |
| `ItemTrait`                      | `NewTypeDecl`                                              |
| `ItemTraitAlias`                 | `NewTypeDecl`                                              |
| `ItemType`                       | `NewTypeDecl`                                              |
| `ItemUnion`                      | `NewTypeDecl`                                              |
| `ItemUse`                        | `NewImport`                                                |
| `Label`                          | `NewJumpTarget`                                            |
| `Lifetime`                       | `NewLifetime`                                              |
| `LifetimeParam`                  | `NewLifetimeParameter`                                     |
| `LitBool`                        | `NewLiteral`                                               |
| `LitByte`                        | `NewLiteral`                                               |
| `LitByteStr`                     | `NewLiteral`                                               |
| `LitCStr`                        | `NewLiteral`                                               |
| `LitChar`                        | `NewLiteral`                                               |
| `LitFloat`                       | `NewLiteral`                                               |
| `LitInt`                         | `NewLiteral`                                               |
| `LitStr`                         | `NewLiteral`                                               |
| `Local`                          | `NewCall` && `NewLocal`                                    |
| `LocalInit`                      | `NewUnknown`                                               |
| `Macro`                          | `NewCall`                                                  |
| `MetaList`                       | `NewAnnotationParameter` && `NewAnnotationParameterAssign` |
| `MetaNameValue`                  | `NewAnnotationParameter` && `NewAnnotationParameterAssign` |
| `ParenthesizedGenericArguments`  | `NewTypeArgument`                                          |
| `PatConst`                       | `NewBlock`                                                 |
| `PatIdent`                       | `NewIdentifier`                                            |
| `PatLit`                         | `NewLiteral`                                               |
| `PatMacro`                       | `NewCall`                                                  |
| `PatOr`                          | `NewCall`                                                  |
| `PatParen`                       | `NewUnknown`                                               |
| `PatPath`                        | `NewIdentifier` \| `NewTypeRef` \| `NewMethodRef`          |
| `PatRange`                       | `NewCall`                                                  |
| `PatReference`                   | `NewCall`                                                  |
| `PatRest`                        | `NewIdentifier`                                            |
| `PatSlice`                       | `NewUnknown`                                               |
| `PatStruct`                      | `NewUnknown`                                               |
| `PatTuple`                       | `NewUnknown`                                               |
| `PatTupleStruct`                 | `NewUnknown`                                               |
| `PatType`                        | `NewUnknown`                                               |
| `PatWild`                        | `NewIdentifier`                                            |
| `Path`                           | `NewIdentifier` \| `NewTypeRef` \| `NewMethodRef`          |
| `PathSegment`                    | `NewUnknown`                                               |
| `PredicateLifetime`              | `NewLifetimeParameter`                                     |
| `PredicateType`                  | `NewTypeParameter`                                         |
| `QSelf`                          | `NewUnknown`                                               |
| `Receiver`                       | `NewMethodParameterIn`                                     |
| `Signature`                      | `NewUnknown`                                               |
| `StmtMacro`                      | `NewCall`                                                  |
| `TraitBound`                     | `NewType` \| `NewLifetime`                                 |
| `TraitItemConst`                 | `NewLocal`                                                 |
| `TraitItemFn`                    | `NewMethod`                                                |
| `TraitItemMacro`                 | `NewCall`                                                  |
| `TraitItemType`                  | `NewTypeDecl`                                              |
| `TypeArray`                      | `NewType`                                                  |
| `TypeBareFn`                     | `NewType`                                                  |
| `TypeGenerics`                   | `NewType`                                                  |
| `TypeGroup`                      | `NewType`                                                  |
| `TypeImplTrait`                  | `NewType`                                                  |
| `TypeInfer`                      | `NewType`                                                  |
| `TypeMacro`                      | `NewType`                                                  |
| `TypeNever`                      | `NewType`                                                  |
| `TypeParam`                      | `NewType`                                                  |
| `TypeParen`                      | `NewType`                                                  |
| `TypePath`                       | `NewIdentifier` \| `NewTypeRef` \| `NewMethodRef`          |
| `TypePtr`                        | `NewType`                                                  |
| `TypeReference`                  | `NewType`                                                  |
| `TypeSlice`                      | `NewType`                                                  |
| `TypeTraitObject`                | `NewType`                                                  |
| `TypeTuple`                      | `NewType`                                                  |
| `UseGlob`                        | `NewImport`                                                |
| `UseGroup`                       | `NewImport`                                                |
| `UseName`                        | `NewImport`                                                |
| `UsePath`                        | `NewImport`                                                |
| `UseRename`                      | `NewImport`                                                |
| `Variadic`                       | `NewMethodParameterIn`                                     |
| `Variant`                        | `NewMember`                                                |
| `VisRestricted`                  | `NewModifier`                                              |
| `WhereClause`                    | `NewLifetimeParameter` \| `NewTypeParameter`               |

# Enum

- Các `enum` là các nút AST đa hình nên không có ánh xạ CPG tương ứng trực tiếp, tuy nhiên vẫn phải xử lý để chuyển đổi từ kiểu đa hình sang kiểu cụ thể

| AST (Rust)           | CPG (Jonern) |
| -------------------- | ------------ |
| `AttrStyle`          |              |
| `BinOp`              |              |
| `Dataderive`         |              |
| `Expr`               |              |
| `FieldMutability`    |              |
| `Fields`             |              |
| `FnArg`              |              |
| `ForeignItem`        |              |
| `GenericArgument`    |              |
| `GenericParam`       |              |
| `ImplItem`           |              |
| `ImplRestriction`    |              |
| `Item`               |              |
| `Lit`                |              |
| `MacroDelimiter`     |              |
| `Member`             |              |
| `Meta`               |              |
| `Pat`                |              |
| `PathArguments`      |              |
| `RangeLimits`        |              |
| `ReturnType`         |              |
| `StaticMutability`   |              |
| `Stmt`               |              |
| `TraitBoundModifier` |              |
| `TraitItem`          |              |
| `Type`               |              |
| `TypeParamBound`     |              |
| `UnOp`               |              |
| `UseTree`            |              |
| `Visibility`         |              |
| `WherePredicate`     |              |
