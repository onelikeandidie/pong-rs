use std::{env, collections::HashMap};

use crate::shared::util::get_neighbor;

#[derive(Debug)]
pub struct CliArgs {
    pub args: Vec<String>,
    pub options: Vec<String>,
    pub map: HashMap<String, String>
}

impl CliArgs {
    pub fn extract_args() -> Self {
        // Collect command arguments
        let args: Vec<String> = env::args().collect();
        let args_clone = args.clone();
        let mut args_map = HashMap::new();
        let mut args_options = vec![];
        for arg in args_clone.clone() {
            if arg.starts_with("--") {
                let reduced_arg = arg[2..].to_string();
                if let Some(data) = get_neighbor(&args_clone, arg.as_str()) {
                    if !data.starts_with("--") {
                        args_map.insert(reduced_arg, data);
                    } else {
                        args_options.push(reduced_arg);
                    }
                } else {
                    args_options.push(reduced_arg);
                }
            }
        }
        Self { 
            args: args.clone(), 
            options: args_options, 
            map: args_map
        }
    }
}
