pub fn raindrops(num: u32) -> String {
    let mut s = String::new();
    if num % 3 == 0 { s.push_str("Pling") }
    if num % 5 == 0 { s.push_str("Plang") }
    if num % 7 == 0 { s.push_str("Plong") }
    if s.is_empty() { return num.to_string() }
    s
}
