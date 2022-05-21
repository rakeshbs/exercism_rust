pub fn abbreviate(phrase: &str) -> String {
    let mut prev = ' ';
    let mut acronym = String::new();
    for c in phrase.chars() {
        if (prev == ' ' || prev == '-') && c.is_alphabetic() {
            acronym.push(c.to_ascii_uppercase());
        } else if c.is_uppercase() && !prev.is_uppercase() {
            acronym.push(c);
        }
        prev = c;
    }
    acronym
}
