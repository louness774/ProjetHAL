mod gpio;
use avr_device::atmega328p::Peripherals; // Importation des périphériques AVR
use gpio::{GpioPin, PinMode};

fn main() {
    // Obtenir les périphériques AVR
    let dp = Peripherals::take().unwrap();
    
    // Utiliser le bloc de registre PORTB du périphérique
    let pin2 = GpioPin::new(&dp.PORTB, 2, PinMode::Output); // Initialisation en sortie

    // Écrire une valeur HIGH sur la broche
    pin2.write(true);

    // Lire l'état de la broche
    let is_high = pin2.read();
    if is_high {
        println!("La broche est HIGH");
    } else {
        println!("La broche est LOW");
    }
}



