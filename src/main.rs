fn main() {
  let s1 = String::from("hello");
  let s2 = s1;

  calculate_length(s2);

  let rect = Rectangle {
    width: 22,
    height: 43
  };

  print!("rect is {:#?}", rect);
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1,
  }
}

fn calculate_length(s: String) -> usize {
  return s.len()
}
