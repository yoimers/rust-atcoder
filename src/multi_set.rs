use cargo_snippet::snippet;

#[snippet("BTreeMultiSet")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTreeMultiSet<T> {
  set: std::collections::BTreeSet<(T, std::cmp::Reverse<usize>)>,
}

#[snippet("BTreeMultiSet")]
impl<T: Ord + Clone + Copy + std::fmt::Debug> BTreeMultiSet<T> {
  pub fn new() -> Self {
    Self {
      set: std::collections::BTreeSet::new(),
    }
  }

  pub fn from(values: &[T]) -> Self {
    let mut retv = Self::new();
    for &v in values {
      retv.insert(v);
    }
    retv
  }

  pub fn len(&self) -> usize {
    self.set.len()
  }

  /// O( log(n) )
  pub fn lower_bound(&self, value: T) -> Option<T> {
    match self.get(value) {
      Some(&(value, _)) => Some(value),
      None => None,
    }
  }

  /// O( log(n) )
  fn get(&self, value: T) -> Option<&(T, std::cmp::Reverse<usize>)> {
    self.set.range((value, std::cmp::Reverse(std::usize::MAX))..).next()
  }

  /// O( log(n) )
  fn pop_first(&mut self) -> Option<T> {
    let retv = self.set.iter().next().map(|&x| x);
    if let Some((v, _)) = retv {
      self.erase(&v);
    }
    retv.map(|x| x.0)
  }

  /// O( log(n) )
  fn pop_last(&mut self) -> Option<T> {
    let retv = self.set.iter().rev().next().map(|&x| x);
    if let Some((v, _)) = retv {
      self.erase(&v);
    }
    retv.map(|x| x.0)
  }

  /// O( log(n) )
  pub fn insert(&mut self, value: T) -> bool {
    if let Some(&(v, std::cmp::Reverse(num))) = self.get(value) {
      if v == value {
        self.set.insert((value, std::cmp::Reverse(num + 1)));
        return false;
      }
    }
    self.set.insert((value, std::cmp::Reverse(0)));
    true
  }

  /// １つ削除する
  /// O( log(n) )
  pub fn erase(&mut self, value: &T) -> usize {
    let mut cnt = 0;
    while let Some(&v) = self.get(*value) {
      if v.0 == *value {
        self.set.remove(&v);
        cnt += 1;
      }
      break;
    }
    cnt
  }
  /// O(n)
  pub fn erase_all(&mut self, value: &T) -> usize {
    let mut cnt = 0;
    while let Some(&v) = self.get(*value) {
      if v.0 == *value {
        self.set.remove(&v);
        cnt += 1;
      } else {
        break;
      }
    }
    cnt
  }

  /// O( log(n) )
  pub fn size(&self, value: T) -> usize {
    if let Some(&(v, std::cmp::Reverse(num))) = self.get(value) {
      if v == value {
        return num + 1;
      }
    }
    0
  }
}

#[cfg(test)]
mod test {
  use crate::multi_set::BTreeMultiSet;

  #[test]
  fn multi_set_test() {
    let mut multiset = BTreeMultiSet::new();
    multiset.insert(1);
    multiset.insert(1);
    multiset.insert(3);
    multiset.insert(6);
    multiset.insert(6);
    multiset.insert(6);
    assert_eq!(multiset.insert(0), true);
    assert_eq!(multiset.insert(0), false);
    assert_eq!(multiset.insert(0), false);
    assert_eq!(multiset.insert(0), false);

    assert_eq!(multiset.lower_bound(0), Some(0));
    assert_eq!(multiset.lower_bound(1), Some(1));
    assert_eq!(multiset.lower_bound(2), Some(3));
    assert_eq!(multiset.lower_bound(3), Some(3));
    assert_eq!(multiset.lower_bound(4), Some(6));

    assert_eq!(multiset.size(4), 0);
    assert_eq!(multiset.size(0), 4);
    assert_eq!(multiset.size(3), 1);
    assert_eq!(multiset.size(-103), 0);

    assert_eq!(multiset.erase_all(&-103), 0);
    assert_eq!(multiset.erase_all(&0), 4);
    assert_eq!(multiset.size(0), 0);
    assert_eq!(multiset.erase(&1), 1);
    assert_eq!(multiset.size(1), 1);
  }

  #[test]
  fn multi_set_pop_test() {
    let mut multiset = BTreeMultiSet::new();
    multiset.insert(1);
    multiset.insert(1);
    multiset.insert(3);
    multiset.insert(6);
    multiset.insert(6);
    multiset.insert(6usize);
    assert_eq!(multiset.size(1), 2);
    assert_eq!(multiset.pop_first(), Some(1));
    assert_eq!(multiset.size(1), 1);
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(3));
    assert_eq!(multiset.pop_last(), Some(1));
    assert_eq!(multiset.pop_last(), None);
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTreeMultiMap<T> {
  map: std::collections::BTreeMap<T, usize>,
  len: usize,
}

impl<T: Ord + Clone + Copy + std::fmt::Debug> BTreeMultiMap<T> {
  pub fn new() -> Self {
    Self {
      map: std::collections::BTreeMap::new(),
      len: 0,
    }
  }

  pub fn from(values: &[T]) -> Self {
    let mut retv = Self::new();
    for &v in values {
      retv.insert(v);
    }
    retv
  }

  pub fn len(&self) -> usize {
    self.len
  }

  /// O( log(n) )
  pub fn lower_bound(&self, value: T) -> Option<T> {
    self.map.range(value..).next().map(|(&x, _)| x)
  }

  /// O( log(n) )
  fn pop_first(&mut self) -> Option<T> {
    let retv = self.map.iter().next().map(|(&x, _)| x);
    if let Some(v) = retv {
      self.erase(&v);
    }
    retv
  }

  /// O( log(n) )
  fn pop_last(&mut self) -> Option<T> {
    let retv = self.map.iter().rev().next().map(|(&x, _)| x);
    if let Some(v) = retv {
      self.erase(&v);
    }
    retv
  }

  /// O( log(n) )
  /// valueが入っていないとき true
  pub fn insert(&mut self, value: T) -> bool {
    let cnt = self.map.entry(value).or_insert(0);
    *cnt += 1;
    self.len += 1;
    if *cnt == 1 {
      true
    } else {
      false
    }
  }

  /// １つ削除する 減らした個数(0 or 1)を返却
  /// O( log(n) )
  pub fn erase(&mut self, value: &T) -> usize {
    let cnt = self.map.entry(*value).or_insert(0);
    if *cnt > 0 {
      *cnt -= 1;
      self.len -= 1;
      if *cnt == 0 {
        self.map.remove(&value);
      }
      1
    } else {
      self.map.remove(&value);
      0
    }
  }

  /// valueを全て削除する 減らした個数を返却
  /// O( log(n) )
  pub fn erase_all(&mut self, value: &T) -> usize {
    let cnt = self.map.entry(*value).or_insert(0);
    let ret = *cnt;
    *cnt = 0;
    self.len -= *cnt;
    self.map.remove(&value);
    ret
  }

  /// O( log(n) )
  pub fn size(&self, value: T) -> usize {
    self.map.get(&value).map(|&x| x).unwrap_or_default()
  }
}

#[cfg(test)]
mod test1 {
  use crate::multi_set::BTreeMultiMap;

  #[test]
  fn multi_set_test() {
    let mut multiset = BTreeMultiMap::new();
    multiset.insert(1);
    multiset.insert(1);
    multiset.insert(3);
    multiset.insert(6);
    multiset.insert(6);
    multiset.insert(6);
    assert_eq!(multiset.insert(0), true);
    assert_eq!(multiset.insert(0), false);
    assert_eq!(multiset.insert(0), false);
    assert_eq!(multiset.insert(0), false);

    assert_eq!(multiset.lower_bound(0), Some(0));
    assert_eq!(multiset.lower_bound(1), Some(1));
    assert_eq!(multiset.lower_bound(2), Some(3));
    assert_eq!(multiset.lower_bound(3), Some(3));
    assert_eq!(multiset.lower_bound(4), Some(6));

    assert_eq!(multiset.size(4), 0);
    assert_eq!(multiset.size(0), 4);
    assert_eq!(multiset.size(3), 1);
    assert_eq!(multiset.size(-103), 0);

    assert_eq!(multiset.erase_all(&-103), 0);
    assert_eq!(multiset.erase_all(&0), 4);
    assert_eq!(multiset.size(0), 0);
    assert_eq!(multiset.erase(&1), 1);
    assert_eq!(multiset.size(1), 1);
  }

  #[test]
  fn multi_set_pop_test() {
    let mut multiset = BTreeMultiMap::new();
    multiset.insert(1);
    multiset.insert(1);
    multiset.insert(3);
    multiset.insert(6);
    multiset.insert(6);
    multiset.insert(6usize);
    assert_eq!(multiset.size(1), 2);
    assert_eq!(multiset.pop_first(), Some(1));
    assert_eq!(multiset.size(1), 1);
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(6));
    assert_eq!(multiset.pop_last(), Some(3));
    assert_eq!(multiset.pop_last(), Some(1));
    assert_eq!(multiset.pop_last(), None);
  }
}
