use proc_macro::{TokenStream, TokenTree};
use syn::{punctuated::Punctuated, ItemFn};
use twox_hash::xxh3::hash128;

#[proc_macro_attribute]
pub fn protected(attr: TokenStream, fn_ts: TokenStream) -> TokenStream {
    println!("{:?}", attr);
    let mut attr = attr.into_iter();
    let prot_type = if let Some(prot_type @ TokenTree::Ident { .. }) = attr.next() {
        prot_type.to_string()
    } else {
        panic!("missing protection type")
    };

    if !["mutate", "virtualize", "ultra"].contains(&prot_type.as_ref()) {
        panic!("unknown protection type: {}", prot_type);
    }

    let lock = match attr.next() {
        Some(TokenTree::Punct(..)) => {
            if let Some(lock @ TokenTree::Ident { .. }) = attr.next() {
                if lock.to_string() != "lock" {
                    panic!("only possible variant is lock")
                }
                true
            } else {
                panic!("only possible variant is lock");
            }
        }

        Some(_) => panic!("expected lock declaration"),
        None => false,
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse(fn_ts).expect("failed to parse as fn");
    let mut name = "vmprotect_".to_owned();
    name.push_str(&prot_type);
    if lock {
        name.push_str("_lock");
    }
    name.push('_');

    name.push_str(&format!("{}", hash128(sig.ident.to_string().as_bytes())));
    let wrapped_ident = syn::Ident::new(&name, sig.ident.span());
    let wrapper = syn::ItemFn {
        attrs: attrs.clone(),
        vis,
        sig: sig.clone(),
        block: {
            let mut args: Punctuated<_, syn::token::Comma> = Punctuated::new();
            for arg in &sig.inputs {
                args.push(match arg {
                    syn::FnArg::Receiver(_r) => panic!("not supported on trait/struct members"),
                    syn::FnArg::Typed(t) => t.pat.clone(),
                });
            }

            syn::parse(
                (quote::quote! {
                    {#wrapped_ident(#args)}
                })
                .into(),
            )
            .unwrap()
        },
    };
    let mut wrapped_sig = sig;
    wrapped_sig.ident = wrapped_ident;
    let wrapped = syn::ItemFn {
        attrs,
        vis: syn::Visibility::Inherited,
        sig: wrapped_sig,
        block,
    };
    (quote::quote! {
        #[inline(never)]
        #[no_mangle]
        #[doc(hidden)]
        #wrapped

        #[inline(always)]
        #wrapper
    })
    .into()
}
