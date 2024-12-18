# **FileCryptor**  
Un outil simple de chiffrement et déchiffrement de fichiers, développé en Rust.

## **Description**  
**FileCryptor** est une application en ligne de commande (CLI) permettant de chiffrer et de déchiffrer des fichiers de manière sécurisée.  
Elle utilise l'algorithme **AES-128**, qui garantit un chiffrement rapide et fiable pour protéger vos données sensibles.  

### **Fonctionnalités principales** :
- **Chiffrement** : Transforme un fichier en un fichier chiffré illisible sans la clé de déchiffrement.
- **Déchiffrement** : Restaure un fichier chiffré dans son état d'origine à l'aide de la même clé.
- **Génération de clé** : Génère une clé aléatoire de 16 octets et la sauvgarder dans un fichier .
- **Interface simple** : Une utilisation intuitive grâce à des arguments en ligne de commande.
- **Sécurité** : Basé sur les standards modernes de cryptographie.

---

## **Installation**

### **Pré-requis** :
- [Rust et Cargo](https://www.rust-lang.org/tools/install) installés sur votre machine.

### **Cloner et configurer le projet** :
1. Clonez le dépôt :  
   ```bash
   git clone https://github.com/votre-repo/FileCryptor.git
   cd FileCryptor
2. Construisez le projet :
    ```bash
    cargo build --release
### **Demo** :
1. Chiffrement d'un fichier :  
   ```bash
   cargo run -- --mode encrypt --input <chemin_vers_fichier_input> --output <chemin_vers_fichier_output> --key <clé_de_chiffrement>
   Exemple :
   cargo run -- --mode encrypt --input ..\tests\test.txt  --output ..\tests\test_encr.txt --key maSuperclefde16c

2. Déchiffrement d'un fichier en utilisant la clef:
    ```bash
   cargo run -- --mode decrypt --input <chemin_vers_fichier_input> --output <chemin_vers_fichier_output> --key <clé_de_chiffrement_init>
   Exemple :
   cargo run -- --mode decrypt --input ..\tests\test_encr.txt  --output ..\tests\test_decr.txt --key maSuperclefde16c

3. Génération de clé: 
    ```bash
    cargo run -- --mode generate-key --output <chemin_vers_fichier_output>
    Exemple :Cela générera une clé aléatoire de 16 octets et la sauvegardera dans le fichier test_generate_key.txt .
    cargo run -- --mode generate-key --output ..\tests\test_generate_key.txt
    
    
## **A propos**
J'ai développé **FileCryptor** dans le cadre d'un projet personnel pour approfondir les compétences en sécurité informatique et Rust.
