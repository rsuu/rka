use std::str::FromStr;

#[proc_macro_attribute]
pub fn postfix_block(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attr: `{attr}`");
    println!("item: `{item}`");

    proc_macro::TokenStream::from_str("").unwrap()
}
