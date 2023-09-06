use std::io::stdin;

use std::error::Error;

pub fn read_password() -> Result<String, Box<dyn Error>> {
    println!("Type your password to encode/decode file");

    let mut pwd = String::new();
    match stdin().read_line(&mut pwd) {
        Ok(_) => {
            if pwd.len() < 4 {
                return Err("Password is too short".into());
            }
            Ok(pwd)
        }
        Err(_) => Err("Password was not provided".into()),
    }
}

