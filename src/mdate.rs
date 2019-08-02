// initial test harness for drem
// everything is done as 64bit, be aware of the math implications!

use std::*;
use nix::*;
use std::libc::*;
use libm::F64Ext::*;


fn my_drem(x: f64, y:f64) -> f64 {
    let n:f64 = x/y;

}

fn my_mod(x: f64, y:f64) -> f64 {

}