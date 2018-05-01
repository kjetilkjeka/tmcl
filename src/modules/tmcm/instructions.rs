//! All instructions available for TMCM modules other than TMCM-100 and Monopack 2.

pub use instructions::{
    ROR,
    ROL,
    MST,
    MVP,
    SAP,
    GAP,
    STAP,
    RFS,
    Move,
    ReferenceSearchAction,
};

use modules::tmcm::TmcmInstruction;

use modules::tmcm::{
    TmcmAxisParameter,
    WriteableTmcmAxisParameter,
    ReadableTmcmAxisParameter,
};


impl TmcmInstruction for ROR {}
impl TmcmInstruction for ROL {}
impl TmcmInstruction for MST {}
impl TmcmInstruction for MVP {}
impl<T: WriteableTmcmAxisParameter> TmcmInstruction for SAP<T> {}
impl<T: ReadableTmcmAxisParameter> TmcmInstruction for GAP<T> {}
impl<T: TmcmAxisParameter> TmcmInstruction for STAP<T> {}
impl TmcmInstruction for RFS {}
