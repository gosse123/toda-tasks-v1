use std::io;

#[derive(Debug)]
pub struct Taks {
    id: u32,
    name: String,
    status: bool,
}

fn lire_int() -> u32 {
    let mut id: String = String::new();
    println!("entrer l'id de la taches");
    io::stdin().read_line(&mut id).expect("errreur de lecture");
    let id: u32 = id.trim().parse().expect("valeur incorrect");
    id
}
fn lire_bool() -> bool {
    let mut status: String = String::new();
    println!("entrer le status de la taches");
    io::stdin()
        .read_line(&mut status)
        .expect("errreur de lecture");
    let status: bool = status.trim().parse().expect("valeur incorrect");
    status
}
fn lire_str() -> String {
    let mut name = String::new();
    println!("entrer le nom de la taches");
    io::stdin()
        .read_line(&mut name)
        .expect("errreur de lecture");
    name
}

impl Taks {
    pub fn add(data: &mut Vec<Taks>) {
        let name = lire_str();
        let id = lire_int();
        let status = lire_bool();
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
