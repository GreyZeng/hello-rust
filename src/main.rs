fn main() {
    println!("hello, world!");
    let result = gcd(64, 34);
    println!("{}", result)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // Rust 中有 return 语句，但是这里的gcd函数不需要，如果一个函数以没有尾随着分号的表达式结尾，那么这个表达式就是函数的返回值
    n
}

// Rust 内置单元测试
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}