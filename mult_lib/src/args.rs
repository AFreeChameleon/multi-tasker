use std::env;

use crate::error::{MultError, MultErrorTuple};

const WATCH_FLAG: &str = "--watch";
const FLAGS: [(&str, bool); 1] = [
    (WATCH_FLAG, false)
];

pub struct ParsedArgs {
    pub flags: Vec<String>,
    pub value_flags: Vec<(String, Option<String>)>,
    pub values: Vec<String>
}

// flags is an array of the name of the flag like --watch and if the flag has a value
pub fn parse_args(flags: &[(&str, bool)]) -> Result<ParsedArgs, MultErrorTuple> {
    let mut parsed_args = ParsedArgs {
        flags: Vec::new(),
        value_flags: Vec::new(),
        values: Vec::new()
    };
    let mut arg_idx = 1;
    loop {
        // Adding addition here because I didn't want to repeat
        arg_idx += 1;
        if let Some(arg) = env::args().nth(arg_idx) {
            if let Some(flag) = flags.iter().find(|val| val.0 == arg) {
                if flag.1 {
                    parsed_args.value_flags.push(
                        (arg, env::args().nth(arg_idx + 1))
                    );
                    // Skipping next arg since it's a value
                    arg_idx += 1;
                    continue;
                }
                parsed_args.flags.push(arg);
                continue;
            }
            parsed_args.values.push(arg);
            continue;
        }
        break;
    }
    Ok(parsed_args)
}

