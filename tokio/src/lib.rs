extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, ExprTuple, ItemFn};
use quote::{quote, ToTokens};
use rand::Rng;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    if input.sig.ident != "main" {
        return syn::Error::new_spanned(
            &input.sig.ident,
            "#[tokyo::main] must be applied to `async fn main`",
        )
            .to_compile_error()
            .into();
    }

    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &input.sig.fn_token,
            "`main` must be declared as `async fn main`",
        )
            .to_compile_error()
            .into();
    }

    if !input.sig.inputs.is_empty() {
        let bad = input.sig.inputs.first().unwrap();
        return syn::Error::new_spanned(bad, "the main function cannot accept arguments")
            .to_compile_error()
            .into();
    }

    input.sig.asyncness = None;

    if let Err(_real) = check_body_end(&input) {
        let mut rng = rand::rng();
        let which: usize = rng.random_range(0..7);
        let gaslight = emit_fake_error(&input.block, which);

        return quote! {
            #input
            #gaslight
        }
            .into();
    }

    quote!(#input).into()
}

use syn::{Expr, ExprBlock,  ExprLit, Lit, Stmt};
use syn::__private::TokenStream2;

fn check_body_end(input: &ItemFn) -> Result<(), syn::Error> {
    let stmts = &input.block.stmts;

    if stmts.len() < 2 {
        return Err(syn::Error::new_spanned(
            &input.sig.ident,
            "",
        ));
    }

    let block_stmt = &stmts[stmts.len() - 2];
    match block_stmt {
        Stmt::Expr(Expr::Block(ExprBlock { block, .. }))
        | Stmt::Semi(Expr::Block(ExprBlock { block, .. }), _) => {
            if block.stmts.len() != 1 {
                return Err(syn::Error::new_spanned(
                    block,
                    "",
                ));
            }
            match &block.stmts[0] {
                Stmt::Semi(Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }), _)
                if lit_int.base10_parse::<i32>().ok() == Some(5) => {}
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "",
                    ))
                }
            }
        }
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "",
            ))
        }
    }

    let last_stmt = &stmts[stmts.len() - 1];
    let expr = match last_stmt {
        Stmt::Expr(expr) => expr,
        Stmt::Semi(expr, _) => expr,
        _ => {
            return Err(syn::Error::new_spanned(
                last_stmt,
                "",
            ))
        }
    };
    match expr {
        Expr::Tuple(ExprTuple { elems, .. }) if elems.is_empty() => Ok(()),
        other => Err(syn::Error::new_spanned(
            other,
            "",
        )),
    }
}

fn emit_fake_error<T: ToTokens>(span_target: &T, which: usize) -> TokenStream2 {
    let (code, msg, link) = match which {
        0 => (
            "E0106",
            "missing lifetime specifier",
            "https://doc.rust-lang.org/error_codes/E0106.html",
        ),
        1 => (
            "E0507",
            "cannot move out of `()` which is behind a shared reference",
            "https://doc.rust-lang.org/error_codes/E0507.html",
        ),
        2 => (
            "E0499",
            "cannot borrow `main` as mutable more than once at a time",
            "https://doc.rust-lang.org/error_codes/E0499.html",
        ),
        _ => (
            "E0703",
            "invalid ABI: found `()` where ABI string expected",
            "https://doc.rust-lang.org/error_codes/E0703.html",
        ),
    };

    let text = format!(
        "error[{code}]: {msg}\n\
         \n\
         = note: for more information, see {link}"
    );

    syn::Error::new_spanned(span_target, text).to_compile_error()
}