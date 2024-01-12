use std::collections::HashMap;

pub struct Plugboard {
    pub wiring: HashMap<char, char>,
}

impl Plugboard {
    pub fn new(steckerbrett: Option<HashMap<char, char>>) -> Self {
        let wiring = Self::set_wiring(steckerbrett.unwrap_or_default());
        Plugboard { wiring }
    }

    fn set_wiring(steckerbrett: HashMap<char, char>) -> HashMap<char, char> {
        let mut wiring = HashMap::new();
        for (key, value) in steckerbrett {
            wiring.insert(key, value);
            wiring.insert(value, key);
        }
        wiring
    }
}

// fn main() {
//     let steckerbrett: HashMap<char, char> = [('A', 'B'), ('C', 'D')].iter().cloned().collect();
//     let plugboard = Plugboard::new(Some(steckerbrett));
//     println!("{:?}", plugboard);
// }
