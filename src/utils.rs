use rand::Rng;

pub fn gen_id() -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| HEX_CHARS[rng.gen_range(0..16)] as char)
        .collect()
}
