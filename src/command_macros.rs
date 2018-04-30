#[macro_export]
macro_rules! tmcl_instruction {
    (ROR $motor_number:expr, $velocity:expr) => {ROR::new($motor_number, $velocity)};
    (ROL $motor_number:expr, $velocity:expr) => {ROL::new($motor_number, $velocity)};
    (MST $motor_number:expr) => {MST::new($motor_number)};

    // SAP instructions with tmcm mnemonics
    (SAP RLSD, $motor_number:expr, 1) => {SAP::new($motor_number, RightLimitSwitchDisable::disabled())};
    (SAP RLSD, $motor_number:expr, 0) => {SAP::new($motor_number, RightLimitSwitchDisable::enabled())};
    (SAP LLSD, $motor_number:expr, 1) => {SAP::new($motor_number, LeftLimitSwitchDisable::disabled())};
    (SAP LLSD, $motor_number:expr, 0) => {SAP::new($motor_number, LeftLimitSwitchDisable::enabled())};

    // GAP instructions with tmcm mnemonics
    (GAP RLSD, $motor_number:expr) => {GAP::<RightLimitSwitchDisable>::new($motor_number)};
    (GAP LLSD, $motor_number:expr) => {GAP::<LeftLimitSwitchDisable>::new::($motor_number)};
    (GAP AS, $motor_number:expr) => {GAP::<ActualSpeed>::new($motor_number)};
    (GAP AP, $motor_number:expr) => {GAP::<ActualPosition>::new($motor_number)};
}