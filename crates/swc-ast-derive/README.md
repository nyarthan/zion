# swc_ast_derive

Procedural Macro for generating swc AST nodes for simple rust types.

See [./tests/derive.rs](./tests/derive.rs) for usage example.

Works with following types:

- `String` -> string literal
- `Vec<T>` where `T` is any of the listed types -> array literal
- `HashSet<T>` where `T` is any of the listed types -> array literal
- `HashMap<T>` -> object literal
- `boolean` -> boolean
- `tuple` -> _not implemented_
- `Option` -> derive for `T` or `undefined`
- enum -> string literal
- struct -> object literal
