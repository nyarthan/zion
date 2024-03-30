use sha1::{Digest, Sha1};

pub fn gen_coverage_symbol(input: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let hash = hasher.finalize();
    let hex_string = hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .take(10 / 2) // 5 bytes -> 10 chars
        .collect::<Vec<_>>()
        .join("");

    hex_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn produces_string_length_10() {
        let hash = gen_coverage_symbol("this is a string");
        assert_eq!(hash.len(), 10);
    }
}
