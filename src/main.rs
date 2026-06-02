//use colored_text::Colorize;
mod todo;
use std::{io, process::exit};
use crate::todo::todo_struct::{Taks};

// enum FormError {
//     EmptyInput,
//     InvalidEnter,
//     AgeTooLow,
// }

fn main() {
    let mut base: Vec<Taks> = Vec::new();
    loop {
        let mut choise = String::new();
        println!("1 Ajouter une taches");
        println!("2 Afficher la liste des taches");
        println!("3 marker une taches comme faire");
        println!("4 supprimer une taches");
        println!("5 quiter");

        io::stdin()
            .read_line(&mut choise)
            .expect("erreur de lecture");

        let choise: u32 = choise.trim().parse().expect("valeur invalide");

        match choise {
            1 => Taks::add(&mut base),
            2 => Taks::show(&base),
            3 => {
                Taks::update_status(&mut base);
            }
            4 => Taks::update(&mut base),
            5 => {
                println!("merci pour votre utilisation");
                exit(0);
            }
            _ => println!("valeur ivalide"),
        }
    }
}
