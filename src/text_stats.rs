//! Text statistics utilities.

/// Statistics about a piece of text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextStats {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
    pub longest_word_len: usize,
}

/// Compute text statistics for the given input.
///
/// Returns a [`TextStats`] with counts. An empty input yields all-zero stats.
pub fn analyze(text: &str) -> TextStats {
    let chars = text.chars().count();
    let words: Vec<&str> = text.split_whitespace().collect();
    let longest_word_len = words.iter().map(|w| w.chars().count()).max().unwrap_or(0);
    let lines = if text.is_empty() {
        0
    } else {
        text.lines().count()
    };
    TextStats {
        chars,
        words: words.len(),
        lines,
        longest_word_len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let s = analyze("");
        assert_eq!(
            s,
            TextStats {
                chars: 0,
                words: 0,
                lines: 0,
                longest_word_len: 0
            }
        );
    }

    #[test]
    fn simple_sentence() {
        let s = analyze("hello world");
        assert_eq!(s.chars, 11);
        assert_eq!(s.words, 2);
        assert_eq!(s.lines, 1);
        assert_eq!(s.longest_word_len, 5);
    }

    #[test]
    fn multiline() {
        let s = analyze("a\nbb\nccc");
        assert_eq!(s.lines, 3);
        assert_eq!(s.longest_word_len, 3);
    }
}
