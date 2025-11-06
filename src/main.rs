use std::collections::HashMap;

struct Rule<'a> {
    description: &'a str,
    check: Box<dyn Fn(&str) -> bool>
}

fn main() {
    let input = String::from("Rust is fast and safe. Rust is empowering developers.");
    let analyzer = TextAnalyzer::new(input);

    println!("Word count: {}", analyzer.word_count());
    println!("Unique words: {:?}", analyzer.unique_word_count());
    println!("longest word: {:?}", analyzer.longest_word());
    println!("Words longer than 3 letters: {:?}", analyzer.filter_words(|w| w.len() > 3) );
    println!("Awerage word length: {:.2}", analyzer.awerage_word_length());
    println!("Word frequencies: {:?}", analyzer.word_frequencies());
}
struct TextAnalyzer {
    text: String,
    words: Vec<String>,
}

impl TextAnalyzer {
    fn new(text: String) -> TextAnalyzer {
        let words = text
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>();

        TextAnalyzer { text, words }
    }

    fn word_count(&self) -> usize {
        self.words.len()
    }

    fn unique_word_count(&self) -> usize {
        use std::collections::HashSet;
        self.words.iter().collect::<HashSet<_>>().len()
    }

    fn longest_word(&self) -> Option<&String> {
        self.words.iter().max_by_key(|w| w.len())
    }

    fn filter_words<F>(&self, condition: F) -> Vec<&String>
    where
        F: Fn(&String) -> bool,
    {
        self.words.iter().filter(|w| condition(w)).collect()
    }

    fn awerage_word_length(&self) -> f64 {
        let total_length: usize = self.words.iter().map(|w| w.len()).sum();
        let count = self.words.len();

        if count == 0 {
            0.0
        } else {
            total_length as f64 / count as f64
        }
    }

    fn word_frequencies(&self) -> HashMap<&String, usize> {
        let mut freqs = HashMap::new();
        self.words.iter().for_each(|w| {
            *freqs.entry(w).or_insert(0) += 1;
        });
        freqs
    }
}