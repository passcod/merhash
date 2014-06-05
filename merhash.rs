#[cfg(test)] extern crate test;

mod gen;

fn hash_code(input: &String) -> i32 {
  let mut hash: i32 = 0;
  if input.len() == 0 {
    0
  } else {
    for c in input.as_slice().chars() {
      hash = ((hash << 5) - hash) + (c as i32);
      hash |= 0;
    }
    hash
  }
}

fn main() {
  let mut hash: i32 = 0;
  let mut count: u64 = 0;
  let mut mcount: f32 = 0.0;
  let mut stderr = std::io::stdio::stderr();
  loop {
    count += 1;
    let strg = gen::next();
    hash = hash_code(&strg);
    if hash == 666 {
      println!("{:s}: {:d}", strg, hash);
    }
    if count % 100000 == 0 {
      println!("{:s}: {:d}", strg, hash);
      mcount += 0.1;
      count = 0;
      write!(&mut stderr, "{:f} million hashes computed\n", mcount);
    }
  }
}

#[bench]
fn hash_a_static(b: &mut test::Bencher) {
  b.iter(|| {
    let strg = "alphabetagamma".to_string();
    hash_code(&strg);
  });
}

#[bench]
fn hash_a_generated(b: &mut test::Bencher) {
  b.iter(|| {
    let strg = gen::next();
    hash_code(&strg);
  });
}
