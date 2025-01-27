A rendered version of this document can be found 
[here](https://koto.dev/about).

See the neighbouring [readme](./README.md) for an explanation of the
`print!` and `check!` commands used in the code examples.

---

# About Koto

Koto is a simple and expressive programming language, usable as an extension
language for [Rust][rust] applications, or as a standalone scripting language.

```koto
print 'Hello, World!'
check! Hello, World!

square = |n| n * n
print! '8 squared is {square 8}'
check! 8 squared is 64

print! (2, 4, 6, 8)
  .each square
  .to_list()
check! [4, 16, 36, 64]
```

## Background

Koto was started in 2020 with the goal to create an ideal language for adding
scripting to applications developed in Rust. Of particular interest were
interactive systems like animation or game engines, where rapid iteration
demands a lightweight programming interface that compiles and runs quickly.

The guiding design principle is that Koto should be _simple_, 
conceptually as well as visually. To that end, a focus throughout the language's
development has been on reducing syntax noise and minimizing core concepts
wherever possible.

## Current State

Koto is a new language and should be considered to have a prominent
'use at your own risk' disclaimer.

With that said, Koto is starting to feel more stable, and although we're still
some way from a `1.0` release,
breaking changes are becoming much less frequent.

Early adopter feedback is invaluable, so if you do try out Koto please
get in touch and share your experiences, positive or otherwise!
You're welcome to reach out in [Discussions][discussions],
or on [Discord][discord], or by opening an [issue][issues].

You can read [the guide](./language_guide.md),
try it out in [the playground][playground] or
the [CLI](./cli.md), and see how well it works in your
[existing Rust application](./api.md).

## Features

- **Simple and clean syntax:** Koto aims to reduce visual noise and cognitive
  load wherever possible, while still enabling full intuitive control of your
  program.
- **Easy integration with Rust:** Koto is implemented in Rust, and is designed
  to be easily added to existing applications.
  Custom value types can be added to the Koto runtime by implementing the
  [`KotoObject`][koto-object] trait.
- **Fast compilation:** The compiler has been written with rapid iteration in
  mind, with the goal of compiling a script as quickly as possible.
- **Rich iterator support:** Koto has a focus on using iterators for data
  manipulation, with a large collection of iterator generators, adaptors,
  and consumers available in the core library's [iterator module][iterator].
- **Built-in testing:** Automated testing has
  [first-class support in Koto][testing], making it natural to write tests along
  with your code.

### Missing/Incomplete Features

- **Type Checking:** Koto is currently a dynamically typed language without
  support for type checking, but type hints [could be added][type-hints] in the
  future.
- **async tasks:** Koto doesn't have support for asynchronous tasks, 
  support [could be added][async] in the future.
- **Integration with other languages:** There's currently no C API for Koto,
  which would allow it to be integrated with other languages.
- **Tooling:** Some basic [editor support](#tooling) is available for Koto,
  but modern tooling like an LSP implementation, auto-formatting,
  or linting are still topics for the future.


## Tooling

Basic editor support is available for Koto:
- [Visual Studio Code](https://github.com/koto-lang/koto-vscode)
- [Vim / Neovim](https://github.com/koto-lang/koto.vim)
- [Sublime Text](https://github.com/koto-lang/koto-sublime)
- [Tree-sitter](https://github.com/koto-lang/tree-sitter-koto)


[async]: https://github.com/koto-lang/koto/issues/277
[discord]: https://discord.gg/JeV8RuK4CT
[discussions]: https://github.com/koto-lang/koto/discussions
[issues]: https://github.com/koto-lang/koto/issues
[iterator]: ./core_lib/iterator.md
[koto]: https://koto.dev
[koto-object]: https://github.com/koto-lang/koto/blob/main/crates/runtime/src/types/object.rs
[playground]: https://koto.dev/play
[rust]: https://rust-lang.org
[testing]: ./language_guide.md#testing
[type-hints]: https://github.com/koto-lang/koto/issues/298
