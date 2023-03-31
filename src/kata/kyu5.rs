pub fn make_readable(seconds: u32) -> String {
    let mut _seconds: u32 = seconds % 60;
    let mut _minutes: u32 = (seconds / 60) % 60;
    let mut _hours: u32 = seconds / 60 / 60;
    format!("{:02}:{:02}:{:02}", _hours, _minutes, _seconds)
}
fn int32_to_ip(int: u32) -> String {
    let binary_string: String = format!("{int:032b}");
    format!(
        "{}.{}.{}.{}",
        i32::from_str_radix(&binary_string[0..8], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[8..16], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[16..24], 2).expect("Not a Binary Value"),
        i32::from_str_radix(&binary_string[24..32], 2).expect("Not a Binary Value"),
    )
}
