use clap::Parser;

mod cmd;
mod terminals;

fn main() {
    let args = cmd::Args::parse(); // TODO: Use args in some kind of way

    // TODO: The "".toString() is too ugly. Learn a better way... if any
    let handler = terminals::create_terminal(args.global);
    
    match args.action {
        cmd::Action::Add{var_name, value, overwrite} => {

            // TODO: This shit is so bad... figure it out now

            let name = &var_name;
            let val = &value;

            handler.add_variable(name.to_string(), val.to_string());

            println!("Variable {} with value {} added", name, val); 
        },
        cmd::Action::ListEVTerminals => {
            let terminal_names: Vec<String> = handler.list_terminals();
            
            display_list("These are the terminals found:".to_string(), terminal_names);
        },
        cmd::Action::List { filter } => {
            let evs = handler.list_variables(filter);

            display_list("Environment Variables:".to_string(), evs); // TODO: Figure out the differences between str and String, and then potentially remove the .to_string()
        },
        cmd::Action::Remove { var_name } => {
            let name = &var_name;

            handler.remove_variable(name.to_string());
            println!("Variable {} has been removed", name);
        }
    }
}

fn display_list(message: String, list: Vec<String>) {
    if(message.len() > 0) {
        println!("{}", message);

        // Create seperator line
        for x in 1..message.len() {
            print!("-");
        }

        // Go to new line and then create space to the list
        println!();
        println!()
    }

    for item in list {
        println!("- {}", item);
    }
}
