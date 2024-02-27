use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use syn::*;

pub fn parse(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f: SynFn = syn::parse(ts).unwrap();
    let call = &f.call;

    let fn_named = &f.named;
    let call_named = &call.named;
    let in_brace = &f.in_brace;

    let flag_fix_order = call_named.len() <= fn_named.len();

    let ts_f = {
        let SynFn {
            fn_name,
            bind,
            unnamed,
            named,
            ..
        } = &f;

        let ts_unnamed = {
            quote! { #unnamed }
        };
        let ts_named = {
            quote! { #named }
        };
        let ts_in_brace = {
            if named.is_empty() {
                quote! { #in_brace }
            } else {
                let mut tmp: Punctuated<&Ident, Token![,]> = Punctuated::new();
                for SynFnArg { ident, .. } in fn_named.iter() {
                    tmp.push(ident);
                }

                quote! {
                    let #bind @ ( #tmp ) = ( #tmp );

                    #in_brace
                }
            }
        };

        quote! {
            fn #fn_name( #ts_unnamed #ts_named ) {
                #ts_in_brace
            }
        }
    };
    let ts_call = {
        let SynCall {
            call_name,
            trait_call,
            unnamed,
            ..
        } = &call;

        let flag_fix_trait = trait_call.is_some() && f.named_impl_trait.is_some();

        let ts_unnamed = {
            quote! { #unnamed }
        };
        let ts_named = {
            let mut fixed_named: Punctuated<TokenStream, Token![,]> = Punctuated::new();

            // fixed order
            if flag_fix_order && !flag_fix_trait {
                'l1: for fn_named in fn_named.iter() {
                    for call_named in call_named.iter() {
                        if fn_named.ident == call_named.ident {
                            let expr = &call_named.expr;
                            fixed_named.push(quote! { #expr });

                            continue 'l1;
                        }
                    }
                }

                quote! { #fixed_named }
            }
            // fixed order
            // fixed trait
            else if flag_fix_order && flag_fix_trait {
                'l1: for fn_named in fn_named.iter() {
                    for call_named in call_named.iter() {
                        if fn_named.ident == call_named.ident {
                            let expr = &call_named.expr;
                            fixed_named.push(quote! { #expr });

                            continue 'l1;
                        }
                    }

                    let ty = &fn_named.ty;

                    // fixed trait
                    let SynCallNamedTrait { ident, fn_ident } = trait_call.as_ref().unwrap();
                    let fn_trait_name = f.named_impl_trait.as_ref().unwrap();
                    let call_trait_name = ident;

                    assert_eq!(fn_trait_name, call_trait_name);
                    let call_trait_fn = fn_ident;
                    fixed_named.push(quote! {
                        <#ty as #call_trait_name>::#call_trait_fn()
                    })
                }

                //dbg!(&fixed_named);
                quote! { #fixed_named }
            } else {
                panic!()
            }
        };

        quote! {
            #call_name( #ts_unnamed #ts_named )
        }
    };

    let ts = quote! {
        {
            #ts_f

            #ts_call
        }
    };
    //dbg!(&ts);

    ts.into()
}

#[derive(Debug)]
struct SynFn {
    _kw: Token![fn],
    fn_name: Ident,

    _p: Paren,
    unnamed: Punctuated<SynFnArg, Token![,]>,
    bind: Option<Ident>,
    named: Punctuated<SynFnArg, Token![,]>,
    named_impl_trait: Option<Ident>,

    _b: Brace,
    in_brace: TokenStream,

    call: SynCall,
}

#[derive(Debug)]
struct SynCall {
    call_name: Ident,

    _p: Paren,
    unnamed: Punctuated<SynCallUnnamed, Token![,]>,
    named: Punctuated<SynCallNamed, Token![,]>,

    trait_call: Option<SynCallNamedTrait>,
}

#[derive(Debug)]
struct SynFnArg {
    ident: Ident,
    ty: Path,
}

#[derive(Debug)]
struct SynCallUnnamed {
    expr: Expr,
}

#[derive(Debug)]
struct SynCallNamed {
    ident: Ident,
    expr: Expr,
}

#[derive(Debug)]
struct SynCallNamedTrait {
    ident: Ident,
    fn_ident: Ident,
}

impl Parse for SynFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let ts_paren; // ( ... )
        let body; //
        let mut bind = None;

        Ok(Self {
            _kw: input.parse()?,
            fn_name: input.parse()?,
            _p: parenthesized!(ts_paren in input),
            unnamed: {
                let mut res = Punctuated::new();

                // Ident : Path ,
                while let Ok(ident) = ts_paren.parse() {
                    if ts_paren.peek(Token![:]) {
                        let _: Token![:] = ts_paren.parse()?;
                        let ty: Path = ts_paren.parse()?;

                        res.push_value(SynFnArg { ident, ty });

                        if ts_paren.peek(Token![,]) {
                            res.push_punct(ts_paren.parse()?);
                        }
                    } else if ts_paren.peek(Token![@]) {
                        let _: Token![@] = ts_paren.parse()?;
                        bind = Some(ident);
                        break;
                    }
                }

                res
            },
            named: {
                let mut res: Punctuated<SynFnArg, Token![,]> = Punctuated::new();

                if bind.is_none() {
                } else {
                    let ts_brace;
                    braced!(ts_brace in ts_paren);

                    // Ident : Path ,
                    while let Ok(i) = ts_brace.parse() {
                        let _: Token![:] = ts_brace.parse()?;
                        let ty: Path = ts_brace.parse()?;

                        res.push_value(SynFnArg { ident: i, ty });

                        if ts_brace.peek(Token![,]) {
                            res.push_punct(ts_brace.parse()?);
                        }
                    }
                }

                res
            },
            named_impl_trait: {
                if bind.is_none() {
                    None
                } else {
                    if !ts_paren.peek(Token![:]) {
                        panic!()
                    }

                    let _: Token![:] = ts_paren.parse()?;
                    let _: Token![impl] = ts_paren.parse()?;
                    let ident = ts_paren.parse()?;

                    if ts_paren.peek(Token![,]) {
                        let _: Token![,] = ts_paren.parse()?;
                    }

                    Some(ident)
                }
            },
            bind,
            _b: braced!(body in input),
            in_brace: body.parse()?,
            call: input.parse()?,
        })
    }
}

impl Parse for SynFnArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for SynCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args;
        let mut trait_call = None;

        Ok(SynCall {
            call_name: input.parse()?,
            _p: parenthesized!(args in input),
            unnamed: {
                let mut res = Punctuated::new();

                while let Ok(expr) = args.parse() {
                    res.push_value(SynCallUnnamed { expr });

                    if args.peek(Token![,]) {
                        res.push_punct(args.parse()?);
                    } else {
                        break;
                    }
                }

                res
            },

            // e.g.
            //   `@ { a: 1, b: 2, }: impl Default`
            named: {
                let mut res = Punctuated::new();

                if args.peek(Token![@]) {
                    let _: Token![@] = args.parse()?;
                    let brace_args;
                    let _ = braced!(brace_args in args);

                    while let Ok(ident) = brace_args.parse() {
                        let _: Token![:] = brace_args.parse()?;
                        let expr: Expr = brace_args.parse()?;

                        res.push_value(SynCallNamed { ident, expr });
                        res.push_punct(brace_args.parse()?);
                    }

                    if brace_args.peek(Token![..]) {
                        let _: Token![..] = brace_args.parse()?;
                        let ident = brace_args.parse()?;
                        let _: Token![::] = brace_args.parse()?;
                        let fn_ident = brace_args.parse()?;

                        let _v;
                        parenthesized!(_v in brace_args);

                        trait_call = Some(SynCallNamedTrait { ident, fn_ident });
                    }
                }

                res
            },
            trait_call,
        })
    }
}

impl ToTokens for SynFnArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let l = &self.ident;
        let r = &self.ty;
        tokens.extend(quote! {
            #l: #r
        })
    }
}

impl ToTokens for SynCallUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { expr } = self;
        tokens.extend(quote! { #expr })
    }
}

impl Parse for SynCallUnnamed {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

impl ToTokens for SynCallNamed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { expr, .. } = self;
        tokens.extend(quote! { #expr })
    }
}
