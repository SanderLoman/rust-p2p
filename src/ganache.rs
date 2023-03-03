use std::process::Command;

pub fn ganache() {
    let mut command = Command::new("yarn");
    println!("Running command: {:?}", command);
    command.arg("hardhat").arg("node");
    println!("Running command: {:?}", command);
}
