use std::collections::HashMap;

fn main() {
    let input = String::from("Rust is empowering developers and fast to learn.");
    let analyzer = TextAnalyzer::new(input);

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

    fn average_word_length(&self) -> f64 {
        let total_length: usize = self.words.iter().map(|w| w.len()).sum();
        if self.words.is_empty() {
            0.0
        } else {
            total_length as f64 / self.words.len() as f64
        }
    }

    fn iter_words(&self) -> WordIterator {
        WordIterator {
            words: &self.words,
            index: 0,
        }
    }

    fn word_pairs(&self) -> WordPairIterator {
        WordPairIterator {
            words: &self.words,
            index: 0,
        }
    }
}

/// Custom iterator for single words
struct WordIterator<'a> {
    words: &'a [String],
    index: usize,
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.words.len() {
            let result = &self.words[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Custom iterator for consecutive word pairs
struct WordPairIterator<'a> {
    words: &'a [String],
    index: usize,
}

impl<'a> Iterator for WordPairIterator<'a> {
    type Item = (&'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 < self.words.len() {
            let pair = (&self.words[self.index], &self.words[self.index + 1]);
            self.index += 1;
            Some(pair)
        } else {
            None
        }
    }
}
