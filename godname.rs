#[cfg(test)] extern crate test;

use std::string::String;
use std::vec::Vec;

static particles: &'static [&'static [&'static str]] = &[
  &["ll", "mn", "m", "z", "n", "p", "d", "rh", "g", "r"],
  &["ts", "s", "r", "h", "th", "sh", "j", "n", "l"],
  &["a", "e", "u", "o", "ae", "ea", "eu"],
  &["es", "os", "on", ""]
];

enum Particle {
  Middle,
  Consonant,
  Vowel,
  Ending
}

fn pick(typ: Particle) -> String {
  let i: f32 = std::rand::random::<f32>();
  let len: uint = particles[typ as uint].len();
  let n: uint = (i * ((len - 1) as f32)).ceil() as uint;
  particles[typ as uint][n].to_string()
}

struct GodName {
  len: uint,
  body: Vec<String>
}

impl GodName {
  fn prepend(&mut self, new: Particle) {
    self.body.unshift(pick(new));
    match new {
      Middle => self.prepend(Vowel),
      Consonant => {
        if self.body.len() < self.len {
          self.prepend(Vowel);
        }
      },
      Vowel => {
        if self.body.len() >= self.len - 1 {
          self.prepend(Consonant)
        } else {
          self.prepend(Middle)
        }
      },
      Ending => self.prepend(Vowel)
    };
  }

  fn build(&mut self) {
    self.prepend(Ending);
  }

  fn to_string(&mut self) -> String {
    let mut name = "".to_string();
    for piece in self.body.iter() {
      name = format_args!(std::fmt::format, "{:s}{:s}", name, *piece);
    }
    name
  }
}

pub fn next() -> String {
  let mut name = GodName {len: 30, body: Vec::new()};
  name.build();
  name.to_string()
}

#[bench]
fn make_a_name(b: &mut test::Bencher) {
  b.iter(|| {
    let mut name = GodName {len: 30, body: Vec::new()};
    name.build();
    name.to_string();
  });
}
