use avr_device::atmega328p::PORTB; // Importation du port B

// Enum pour définir les modes de la broche
pub enum PinMode {
    Input,
    Output,
    InputPullUp,
}

// Structure représentant une broche GPIO
pub struct GpioPin<'a> {
    port: &'a PORTB, // Référence au registre PORTB
    pin: u8,         // Numéro de la broche
}

impl<'a> GpioPin<'a> {
    // Constructeur qui initialise une broche et configure son mode
    pub fn new(port: &'a PORTB, pin: u8, mode: PinMode) -> Self {
        let gpio_pin = GpioPin { port, pin };
        gpio_pin.set_mode(mode); // Configure le mode au moment de l'initialisation
        gpio_pin
    }

    // Fonction pour configurer la broche en entrée, sortie ou entrée avec pull-up
    pub fn set_mode(&self, mode: PinMode) {
        match mode {
            PinMode::Input => {
                self.set_ddrb_bit(false); // DDRB à 0 pour entrée
                self.set_portb_bit(false); // PORTB à 0 pour flotter (pas de pull-up)
            }
            PinMode::Output => {
                self.set_ddrb_bit(true); // DDRB à 1 pour sortie
            }
            PinMode::InputPullUp => {
                self.set_ddrb_bit(false); // DDRB à 0 pour entrée
                self.set_portb_bit(true);  // PORTB à 1 pour activer pull-up
            }
        }
    }

    // Fonction pour écrire une valeur HIGH (1) ou LOW (0) sur la broche (si sortie)
    pub fn write(&self, high: bool) {
        self.set_portb_bit(high); // Met la broche à HIGH ou LOW
    }

    // Fonction pour lire l'état de la broche (si entrée)
    pub fn read(&self) -> bool {
        self.read_pinb_bit() // Retourne true si la broche est HIGH, false si LOW
    }

    // --- Méthodes privées pour encapsuler l'accès aux registres ---

    // Méthode privée pour définir le bit dans DDRB (1 pour sortie, 0 pour entrée)
    fn set_ddrb_bit(&self, value: bool) {
        unsafe {
            if value {
                self.port.ddrb.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); // Met à 1 pour sortie
            } else {
                self.port.ddrb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin))); // Met à 0 pour entrée
            }
        }
    }

    // Méthode privée pour définir le bit dans PORTB (1 pour HIGH, 0 pour LOW)
    fn set_portb_bit(&self, value: bool) {
        unsafe {
            if value {
                self.port.portb.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); // Met à 1 pour HIGH
            } else {
                self.port.portb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin))); // Met à 0 pour LOW
            }
        }
    }

    // Méthode privée pour lire l'état de la broche dans PINB
    fn read_pinb_bit(&self) -> bool {
        unsafe {
            let pin_value = self.port.pinb.read().bits(); // Lire la valeur du registre PINB
            (pin_value & (1 << self.pin)) != 0 // Retourne true si la broche est HIGH, false si LOW
        }
    }
}
