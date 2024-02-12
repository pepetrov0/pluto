pub const BID_LENGTH: usize = 16;
pub const BID_ALPHABET: &[char; 36] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let build_id = nanoid::nanoid!(BID_LENGTH, BID_ALPHABET);
    println!("cargo:rustc-env=PLUTO_BUILD_ID={}", build_id);
}
