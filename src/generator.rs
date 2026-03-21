use rand::RngExt;

const WORDS: &str = include_str!("words.txt");

pub fn generate(num_words: usize) -> String {
    let words: Vec<&str> = WORDS.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut rng = rand::rng();
    (0..num_words)
        .map(|_| words[rng.random_range(0..words.len())])
        .collect::<Vec<&str>>()
        .join(" ")
}
