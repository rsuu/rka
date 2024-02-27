#![crate_type = "proc-macro"]

// unit!()
//   dur!
//   size!

mod opt;
mod size;
mod time;

/// Syntactic sugar for [`Duration`].
///
/// **Paterns:**
/// * w: weeks
/// * d: days
/// * h: hours
/// * m: minutes
/// * s: seconds
/// * ms: milliseconds
/// * us: microseconds
/// * ns: nanoseconds
///
/// # Examples
/// ```rust
/// use rka_fn::dur;
/// use core::time::Duration;
///
/// fn main() {
///    assert_eq!(dur!(10w), Duration::from_secs(10 * 60 * 60 * 24 * 7));
///    assert_eq!(dur!(10d), Duration::from_secs(10 * 60 * 60 * 24));
///    assert_eq!(dur!(10m), Duration::from_secs(10 * 60));
///    assert_eq!(dur!(10s), Duration::from_secs(10));
///    assert_eq!(dur!(10ms), Duration::from_millis(10));
///    assert_eq!(dur!(10us), Duration::from_micros(10));
///    assert_eq!(dur!(10ns), Duration::from_nanos(10));
/// }
/// ```
///
/// [`Duration`]: ::core::time::Duration
#[proc_macro]
pub fn dur(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    time::dur(ts)
}

/// Syntactic sugar for [`sleep`].
///
/// # Examples
/// ```rust
/// use rka_fn::sleep;
/// use core::time::Duration;
///
/// fn main() {
///    loop {
///        sleep!(3s);
///        return;
///    }
/// }
/// ```
///
/// [`sleep`]: ::std::thread::sleep
#[proc_macro]
pub fn sleep(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    time::sleep(ts)
}

/// Display [`TokenStream`] for debugging.
///
/// # Examples
/// ```rust
/// use rka_fn::dev;
///
/// fn main() {
///    dev!{ return 123; };
/// }
/// ```
///
/// [`TokenStream`]: ::proc_macro::TokenStream
#[proc_macro]
pub fn dev(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dbg!(&ts);

    proc_macro::TokenStream::new()
}

/// Ignore anything.
///
/// # Examples
/// ```rust
/// use rka_fn::draft;
///
/// fn main() {
///    draft!{ return 123; }
///    println!("hi");
/// }
/// ```
#[proc_macro]
pub fn draft(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

//impl Parse for SynDur {
//    fn parse(input: ParseStream) -> Result<Self> {
//        Ok(Self {})
//    }
//}

//let input = parse_macro_input!(ts as SynDur);
//

// TODO:
// f()
//   .{ f1(this); this }
//   .{ f2(this, v1, v2) }
//   .{ this + 3 }
// ;
#[proc_macro]
pub fn postfix_block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    opt::postfix::block(ts)
}

// TODO:
#[proc_macro]
pub fn postfix_keyword_block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    opt::postfix::keyword_block(ts)
}

// TODO:
// f()
//   .match this.as_str() {
//       "a" => "A",
//       "b" => "B",
//       _ => "",
//   }
//   .to_string()
// ;
#[proc_macro]
pub fn postfix_match_block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    opt::postfix::postfix_match_block(ts)
}

// DROP:
// return <lifetime> <expr>;
// e.g.
//     return 'a 1234;
#[proc_macro]
pub fn return_from_lifetime(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn named_args(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    opt::named_args::parse(ts)
}
