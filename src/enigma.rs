use std::collections::HashMap;

use crate::{reflector::Reflector, plugboard::Plugboard, rotor::Rotor};

const POMLIST: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Enigma {
    rotors: HashMap<u8, Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(rotors: HashMap<u8, Rotor>, reflector: Reflector, plugboard: Plugboard) -> Self {
        Enigma {
            rotors,
            reflector,
            plugboard,
        }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut r1pos = self.rotors[&1].grund;
        let mut r2pos = self.rotors[&2].grund;
        let mut r3pos = self.rotors[&3].grund;
        let mut scrambled = String::new();

        for letter in text.chars() {
            let mut onecipher = letter;

            r3pos += 1;

            let stepagain1 = self.rotors[&3].step == 13 && r3pos == 26;
            let stepagain2 = self.rotors[&2].step == 13 && r2pos + 1 == 26;

            if r3pos == self.rotors[&3].step || stepagain2 || r2pos + 1 == self.rotors[&2].step || stepagain1 {
                r2pos += 1;

                if self.rotors[&2].step == 13 && r2pos == 26 {
                    r1pos += 1;
                }
            }

            r3pos %= 26;
            r2pos %= 26;
            r1pos %= 26;

            onecipher = self.plugboard.wiring.get(&onecipher).copied().unwrap_or(onecipher);

            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() + r3pos) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((self.mapping(&onecipher) - r3pos + r2pos) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((self.mapping(&onecipher) - r2pos + r1pos) % 26) as u8).unwrap();
            onecipher = self.reflector.setting[((self.mapping(&onecipher) - r1pos) % 26) as u8];
            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() + r1pos) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((self.rotors[&1].wiring.iter().position(|&c| c == onecipher).unwrap() as u8) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() + r2pos - r1pos) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((self.rotors[&2].wiring.iter().position(|&c| c == onecipher).unwrap() as u8) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() + r3pos - r2pos) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() + 26) % 26) as u8).unwrap();
            onecipher = POMLIST.chars().nth((self.rotors[&3].wiring.iter().position(|&c| c == onecipher).unwrap() as u8) % 26).unwrap();

            onecipher = POMLIST.chars().nth(((POMLIST.chars().position(|c| c == onecipher).unwrap() - r3pos) % 26) as u8).unwrap();

            onecipher = self.plugboard.wiring.get(&onecipher).copied().unwrap_or(onecipher);

            scrambled.push(onecipher);
        }

        scrambled
    }

    fn mapping(&self, c: &char) -> u8 {
        c as u8 - 'A' as u8
    }
}

