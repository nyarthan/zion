use random_string::{charsets::ALPHA_LOWER, generate};

pub fn random_string(length: usize) -> String {
    if cfg!(test) {
        String::from("a".repeat(length))
    } else {
        generate(length, ALPHA_LOWER)
    }
}
