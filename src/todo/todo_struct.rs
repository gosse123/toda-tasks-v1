use std::io;
use serde::{Serialize,Deserialize};
use crate::utils::fn_lecture::{lire_int,lire_bool,lire_str};


#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct Taks {
    id: u32,
    name: String,
    status: bool,
}


impl Taks {
    

    pub fn add(data: &mut Vec<Taks>) {

        let name = match lire_str() {
            Ok(name)=>{
                name
            },
            Err(errer)=>{
                println!("une erreur c'est produit; {:?}",errer);
                String::from("")
            }
        };

        println!("entrer l'id de la tache");
        let id = match lire_int() {
            Ok(id)=>{
                id
            },
            Err(erre)=>{
                println!("une erreur c'est produit {:?}",erre);
                0
            }
        };
        let status = match lire_bool() {
            Ok(status)=>{
                status
            },
            Err(erreur)=>{
                println!("une erreur c'est produit: {:?}",erreur);
                false
            }
        };
        data.push(Taks { id, name, status });
    }

    pub fn update(data: &mut Vec<Taks>) {
        let mut id_a_supprimer = String::new();
        println!("entrer le id de la taches a supprimer");
        io::stdin()
            .read_line(&mut id_a_supprimer)
            .expect("errreur de lecture");
        let id_a_supprimer: u32 = id_a_supprimer.trim().parse().expect("invalide");

        data.retain(|data| data.id != id_a_supprimer);
    }

    pub fn update_status(data: &mut Vec<Taks>) {
        let mut status: String = String::new();
        let mut name = String::new();
        println!("entrer le nom de la taches a statuer");
        io::stdin()
            .read_line(&mut name)
            .expect("errreur de lecture");

        println!("entrer le status de la taches");
        io::stdin()
            .read_line(&mut status)
            .expect("errreur de lecture");
        let status: bool = status.trim().parse().expect("valeur incorrect");

        for task in data {
            if task.name == name {
                task.status = status;
            }
        }
    }

    pub fn show(data: &Vec<Taks>) {
        println!("------------les Taches --------------------");
        for task in data {
            println!("\tnom: {}", task.name);
            if task.status {
                println!("\tstatus:🎉 Terminer");
            } else {
                println!("\tstatus: ⏳ non terminer");
            }
            println!("\tId : {}", task.id);
            println!("-------------------------------------------------------\n");
        }
    }
}

