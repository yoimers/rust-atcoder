macro_rules! min {
  ($a:expr $(,)*) => {{
    $a
  }};
  ($a:expr, $b:expr $(,)*) => {{
    std::cmp::min($a, $b)
  }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{
    std::cmp::min($a, min!($($rest),+))
  }};
}

macro_rules! chmin {
  ($base:expr, $($cmps:expr),+ $(,)*) => {{
      let cmp_min = min!($($cmps),+);
      if $base > cmp_min {
          $base = cmp_min;
          true
      } else {
          false
      }
  }};
}

macro_rules! max {
  ($a:expr $(,)*) => {{
      $a
  }};
  ($a:expr, $b:expr $(,)*) => {{
      std::cmp::max($a, $b)
  }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{
      std::cmp::max($a, max!($($rest),+))
  }};
}

macro_rules! chmax {
  ($base:expr, $($cmps:expr),+ $(,)*) => {{
      let cmp_max = max!($($cmps),+);
      if $base < cmp_max {
          $base = cmp_max;
          true
      } else {
          false
      }
  }};
}
