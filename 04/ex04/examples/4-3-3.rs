fn print_info(name: &str, s1: &[char]) {
  println!(" {:9} - {}, {:?}, {:?}, {:?}",
    name,
    s1.len(),   // usize
    s1.first(), // Option<char>
    s1[1],      // char
    s1.last(),  // Option<char>
  );
}


fn main() {
  let a1 = ['a', 'b', 'c', 'd'];
  println!("a1: {:?}", a1);

  print_info("&a1[..]", &a1[..]);
  print_info("&a1[", &a1);
  print_info("&a1[1..3]", &a1[1..3]);

  let v1 = vec!['e', 'f', 'g', 'h'];
  println!("v1: {:?}", v1);
  print_info("&v1[..]", &v1[..]);
  print_info("&v1", &v1);
  print_info("&v1[1..3]", &v1[1..3]);

  let mut a2 = [5, 4, 3, 2];
  let s1 = &mut a2[1..3];

  s1[0] = 6;
  s1[1] *= 10;
  s1.swap(0, 1);
  assert_eq!(s1, [30, 6]);

  assert_eq!(a2, [5, 30, 6, 2]); // s1は参照であるため，a2の要素も変更される
}