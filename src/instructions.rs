//! TMCL Instructions

#[cfg(feature="std")]
use std::marker::PhantomData;
#[cfg(not(feature="std"))]
use core::marker::PhantomData;

use {
    WriteableAxisParameter,
    ReadableAxisParameter,
};

/// A `TMCL` `Instruction`
pub trait Instruction {
    /// The command number (sometimes referred to as the instruction number).
    const INSTRUCTION_NUMBER: u8;

    fn type_number(&self) -> u8;

    /// The motor/bank number
    fn motor_bank_number(&self) -> u8;

    /// Return the operand serialized.
    ///
    /// Even though the whole frame (in serialized form) is represented as:
    /// `[..., operand[3], operand[2], operand[1], operand[0], ...]`.
    /// This function instead return the operand:
    /// `[operand[0], operand[1], operand[2], operand[3]]`.
    fn operand(&self) -> [u8; 4];
}

/// An `Instruction` useable in direct mode
pub trait DirectInstruction: Instruction {
    /// The return value when the `Instruction` is executed in direct mode.
    type Return: Return;
}

/// A type that can be used as a return value for an `Instruction`
pub trait Return {

    /// The deserialization function.
    ///
    /// The argument of the deserialize function is the operand bytes array:
    /// `[operand[0], operand[1], operand[2], operand[3]]`.
    /// This is a different to how it is represented in the serialized frame:
    /// `[..., operand[3], operand[2], operand[1], operand[0], ...]`
    fn from_operand(operand: [u8; 4]) -> Self;
}

/// ROR - Rotate Right
///
/// This instruction starts rotation in "right" direction, i.e. increasing the position counter.
#[derive(Debug, PartialEq)]
pub struct ROR {
    motor_number: u8,
    velocity: u32,
}
impl ROR {
    pub fn new(motor_number: u8, velocity: u32) -> ROR {ROR{motor_number, velocity}}
}
impl Instruction for ROR {
    const INSTRUCTION_NUMBER: u8 = 1;

    fn operand(&self) -> [u8; 4] {
        return [
            (self.velocity & 0xff) as u8,
            ((self.velocity >> 8) & 0xff) as u8,
            ((self.velocity >> 16) & 0xff) as u8,
            ((self.velocity >> 24) & 0xff) as u8
        ]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for ROR {
    type Return = ();
}

/// ROL - Rotate Left
///
/// This instruction starts rotation in "left" direction, i.e. decreasing the position counter.
#[derive(Debug, PartialEq)]
pub struct ROL {
    motor_number: u8,
    velocity: u32,
}
impl ROL {
    pub fn new(motor_number: u8, velocity: u32) -> ROL {ROL{motor_number, velocity}}
}
impl Instruction for ROL {
    const INSTRUCTION_NUMBER: u8 = 2;

    fn operand(&self) -> [u8; 4] {
        return [
            (self.velocity & 0xff) as u8,
            ((self.velocity >> 8) & 0xff) as u8,
            ((self.velocity >> 16) & 0xff) as u8,
            ((self.velocity >> 24) & 0xff) as u8
        ]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for ROL {
    type Return = ();
}


/// MST - Motor Stop
///
/// This instruction stops the motor.
#[derive(Debug, PartialEq)]
pub struct MST {
    motor_number: u8,
}
impl MST {
    pub fn new(motor_number: u8) -> MST {MST{motor_number}}
}
impl Instruction for MST {
    const INSTRUCTION_NUMBER: u8 = 3;

    fn operand(&self) -> [u8; 4] {
        return [0, 0, 0, 0]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for MST {
    type Return = ();
}

/// The type and value of a `MVP` instruction
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveOperation {
    /// Moving to an absolute position in the range from -8388608 to +8388608 (-2^23 to +2^23).
    Absolute(i32),

    /// Starting a relative movement by means of an offset to the actual position. In this case,
    /// the resulting new position value must not exceed the above mentioned limits, too.
    Relative(i32),

    /// Moving one or more motors to a (previously stored) coordinate (represented by a coordinate number),
    ///
    /// When moving more than one axis the  module will try  to  interpolate:
    /// The velocities will be calculated so that  all  motors reach their target positions at the same time.
    /// It is important that the maximum accelerations (axis parameter #5) and the ramp  and
    /// pulse dividers (axis parameters #153 and #154) of all axes are set to the same values
    /// as otherwise interpolation will not work correctly.
    Coordinate(u32),
}

/// MVP - Move to Position
///
/// A movement towards the specified position is started, with automatic generation of acceleration
/// and deceleration ramps. The maximum velocity and acceleration are defined by axis parameters #4 and #5.
#[derive(Debug, PartialEq)]
pub struct MVP {
    motor_number: u8,
    value: MoveOperation,
}
impl MVP {
    pub fn new(motor_number: u8, value: MoveOperation) -> MVP {MVP{motor_number, value}}
}
impl Instruction for MVP {
    const INSTRUCTION_NUMBER: u8 = 4;

    fn operand(&self) -> [u8; 4] {
        match self.value {
            MoveOperation::Absolute(x) => {
                [
                    (x & 0xff) as u8,
                    ((x >> 8) & 0xff) as u8,
                    ((x >> 16) & 0xff) as u8,
                    ((x >> 24) & 0xff) as u8
                ]
            },
            MoveOperation::Relative(x) => {
                [
                    (x & 0xff) as u8,
                    ((x >> 8) & 0xff) as u8,
                    ((x >> 16) & 0xff) as u8,
                    ((x >> 24) & 0xff) as u8
                ]
            },
            MoveOperation::Coordinate(x) => {
                [
                    (x & 0xff) as u8,
                    ((x >> 8) & 0xff) as u8,
                    ((x >> 16) & 0xff) as u8,
                    ((x >> 24) & 0xff) as u8
                ]
            },
        }
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for MVP {
    type Return = ();
}


/// SAP - Set Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be set by this function.
#[derive(Debug, PartialEq)]
pub struct SAP<T: WriteableAxisParameter> {
    motor_number: u8,
    axis_parameter: T,
}
impl<T: WriteableAxisParameter> SAP<T> {
    pub fn new(motor_number: u8, axis_parameter: T) -> SAP<T> {
        SAP{
            motor_number,
            axis_parameter
        }
    }
}
impl<T: WriteableAxisParameter> Instruction for SAP<T> {
    const INSTRUCTION_NUMBER: u8 = 5;

    fn operand(&self) -> [u8; 4] {
        self.axis_parameter.operand()
    }

    fn type_number(&self) -> u8 {
        T::NUMBER
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl<T: WriteableAxisParameter> DirectInstruction for SAP<T> {
    type Return = ();
}

/// GAP - Get Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be read by this function.
#[derive(Debug, PartialEq)]
pub struct GAP<T: ReadableAxisParameter> {
    motor_number: u8,
    phantom: PhantomData<T>,
}
impl<T: ReadableAxisParameter> GAP<T> {
    pub fn new(motor_number: u8) -> GAP<T> {
        GAP{
            motor_number,
            phantom: PhantomData,
        }
    }
}
impl<T: ReadableAxisParameter> Instruction for GAP<T> {
    const INSTRUCTION_NUMBER: u8 = 6;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        T::NUMBER
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl<T: ReadableAxisParameter> DirectInstruction for GAP<T> {
    type Return = T;
}

/// STAP - Store Axis Parameter
///
/// Axis parameters are located in RAM memory, so modifications are lost at power down.
/// This instruction enables permanent storing.
#[derive(Debug, PartialEq)]
pub struct STAP<T: WriteableAxisParameter> {
    motor_number: u8,
    phantom: PhantomData<T>,
}
impl<T: WriteableAxisParameter> STAP<T> {
    pub fn new(motor_number: u8) -> STAP<T> {
        STAP{
            motor_number,
            phantom: PhantomData,
        }
    }
}
impl<T: WriteableAxisParameter> Instruction for STAP<T> {
    const INSTRUCTION_NUMBER: u8 = 7;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        T::NUMBER
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl<T: WriteableAxisParameter> DirectInstruction for STAP<T> {
    type Return = ();
}

/// RSAP - Restore Axis Parameter
///
/// For all configuration-related axis parameters, non-volatile memory locations are provided.
/// By default, most parameters are automatically restored after power up (see axis parameter list in
/// chapter 4). A single parameter that has been changed before can be reset by this instruction.
#[derive(Debug, PartialEq)]
pub struct RSAP<T: WriteableAxisParameter> {
    motor_number: u8,
    phantom: PhantomData<T>,
}
impl<T: WriteableAxisParameter> RSAP<T> {
    pub fn new(motor_number: u8) -> RSAP<T> {
        RSAP {
            motor_number,
            phantom: PhantomData,
        }
    }
}
impl<T: WriteableAxisParameter> Instruction for RSAP<T> {
    const INSTRUCTION_NUMBER: u8 = 8;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        T::NUMBER
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl<T: WriteableAxisParameter> DirectInstruction for RSAP<T> {
    type Return = ();
}

/// Choses what action to execute with the `RFS` instruction
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReferenceSearchAction {
    /// Start reference search
    Start = 0,
    /// Stop reference search
    Stop = 1,
    /// Get status
    Status = 2,
}

/// RFS - Reference Search
///
/// A build-in reference point search algorithm can be started (and stopped). The reference search
/// algorithm provides switching point calibration and three switch modes. The status of the
/// reference search can also be queried to see if it has already finished. (In a TMCL program
/// it is better to use the WAIT command to wait for the end of a reference search.)
/// Please see the appropriate parameters in the axis parameter table to configure the
/// reference search algorithm to meet your needs. The reference search can be started or stop
/// ped, or the actual status of the reference search can be checked.
#[derive(Debug, PartialEq)]
pub struct RFS {
    motor_number: u8,
    action: ReferenceSearchAction,
}
impl RFS {
    pub fn new(motor_number: u8, action: ReferenceSearchAction) -> RFS {
        RFS {
            motor_number,
            action
        }
    }
}
impl Instruction for RFS {
    const INSTRUCTION_NUMBER: u8 = 13;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.action as u8
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for RFS {
    // TODO: use const generics (when it lands) to distinguish return between RFS<Status> and RFS<_>
    type Return = bool;
}

/// SIO - Set Output
///
/// This command sets the status of a digital output either to low (0) or to high (1).
#[derive(Debug, PartialEq)]
pub struct SIO {
    bank_number: u8,
    port_number: u8,
    state: bool,
}
impl SIO {
    pub fn new(bank_number: u8, port_number: u8, state: bool) -> Self {
        SIO {bank_number, port_number, state}
    }
}
impl Instruction for SIO {
    const INSTRUCTION_NUMBER: u8 = 14;

    fn operand(&self) -> [u8; 4] {[self.state as u8, 0u8, 0u8, 0u8]}

    fn type_number(&self) -> u8 { self.port_number }

    fn motor_bank_number(&self) -> u8 { self.bank_number }
}
impl DirectInstruction for SIO {
    type Return = ();
}

/// GIO - Get Input / Output
///
/// This function reads a digital or analogue input port. So, digital lines will read 0 and 1,
/// while the ADC channels deliver their 10 bit result in the range of 0...1023. In stand-alone mode
/// the requested value is copied to the "accumulator" (accu) for further processing purposes such
/// as conditioned jumps. In  direct  mode the value is only output in the “value” field of the reply,
/// without affecting the accumulator. The actual status of a digital output line can also be read.
#[derive(Debug, PartialEq)]
pub struct GIO {
    bank_number: u8,
    port_number: u8,
}
impl GIO {
    pub fn new(bank_number: u8, port_number: u8) -> Self {
        GIO {bank_number, port_number}
    }
}
impl Instruction for GIO {
    const INSTRUCTION_NUMBER: u8 = 15;

    fn operand(&self) -> [u8; 4] {[0u8, 0u8, 0u8, 0u8]}

    fn type_number(&self) -> u8 { self.port_number }

    fn motor_bank_number(&self) -> u8 { self.bank_number }
}
impl DirectInstruction for GIO {
    type Return = u32;
}

/// CALC - Calculate
#[derive(Debug, PartialEq)]
pub enum CALC {
    /// Add the operand to the accumulator
    Add(i32),

    /// Subtract the operand from the accumulator
    Sub(i32),

    /// Multiply the accumulator by a the operand
    Mul(i32),

    /// Divide the accumulator by the operand
    Div(i32),

    /// Modulo divide the accumualtor by the operand
    Mod(i32),

    /// Logical and accumulator with operand
    And(i32),

    /// Logical or accumulator with operand
    Or(i32),

    /// Logical xor accumulator with operand
    Xor(i32),

    /// Logical invert accumulator
    Not,

    /// Load operand to accumulator
    Load(i32),
}

impl Instruction for CALC {
    const INSTRUCTION_NUMBER: u8 = 19;

    fn operand(&self) -> [u8; 4] {
        match self {
            CALC::Add(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Sub(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Mul(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Div(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Mod(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::And(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Or(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Xor(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
            CALC::Not => [0u8, 0u8, 0u8, 0u8],
            CALC::Load(x) => [(x >> 0) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8],
        }
    }

    fn type_number(&self) -> u8 {
        match self {
            CALC::Add(_) => 0,
            CALC::Sub(_) => 1,
            CALC::Mul(_) => 2,
            CALC::Div(_) => 3,
            CALC::Mod(_) => 4,
            CALC::And(_) => 5,
            CALC::Or(_) => 6,
            CALC::Xor(_) => 7,
            CALC::Not => 8,
            CALC::Load(_) => 9,
        }
    }

    fn motor_bank_number(&self) -> u8 { 0 }
}
impl DirectInstruction for CALC {
    type Return = ();
}