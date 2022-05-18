pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().for_each(|(i, val)| {
        println!("{}", val);
    });
    return Vec::new();
}
