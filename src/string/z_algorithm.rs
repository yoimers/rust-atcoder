use cargo_snippet::snippet;

#[snippet("Zalgorithm")]
pub fn z_algorithm(s: &Vec<char>) -> Vec<usize> {
  let mut retv = vec![0; s.len()];
  retv[0] = s.len();
  let (mut i, mut j) = (1, 0);
  while i < s.len() {
    while i + j < s.len() && s[j] == s[i + j] {
      j += 1;
    }
    retv[i] = j;
    if j == 0 {
      i += 1;
      continue;
    }
    let mut k = 1;
    while i + k < s.len() && k + retv[k] < j {
      retv[i + k] = retv[k];
      k += 1;
    }
    i += k;
    j -= k;
  }
  retv
}

#[test]
fn test_scc() {
  let s = "aaabaaaab".to_string().chars().collect::<Vec<char>>();
  assert_eq!(z_algorithm(&s), vec![9, 2, 1, 0, 3, 4, 2, 1, 0]);
}
