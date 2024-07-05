#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize
}

impl<'a> Token<'a> {
    fn new(text: &'a str, start: usize, end: usize) -> Self {
        Token {
            text, start, end
        }
    }
}

impl<'a> AsRef<str> for Token<'a> {
    fn as_ref(&self) -> &str {
        self.text
    }
}

pub struct TokenIterator<'a> {
    pub text: &'a str,
    current: usize
}

impl<'a> TokenIterator<'a> {
    pub fn new(text: &'a str) -> TokenIterator<'a> {
        TokenIterator { text, current: 0 }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.text.len() {
            return None;
        }

        let text = &self.text[self.current..];

        let index = text.char_indices()
            .find(|(_, c)| !is_break(*c))
            .map(|(i, _)| i);

        if let Some(index) = index {
            let end = next_break(text, index);

            if let Some(end) = end {
                let index_start = self.current;
                let index_end = self.current + end;
                self.current += end + 1;

                return Some(Token::new(&text[index .. end], index_start, index_end));
            }
        }

        self.current = self.text.len();
        None
    }
}


fn next_break(text: &str, current: usize) -> Option<usize> {
    if current >= text.len() {
        return None;
    }

    for (index, char) in text[current..].char_indices() {
        let end_index = current + index;
        if end_index >= text.len() {
            return Some(end_index);
        }

        if is_break(char) {
            return Some(end_index);
        }
    }

    Some(text.len())
}

fn is_break(char: char) -> bool {
    char.is_whitespace() || char == '<' || char == '>'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_break_on_ascii() {
        assert_eq!(next_break("", 0), None);
        assert_eq!(next_break("  ", 0), Some(0));
        assert_eq!(next_break("hello", 0), Some(5));

        assert_eq!(next_break("hello world!", 0), Some(5));
        assert_eq!(next_break("hello world!", 6), Some(12));
    }

    #[test]
    fn test_iterator() {
        let mut iter = TokenIterator::new("hello Rüther friend");
        assert_eq!(iter.next().unwrap().as_ref(), "hello");
        assert_eq!(iter.next().unwrap().as_ref(), "Rüther");
        assert_eq!(iter.next().unwrap().as_ref(), "friend");
        assert!(iter.next().is_none());


        let mut iter = TokenIterator::new("hello    ");
        assert_eq!(iter.next(), Some(Token::new("hello", 0, 5)));
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("    ");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("One<Two>Three");
        assert_eq!(iter.next().unwrap().as_ref(), "One");
        assert_eq!(iter.next().unwrap().as_ref(), "Two");
        assert_eq!(iter.next().unwrap().as_ref(), "Three");
        assert!(iter.next().is_none());
    }
}