//! All axis parameters useable with TMCM modules other than TMCM-100 and Monopack 2.
//!
//! # Mnemonics for use in macros:
//! - AP - ActualPosition (1)
//! - AS - ActualSpeed (3)
//! - MPS - MaximumPositioningSpeed (4)
//! - AMC - AbolsuteMaxCurrent (6)
//! - SBC - StandbyCurrent (7)
//! - RLSD - RightLimitSwitchDisable (12)
//! - LLSD - LeftLimitSwitchDisable (13)
//! - MSR - MicrostepResolution (140)

use AxisParameter;
use ReadableAxisParameter;
use WriteableAxisParameter;
use Return;

use modules::tmcm::{
    TmcmAxisParameter,
    ReadableTmcmAxisParameter,
    WriteableTmcmAxisParameter,
};


axis_param_rw!(
/// The current position of the motor.
///
/// Should only be overwritten for reference point setting.
ActualPosition, i32, 1
);
impl ReadableTmcmAxisParameter for ActualPosition {}
impl WriteableTmcmAxisParameter for ActualPosition {}

axis_param_r!(
/// The current rotation speed.
///
/// Should never be overwritten.
ActualSpeed, i16, 3
);
impl ReadableTmcmAxisParameter for ActualSpeed {}

axis_param_rw!(
/// The maximum positioning speed.
///
/// Should not exceed the physically highest possible value. Adjust the pulse divisor (no. 154),
/// if the speed value is very  low  (<50)  or  above  the  upper  limit.
/// See TMC 428 datasheet (p.24) for calculation of physical units.
MaximumPositioningSpeed, u16, 4
);
impl MaximumPositioningSpeed {
    pub fn new(speed: u16) -> Self {
        assert!(speed <= 2047);
        MaximumPositioningSpeed(speed)
    }
}
impl TmcmAxisParameter for MaximumPositioningSpeed {}
impl ReadableTmcmAxisParameter for MaximumPositioningSpeed {}
impl WriteableTmcmAxisParameter for MaximumPositioningSpeed {}

axis_param_rw!(
/// The absolute maximum current
///
/// The most important motor setting, since too high values might cause motor damage!
///
/// Note  that  on  the  TMCM-300 the phase current can not be reduced down to zero due
/// to the Allegro A3972 driver hardware. On the TMCM-300, 303, 310, 110, 610, 611 and 612
/// the maximum value is 1500 (which means 1.5A).
/// On all other modules the maximum value is 255 (which means 100% of the maximum current of the module).
AbsoluteMaxCurrent, u16, 6
);
impl AbsoluteMaxCurrent {
    pub fn new(current: u16) -> Self {
        AbsoluteMaxCurrent(current)
    }

}
impl TmcmAxisParameter for AbsoluteMaxCurrent {}
impl ReadableTmcmAxisParameter for AbsoluteMaxCurrent {}
impl WriteableTmcmAxisParameter for AbsoluteMaxCurrent {}

axis_param_rw!(
/// The absolute maximum current
///
/// The most important motor setting, since too high values might cause motor damage!
///
/// Note  that  on  the  TMCM-300 the phase current can not be reduced down to zero due
/// to the Allegro A3972 driver hardware. On the TMCM-300, 303, 310, 110, 610, 611 and 612
/// the maximum value is 1500 (which means 1.5A).
/// On all other modules the maximum value is 255 (which means 100% of the maximum current of the module).
StandbyCurrent, u16, 7
);
impl StandbyCurrent {
    pub fn new(current: u16) -> Self {
        StandbyCurrent(current)
    }
}
impl TmcmAxisParameter for StandbyCurrent {}
impl ReadableTmcmAxisParameter for StandbyCurrent {}
impl WriteableTmcmAxisParameter for StandbyCurrent {}

axis_param_rw!(
/// If set, deactivates the stop function of the right switch
RightLimitSwitchDisable, bool, 12
);
impl RightLimitSwitchDisable {
    pub fn disabled() -> Self {
        RightLimitSwitchDisable(true)
    }
    pub fn enabled() -> Self {
        RightLimitSwitchDisable(false)
    }
}
impl TmcmAxisParameter for RightLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for RightLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for RightLimitSwitchDisable {}

axis_param_rw!(
/// Deactivates the stop function of the left switch resp. reference switch if set.
LeftLimitSwitchDisable, bool, 13
);
impl LeftLimitSwitchDisable {
    pub fn disabled() -> Self {
        LeftLimitSwitchDisable(true)
    }
    pub fn enabled() -> Self {
        LeftLimitSwitchDisable(false)
    }
}
impl TmcmAxisParameter for LeftLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for LeftLimitSwitchDisable {}

/// Microstep Resolution
///
/// Note that modifying this parameter will affect the rotation speed in the same relation.
/// Even if the module is specified for 16 microsteps only, switching to 32 or 64 microsteps still
/// brings an enhancement in resolution and smoothness. The position counter will use the full
/// resolution, but, however, the motor will resolve a maximum of 24 different microsteps only
/// for the 32 or 64 microstep units.
///
/// *) Please note that the fullstep setting as well as the half step setting are not optimized for
/// use without an adapted microstepping table. These settings just step through the microstep table
/// in steps of 64 respectively 32. To get real full stepping use axis parameter 211 or load an
/// adapted microstepping table.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MicrostepResolution {
    /// Fullstep
    Full = 0,
    /// Halfstep
    Half = 1,
    /// 4 microsteps
    Micro4 = 2,
    /// 8 microsteps
    Micro8 = 3,
    /// 16 microsteps
    Micro16 = 4,
    /// 32 microsteps
    Micro32 = 5,
    /// 64 microsteps
    Micro64 = 6,
}
impl MicrostepResolution {
    fn try_from_u8(v: u8) -> Result<Self, ()> {
        match v {
            0 => Ok(MicrostepResolution::Full),
            1 => Ok(MicrostepResolution::Half),
            2 => Ok(MicrostepResolution::Micro4),
            3 => Ok(MicrostepResolution::Micro8),
            4 => Ok(MicrostepResolution::Micro16),
            5 => Ok(MicrostepResolution::Micro32),
            6 => Ok(MicrostepResolution::Micro64),
            _ => Err(()),
        }
    }
}
impl AxisParameter for MicrostepResolution {
    const NUMBER: u8 = 140;
}
impl Return for MicrostepResolution {
    fn from_operand(array: [u8; 4]) -> Self {MicrostepResolution::try_from_u8(array[0]).unwrap()}
}
impl TmcmAxisParameter for MicrostepResolution {}
impl ReadableAxisParameter for MicrostepResolution {}
impl ReadableTmcmAxisParameter for MicrostepResolution {}
impl WriteableAxisParameter for MicrostepResolution {
    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, *self as u8]
    }
}
impl WriteableTmcmAxisParameter for MicrostepResolution {}
