//! A `TMCM` type useable with TMCM modules other than TMCM-100 and Monopack 2.

use lib::cell::RefCell;
use lib::ops::Deref;

pub mod instructions;
pub mod axis_parameters;

use Error;
use Instruction;
use instructions::DirectInstruction;
use Interface;
use Return;
use Status;
use Command;
use AxisParameter;
use ReadableAxisParameter;
use WriteableAxisParameter;


/// This type represennts TMCM modules other than TMCM-100 and Monopack 2
#[derive(Debug)]
pub struct TmcmModule<I: Interface, T: Deref<Target=RefCell<I>>> {
    /// The module address
    address: u8,
    interface: T,
}

impl<I: Interface, T: Deref<Target=RefCell<I>>> TmcmModule<I, T> {
    /// Create a new module
    pub fn new(interface: T, address: u8) -> Self {
        TmcmModule{address, interface}
    }

    /// Synchronously write a command and wait for the Reply
    pub fn write_command<C: TmcmInstruction + DirectInstruction>(&self, instruction: C) -> Result<C::Return, Error<I::Error>> {
        let mut interface = self.interface.try_borrow_mut().or(Err(Error::InterfaceBusy))?;
        interface.transmit_command(&Command::new(self.address, instruction)).map_err(|e| Error::InterfaceError(e))?;
        let reply = interface.receive_reply().map_err(|e| Error::InterfaceError(e))?;
        match reply.status() {
            Status::Ok(_) => Ok(<C::Return as Return>::deserialize(reply.value())),
            Status::Err(e) => Err(e.into()),
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
