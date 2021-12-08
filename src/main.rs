use std::io::{self, stdout, Write};
use std::process::Command;

fn main() {
        loop {
                print!(":: ➜ ");
                stdout().flush();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("error occurred !");

                //let input: String = input.trim().parse().unwrap();
                //let command = input.trim();

                let mut parts = input.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;
                let mut child = Command::new(command).args(args).spawn().unwrap();

                //Command::new(command).spawn().unwrap();

                child.wait();
        }
}
