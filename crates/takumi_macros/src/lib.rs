use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitInt};

fn parse_packet_id(input: &DeriveInput) -> syn::Result<i32> {
    for attr in &input.attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        let mut id = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value: LitInt = meta.value()?.parse()?;
                id = Some(lit_int_to_i32(&value)?);
            }
            Ok(())
        })?;

        if let Some(id) = id {
            return Ok(id);
        }
    }

    Err(syn::Error::new(
        input.ident.span(),
        "missing #[packet(id = ...)] attribute",
    ))
}

fn lit_int_to_i32(value: &LitInt) -> syn::Result<i32> {
    let s = value.to_string();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i32::from_str_radix(hex, 16)
            .map_err(|_| syn::Error::new(value.span(), "invalid hex packet id"))
    } else {
        s.parse()
            .map_err(|_| syn::Error::new(value.span(), "invalid packet id"))
    }
}

fn expand_packet_meta(input: &DeriveInput, direction: TokenStream2) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let id = parse_packet_id(input)?;

    Ok(quote! {
        impl crate::packet::PacketMeta for #name {
            const ID: i32 = #id;
            const DIRECTION: crate::packet::PacketDirection = #direction;
        }
    })
}

#[proc_macro_derive(PacketIn, attributes(packet))]
pub fn derive_packet_in(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_packet_meta(
        &input,
        quote! { crate::packet::PacketDirection::In },
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}

#[proc_macro_derive(PacketOut, attributes(packet))]
pub fn derive_packet_out(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_packet_meta(
        &input,
        quote! { crate::packet::PacketDirection::Out },
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}
