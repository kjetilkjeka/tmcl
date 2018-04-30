#[macro_export]
macro_rules! tmcl_instruction {
    (ROR $motor_number:expr, $velocity:expr) => {ROR::new($motor_number, $velocity)};
    (ROL $motor_number:expr, $velocity:expr) => {ROL::new($motor_number, $velocity)};
    (MST $motor_number:expr) => {MST::new($motor_number)};

    // SAP instructions with tmcm mnemonics
    (SAP $motor_number:expr, RLSD, 1) => {SAP::new($motor_number, RightLimitSwitchDisable::disabled())};
    (SAP $motor_number:expr, RLSD, 0) => {SAP::new($motor_number, RightLimitSwitchDisable::enabled())};
    (SAP $motor_number:expr, LLSD, 1) => {SAP::new($motor_number, LeftLimitSwitchDisable::disabled())};
    (SAP $motor_number:expr, LLSD, 0) => {SAP::new($motor_number, LeftLimitSwitchDisable::enabled())};

}