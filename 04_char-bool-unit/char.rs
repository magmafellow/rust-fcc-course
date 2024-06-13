use std::mem::size_of_val;

fn first() {
  let c1: char = 'a'; // 4 bytes
  assert_eq!(size_of_val(&c1), 4);

  let c2: char = 'f';
  assert_eq!(size_of_val(&c2), 4);

  println!("Success!");
}

// -------------------------

fn second() {
  let c1: char = '&';
  print_char(c1);
}
fn print_char(c: char) {
  println!("{}", c)
}

// -------------------------

fn third() {
  let f: bool = false;

  let t: bool = true;

  if !f {
    println!("Well!");
  }
}

// -------------------------

fn four() {
  let f: bool = true;
  let t: bool = true || false;

  assert_eq!(t, f);

  println!("Well")
}
