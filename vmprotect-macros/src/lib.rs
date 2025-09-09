use std::ffi::CString;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned as _;
use syn::visit_mut::VisitMut;
use syn::{
    braced, parse_macro_input, Attribute, FnArg, LitCStr, LitStr, Pat, PatIdent, Path, Signature,
    Token, Visibility,
};
use twox_hash::XxHash3_64;

struct ItemFnMini {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: TokenStream,
}
impl Parse for ItemFnMini {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let content;
        let _brace = braced!(content in input);
        let block = content.parse()?;
        Ok(Self {
            attrs,
            vis,
            sig,
            block,
        })
    }
}

mod kw {
    syn::custom_keyword!(mutate);
    syn::custom_keyword!(virtualize);
    syn::custom_keyword!(ultra);
    syn::custom_keyword!(destroy);
    syn::custom_keyword!(lock);
}

enum ProtectionType {
    Mutate,
    Virtualize,
    Ultra,
    Destroy,
}
impl ProtectionType {
    fn name(&self) -> &'static str {
        match self {
            ProtectionType::Mutate => "mutate",
            ProtectionType::Virtualize => "virtualize",
            ProtectionType::Ultra => "ultra",
            ProtectionType::Destroy => "destroy",
        }
    }
}
impl Parse for ProtectionType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(kw::mutate) {
            input.parse::<kw::mutate>()?;
            Self::Mutate
        } else if lookahead.peek(kw::virtualize) {
            input.parse::<kw::virtualize>()?;
            Self::Virtualize
        } else if lookahead.peek(kw::ultra) {
            input.parse::<kw::ultra>()?;
            Self::Ultra
        } else if lookahead.peek(kw::destroy) {
            input.parse::<kw::destroy>()?;
            Self::Destroy
        } else {
            return Err(lookahead.error());
        })
    }
}
struct ProtectMacroAttr {
    ty: ProtectionType,
    lock: bool,
}
impl Parse for ProtectMacroAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: ProtectionType = input.parse()?;
        let lookahead = input.lookahead1();
        let lock = if lookahead.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<kw::lock>()?;
            true
        } else if !input.is_empty() {
            return Err(lookahead.error());
        } else {
            false
        };
        Ok(Self { ty, lock })
    }
}

#[proc_macro]
pub fn marker_name(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let s = parse_macro_input!(s as LitStr);
    let Ok(cstring) = CString::new(s.value()) else {
        return quote_spanned! {s.span() => compile_error!("marker name should not include internal NULs")}.into();
    };
    let s = LitCStr::new(cstring.as_c_str(), s.span());

    quote! {::core::ffi::CStr::as_ptr(#s)}.into()
}

enum WrappedVisitorMode {
    RetainNormal,
    RetainWrapped,
}
struct WrappedAttrVisitor {
    mode: WrappedVisitorMode,
    found_normal: bool,
    found_wrapped: bool,
}
impl WrappedAttrVisitor {
    fn retain_wrapped() -> Self {
        Self {
            mode: WrappedVisitorMode::RetainWrapped,
            found_normal: false,
            found_wrapped: false,
        }
    }
    fn retain_normal() -> Self {
        Self {
            mode: WrappedVisitorMode::RetainNormal,
            found_normal: false,
            found_wrapped: false,
        }
    }
}
impl VisitMut for WrappedAttrVisitor {
    fn visit_attributes_mut(&mut self, i: &mut Vec<syn::Attribute>) {
        i.retain(|a| {
            let wrapped = a.path().is_ident("wrapped");
            self.found_normal |= !wrapped;
            self.found_wrapped |= wrapped;
            match self.mode {
                WrappedVisitorMode::RetainNormal => !wrapped,
                WrappedVisitorMode::RetainWrapped => wrapped,
            }
        });
    }
}

#[proc_macro_attribute]
pub fn protected(
    attr: proc_macro::TokenStream,
    fn_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as ProtectMacroAttr);

    let ItemFnMini {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(fn_ts as ItemFnMini);

    // For functions with the same name, we want a deterministic, yet unique identifier
    // Until there is no way to look at span position, we can just use its debug repr.
    // Next Rust release it will be possible to do that: https://github.com/rust-lang/rust/pull/140514
    let s = format!("{:?}", sig.ident.span());
    let id = XxHash3_64::oneshot(s.as_bytes());

    let wrapped_ident = format_ident!(
        "VMPROTECT_MARKER_{}{}_END_{id}",
        attr.ty.name(),
        if attr.lock { "_lock" } else { "" },
    );

    let mut wrapped_attrs = attrs.clone();
    let mut wrapped_sig = sig.clone();
    {
        let mut visitor = WrappedAttrVisitor::retain_wrapped();
        visitor.visit_attributes_mut(&mut wrapped_attrs);
        visitor.visit_signature_mut(&mut wrapped_sig);
    }
    wrapped_sig.ident = wrapped_ident.clone();

    let mut wrapper_attrs = attrs.clone();
    let mut wrapper_sig = sig;
    {
        let mut visitor = WrappedAttrVisitor::retain_normal();
        visitor.visit_attributes_mut(&mut wrapper_attrs);
        // Signature is handled below
    }

    let should_inline_wrapper = !wrapper_attrs.iter().any(|v| {
        let mut path = v.path().to_owned();
        if path.is_ident("unsafe") {
            if let Ok(ipath) = v.parse_args::<Path>() {
                path = ipath;
            }
        }
        path.is_ident("inline")
            || path.is_ident("no_mangle")
            || path.is_ident("export_name")
            || path.is_ident("link_section")
    });
    let wrapper_inline_attr = should_inline_wrapper.then(|| quote!(#[inline(always)]));

    if let Some(gt) = wrapped_sig.generics.gt_token {
        // Maybe provide some macro for easier multi-specialization?
        // TODO: Allow lifetime annotations
        return quote_spanned! {gt.span() => compile_error!("Protected functions don't support generics.\nCreate manually monomorphized versions of the function, and then protect those.")}.into();
    }
    if let Some(asyncness) = wrapped_sig.asyncness {
        // TODO: Document how does it work, limitations et cetera?
        return quote_spanned! {asyncness.span() => compile_error!("VMProtect won't work as you would expect with async functions.")}.into();
    }

    if let Some(receiver) = wrapped_sig.receiver() {
        // In theory it should be possible to do
        //
        // fn wrapper(self: &Ty) {
        //    fn wrapped(this: &Ty) {}
        // }
        //
        // I.e require user to specify full receiver type, and then rename `self` in code to `this`,
        // but this it too complicated.
        return quote_spanned! {receiver.span() => compile_error!("Receivers are not supported on protected functions.\nExtract your impl block function, and pass the call arguments manualy.")}.into();
    }
    let mut args = vec![];
    for (i, ele) in wrapper_sig.inputs.iter_mut().enumerate() {
        let mut visitor = WrappedAttrVisitor::retain_normal();
        let FnArg::Typed(t) = ele else {
            unreachable!("receivers are forbidden");
        };
        match &mut *t.pat {
            Pat::Ident(PatIdent {
                attrs,
                ident,
                subpat: None,
                by_ref,
                mutability,
            }) => {
                visitor.visit_attributes_mut(attrs);
                *by_ref = None;
                *mutability = None;
                args.push(ident.clone());
            }
            p => {
                visitor.visit_pat_mut(p);
                if visitor.found_normal {
                    return quote_spanned! {p.span() => compile_error!("Non-wrapped attributes are only supported on non-pattern arguments")}.into();
                }
                let ident = format_ident!("unnamed_{i}");
                args.push(ident.clone());
                t.pat = Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident,
                    subpat: None,
                }))
            }
        }
    }

    (quote! {
        #wrapper_inline_attr
        #(#wrapper_attrs)*
        #vis
        #wrapper_sig
        {
            #[inline(never)]
            #[doc(hidden)]
            #(#wrapped_attrs)*
            #wrapped_sig
            { #block }
            #wrapped_ident(#(#args),*)
        }


    })
    .into()
}
