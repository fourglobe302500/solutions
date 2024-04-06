use std::{borrow::Borrow, collections::HashMap, env};

pub mod helpers;
pub mod problems;
use problems::*;

macro_rules! get_function {
    ($i:ident) => {
        (stringify!($i), Box::new($i::solution as fn()) as Box<fn()>)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let functions = HashMap::from([
        get_function!(three_n_plus_one),
        get_function!(minesweeper),
        get_function!(making_change),
    ]);

    if args.len() == 1 {
        println!(
            "wrong number of arguments, use '{} --list' for possible arguments",
            args[0]
        );
    } else if args.contains(&"--list".into()) {
        for (name, _f) in functions.into_iter() {
            println!("{}", name);
        }
    } else {
        let names: Vec<_> = args.into_iter().skip(1).collect();

        for name in names {
            println!("Running {}", &name);

            match functions.get::<str>(name.borrow()) {
                None => println!("problem '{}' does not exist", name),
                Some(f) => f(),
            }

            println!();
        }
    }
}
