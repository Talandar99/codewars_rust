pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if a + b > c && b + c > a && c + a > b {
        return true;
    }
    return false;
}
