//use colored_text::Colorize;
use std::io;

struct Taks{
    id:u32,
    name:String,
    status:bool
}

fn add(taks:&Taks,data:&mut Vec<Taks>){
    let mut name = String::new();
    let mut status:String = String::new();
    let mut id :String= String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("errreur de lecture");

    io::stdin()
        .read_line(&mut status)
        .expect("errreur de lecture");
    let status:bool = status.trim()
                            .parse()
                            .expect("valeur incorrect");
    io::stdin()
        .read_line(&mut id)
        .expect("errreur de lecture");
    let id:u32 = id.trim()
                            .parse()
                            .expect("valeur incorrect");

    data.push(Taks { id, name, status });
}


fn main() {
    let base:Vec<Taks> = Vec::new();
    let mut choise = String::new();
    loop {
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
           1=>add(&Taks, &mut base),
        }
    }
}
