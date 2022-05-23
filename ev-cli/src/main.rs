use clap::Parser;

/// An easy way to edit environment variables
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// Use the user specific variables
    #[clap(short)]
    user_variables: bool,

    #[clap(subcommand)]
    action: Action
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Add,
    Remove,
    List
}

fn main() {
    let args = Args::parse();

    println!("Should use User Variable: {}", args.user_variables);
    println!("Action: {:?}", args.action)

}
