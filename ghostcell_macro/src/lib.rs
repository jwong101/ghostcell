// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate proc_macro;

mod entry;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ghost(args: TokenStream, input: TokenStream) -> TokenStream {
    entry::main(args, input)
}
