# take-if

A tiny utility for conditionally taking the contents from an [`Option`]. See also [`Option::take`].

```rust
use take_if::TakeIf;

let mut maybe_greeting = Some("Hello, World!");

if let Some(greeting) = maybe_greeting.take_if(|greeting| greeting.starts_with("Hello")) {
    println!(r#"Greeting {:?} starts with "Hello""#, greeting);
} else {
    println!(r#"There was no greeting, or it didn't start with "Hello""#);
}
```

## Getting Started

Add take-if to your `Cargo.toml`:

```toml
[dependencies]
take-if = "1.0.0"
```

Import the `TakeIf` trait in your modules to add the `take_if` method to `Option`:

```rust
use take_if::TakeIf;

let taken = maybe_value.take_if(|value| value.id == 5);
```

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
