use cargo_snippet::snippet;

#[snippet("gcd")]
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[snippet("lcd")]
fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
#[snippet("extgcd")]
//ax+by=1, (d, x, y)
fn extgcd1(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (d, mut y, x) = extgcd1(b, a % b);
        y -= (a / b) * x;
        (d, x, y)
    } else {
        (a, 1, 0)
    }
}
#[snippet("extgcd")]
fn extgcd2(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b != 0 {
        d = extgcd2(b, a % b, y, x);
        *y -= (a / b) * (*x);
    } else {
        *x = 1;
        *y = 0;
    }
    d
}

#[snippet("modinv")]
fn modinv(mut a: i64, modulo: i64) -> i64 {
    let mut b = modulo;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= modulo;
    if u < 0 {
        u += modulo;
    }
    u
}
