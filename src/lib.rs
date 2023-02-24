//! This crate provides macros to efficiently concatenate strings without extra
//! side-effect.
//!
//! # Usage
//! ## Basic usage
//! ```
//! use str_cat::str_cat;
//!
//! let world = "World".to_owned(); // not a literal, so you can't use `concat!`
//! let s = str_cat!("Hello", " ", world, "!");
//! assert_eq!(s, "Hello World!");
//! ```
//!
//! which is roughly equivalent to
//!
//! ```
//! # let world = "World".to_owned();
//! let mut s = String::with_capacity("Hello".len() + " ".len() + world.len() + "!".len());
//! s.push_str("Hello");
//! s.push_str(" ");
//! s.push_str(&world);
//! s.push_str("!");
//! ```
//!
//! ## No extra side-effect
//! The macro runs without extra side-effect, which means all involved
//! expressions are evaluated exactly once.
//!
//! ```
//! # use str_cat::str_cat;
//! let mut get_world_calls = 0;
//! let mut get_world = || {
//!     get_world_calls += 1;
//!     "World"
//! };
//! let s = str_cat!("Hello", " ", get_world(), "!");
//! assert_eq!(s, "Hello World!");
//! assert_eq!(get_world_calls, 1);
//! ```
//!
//! which is roughly equivalent to
//!
//! ```
//! # let get_world = || "World";
//! let world = get_world(); // evaluate the expression and store it for later use
//! let mut s = String::with_capacity("Hello".len() + " ".len() + world.len() + "!".len());
//! s.push_str("Hello");
//! s.push_str(" ");
//! s.push_str(&world);
//! s.push_str("!");
//! ```
//!
//! ## Append to an existing string
//! ```
//! # use str_cat::str_cat;
//! let mut s = "Hello".to_owned();
//! str_cat!(&mut s; " ", "World!");
//! assert_eq!(s, "Hello World!");
//! ```
//!
//! ## Reuse existing allocation
//! ```
//! # use str_cat::str_cat;
//! let mut s = "Hello World!".to_owned();
//! let ptr = s.as_ptr();
//! let cap = s.capacity();
//!
//! s.clear();
//! str_cat!(&mut s; "Hello");
//! assert_eq!(s, "Hello");
//! // Did not grow
//! assert_eq!(s.as_ptr(), ptr);
//! assert_eq!(s.capacity(), cap);
//! ```
//!
//! ## Custom minimum capacity
//! ```
//! # use str_cat::str_cat;
//! let s = str_cat!(String::with_capacity(16); "foo", "bar");
//! assert_eq!(s, "foobar");
//! assert!(s.capacity() >= 16);
//! ```
//!
//! ## Argument types
//! Works with any expressions that can dereference to [`str`](str) when
//! evaluated. Although it should be more simple and efficient to use
//! [`format!`](format) instead when you can't avoid explicit `.to_string()`
//! calls.
//!
//! ```
//! # use str_cat::str_cat;
//! // Just an example. It's better to use `format!` in this case.
//! let s = str_cat!(
//!     "Hello".to_owned(),
//!     Box::new(" "),
//!     ['W', 'o', 'r', 'l', 'd'].iter().collect::<String>(),
//!     '!'.to_string(),
//!     123456.to_string(),
//! );
//! assert_eq!(s, "Hello World!123456");
//! ```
//!
//! ## Variants
//! There are also variants for [`PathBuf`](std::path::PathBuf),
//! [`OsString`](std::ffi::OsString) and [`Vec`](Vec).
//!
//! ```
//! use str_cat::os_str_cat;
//! # use std::ffi::OsStr;
//! # use std::path::Path;
//!
//! // Works for anything that implements AsRef<OsStr>.
//! let s = os_str_cat!(
//!     OsStr::new("Hello"),
//!     OsStr::new(" ").to_owned(),
//!     Path::new("World"),
//!     "!",
//! );
//! assert_eq!(s, OsStr::new("Hello World!"));
//! ```

/// Concatenate strings for a [`String`](String).
///
/// It requires all elements to be able to dereference to [`str`](str) (impl [`Deref<Target = str>`](std::ops::Deref)).
///
/// # Example
///
/// ```
/// use str_cat::str_cat;
///
/// let mut s = str_cat!("Hello", " ", "World", "!");
/// assert_eq!(s, "Hello World!");
///
/// // Reusing allocation.
/// s.clear();
/// str_cat!(&mut s; "foo", "bar");
/// assert_eq!(s, "foobar");
/// ```
#[macro_export]
macro_rules! str_cat {
    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*;) => {
        $input.reserve($additional);
        $($input.push_str($values_coerced);)*
    };

    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*; $head:expr, $($tail:expr,)*) => {
        let value = $head;
        let value_coerced: &str = &*value;
        $additional += value_coerced.len();
        $crate::str_cat!(@stack $input, $additional; $($values_coerced,)* value_coerced,; $($tail,)*);
    };

    ($input:expr; $($el:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut input = $input;
        let mut additional = 0;
        $crate::str_cat!(@stack input, additional; ; $($el,)*);
        input
    }};

    ($($el:expr),+ $(,)?) => {
        $crate::str_cat!(::std::string::String::new(); $($el,)*)
    };
}

/// Concatenate paths for a [`PathBuf`](std::path::PathBuf).
///
/// It requires all elements to implement [`AsRef<Path>`](AsRef).
///
/// # Example
///
/// ```
/// use str_cat::path_cat;
/// use std::ffi::OsStr;
/// use std::path::{Path, PathBuf};
///
/// let mut s = path_cat!("Hello", "space".to_owned(), Path::new("World"), OsStr::new("bang"));
/// assert_eq!(s, ["Hello", "space", "World", "bang"].iter().collect::<PathBuf>());
///
/// // Reusing allocation.
/// s.clear();
/// path_cat!(&mut s; "foo", "bar");
/// assert_eq!(s, ["foo", "bar"].iter().collect::<PathBuf>());
/// ```
#[macro_export]
macro_rules! path_cat {
    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*;) => {
        $input.reserve($additional);
        $($input.push($values_coerced);)*
    };

    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*; $head:expr, $($tail:expr,)*) => {
        let value = $head;
        let value_coerced = ::core::convert::AsRef::<::std::path::Path>::as_ref(&value);
        $additional += value_coerced.as_os_str().len();
        $crate::path_cat!(@stack $input, $additional; $($values_coerced,)* value_coerced,; $($tail,)*);
    };

    ($input:expr; $($el:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut input = $input;
        let mut additional = 0;
        $crate::path_cat!(@stack input, additional; ; $($el,)*);
        input
    }};

    ($($el:expr),+ $(,)?) => {
        $crate::path_cat!(::std::path::PathBuf::new(); $($el,)*)
    };
}

/// Concatenate OS strings for a [`OsString`](std::ffi::OsString).
///
/// It requires all elements to implement [`AsRef<OsStr>`](AsRef).
///
/// # Example
///
/// ```
/// use str_cat::os_str_cat;
/// use std::ffi::OsStr;
/// use std::path::Path;
///
/// let mut s = os_str_cat!("Hello", " ".to_owned(), Path::new("World"), OsStr::new("!"));
/// assert_eq!(s, OsStr::new("Hello World!"));
///
/// // Reusing allocation.
/// s.clear();
/// os_str_cat!(&mut s; "foo", "bar");
/// assert_eq!(s, OsStr::new("foobar"));
/// ```
#[macro_export]
macro_rules! os_str_cat {
    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*;) => {
        $input.reserve($additional);
        $($input.push($values_coerced);)*
    };

    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*; $head:expr, $($tail:expr,)*) => {
        let value = $head;
        let value_coerced = ::core::convert::AsRef::<::std::ffi::OsStr>::as_ref(&value);
        $additional += value_coerced.len();
        $crate::os_str_cat!(@stack $input, $additional; $($values_coerced,)* value_coerced,; $($tail,)*);
    };

    ($input:expr; $($el:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut input = $input;
        let mut additional = 0;
        $crate::os_str_cat!(@stack input, additional; ; $($el,)*);
        input
    }};

    ($($el:expr),+ $(,)?) => {
        $crate::os_str_cat!(::std::ffi::OsString::new(); $($el,)*)
    };
}

/// Concatenate elements for a [`Vec`](Vec).
///
/// # Example
///
/// ```
/// use str_cat::vec_cat;
/// use std::ffi::OsStr;
/// use std::path::Path;
///
/// let mut s = vec_cat!(b"Hello", b" ", "World".as_bytes(), &[b'!']);
/// assert_eq!(s, b"Hello World!");
///
/// // Reusing allocation.
/// s.clear();
/// vec_cat!(&mut s; b"foo", b"bar");
/// assert_eq!(s, b"foobar");
/// ```
#[macro_export]
macro_rules! vec_cat {
    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*;) => {
        $input.reserve($additional);
        $($input.extend_from_slice($values_coerced);)*
    };

    (@stack $input:ident, $additional:ident; $($values_coerced:ident,)*; $head:expr, $($tail:expr,)*) => {
        let value = $head;
        let value_coerced = &*value;
        $additional += value_coerced.len();
        $crate::vec_cat!(@stack $input, $additional; $($values_coerced,)* value_coerced,; $($tail,)*);
    };

    ($input:expr; $($el:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut input = $input;
        let mut additional = 0;
        $crate::vec_cat!(@stack input, additional; ; $($el,)*);
        input
    }};

    ($($el:expr),+ $(,)?) => {
        $crate::vec_cat!(::std::vec![]; $($el,)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn currently_doc_tests_covered_everything() {}
}
