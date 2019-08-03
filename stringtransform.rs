fn capitalize(s: &str) -> String {
    s.chars().enumerate()
        .map(|(i, c)| 
            if i == 0 { c.to_uppercase().nth(0).unwrap() } 
            else { c })
        .collect()
}
