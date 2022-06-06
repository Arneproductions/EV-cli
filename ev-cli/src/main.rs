use clap::Parser;

use crate::cmd::Args;

mod cmd;

fn main() {
    let args = Args::parse();

    println!("Should use User Variable: {}", args.user_variables);
    println!("Action: {:?}", args.action)

}
