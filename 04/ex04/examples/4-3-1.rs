fn main() {
  let t1 = (88, true);

  assert_eq!(t1, (88, true));
  assert_eq!(t1.0, 88);
  assert_eq!(t1.1, true);


  // 可変のtuple
  let mut t2 = ((0, 5), (10, -1));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr += 3;
  *y1_ptr *= -1;

  assert_eq!(t2, ((3, -5), (10, -1)));
}