use std::collections::HashMap;

use crate::{reflector::Reflector, plugboard::Plugboard };// enigma::Enigma};

//mod enigma;
mod plugboard;
mod reflector;
mod rotor;

// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");

  //   let rotors = [
  //     (1, rotor::Rotor::new("I", 0, 17)),
  //     (2, rotor::Rotor::new("II", 0, 5)),
  //     (3, rotor::Rotor::new("III", 0, 22)),
  // ];

  let reflector = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
  //let plugboard = Plugboard::new(HashMap::new());

  //let enigma = Enigma::new(rotors, reflector, plugboard);

  let plaintext = "HELLO";
  //let ciphertext = enigma.encrypt(plaintext);

  println!("Plaintext: {}", plaintext);
  //println!("Ciphertext: {}", ciphertext);
}


