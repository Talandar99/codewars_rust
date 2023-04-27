fn main() {
    print!("{}", binary_slice_to_number(&[0, 0, 1, 0]));
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, &bit| acc * 2 + bit)
}
