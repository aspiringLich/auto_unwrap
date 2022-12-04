#![feature(default_free_fn)]
#![feature(extend_one)]
#![feature(let_chains)]
#![feature(assert_matches)]

///! # auto_unwrap
///!
///! Have you every written a function and were too lazy to have it return `Result<T, E>` but still wanted to use the `?` operator? I present to you:
///!
///! ```
///! use auto_unwrap::auto_unwrap;
///!
///! #[auto_unwrap]
///! fn fn_1() -> i32 {
///!     let s = "does it detect this question mark? (no)";
///!     println!("{}", s);
///!     let x: Result<i32, ()> = Ok(23);
///!     return x?; // gets replaced with x.unwrap();
///! }
///!
///! assert_eq!(fn_1(), 23);
///! ```
///!
///! Is there someplace you would like to keep the `?`?
///!
///! ```
///! use auto_unwrap::auto_unwrap;
///!
///! #[auto_unwrap]
///! fn fn_2() {
///!     #[skip_auto_unwrap] // skips until (and including) the next brace-delimited group or semicolon
///!     let closure = || -> Result<u32, f32> {
///!         let ok: Result<u32, f32> = Ok(1);
///!         assert_eq!(ok?, ok.unwrap());
///!
///!         let err: Result<u32, f32> = Err(2.0);
///!         assert_eq!(err?, err.unwrap()); // without the skip this would panic!
///!
///!         Ok(2)
///!     };
///!
///!     assert_eq!(closure(), Err(2.0));
///! }
///! ```
///!
///! this is updated from some previous code i wrote: [yauc](https://github.com/aspiringLich/yauc)
///!
///! I made this for one specific use case: Bevy systems. With all the queries you have to do there can be a lot of `.unwrap()`'s necessary, and that gets annoying.
///!
///! Probably better practice to use `.except()` but eh. And I made this mostly to learn how to do procedural macros anyway.
extern crate core;
extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};
use std::default::default;

fn is_skip(iter: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>) -> bool {
    // turns a Group -> Some((delimeter: char, stream: TokenStream)) and everything else into a None
    let unwrap_group = |tt: &TokenTree| match tt {
        TokenTree::Group(group) => Some((group.delimiter(), group.stream())),
        _ => None,
    };
    const TERLY: &str = "Does not terminate earlier than expected";
    // we want #[skip_auto_default]
    if let Some((delimiter, stream)) = unwrap_group(iter.peek().expect(TERLY)) && delimiter == Delimiter::Bracket{
        // consume the peek for real
        iter.next();
        stream.to_string() == "skip_auto_unwrap"
    } else {
        false
    }
}

fn unwrap_inner(input: TokenStream) -> TokenStream {
    let mut out: TokenStream = default();
    let mut iter = input.into_iter().peekable();
    let mut ignore = false;
    while let Some(token) = iter.next() {
        match token {
            // another group, recurse!
            // also we stop ignoring here if its braces
            TokenTree::Group(ref group) => {
                // if ignore just add the unprocessed token
                // else process it
                // also if this is a brace-delimited group stop ignoring stuff
                if ignore {
                    if group.delimiter() == Delimiter::Brace {
                        ignore = false;
                    }
                    out.extend_one(token);
                } else {
                    out.extend_one(TokenTree::Group(Group::new(
                        group.delimiter(),
                        unwrap_inner(group.stream()),
                    )))
                }
            }
            // punctuation woo
            TokenTree::Punct(ref punct) => match punct.as_char() {
                // replace `?` with `unwrap` if we arent ignoring stuff
                '?' if !ignore => out.extend([
                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("unwrap", punct.span())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, default())),
                ]),
                // ah a hashtag i wonder if its a `#[ignore]`?
                '#' => {
                    if is_skip(&mut iter) {
                        ignore = true;
                    } else {
                        out.extend_one(token);
                    }
                }
                // meh
                _ => out.extend_one(token),
            },
            // eh
            other => out.extend_one(other),
        }
    }
    out
}

/// Automatically replaces every instance of the `?` operator with `.unwrap()`
///
/// Will not preform this replacement for statements following an `#[ignore]`
///
/// # Example
///
/// ```
/// use auto_unwrap::auto_unwrap;
///
/// #[auto_unwrap]
/// fn fn_1() -> i32 {
///     let s = "does it detect this question mark? (no)";
///     println!("{}", s);
///     let x: Result<i32, ()> = Ok(23);
///     return x?; // gets replaced with x.unwrap();
/// }
///
/// assert_eq!(fn_1(), 23);
/// ```
///
/// ```
/// use auto_unwrap::auto_unwrap;
///
/// #[auto_unwrap]
/// fn fn_2() {
///     #[skip_auto_default] // skips until (and including) the next brace-delimited group or semicolon
///     let closure = || -> Result<u32, f32> {
///         let ok: Result<u32, f32> = Ok(1);
///         assert_eq!(ok?, ok.unwrap());
///
///         let err: Result<u32, f32> = Err(2.0);
///         assert_eq!(err?, err.unwrap()); // without the skip this would panic!
///
///         Ok(2)
///     };
///
///     assert_eq!(closure(), Err(2.0));
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_unwrap(_args: TokenStream, input: TokenStream) -> TokenStream {
    // dont process anything until the function body. dont even look at anything.
    let mut args = input.into_iter().collect::<Vec<_>>();
    let mut out = TokenStream::new();
    out.extend(args.drain(..args.len() - 1));

    // now it should just be the final {..} group left
    assert!(args.len() == 1);
    let token = args.into_iter().next().unwrap();
    match token {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
            out.extend_one(TokenTree::Group(Group::new(
                Delimiter::Brace,
                unwrap_inner(group.stream()),
            )));
        }
        _ => panic!("No/Invalid function body(???)"),
    }
    // panic!("{}", out);
    out
}
