//! TODO `send_no_op` sequence docs.

#![no_std]
#![no_main]

use ref_sequences::*;

#[fprime_main]
pub fn main() {
    // `Checked` panics the sequence command does not return `OK`.
    set_fail_mode(FailMode::Checked);
    //CdhCore.cmdDisp.CMD_NO_OP_STRING("Hello wasm!");
    CdhCore.cmdDisp.CMD_NO_OP();
}
