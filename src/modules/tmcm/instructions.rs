//! All instructions available for TMCM modules other than TMCM-100 and Monopack 2.

pub use instructions::{
    ROR,
    ROL,
    MST,
    SAP,
};

use modules::tmcm::TmcmInstruction;

use modules::tmcm::{
    WriteableTmcmAxisParameter,
    ReadableTmcmAxisParameter,
    StorableTmcmAxisParameter,
};


impl TmcmInstruction for ROR {}
impl TmcmInstruction for ROL {}
impl TmcmInstruction for MST {}
impl<T: WriteableTmcmAxisParameter> TmcmInstruction for SAP<T> {}
