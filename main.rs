use std::process::Command;

fn main() {
    println!("running vim: ");
    let input = Command::new("vim").output().expect("failed to execute");
}
