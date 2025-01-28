use enum_attributes::{data_enum_display_impl, data_enum_field_impl};
use proc_macro::{self, TokenStream};
use quote::quote;
use strip_option::extract_type_from_option;
use syn::{parse_macro_input, DeriveInput, Expr, Ident, Lit};

mod enum_attributes;
mod param_display_impl;
mod strip_option;

use darling::{FromAttributes, FromDeriveInput};
use param_display_impl::{ParamFieldOpt, ParamOpt};

#[proc_macro_derive(ParamDisplay, attributes(param_display))]
pub fn derive_param_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let struct_opts = ParamOpt::from_derive_input(&input).expect("Wrong options");
    let data = if let syn::Data::Struct(data) = &input.data {
        data
    } else {
        unimplemented!()
    };
    let fields = data.fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let opts = ParamFieldOpt::from_attributes(&f.attrs).expect("Wrong option");
        let field = match opts.use_ref {
            Some(true) => quote! {
                self.#name.as_ref()
            },
            Some(false) | None => quote! {
                self.#name
            },
        };
        let output_func = if let Some(true) = struct_opts.use_display {
            quote! {
                to_string()
            }
        } else {
            match opts.display {
                Some(e) => quote! {
                    #e
                },
                None => quote! {
                    output()
                },
            }
        };
        quote! {
            #field.map(|v| v.#output_func)
        }
    });
    let expanded = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = [
                    #(#fields, )*
                ].into_iter().flatten().collect::<Vec<String>>().join("\n");
                write!(f, "{output}")
            }
        }
    };
    expanded.into()
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(keyword_display))]
pub(crate) struct Opts {
    field: String,
    specified_fields: Option<bool>,
    direct_display: Option<bool>,
    display_format: Option<String>,
    from: Option<Ident>,
    value: Option<Ident>,
    borrowed_value: Option<Ident>,
    default_value: Option<Expr>,
}

#[derive(FromAttributes, Default)]
#[darling(default, attributes(keyword_display))]
struct FieldAttrs {
    is_option: Option<bool>,
}

#[proc_macro_derive(KeywordDisplayStruct, attributes(keyword_display))]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong option");
    let DeriveInput { ident, .. } = input;
    let field = opts.field;
    let field_text = quote! {
        fn field(&self) -> String {
            #field.to_string()
        }
    };
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };
    let formatted_fields = fields.iter().map(|f| {
        let opts = FieldAttrs::from_attributes(&f.attrs).expect("Wrong attrs");
        let ident = f
            .ident
            .as_ref()
            .expect("Tuple struct does not have named field");
        if let Some(true) = opts.is_option {
            quote! {self.#ident.map(|v| v.output()).unwrap_or_default()}
        } else {
            quote! {self.#ident}
        }
    });
    let expr = opts.display_format.unwrap_or("{}".to_string());
    let display_impl = quote! {
    use crate::param::KeywordDisplay;
    impl std::fmt::Display for #ident {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, #expr, #(#formatted_fields), *)
        }
    }
    };
    let derive_from = match opts.from {
        Some(x) => {
            let field_values = fields.iter().map(|f| {
                let opts = FieldAttrs::from_attributes(&f.attrs).expect("Wrong attrs");
                let ident = f
                    .ident
                    .as_ref()
                    .expect("Tuple struct does not have named field");
                if let Some(true) = opts.is_option {
                    quote! {#ident: Default::default()}
                } else {
                    quote! {#ident: value }
                }
            });
            quote! {
            impl From<#x> for #ident {
                fn from(value: #x) -> #ident {
                        #ident {
                        #(#field_values), *
                    }
                }
            }
            }
        }
        None => quote! {},
    };

    let default = if let Some(x) = opts.default_value {
        let field_values = fields.iter().map(|f| {
            let opts = FieldAttrs::from_attributes(&f.attrs).expect("Wrong attrs");
            let ident = f
                .ident
                .as_ref()
                .expect("Tuple struct does not have named field");
            if let Some(true) = opts.is_option {
                quote! {#ident: Default::default()}
            } else {
                quote! {#ident: #x }
            }
        });
        quote! {
        impl Default for #ident {
            fn default() -> #ident {
                #ident {
                    #(#field_values), *
                }
            }
        }
        }
    } else {
        quote! {}
    };
    let output = quote! {
        impl crate::param::KeywordDisplay for #ident {
            #field_text
        }
        #display_impl
        #derive_from
        #default
    };
    output.into()
}

#[proc_macro_derive(KeywordDisplay, attributes(keyword_display))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong option");
    let DeriveInput { ident, .. } = input;
    let field = opts.field;
    let field_text = match &input.data {
        syn::Data::Struct(_) => {
            quote! {
                fn field(&self) -> String {
                    #field.to_string()
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            if let Some(true) = opts.specified_fields {
                data_enum_field_impl(data_enum, &ident)
            } else {
                quote! {
                    fn field(&self) -> String {
                        #field.to_string()
                    }
                }
            }
        }
        syn::Data::Union(_) => unimplemented!(),
    };

    let from = match opts.from {
        Some(x) => quote! {
            impl From<#x> for #ident {
                fn from(value: #x) -> Self {
                    Self(value)
                }
            }
        },
        None => quote! {},
    };

    let value = match opts.value {
        Some(x) => quote! {
            impl #ident {
                pub fn value(&self) -> #x {
                    self.0
                }
            }
        },
        None => quote! {},
    };

    let borrowed_value = if let Some(x) = opts.borrowed_value {
        quote! {
            impl #ident {
                pub fn value(&self) -> &#x {
                    &self.0
                }
            }
        }
    } else {
        quote! {}
    };

    let direct_display = if let Some(false) = opts.direct_display {
        quote! {}
    } else {
        let expr = if let Some(expr) = opts.display_format {
            quote! {#expr}
        } else {
            quote! {"{}"}
        };
        match &input.data {
            syn::Data::Struct(_) => {
                quote! {
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, #expr, self.0)
                    }
                }
                }
            }
            syn::Data::Enum(data_enum) => data_enum_display_impl(data_enum, &ident),
            // We don't use union so far
            syn::Data::Union(_) => unimplemented!(),
        }
    };
    let default = if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Unnamed(fields) = &data_struct.fields {
            if let Some(x) = opts.default_value {
                let values = fields.unnamed.iter().map(|_| quote! {#x});
                quote! {
                impl Default for #ident {
                    fn default() -> #ident {
                        #ident(#(#values),*)
                    }
                }
                }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    let output = quote! {
        impl crate::param::KeywordDisplay for #ident {
            #field_text
        }
        #from
        #value
        #borrowed_value
        #direct_display
        #default
    };
    output.into()
}

#[proc_macro_derive(ParamEnumFromStr)]
pub fn enum_from_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = match input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => unimplemented!(),
    };
    let enum_ident = input.ident;
    let variant_matches = data.variants.iter().map(|v| match &v.fields {
        syn::Fields::Named(_) => unimplemented!(),
        syn::Fields::Unnamed(_fields_unnamed) => todo!(),
        syn::Fields::Unit => {
            let ident = &v.ident;
            let binding = ident.to_string().to_lowercase();
            let matched = binding.as_str();
            quote! {
                #matched => Some(#enum_ident::#ident)
            }
        }
    });
    quote! {
        impl #enum_ident {
            fn from_str(input: &str) -> Option<Self> {
                let lowercase = input.to_lowercase();
                match lowercase.as_str() {
                    #(#variant_matches,)*
                    _ => None
                }
            }
        }
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(pest_rule))]
struct BuildFromPairsOpt {
    rule: Expr,
    keyword: Lit,
}

#[proc_macro_derive(BuildFromPairs, attributes(pest_rule))]
pub fn derive_consume_pairs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let opts = BuildFromPairsOpt::from_derive_input(&input).expect("Expect Option");
    let rule = &opts.rule;
    let keyword = opts.keyword;
    quote! {

    impl<'a> crate::parser::ConsumeKVPairs<'a> for #ident {
        type Item = #ident;

        fn find_from_pairs(pairs: &'a [crate::parser::KVPair<'a>]) -> Option<Self::Item> {
            let found_index = pairs
                .iter()
                .position(|pair| pair.keyword().to_uppercase() == #keyword);
            match found_index {
                Some(i) => {
                    let pair = pairs[i];
                    let kvpair = pair.to_string();
                    let mut parse = crate::parser::ParamParser::parse(#rule, &kvpair).unwrap();
                    // let err_message = format!("Incorrect value for {}", #ident.field());
                    Some(#ident::from_pest(&mut parse).unwrap())
                }
                None => None,
            }
        }
    }
        }
    .into()
}

#[proc_macro_derive(StructBuildFromPairs)]
pub fn derive_struct_consume_pairs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };
    let fields_idents = fields.iter().filter_map(|f| f.ident.as_ref());
    let fields_builder = fields.iter().filter_map(|f| {
        f.ident.as_ref().map(|ident| {
            let ty = extract_type_from_option(&f.ty).expect("The type T is wrapped in Option<T>");
            quote! {
                let #ident = #ty::find_from_pairs(pairs);
            }
        })
    });
    quote! {
        impl<'a> crate::parser::ConsumeKVPairs<'a> for #struct_ident {
            type Item = #struct_ident;

            fn find_from_pairs(pairs: &'a [crate::parser::KVPair<'a>]) -> Option<Self::Item> {
                #(#fields_builder)*
                Some(Self {
                    #(#fields_idents,)*
                })
            }
        }
    }
    .into()
}
