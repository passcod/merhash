use std::string::String;
use std::rand;
use std::rand::Rng;

fn chr() -> Option<&u8> {
  let alphabet = bytes!("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890_-[]{}^|\\`");
  rand::task_rng().choose(alphabet)
}

fn string(len: uint) -> String {
  let mut result = String::with_capacity(len);
  for _ in range(0, len) {
    result.push_char(match chr() {
      Some(c) => *c,
      None => 48 as u8
    } as char);
  }
  result
}

fn length(max: uint) -> uint {
  (rand::random::<f32>() * ((max - 1) as f32)).ceil() as uint
}

pub fn next() -> String {
  string(length(30))
}
