use proc_macro2::TokenStream;
use quote::format_ident;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::*;

pub struct FnWhere {
    kw_pub: Option<Token![pub]>,
    kw_fn: Token![fn],

    fn_name: Ident,

    paren_token: token::Paren,
    fn_fields: Punctuated<Arg, Token![,]>,

    return_token: Option<Token![->]>,
    return_type: Option<Type>,

    kw_where: Option<Token![where]>,
    where_fields: Punctuated<ArgWhereField, Token![,]>,

    brace_token: token::Brace,
    brace_body: TokenStream,
}

struct Arg {
    name: Ident,
    token: Token![:],
    ty: Type,
}

struct ArgWhereField {
    ty: ArgWhereTy,
    token: Token![:],
    traits: Punctuated<ArgWhereTrait, Token![,]>,
}

enum ArgWhereTy {
    Ident(syn::Ident),
    KwFn(Token![fn]),
}

enum ArgWhereTrait {
    Ident(syn::Ident),
    KwAsync(Token![async]),
}

impl syn::parse::Parse for FnWhere {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_fields;
        let return_token: Option<Token![->]>;
        let brace_body;

        Ok(Self {
            kw_pub: input.parse()?,
            kw_fn: input.parse()?,

            fn_name: input.parse()?,

            paren_token: parenthesized!(fn_fields in input),
            fn_fields: fn_fields.parse_terminated(Arg::parse, Token![,])?,

            return_token: {
                return_token = input.parse()?;
                return_token.clone()
            },
            return_type: {
                if return_token.is_some() {
                    Some(input.parse()?)
                } else {
                    None
                }
            },

            kw_where: input.parse()?,
            where_fields: {
                let mut res = Punctuated::new();

                loop {
                    if input.is_empty() || input.peek(token::Brace) || input.peek(Token![,]) {
                        break;
                    }

                    let v = input.parse()?;
                    input.parse::<Token![,]>()?;
                    res.push(v);
                }

                res
            },
            brace_token: braced!(brace_body in input),
            brace_body: brace_body.parse()?,
        })
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl syn::parse::Parse for ArgWhereTy {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(if input.peek(Token![fn]) {
            Self::KwFn(input.parse()?)
        } else {
            Self::Ident(input.parse()?)
        })
    }
}

impl Parse for ArgWhereField {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            token: input.parse()?,
            traits: {
                let mut res = Punctuated::new();
                loop {
                    if input.is_empty() || input.peek(token::Brace) || input.peek(Token![,]) {
                        break;
                    }

                    let v = if input.peek(Token![async]) {
                        ArgWhereTrait::KwAsync(input.parse()?)
                    } else {
                        ArgWhereTrait::Ident(input.parse()?)
                    };
                    res.push(v);

                    while let Ok(..) = input.parse::<Token![+]>() {
                        if input.is_empty() || input.peek(token::Brace) || input.peek(Token![,]) {
                            break;
                        }

                        let v = if input.peek(Token![async]) {
                            ArgWhereTrait::KwAsync(input.parse()?)
                        } else {
                            ArgWhereTrait::Ident(input.parse()?)
                        };
                        res.push(v);
                    }
                }

                res
            },
        })
    }
}
