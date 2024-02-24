// TODO:

use proc_macro2::{Group, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input, Item,
};

pub fn size(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tmp = ts.to_string();

    SynSize::from_str(&tmp).into_ts()
}

struct SynSize {
    num: u64,
    suffix: String,
}

impl SynSize {
    fn from_str(v: &str) -> Self {
        let iter = v.chars();

        let mut num = Vec::with_capacity(10);
        let mut suffix = Vec::with_capacity(2);
        for c in iter {
            match c {
                '0'..='9' => num.push(c),
                'a'..='z' => suffix.push(c),

                _ => unreachable!(),
            }
        }

        Self {
            num: String::from_iter(num).parse().unwrap(),
            suffix: String::from_iter(suffix),
        }
    }

    fn into_ts(self) -> proc_macro::TokenStream {
        let Self { num, suffix } = self;

        let (f, num) = {
            match suffix.as_str() {
                "ns" => (quote! { from_nanos }, num),
                "us" => (quote! { from_micros }, num),
                "ms" => (quote! { from_millis }, num),
                "s" => (quote! { from_secs }, num),
                "m" => (quote! { from_secs }, num * 60),
                "h" => (quote! { from_secs }, num * 60 * 60),
                "d" => (quote! { from_secs }, num * 60 * 60 * 24),
                "w" => (quote! { from_secs }, num * 60 * 60 * 24 * 7),

                _ => unreachable!(),
            }
        };

        quote! {
            ::core::time::Sizeation::#f(#num)
        }
        .into()
    }
}
