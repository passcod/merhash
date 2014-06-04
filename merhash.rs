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
  let mut mcount: u64 = 0;
  let mut stderr = std::io::stdio::stderr();
  loop {
    count += 1;
    let strg = gen::next();
    hash = hash_code(&strg);
    if hash == 666 {
      println!("{:s}: {:d}", strg, hash);
    }
    if count % 10000000 == 0 {
      mcount += 1;
      count = 0;
      write!(&mut stderr, "{:u} million hashes computed\n", mcount);
    }
  }
}
