use std::io::{stdin, stdout, Write};

pub fn get_number() -> u32 {
    let mut valid = false;
    let mut v = 0;
    while !valid {
        print!("Enter number: ");
        let _ = stdout().flush();
        let mut line = "".to_owned();
        match stdin().read_line(&mut line) {
            Err(_) => println!("Invalid line"),
            Ok(_) => match line.trim().parse() {
                Err(_) => println!("Invalid number"),
                Ok(n) => {
                    v = n;
                    valid = true;
                }
            },
        }
    }
    v
}

pub fn end() -> bool {
    let mut valid = false;
    let mut line = "".to_owned();
    while !valid {
        print!("Exit solution(y|Y)?: ");
        let _ = stdout().flush();
        match stdin().read_line(&mut line) {
            Err(_) => println!("Invalid line"),
            Ok(_) => valid = true,
        }
    }
    line.trim() == "Y" || line.trim() == "y"
}
