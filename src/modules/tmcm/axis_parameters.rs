//! All axis parameters useable with TMCM modules other than TMCM-100 and Monopack 2.

use AxisParameter;
use ReadableAxisParameter;
use WriteableAxisParameter;
use StorableAxisParameter;

use modules::tmcm::{
    TmcmAxisParameter,
    ReadableTmcmAxisParameter,
    WriteableTmcmAxisParameter,
    StorableTmcmAxisParameter,
};


#[derive(Debug, PartialEq)]
pub struct RightLimitSwitchDisable {
    status: bool,
}
impl RightLimitSwitchDisable {
    pub fn disabled() -> Self {
        RightLimitSwitchDisable{status: true}
    }
    pub fn enabled() -> Self {
        RightLimitSwitchDisable{status: false}
    }
}
impl AxisParameter for RightLimitSwitchDisable {
    const NUMBER: u8 = 12;

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl TmcmAxisParameter for RightLimitSwitchDisable {}
impl ReadableAxisParameter for RightLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for RightLimitSwitchDisable {}
impl WriteableAxisParameter for RightLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for RightLimitSwitchDisable {}
impl StorableAxisParameter for RightLimitSwitchDisable {}
impl StorableTmcmAxisParameter for RightLimitSwitchDisable {}

#[derive(Debug, PartialEq)]
pub struct LeftLimitSwitchDisable {
    status: bool,
}
impl LeftLimitSwitchDisable {
    pub fn disabled() -> Self {
        LeftLimitSwitchDisable{status: true}
    }
    pub fn enabled() -> Self {
        LeftLimitSwitchDisable{status: false}
    }
}
impl AxisParameter for LeftLimitSwitchDisable {
    const NUMBER: u8 = 13;

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl TmcmAxisParameter for LeftLimitSwitchDisable {}
impl ReadableAxisParameter for LeftLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl WriteableAxisParameter for LeftLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl StorableAxisParameter for LeftLimitSwitchDisable {}
impl StorableTmcmAxisParameter for LeftLimitSwitchDisable {}