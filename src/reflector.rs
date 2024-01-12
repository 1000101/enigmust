use std::collections::HashMap;

pub struct Reflector {
    options: HashMap<&'static str, &'static str>,
    typ: &'static str,
    //setting: &'static str,
}

impl Reflector {
    pub fn new(umkehrwalze: &'static str) -> Result<Self, &'static str> {
        let options = HashMap::from([
            ("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            ("Bt", "ENKQAUYWJICOPBLMDXZVFTHRGS"),
            ("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
            ("Ct", "RDOBJNTKVEHMLFCWZAXGYIPSUQ"),
        ]);

        // if !options.contains_key(umkehrwalze) {
        //     return Err(&format!(
        //         "\"{}\" is not a valid reflector type! Possible options are: [{}]",
        //         umkehrwalze,
        //         options.keys().map(|s| *s).collect::<Vec<_>>().join(", ")
        //     ));
        // }

        //let setting = options[umkehrwalze];

        Ok(Reflector {
            options,
            typ: umkehrwalze,
            //setting,
        })
    }
}

// impl std::fmt::Debug for Reflector {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(f, "Reflector {}: {}", self.typ, self.setting)
//   }
// }

// fn main() {
//     match Reflector::new("B") {
//         Ok(reflector) => println!("{:?}", reflector),
//         Err(err) => eprintln!("{}", err),
//     }
// }
