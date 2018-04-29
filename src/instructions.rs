use {
    AxisParameter,
    WriteableAxisParameter,
    ReadableAxisParameter,
    StorableAxisParameter,
};

pub trait Instruction {
    /// The return value when the `Instruction` is executed.
    type Return: Return;

    /// The command number (sometimes referred to as the instruction number).
    const INSTRUCTION_NUMBER: u8;

    fn type_number(&self) -> u8;
    fn motor_number(&self) -> u8;
    fn serialize_value(&self) -> [u8; 4];
}

/// A type that can be used as a return value for an `Instruction`
pub trait Return {
    fn deserialize([u8; 4]) -> Self;
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
    type Return = ();

    const INSTRUCTION_NUMBER: u8 = 1;

    fn serialize_value(&self) -> [u8; 4] {
        return [
            ((self.velocity >> 24) & 0xff) as u8,
            ((self.velocity >> 16) & 0xff) as u8,
            ((self.velocity >> 8) & 0xff) as u8,
            (self.velocity & 0xff) as u8
        ]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_number(&self) -> u8 {
        self.motor_number
    }
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
    type Return = ();

    const INSTRUCTION_NUMBER: u8 = 2;

    fn serialize_value(&self) -> [u8; 4] {
        return [
            ((self.velocity >> 24) & 0xff) as u8,
            ((self.velocity >> 16) & 0xff) as u8,
            ((self.velocity >> 8) & 0xff) as u8,
            (self.velocity & 0xff) as u8
        ]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_number(&self) -> u8 {
        self.motor_number
    }
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
    type Return = ();

    const INSTRUCTION_NUMBER: u8 = 3;

    fn serialize_value(&self) -> [u8; 4] {
        return [0, 0, 0, 0]
    }

    fn type_number(&self) -> u8 {
        0
    }

    fn motor_number(&self) -> u8 {
        self.motor_number
    }
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
    type Return = ();

    const INSTRUCTION_NUMBER: u8 = 5;

    fn serialize_value(&self) -> [u8; 4] {
        self.axis_parameter.serialize_value()
    }

    fn type_number(&self) -> u8 {
        T::NUMBER
    }

    fn motor_number(&self) -> u8 {
        self.motor_number
    }
}
