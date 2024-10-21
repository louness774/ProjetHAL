mod gpio;

fn main() {
    let pin = 2;

    // Configurer la broche 2 comme sortie
    gpio::configurer_pin_en_sortie(pin);
    gpio::ecrire_pin(pin, true);  // Écrire un signal HAUT sur la broche 2

    let valeur_lue = gpio::lire_pin(pin);
    println!("La valeur lue sur la broche {} est : {}", pin, valeur_lue);

    // Maintenant on va tester la broche comme entrée
    gpio::configurer_pin_en_entree(pin);
    println!("Broche {} configurée comme entrée.", pin);
}


