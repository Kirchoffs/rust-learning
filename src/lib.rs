extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn print_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    /*
        // This version does not call the original function
        let input = parse_macro_input!(item as ItemFn);
        let name = &input.sig.ident;
        let expanded = quote! {
            fn #name() {
                println!("Executing function: {}", stringify!(#name));
                #input
            }
        };
        TokenStream::from(expanded)
    */
    
    let mut input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let stmt = syn::parse2(quote! {
        println!("Executing function: {}", stringify!(#name));
    }).unwrap();
    input.block.stmts.insert(0, stmt);
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}
