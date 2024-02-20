use proc_macro2::{Group, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input, Expr, Item,
};

pub fn block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //dbg!(&ts);

    let s = parse_macro_input!(ts as syn::Expr);
    //dbg!(&s);

    let mut ts = quote! {};
    <Expr as ExpandPostfixBlock>::expand(&s, &mut ts);
    //dbg!(&ts);

    //    let f = syn::parse2::<syn::File>(quote! {
    //        fn main() {
    //            let res = {
    //                #t
    //
    //                this
    //            };
    //        }
    //    })
    //    .unwrap();
    //    let f = prettyplease::unparse(&f);
    //    println!("{f}");

    quote! {
        // This is block.
        {
            #ts

            // Return vaule.
            this
        }
    }
    .into()
}

pub fn keyword_block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

trait ExpandPostfixBlock {
    fn expand(&self, ts: &mut TokenStream);
}

impl ExpandPostfixBlock for syn::Expr {
    fn expand(&self, ts: &mut TokenStream) {
        match self {
            Self::MethodCall(v) => v.expand(ts),
            _ => {
                *ts = quote! {
                    let mut this = { #self };
                }
            }
        }
    }
}

pub fn postfix_match_block(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

impl ExpandPostfixBlock for syn::ExprMethodCall {
    fn expand(&self, ts: &mut TokenStream) {
        self.receiver.expand(ts);

        if self.method == "this" {
            let expr = {
                if self.args.is_empty() {
                    quote! {}
                } else {
                    (&self.args[0]).into_token_stream()
                }
            };

            *ts = quote! {
                let mut this = {
                    #ts

                    #expr

                    this
                };
            };
        } else if self.method == "pipe" {
            let res = &self.args[0];
            *ts = quote! {
                let mut this = {
                    #ts

                    #res
                };
            };
        } else {
            let method = &self.method;
            let turbofish = {
                if let Some(v) = &self.turbofish {
                    quote! { #v }
                } else {
                    quote! {}
                }
            };
            let args = &self.args;

            *ts = quote! {
                let mut this = {
                    #ts

                    this
                }. #method #turbofish (#args);
            };
        }
    }
}
