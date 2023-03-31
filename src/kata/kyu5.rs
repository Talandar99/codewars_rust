pub fn make_readable(seconds: u32) -> String {
    let mut _seconds: u32 = seconds % 60;
    let mut _minutes: u32 = (seconds / 60) % 60;
    let mut _hours: u32 = seconds / 60 / 60;
    format!("{:02}:{:02}:{:02}", _hours, _minutes, _seconds)
}
pub fn int32_to_ip(int: u32) -> String {
    let binary_string: String = format!("{int:032b}");
    format!(
        "{}.{}.{}.{}",
        i32::from_str_radix(&binary_string[0..8], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[8..16], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[16..24], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[24..32], 2).expect("Not a Binary Value"),
    )
}
pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut max_sum = 0;

    for i in 0..seq.len() {
        let mut temp_sum = 0;

        for j in i..seq.len() {
            temp_sum = seq[j] + temp_sum;
            if temp_sum > max_sum {
                max_sum = temp_sum.clone();
            }
        }
    }
    max_sum
}
