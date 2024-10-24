#![no_std]
#![no_main]  // Utilisé pour les microcontrôleurs, où on définit une fonction d'entrée personnalisée

mod gpio;
use avr_device::atmega328p::Peripherals; // Importation des périphériques AVR
use gpio::{GpioPin, PinMode};
use core::panic::PanicInfo;  // Importation nécessaire pour le gestionnaire de panique

// Fonction principale, utilisée comme point d'entrée au lieu de `main`
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Obtenir les périphériques AVR
    let dp = Peripherals::take().unwrap();
    
    // Utiliser le bloc de registre PORTB du périphérique
    let pin2 = GpioPin::new(&dp.PORTB, 2, PinMode::Output); // Initialisation en sortie

    // Écrire une valeur HIGH sur la broche
    pin2.write(true);

    // Lire l'état de la broche
    let is_high = pin2.read();
    if is_high {
        // Dans un environnement embarqué, au lieu de `println!`, tu peux envoyer des données par UART ou utiliser des LEDs pour signaler l'état
        // Par exemple, tu pourrais définir une fonction pour allumer ou éteindre une LED ici.
    }

    loop {}
}

// Gestionnaire de panique personnalisé
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

