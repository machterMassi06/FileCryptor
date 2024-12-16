use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::io::Write;
use std::{fs::File, io::Read};

pub fn decrypt_file(input_f :&str,output_f:&str,key:&[u8]){

    println!("Déchiffrement de fichier :{}",input_f);

    // Lecture de fichier a dechiffré 
    let mut file = File::open(input_f).expect("ERR Lors d'ouverture du fichier!");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Err Lors de lecture de fichier !");

    // Séparer l'IV (16 premiers octets) et les données chiffrées
    let (iv, data) = data.split_at(16);

    //Le déchiffreur AES en mode CBC avec padding PKCS7 
    let decr=Cbc::<Aes128,Pkcs7>::new_from_slices(key, &iv).expect("ERR lors de déchiffrement.");

    // DéChiffrement des donnes de fichier 
    let decrypted_data=decr.decrypt_vec(&data).expect("ERR lors de déchiffrement.");

    // Sauvgarde dans fichier de sortie 
    let mut output_file = File::create(output_f).expect("ERR Lors de la creation de fichier!");
    output_file.write_all(&decrypted_data).expect("Err lors d'ecriture dans le fichier!");

    println!("Déchiffrement de fichier {input_f} effectué avec succès dans le fichier : {output_f} .");
}