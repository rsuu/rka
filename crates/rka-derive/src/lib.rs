mod parse_fn_where;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use parse_fn_where::FnWhere;

#[proc_macro]
pub fn rka(ts: TokenStream1) -> TokenStream1 {
    let ts = TokenStream::from(ts);

    quote! { #ts }.into()
}

#[proc_macro]
pub fn parse_fn_where(ts: TokenStream1) -> TokenStream1 {
    let ts = TokenStream::from(ts);
    dbg!(&ts);

    let f: FnWhere = syn::parse2(ts.clone()).unwrap();
    quote! {}.into()
}
