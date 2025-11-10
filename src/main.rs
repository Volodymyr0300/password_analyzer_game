use password_analyzer_game::TextAnalyzer;

fn main() {
    let input = String::from("Rust is empowering developers and fast to learn.");
    let analyzer = TextAnalyzer::new(input);

    println!("--- Function Word Analysis ---");

    let short_words: Vec<_> = analyzer 
        .iter_words() 
        .filter(|word| word.len() < 4) 
        .collect();

    println!("Short words (<4 letters): {:?}", short_words);

    let uppercased: Vec<_> = analyzer
        .iter_words()
        .map(|word| word.to_uppercase())
        .collect();

    println!("Uppercased words: {:?}", uppercased);

    let mut seen = Vec::new();
    let unique_long_words: Vec<_> = analyzer
        .iter_words()
        .filter(|word| word.len() > 4)
        .filter(|word| {
            if seen.contains(word) {
                false
            } else {
                seen.push(word);
                true
            }
        })
        .take(3)
        .collect();

    println!("First 3 unique long words : {:?}", unique_long_words);

    let count_r = analyzer
        .iter_words()
        .filter(|word| word.starts_with('r') || word.starts_with('R'))
        .count();

    println!("Words starting with 'r': {}", count_r);
    println!("Average word length: {:.2}", analyzer.average_word_length());

    println!("\nIterating over words using our custom iterator:");
    for word in analyzer.iter_words() {
        println!("- {}", word);
    }

    println!("\nIterating over word pairs:");
    for (w1, w2) in analyzer.word_pairs() {
        println!("({w1}, {w2})");
    }
}
