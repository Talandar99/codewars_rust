pub fn solution(num: i32) -> i32 {
    let mut sum: i32 = 0;
    if num > 0 {
        for x in 0..num {
            if (x % 3 == 0) || (x % 5 == 0) {
                sum = sum + x;
            }
        }
    }
    return sum;
}
pub fn count_bits(n: i64) -> u32 {
    let mut number: i64 = n;
    let mut accumulator: u32 = 0;

    while number != 0 {
        if number % 2 == 1 {
            accumulator = accumulator + 1;
        }
        number = number / 2;
    }
    return accumulator;
}
