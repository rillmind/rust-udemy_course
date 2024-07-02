use std::env::args;
use d_control_flow_strings::*;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg)
        }
    }
}

