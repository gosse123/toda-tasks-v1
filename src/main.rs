//use colored_text::Colorize;
use std::{io, process::exit};

#[derive(Debug)]
struct Taks{
    id:u32,
    name:String,
    status:bool
}

fn lire_int(){
    let mut id :String= String::new();
    println!("entrer l'id de la taches");
    io::stdin()
        .read_line(&mut id)
        .expect("errreur de lecture");
    let id:u32 = id.trim()
                            .parse()
                            .expect("valeur incorrect");
}
fn lire_bool(){
    let mut status:String = String::new();
    println!("entrer le status de la taches");
    io::stdin()
        .read_line(&mut status)
        .expect("errreur de lecture");
    let status:bool = status.trim()
                            .parse()
                            .expect("valeur incorrect");
}
fn lire_str(){
    let mut name = String::new();
    println!("entrer le nom de la taches");
    io::stdin()
        .read_line(&mut name)
        .expect("errreur de lecture");
}

impl Taks {
fn add(data:&mut Vec<Taks>){
   
    
    

    
   

    data.push(Taks { id, name, status });
}

fn update(data:&mut Vec<Taks>){
    let mut id_a_supprimer = String::new();
    println!("entrer le nom de la taches a supprimer");
    io::stdin()
        .read_line(&mut id_a_supprimer)
        .expect("errreur de lecture");
    let id_a_supprimer:u32 = id_a_supprimer.trim().parse().expect("invalide");

   data.retain(|data| data.id != id_a_supprimer);
}

fn update_status(data:&mut Vec<Taks>){
     let mut status:String = String::new();
    let mut name = String::new();
    println!("entrer le nom de la taches a statuer");
    io::stdin()
        .read_line(&mut name)
        .expect("errreur de lecture");

    println!("entrer le status de la taches");
    io::stdin()
        .read_line(&mut status)
        .expect("errreur de lecture");
    let status:bool = status.trim()
                            .parse()
                            .expect("valeur incorrect");


    for task in data{
        if task.name == name{
            task.status = status;
        }
    }
}

fn show(data:& Vec<Taks>){
    println!("------------les Taches --------------------");
    for task in data{
        println!("nom: {}",task.name);
        if task.status {
            println!("status:🎉 Terminer");
        }else {
             println!("status: ⏳ non terminer");
        }
        println!("Id : {}",task.id);
        println!("-------------------------------------------------------");
    }
    println!("");
}
}

fn main() {
    let mut base:Vec<Taks> = Vec::new();
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

        let choise:u32 = choise
                        .trim()
                        .parse()
                        .expect("valeur invalide");

        match choise {
           1=>Taks::add(&mut base),
           2=>Taks::show(&base),
           3=>{Taks::update_status(&mut base);}
           4=>Taks::update(&mut base),
           5=>{println!("merci pour votre utilisation");exit(0);},
           _=>println!("valeur ivalide")
        }
    }
}
