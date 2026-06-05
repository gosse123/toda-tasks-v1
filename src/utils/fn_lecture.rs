use std::io;
#[derive(Debug)]
pub enum FormError {
    EmptyInput,
    InvalidEnter,
    ValueTooLow,
    FileNotFound,
    FaiilledRead,
    DesError
}

pub fn lire_int() -> Result<u32,FormError> {
    let mut id: String = String::new();
    io::stdin().read_line(&mut id);
    if id.is_empty() {
        return Err(FormError::EmptyInput);
    }
    let id: u32 = id.trim().parse().map_err(|_|FormError::InvalidEnter)?;
    if id > 6 {
       return  Err(FormError::ValueTooLow);
    }
    Ok(id)
}

pub fn lire_bool() -> Result<bool,FormError> {
    let mut status: String = String::new();
    println!("entrer le status de la taches");
    io::stdin()
        .read_line(&mut status)
        .expect("errreur de lecture");

    match status.trim().to_lowercase().as_str() {
        "vrai" | "oui" | "yes" | "true" | "y" | "o" =>  Ok(true),
        "faux" | "non" | "no"  | "false" | "n"      =>  Ok(false),
        _ =>  Err(FormError::InvalidEnter)
    }
    
}

pub fn lire_str() -> Result<String,FormError> {
    let mut name = String::new();
    println!("entrer le nom de la taches");
    io::stdin()
        .read_line(&mut name).map_err(|_|FormError::InvalidEnter)?;
    if name.is_empty() {
        return Err(FormError::EmptyInput);
    }
    Ok(name)
}