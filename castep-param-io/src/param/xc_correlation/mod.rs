use castep_param_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::KeywordDisplay;

mod nlxc_options;
mod spin_polarised;
mod xc_definition;
mod xc_functional;

pub use nlxc_options::*;
pub use spin_polarised::SpinPolarised;
pub use xc_definition::*;
pub use xc_functional::XCFunctional;

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, Builder, ParamDisplay, PartialEq, PartialOrd,
)]
#[builder(setter(into, strip_option), default)]
pub struct XcParam {
    pub xc_functional: Option<XCFunctional>,
    #[param_display(use_ref=true, display=to_string())]
    pub xc_definition: Option<XCDefinition>,
    pub spin_polarised: Option<SpinPolarised>,
    #[param_display(display=to_string())]
    pub nlxc_options: Option<NLXCOptions>,
}

#[cfg(test)]
mod test {
    use super::{
        nlxc_options::NLXCOptionsBuilder, xc_functional::XCFunctional, NLXC_PPD_Int,
        XCDefinitionBuilder, XcParamBuilder,
    };

    #[test]
    fn xc_param() {
        let xc_param = XcParamBuilder::default()
            .xc_functional(XCFunctional::HSE03)
            .spin_polarised(true)
            .nlxc_options(
                NLXCOptionsBuilder::default()
                    .impose_trs(false)
                    .build()
                    .unwrap(),
            )
            .xc_definition(
                XCDefinitionBuilder::default()
                    .nlxc_ppd_int(NLXC_PPD_Int::On)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        println!("{}", xc_param);
    }
}
