fn quote(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    format!("\"{}\"", s)
}

pub fn main() {
    winres::WindowsResource::new().compile().unwrap();

    let preload = std::fs::read_to_string("preload.dat").unwrap();
    let preload = preload.lines().collect::<Vec<_>>();
    let preload = format!("pub const PRELOAD: [&str; {}] = [\n    {}\n];\n", preload.len(), quote(&preload.join("\",\n    \"")),);
    std::fs::write(format!("{}/preload.rs", std::env::var("OUT_DIR").unwrap()), preload).unwrap();
}
