//! A minimal greeting library.
//!
//! ```
//! assert_eq!(hello_world_crate::hello(), "Hello, world!");
//! ```

/// Returns a greeting without allocating memory.
///
/// ```
/// use hello_world_crate::hello;
///
/// assert_eq!(hello(), "Hello, world!");
/// ```
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello(), "Hello, world!");
    }
}
