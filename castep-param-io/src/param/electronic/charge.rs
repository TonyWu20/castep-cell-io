
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// Description
/// This keyword specifies the total charge of the system. It can be used instead of `NELECTRONS`.
/// It is not possible to specify the `NELECTRONS`, `NUP`, or `NDOWN` keywords when the `CHARGE` keyword is specified.
/// Default
/// 0
/// Example
/// CHARGE : 3
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplay)]
#[keyword_display(field="CHARGE", from=f64, value=f64)]
pub struct Charge(f64);
