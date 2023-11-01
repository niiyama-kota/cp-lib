use std::collections::HashMap;

pub fn factoring(mut n: u64) -> HashMap<u64, u64> {
    let mut ret = HashMap::<u64, u64>::new();
    let mut x = 2;
    while n > 1 {
        while n % x == 0 {
            n /= x;
            ret.insert(x, *ret.get(&x).unwrap_or(&0u64) + 1);
        }
        x += 1;
    }

    return ret;
}

#[test]
fn test_factoring() {
    let n = 12345u64;
    let mp = factoring(n);
    println!("{:?}", mp);
}
