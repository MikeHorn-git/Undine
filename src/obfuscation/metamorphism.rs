/*
   https://nu11busters.github.io/rust-maldev-course/obfuscation/metamorphism/
   https://onlinelibrary.wiley.com/doi/10.1155/2023/8227751
*/

use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn metamorphism(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let block = &input.block;

    let mut modified_statements = quote! {};
    for stmt in &block.stmts {
        let num = rand::rng().random_range(1..10);
        let mut dead = quote! {};
        for _ in 0..=num {
            dead = quote! {
                #dead
                {
                    let mut r: u32 = 0;
                    unsafe {
                        core::arch::asm!(
                            "mov {r}, 0",
                            "push rbx",
                            "nop",
                            "pop rbx",
                            "add {r}, 5",
                            "sub {r}, 5",
                            r = inout(reg) r,
                            options(nostack, preserves_flags),
                        );
                    }
                }
            }
        }

        modified_statements = quote! {
            #modified_statements
            #stmt
            unsafe {
                #dead
            };
        };
    }

    quote! {
        fn #fn_name() {
            #modified_statements
        }
    }
    .into()
}
