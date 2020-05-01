//! Conditionally take a value out of an option.
//!
//! This crate adds a `take_if` extension method to [`Option`] which conditionally
//! takes the value out of an option, leaving `None` in its place if the value was
//! taken. The predicate function is only called if the option is `Some`, and
//! receives a reference to the option's contents.
//!
//! If you don't need to take the value conditionally, i.e. you always need to take
//! the value, use [`Option::take`] instead.
//!
//! # Examples
//!
//! ```
//! use take_if::TakeIf;
//!
//! let mut maybe_greeting = Some("Hello, World!");
//!
//! if let Some(greeting) = maybe_greeting.take_if(|greeting| greeting.starts_with("Hello")) {
//!     println!(r#"Greeting {:?} starts with "Hello""#, greeting);
//! } else {
//!     println!(r#"There was no greeting, or it didn't start with "Hello""#);
//! }
//! ```
//!
//! [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
//! [`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take

/// Extension trait for `Option<T>` that adds the `take_if` method.
///
/// See the [crate-level documentation](./index.html) for more information.
pub trait TakeIf {
    /// The type contained in the `Option`.
    type Inner;

    /// Takes value out of the `Option` if `predicate` returns `true`.
    ///
    /// See the [crate-level documentation](./index.html) for more information.
    fn take_if<F: FnOnce(&Self::Inner) -> bool>(&mut self, predicate: F) -> Option<Self::Inner>;
}

impl<T> TakeIf for Option<T> {
    type Inner = T;

    fn take_if<F: FnOnce(&Self::Inner) -> bool>(&mut self, predicate: F) -> Option<Self::Inner> {
        if self.as_ref().map(predicate).unwrap_or(false) {
            self.take()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TakeIf;

    #[test]
    fn conditional_take() {
        let mut option = Some(5);
        assert_eq!(None, option.take_if(|_| false));
        assert_eq!(Some(5), option.take_if(|_| true));
        assert_eq!(None, option.take_if(|_| true));
    }
}
