# caoiscript

This is a very simple scripting language that I made for fun. It is not useful, at all!

## Features

- [x] Basic AST generation
- [x] Type-verification
- [ ] Interpreter / Code Generation
- [ ] REPL

## Syntax

The syntax is pretty simple, and similar to other languages. Here is an example:

```
# This is a comment
let x = 5

# Type annotations are optional!
let y: Int = 10

return x + y
```

> **Note**:
> At the minute, `return` isn't implemented. It's just there to show what the syntax will look like.

The typechecker (more of a verifier at the minute) will verify that the types on either side of a `BinaryOperation` are
the same, or convertible.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Not that you'd steal
anything from here... it's really silly.