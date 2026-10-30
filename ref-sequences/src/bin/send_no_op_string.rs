//! TODO `send_no_op_string` sequence docs.

#![no_std]
#![no_main]

use ref_sequences::*;

#[fprime_main]
pub fn main() {
    // `Checked` panics the sequence command does not return `OK`.
    set_fail_mode(FailMode::Checked);
    
    let my_var = "hello";

    CdhCore.cmdDisp.CMD_NO_OP_STRING(
        &format!(16, "format {my_var}")
    );

}
