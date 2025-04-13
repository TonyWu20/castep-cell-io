use darling::{FromAttributes, FromDeriveInput};
use syn::Expr;

#[derive(FromAttributes, Default)]
#[darling(default, attributes(param_display))]
pub(crate) struct ParamFieldOpt {
    pub use_ref: Option<bool>,
    pub display: Option<Expr>,
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(param_display))]
pub(crate) struct ParamOpt {
    pub use_display: Option<bool>,
}
