//use colored_text::Colorize;
mod todo;
mod utils;
use std::f32::consts::E;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{ process::exit,fs};
use serde::de::value;

use crate::todo::todo_struct::{Taks};
use crate::utils::fn_lecture::{ FormError, lire_int};
use std::path::Path;

const FILE_PATH : &str = "data/data.json";

pub fn load_data()-> Result<Vec<Taks>,FormError>{
    if !Path::new(FILE_PATH).exists(){
        return  Ok(Vec::new());
    }

    let mut fichier = match File::open(FILE_PATH) {
        Ok(value)=>value,
        Err(e)=>{return Err(FormError::FileNotFound);}
    };

    let mut content = String::new();
    fichier.read_to_string(&mut content).map_err(|_|return Err(FormError::FaiilledRead));

    if content.is_empty(){
        return  Ok(Vec::new());
    }else {
        
    }

}

fn main() {
    let mut base: Vec<Taks> = Vec::new();
    let base_json = serde_json::to_string_pretty(&base).unwrap();

    
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
