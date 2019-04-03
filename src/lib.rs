extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};


#[proc_macro_attribute]
pub fn ex_simple(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("[m] attr: {:?}", attr);
    println!("[m] item: {:?}", item);
    "fn answer() -> u32 {42}".parse().unwrap()
}


#[proc_macro_attribute]
pub fn ex_0(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("[m] attr: {:?}", attr);
    println!("[m] item: {:?}", item);
    item
}



#[proc_macro_attribute]
pub fn ex_1(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);
    let item2 = item.clone();
    let input = parse_macro_input!(item2 as ItemFn);
    let name = &input.ident;
    let parsed_attr_name = attr_args[0].clone();
    println!("name->{:}",name);
    println!("name_function->{:?}", attr_args[0]);
    println!("attr_args->{:?}", attr_args);
    println!("input->{:?}", input);
    let result = quote! {
        fn my_new_answer() -> u32 { 21 }
        fn #parsed_attr_name() -> u32 { 10 }
        fn #name() -> u32 { 11 }
    };
    result.into()
}

