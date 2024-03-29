/// [bluss/permutohedron](https://github.com/bluss/permutohedron)
use cargo_snippet::snippet;

#[snippet("LexicalPermutation")]
pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

#[snippet("LexicalPermutation")]
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}
#[test]
fn test_lexical_permutation() {
    let mut x = vec![0, 1, 2, 3];
    loop {
        // 0,1,2,3
        // 0,1,3,2
        // 0,2,1,3
        // 0,2,3,1
        // ...
        dbg!(&x);
        if !x.next_permutation() {
            break;
        }
    }
}
