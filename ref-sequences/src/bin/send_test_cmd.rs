//! TODO `send_test_cmd` sequence docs.

#![no_std]
#![no_main]

use ref_sequences::*;

#[fprime_main]
pub fn main() {
    // `Checked` panics the sequence command does not return `OK`.
    set_fail_mode(FailMode::Checked);

    CdhCore.cmdDisp.CMD_TEST_CMD_1(1,2.0,3);
}
