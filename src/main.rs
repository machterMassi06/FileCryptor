use clap::{Command, Arg};

mod encrypt;
mod decrypt;
mod utils;

fn main(){
    
    let matches = Command::new("FileCryptor")
        .version("0.1.0")
        .author("Massinissa MACHTER")
        .about("Chiffrement et déchiffrement de fichiers avec AES-128")
        .arg(Arg::new("mode")
            .long("mode")
            .required(true)
            .value_parser(["encrypt", "decrypt"])  
            .help("Mode d'opération : encrypt ou decrypt"))
        .arg(Arg::new("input")
            .long("input")
            .required(true)
            .value_parser(clap::value_parser!(String)) 
            .help("Chemin vers le fichier à traiter"))
        .arg(Arg::new("output")
            .long("output")
            .required(true)
            .value_parser(clap::value_parser!(String)) 
            .help("Chemin vers le fichier de sortie"))
        .arg(Arg::new("key")
            .long("key")
            .required(true)
            .value_parser(clap::value_parser!(String)) 
            .help("Clé de chiffrement de 16 octets (128 bits)"))
        .get_matches();

    // Récupérer les valeurs des arguments
    let mode = matches.get_one::<String>("mode").unwrap();
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let key = matches.get_one::<String>("key").unwrap().as_bytes();

    // Vérifier si la clé fait 16 octets 
    if key.len() != 16 {
        println!("La clé doit être de 16 octets pour AES-128.");
    }

    //déterminer le mode (chiffrement/déchiffrement)
    match mode.as_str() {
        "encrypt" => {
            encrypt::encrypt_file(input, output, key);
        },
        /*"decrypt" => {
            decrypt::decrypt_file(input, output, key)?;
        }*/
        _ => {
            println!("Mode invalide. Utilisez 'encrypt' ou 'decrypt'.");
            
        },
    }
}
