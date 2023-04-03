extern crate proc_macro;

// represet a sequence of tokes
// take some inp code manipuilate return tokens stream
use proc_macro::TokenStream;
use quote::quote; //token stream out of syntax tree
use syn::{parse_macro_input, ItemFn}; //syn puts on a syntax tree

#[proc_macro_attribute]
pub fn debug_print(_att: TokenStream, item: TokenStream) -> TokenStream {
    // item contains the rust code
    let mut item_fn = parse_macro_input!(item as ItemFn);

    let ident = &item_fn.sig.ident; // gets the funcion name

    item_fn.block.stmts.insert(
        0,
        syn::parse_quote!(println!("ENTENRING FUNCTION {}", stringfy!(#ident)))
    );
    TokenStream::from(quote! {
        #item_fn
    })
}