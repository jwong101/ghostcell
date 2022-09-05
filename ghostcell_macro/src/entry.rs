// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    ItemFn, Token,
};

struct GhostToken {
    mutable: Option<Token![mut]>,
    ident: Ident,
}

impl Parse for GhostToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mutable = input.parse::<Token![mut]>().ok();
        let ident = input.parse::<Ident>()?;
        Ok(Self { mutable, ident })
    }
}

pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let ghost_tokens = Punctuated::<GhostToken, Token![,]>::parse_terminated
        .parse(args)
        .unwrap();
    let input = parse_macro_input!(item as ItemFn);
    let new_fun = input.clone();
    let stmts = new_fun.block.stmts;
    let block = quote! {
        #(#stmts)*
    };
    let new_block = ghost_tokens.into_iter().fold(block, |block, ghost_token| {
        let ident = ghost_token.ident;
        let mutable = ghost_token.mutable;
        quote! {
            <GhostToken>::with(|#mutable #ident| {
                #block
            })
        }
    });
    let ItemFn {
        attrs,
        vis,
        sig,
        block: _,
    } = input;
    let expanded = quote! {
        #(#attrs)*
        #vis #sig
        {
            #new_block
        }
    };
    expanded.into()
}
