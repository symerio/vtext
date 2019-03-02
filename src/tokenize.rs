extern crate regex;
extern crate unicode_segmentation;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// Unicode Segmentation tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation` crate
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
#[derive(Debug)]
pub struct UnicodeSegmentTokenizer {
    pub word_bounds: bool,
}

impl UnicodeSegmentTokenizer {
    /// Create a new instance
    pub fn new(word_bounds: bool) -> UnicodeSegmentTokenizer {
        UnicodeSegmentTokenizer {
            word_bounds: word_bounds,
        }
    }
    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        if self.word_bounds {
            let res = text.split_word_bounds().filter(|x| x != &" ");
            return res.collect::<Vec<&str>>();
        } else {
            return text.unicode_words().collect::<Vec<&str>>();
        }
    }
}

/// Regular expression tokenizer
///
#[derive(Debug)]
pub struct RegexpTokenizer {
    pub pattern: String,
    regexp: Regex,
}

impl RegexpTokenizer {
    /// Create a new instance
    pub fn new(pattern: String) -> RegexpTokenizer {
        let regexp = Regex::new(&pattern).unwrap();

        RegexpTokenizer {
            pattern: pattern,
            regexp: regexp,
        }
    }
    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.regexp.find_iter(text).map(|m| m.as_str()).collect() //.collect::Vec<&str>
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::{RegexpTokenizer, UnicodeSegmentTokenizer};

    #[test]
    fn test_unicode_tokenizer() {
        let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: false };
        let tokens = tokenizer.tokenize(s);
        let b: &[_] = &[
            "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right",
        ];
        assert_eq!(tokens, b);

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: true };
        let tokens = tokenizer.tokenize(s);
        let b: &[_] = &[
            "The", "quick", "(", "\"", "brown", "\"", ")", "fox", "can't", "jump", "32.3", "feet",
            ",", "right", "?",
        ];
        assert_eq!(tokens, b);
    }

    #[test]
    fn test_regexp_tokenizer() {
        let s = "fox can't jump 32.3 feet, right?";

        let tokenizer = RegexpTokenizer::new(r"\b\w\w+\b".to_string());
        let tokens = tokenizer.tokenize(s);
        let b: &[_] = &["fox", "can", "jump", "32", "feet", "right"];
        assert_eq!(tokens, b);
    }
}
