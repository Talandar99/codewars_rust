pub fn make_readable(seconds: u32) -> String {
    let mut _seconds: u32 = seconds % 60;
    let mut _minutes: u32 = (seconds / 60) % 60;
    let mut _hours: u32 = seconds / 60 / 60;
    format!("{:02}:{:02}:{:02}", _hours, _minutes, _seconds)
}
