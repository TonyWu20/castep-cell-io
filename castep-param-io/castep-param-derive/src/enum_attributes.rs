use darling::FromAttributes;
use quote::quote;
use syn::{DataEnum, Ident};

#[derive(FromAttributes, Default)]
#[darling(default, attributes(keyword_display))]
pub struct EnumAttrs {
    pub field: String,
    pub display_format: Option<String>,
}

pub fn data_enum_display_impl(
    data_enum: &DataEnum,
    struct_ident: &Ident,
) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|v| {
        let name = &v.ident;
        let opts = EnumAttrs::from_attributes(&v.attrs).expect("Wrong attrs");
        let display_format = if let Some(s) = opts.display_format {
            quote!(#s)
        } else {
            quote!("{}")
        };
        match &v.fields {
            // Don't expect this in enum
            syn::Fields::Named(_) => unimplemented!(),
            syn::Fields::Unnamed(_) => quote! {
            #struct_ident::#name(t) => write!(f, #display_format, t)},
            syn::Fields::Unit => quote! {#struct_ident::#name => write!(f, "{:?}", self)},
        }
    });
    quote! {
        impl std::fmt::Display for #struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#variants,)*
                }
            }
        }
    }
}

pub fn data_enum_field_impl(
    data_enum: &DataEnum,
    struct_ident: &Ident,
) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|v| {
        let name = &v.ident;
        let variant_expr = match v.fields {
            // Don't expect this in enum
            syn::Fields::Named(_) => unimplemented!(),
            syn::Fields::Unnamed(_) => quote! {#struct_ident::#name(_)},
            syn::Fields::Unit => quote! {#struct_ident::#name},
        };
        let opts = EnumAttrs::from_attributes(&v.attrs).expect("Wrong attrs");
        let field = opts.field;
        quote! {
            #variant_expr => #field.to_string()
        }
    });
    quote! {
        fn field(&self) -> String {
            match self {
                #(#variants,)*
            }
        }
    }
}
