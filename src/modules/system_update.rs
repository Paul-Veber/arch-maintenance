use std::io;

use cmd_lib::*;

pub fn update() {
    println!("");
    println!("update the system");
    println!("which programme do you wish to use ? ");

    loop {
        println!("pac: pacman, yay: yay");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("failed to read commande");

        match command.trim() {
            "pac" => run_cmd!(sudo pacman -Syu),
            "yay" => run_cmd!(yay -Syu),
            _ => {
                println!("input a valid command");
                continue;
            }
        };
    }
}
