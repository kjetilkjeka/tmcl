//! All axis parameters useable with TMCM modules other than TMCM-100 and Monopack 2.
//!
//! # Mnemonics for use in macros:
//! - AP - ActualPosition (1)
//! - AS - ActualSpeed (3)
//! - MPS - MaximumPositioningSpeed (4)
//! - AMC - AbolsuteMaxCurrent (6)
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


/// The current position of the motor.
///
/// Should only be overwritten for reference point setting.
#[derive(Debug, PartialEq)]
pub struct ActualPosition {
    pos: i32,
}
impl ActualPosition {
    pub fn value(&self) -> i32 {
        self.pos
    }
}
impl AxisParameter for ActualPosition {
    const NUMBER: u8 = 1;
}
impl Return for ActualPosition {
    fn deserialize(array: [u8; 4]) -> Self {
        ActualPosition{pos:
            (array[3] as u32 |
            ((array[2] as u32) << 8) |
            ((array[1] as u32) << 16) |
            ((array[0] as u32) << 24)) as i32
        }
    }
}
impl ReadableAxisParameter for ActualPosition {}
impl ReadableTmcmAxisParameter for ActualPosition {}
impl WriteableAxisParameter for ActualPosition {
    fn serialize_value(&self) -> [u8; 4] {
        [(self.pos >> 24) as u8, (self.pos >> 16) as u8, (self.pos >> 8) as u8 , self.pos as u8]
    }
}
impl WriteableTmcmAxisParameter for ActualPosition {}

/// The current rotation speed.
///
/// Should never be overwritten.
#[derive(Debug, PartialEq)]
pub struct ActualSpeed {
    speed: i16,
}
impl ActualSpeed {
    pub fn value(&self) -> i16 {
        self.speed
    }
}
impl AxisParameter for ActualSpeed {
    const NUMBER: u8 = 3;
}
impl Return for ActualSpeed {
    fn deserialize(array: [u8; 4]) -> Self {
        ActualSpeed{speed:
            (array[3] as u16 |
            ((array[2] as u16) << 8)) as i16
        }
    }
}
impl ReadableAxisParameter for ActualSpeed {}
impl ReadableTmcmAxisParameter for ActualSpeed {}

/// The maximum positioning speed.
///
/// Should not exceed the physically highest possible value. Adjust the pulse divisor (no. 154),
/// if the speed value is very  low  (<50)  or  above  the  upper  limit.
/// See TMC 428 datasheet (p.24) for calculation of physical units.
#[derive(Debug, PartialEq)]
pub struct MaximumPositioningSpeed {
    speed: u16,
}
impl MaximumPositioningSpeed {
    pub fn new(speed: u16) -> Self {
        assert!(speed <= 2047);
        MaximumPositioningSpeed{speed}
    }
    pub fn value(&self) -> u16 {
        self.speed
    }
}
impl AxisParameter for MaximumPositioningSpeed {
    const NUMBER: u8 = 4;
}
impl Return for MaximumPositioningSpeed {
    fn deserialize(array: [u8; 4]) -> Self {
        MaximumPositioningSpeed{speed:
        (array[3] as u16 |
            ((array[2] as u16) << 8))
        }
    }
}
impl TmcmAxisParameter for MaximumPositioningSpeed {}
impl ReadableAxisParameter for MaximumPositioningSpeed {}
impl ReadableTmcmAxisParameter for MaximumPositioningSpeed {}
impl WriteableAxisParameter for MaximumPositioningSpeed {
    fn serialize_value(&self) -> [u8; 4] {
        [0, 0, (self.speed >> 8) as u8, self.speed as u8]
    }
}
impl WriteableTmcmAxisParameter for MaximumPositioningSpeed {}

/// The absolute maximum current
///
/// The most important motor setting, since too high values might cause motor damage!
///
/// Note  that  on  the  TMCM-300 the phase current can not be reduced down to zero due
/// to the Allegro A3972 driver hardware. On the TMCM-300, 303, 310, 110, 610, 611 and 612
/// the maximum value is 1500 (which means 1.5A).
/// On all other modules the maximum value is 255 (which means 100% of the maximum current of the module).
#[derive(Debug, PartialEq)]
pub struct AbsoluteMaxCurrent {
    current: u16,
}
impl AbsoluteMaxCurrent {
    pub fn new(current: u16) -> Self {
        AbsoluteMaxCurrent{current}
    }
    pub fn value(&self) -> u16 {
        self.current
    }
}
impl AxisParameter for AbsoluteMaxCurrent {
    const NUMBER: u8 = 6;
}
impl Return for AbsoluteMaxCurrent {
    fn deserialize(array: [u8; 4]) -> Self {
        AbsoluteMaxCurrent{current:
        (array[3] as u16 |
            ((array[2] as u16) << 8))
        }
    }
}
impl TmcmAxisParameter for AbsoluteMaxCurrent {}
impl ReadableAxisParameter for AbsoluteMaxCurrent {}
impl ReadableTmcmAxisParameter for AbsoluteMaxCurrent {}
impl WriteableAxisParameter for AbsoluteMaxCurrent {
    fn serialize_value(&self) -> [u8; 4] {
        [0, 0, (self.current >> 8) as u8, self.current as u8]
    }
}
impl WriteableTmcmAxisParameter for AbsoluteMaxCurrent {}

/// If set, deactivates the stop function of the right switch
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
}
impl Return for RightLimitSwitchDisable {
    fn deserialize(array: [u8; 4]) -> Self {RightLimitSwitchDisable{status: array[3] != 0}}
}
impl TmcmAxisParameter for RightLimitSwitchDisable {}
impl ReadableAxisParameter for RightLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for RightLimitSwitchDisable {}
impl WriteableAxisParameter for RightLimitSwitchDisable {
    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl WriteableTmcmAxisParameter for RightLimitSwitchDisable {}

/// Deactivates the stop function of the left switch resp. reference switch if set.
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
}
impl Return for LeftLimitSwitchDisable {
    fn deserialize(array: [u8; 4]) -> Self {LeftLimitSwitchDisable{status: array[3] != 0}}
}
impl TmcmAxisParameter for LeftLimitSwitchDisable {}
impl ReadableAxisParameter for LeftLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl WriteableAxisParameter for LeftLimitSwitchDisable {
    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
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
    fn deserialize(array: [u8; 4]) -> Self {MicrostepResolution::try_from_u8(array[3]).unwrap()}
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
