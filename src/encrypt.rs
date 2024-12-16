use aes::Aes128;
use block_modes::{BlockMode, Cbc}; // Mode CBC pour le chiffrement par blocs
use block_modes::block_padding::Pkcs7; // Padding PKCS7 pour remplir les blocs
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write}; 

pub fn encrypt_file(input_f :&str,output_f:&str,key:&[u8]){
    println!("Chiffrement de fichier :{}",input_f);
    // Lecture fichier en entrée 
    let mut file=File::open(input_f).expect("ERR Lors d'ouverture de fichier {input_f}");
    let mut data= Vec::new();
    file.read_to_end(&mut data).expect("ERR Lors de la lecture de fichier{input_f}");

    // Générer un vecteur d'initialisation (IV) aléatoire de 16 octets
    let iv=rand::thread_rng().gen::<[u8;16]>();

    // Créer le chiffreur AES en mode CBC avec padding PKCS7
    let chiff = Cbc::<Aes128,Pkcs7>::new_from_slices(key, &iv).expect("ERR lors de chiffrement");
    // Chiffrement des donnes de fichier 
    let encrypted_data=chiff.encrypt_vec(&data);

    // Sauvegarde dans le fichier de sortie : IV + données chiffrées 
    let mut file_output =File::create(output_f).expect("ERR Lors de la creation de fichier {output_f}");
    output_file.write_all(&iv).expect("ERR Lors de l'écriture de l'IV dans le fichier!"); 
    file_output.write_all(&encrypted_data).expect("ERR lors d'ecriture dans le fichier {output_f}");

    println!("Chiffrement de fichier {input_f} effectué avec succès dans le fichier : {output_f} .");
}