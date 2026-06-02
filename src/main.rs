//use colored_text::Colorize;
mod todo;
mod utils;
use std::{ process::exit};
use crate::todo::todo_struct::{Taks};
use crate::utils::fn_lecture::{ lire_int};



fn main() {
    let mut base: Vec<Taks> = Vec::new();
    loop {
        println!("1 Ajouter une taches");
        println!("2 Afficher la liste des taches");
        println!("3 marker une taches comme faire");
        println!("4 supprimer une taches");
        println!("5 quiter");

        let  choise = match lire_int() {
            Ok(nombre)=>{
                nombre
            },
            Err(errer)=>{
                println!("une erreur c'est produite: {:?}",errer);
                5
            }
        };

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
