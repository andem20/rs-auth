pub fn capitalize(string: &String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    
    chars.into_iter().collect()
}