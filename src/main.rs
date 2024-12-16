use clap::{Command, Arg};
mod encrypt;
mod decrypt;
mod utils;

fn main(){
    
    let matches = Command::new("FileCryptor")
        .version("0.1.0")
        .author("MassMACH")
        .about("Chiffrement et déchiffrement de fichiers avec AES-128")
        .arg(Arg::new("mode")
            .long("mode")
            .required(true)
            .value_parser(["encrypt", "decrypt","generate-key"])  
            .help("Mode d'opération : encrypt ou decrypt ou generate-key"))
        .arg(Arg::new("input")
            .long("input")
            .required(false)
            .value_parser(clap::value_parser!(String)) 
            .help("Chemin vers le fichier à traiter (requis pour encrypt et decrypt)"))
        .arg(Arg::new("output")
            .long("output")
            .required(true)
            .value_parser(clap::value_parser!(String)) 
            .help("Chemin vers le fichier de sortie (requis pour encrypt et decrypt et generate-key)"))
        .arg(Arg::new("key")
            .long("key")
            .required(false)
            .value_parser(clap::value_parser!(String)) 
            .help("Clé de chiffrement de 16 octets (128 bits)"))
        .get_matches();

    // Récupérer les valeurs des arguments
    let mode = matches.get_one::<String>("mode").unwrap();




    //déterminer le mode (chiffrement/déchiffrement ou Generation de clé)
    match mode.as_str() {
        "encrypt" | "decrypt" => {
            let input = matches.get_one::<String>("input").unwrap();
            let output = matches.get_one::<String>("output").unwrap();
            let key = matches.get_one::<String>("key").unwrap().as_bytes();
            utils::check_key_is_16_o(key).expect("La clé doit être de 16 octets!");
            match mode.as_str(){
                "encrypt"=> encrypt::encrypt_file(input, output, key),
                "decrypt"=> decrypt::decrypt_file(input, output, key),
                _ => unreachable!(),
            }
            
        },
        "generate-key"=>{
            let output = matches.get_one::<String>("output").unwrap();
            let key=utils::generate_key();
            utils::save_key_to_file(&key,output);
        },
        _ => {
            println!("Mode invalide. Utilisez 'encrypt' ou 'decrypt' ou 'generate-key'.");
            
        },
    }
}
