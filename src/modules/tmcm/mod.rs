//! A `TMCM` type useable with TMCM modules other than TMCM-100 and Monopack 2.

pub mod instructions;
pub mod axis_parameters;

use Instruction;
use instructions::DirectInstruction;
use Interface;
use Return;
use Status;
use ErrStatus;
use Command;
use AxisParameter;
use ReadableAxisParameter;
use WriteableAxisParameter;


/// This type represennts TMCM modules other than TMCM-100 and Monopack 2
#[derive(Debug)]
pub struct TmcmModule {
    /// The module address
    address: u8,
}

impl TmcmModule {
    /// Create a new module
    pub fn new(address: u8) -> Self {
        TmcmModule{address}
    }

    /// Synchronously write a command and wait for the Reply
    pub fn write_command<I: Interface, C: TmcmInstruction + DirectInstruction>(&self, interface: &I, instruction: C) -> Result<Result<C::Return, ErrStatus>, I::Error> {
        interface.transmit_command(&Command::new(self.address, instruction))?;
        let reply = interface.receive_reply()?;
        match reply.status() {
            Status::Ok(_) => Ok(Ok(<C::Return as Return>::deserialize(reply.value()))),
            Status::Err(e) => Ok(Err(e)),
        }
    }
}


/// An `AxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait TmcmInstruction: Instruction {}


/// An `AxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait TmcmAxisParameter: AxisParameter {}

/// A `ReadableAxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait ReadableTmcmAxisParameter: ReadableAxisParameter {}

/// A `WriteableAxisParamtere` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait WriteableTmcmAxisParameter: WriteableAxisParameter {}
