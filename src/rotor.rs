#[derive(Clone,Copy)]
enum RotorType {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}
#[derive(Clone)]
pub struct Rotor {
    rotor_type: RotorType,
    grund: u8,
    ring: u8,
    step: u8,
    wiring: String,
}

fn steps(rotor_type: RotorType) -> u8 {
    match rotor_type {
        RotorType::I => 17,
        RotorType::II => 5,
        RotorType::III => 22,
        RotorType::IV => 10,
        RotorType::V => 26,
        RotorType::VI => 13,
        RotorType::VII => 13,
        RotorType::VIII => 13,
    }
}

fn wiring(rotor_type: RotorType) -> String {
    match rotor_type {
        RotorType::I => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
        RotorType::II => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
        RotorType::III => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
        RotorType::IV => "ESOVPZJAYQUIRHXLNFTGKDCMWB",
        RotorType::V => "VZBRGITYUPSDNHLXAWMJQOFECK",
        RotorType::VI => "JPGVOUMFYQBENHZRDKASXLICTW",
        RotorType::VII => "NZJHGRCXMYSWBOUFAIVLPEKQDT",
        RotorType::VIII => "FKQHTLXOCBJSPDZRAMEWNIUYGV",
    }
    .to_string()
}

impl Rotor {
    pub fn new(&self, walzen: RotorType, ringstellung: u8, grundstellung: u8) -> Self {
        // let wiring = Rotor::set_wiring(&wiring, ringstellung)

        Rotor {
            rotor_type: walzen,
            grund: grundstellung,
            ring: ringstellung,
            step: steps(walzen),
            wiring: wiring(walzen),
        }
    }

    // fn set_wiring(wiring: &str, ringstellung: u8) -> String {
    //     if ringstellung > 0 {
    //         let pomlist = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    //         let pom = String::new();

    //         let vec = wiring.chars().count();
    //         // put wires in the right spot
    //         // pom.push_str(&wiring[wiring.len() - ringstellung]);
    //         // pom.push_str(&wiring[..26 - ringstellung]);

    //         // adjust the letter + ringstellung
    //         for letter in pom.chars() {
    //             let index = pomlist.chars().position(|c| c == letter).unwrap();
    //            // pom.push(pomlist.chars().cycle().nth(index + ringstellung).unwrap());
    //         }

    //         pom
    //     } else {
    //         wiring.to_string()
    //     }
    // }
}

// fn main() {
//   let rotor = Rotor::new("I", 10, 'A');
//   println!("{:?}", rotor);
// }
