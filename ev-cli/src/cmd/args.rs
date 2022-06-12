use clap::Parser;

/// An easy way to edit environment variables
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

    /// Use the user specific variables
    #[clap(short, parse(from_flag))]
    pub global: bool,

    #[clap(subcommand)]
    pub action: Action
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    Add {
        #[clap(short = 'n')]
        var_name: String,
        
        #[clap(short = 'v')]
        value: String,
        
        /// If this flag is set then it will overwrite the previous values, if not then it will append it.
        #[clap(short, parse(from_flag))]
        overwrite: bool,
    },
    Remove {
        /// Which variable name we should remove. If the variable name does not match specifically then it will not be removed
        #[clap(short = 'n')]
        var_name: String
    },
    List {
        /// Is a filter to which variables should be displayed. It filters the name be the pattern used e.g. PAT* will give the variabel PATH
        #[clap(short)]
        filter: String
    },
    ListEVTerminals,
}