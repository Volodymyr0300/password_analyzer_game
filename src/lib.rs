pub struct TextAnalyzer {
    text: String,
    words: Vec<String>,
}

impl TextAnalyzer {
    pub fn new(text: String) -> TextAnalyzer {
        let words = text
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>();

        TextAnalyzer { text, words }
    }

    pub fn average_word_length(&self) -> f64 {
        let total_length: usize = self.words.iter().map(|w| w.len()).sum();
        if self.words.is_empty() {
            0.0
        } else {
            total_length as f64 / self.words.len() as f64
        }
    }

    pub fn iter_words(&self) -> WordIterator {
        WordIterator {
            words: &self.words,
            index: 0,
        }
    }

    pub fn word_pairs(&self) -> WordPairIterator {
        WordPairIterator {
            words: &self.words,
            index: 0,
        }
    }
}

/// Custom iterator for single words
pub struct WordIterator<'a> {
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
pub struct WordPairIterator<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_over_words() {
        let input = String::from("hello world from rust");
        let analyzer = TextAnalyzer::new(input);

        let words: Vec<_> = analyzer.iter_words().collect();
        assert_eq!(words, vec!["hello", "world", "from", "rust"]);
    }

    #[test]
    fn filters_short_words() {
        let input = String::from("hi my rust friends");
        let analyzer = TextAnalyzer::new(input);

        let short_words: Vec<_> = analyzer
            .iter_words()
            .filter(|word| word.len() < 3)
            .collect();

        assert_eq!(short_words, vec!["hi", "my"]);
    }

    #[test]
    fn maps_to_uppercase() {
        let input = String::from("rust lang");
        let analyzer = TextAnalyzer::new(input);

        let uppercased: Vec<_> = analyzer
            .iter_words()
            .map(|word| word.to_uppercase())
            .collect();

        assert_eq!(uppercased, vec!["RUST", "LANG"]);
    }

    #[test]
fn unique_long_words() {
    let input = String::from("rust makes rust powerful and efficient");
    let analyzer = TextAnalyzer::new(input);

    let mut seen: Vec<String> = Vec::new();
    let unique: Vec<_> = analyzer
        .iter_words()
        .filter(|word| word.len() > 4)
        .filter(|word| {
            if seen.contains(&word.to_string()) {
                false
            } else {
                seen.push(word.to_string());
                true
            }
        })
        .take(3)
        .collect();

    assert_eq!(unique, vec!["makes", "powerful", "efficient"]);
}


    #[test]
    fn count_r_words() {
        let input = String::from("Rust rules right now");
        let analyzer = TextAnalyzer::new(input);

        let count_r = analyzer
            .iter_words()
            .filter(|word| word.starts_with('r') || word.starts_with('R'))
            .count();

        assert_eq!(count_r, 3);
    }
}
