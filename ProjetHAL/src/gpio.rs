// Module GPIO pour abstraire le comportement des broches numériques de l'Atmega328p

// Configuration d'une broche comme sortie
pub fn configurer_pin_en_sortie(pin: u8) {
    // Ici, on configurera la broche pin comme sortie.
    println!("Configuration de la broche {} en sortie.", pin);
}

// Configuration d'une broche comme entrée
pub fn configurer_pin_en_entree(pin: u8) {
    // Ici, on configurera la broche pin comme entrée.
    println!("Configuration de la broche {} en entrée.", pin);
}

// Écriture d'une valeur (HIGH/LOW) sur une broche
pub fn ecrire_pin(pin: u8, valeur: bool) {
    if valeur {
        println!("Écriture de la valeur HAUT sur la broche {}.", pin);
    } else {
        println!("Écriture de la valeur BAS sur la broche {}.", pin);
    }
}

// Lecture de la valeur d'une broche (HIGH/LOW)
pub fn lire_pin(pin: u8) -> bool {
    println!("Lecture de la valeur sur la broche {}.", pin);
    // Ici, tu devras lire la vraie valeur de la broche en utilisant des instructions spécifiques.
    true // Placeholder, retourne toujours "HIGH" pour l'instant.
}
