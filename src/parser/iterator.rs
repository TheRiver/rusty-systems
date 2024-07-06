use crate::error::{Error, ErrorKind};

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
            .find(|(_, c)| !is_consume(*c))
            .map(|(i, _)| i);

        if let Some(index) = index {
            let brk = next_break(text, index);

            if let Break::Token(i, c) = brk {
                if i == 0 {
                    let mut chars : Vec<char> = vec![c];
                    let mut lookahead = next_break(text, index + 1);
                    while lookahead.is_token() && lookahead.index() == index + chars.len() {
                        chars.push(c);
                        lookahead = next_break(text, index + chars.len())
                    }

                    let index_start = self.current;
                    let index_end = self.current + chars.len();
                    self.current += chars.len();

                    return Some(Token::new(&text[index..index + chars.len()], index_start, index_end));
                }
            }

            let index_start = self.current;
            let index_end = self.current + brk.index();
            self.current += brk.index();

            if brk.is_ignore() {
                self.current += 1;
            }

            if brk.is_end() {
                self.current = self.text.len();
                return Some(Token::new(&text[index..brk.index()], index_start, index_end));
            }

            return Some(Token::new(&text[index .. brk.index()], index_start, index_end));
        }

        self.current = self.text.len();
        None
    }
}


fn next_break(text: &str, current: usize) -> Break {
    if current >= text.len() {
        return Break::end(current)
    }

    for (index, char) in text[current..].char_indices() {
        let end_index = current + index;

        if is_break(char) {
            return Break::build(end_index, char).unwrap();
        }
    }

    Break::end(text.len())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Break {
    /// There is nothing left. We are done with the string.
    End(usize),
    /// We should ignore this break character. Just move on.
    Ignore(usize, char),
    /// This break character will be a part of the next token
    Token(usize, char)
}

impl Break {
    fn build(index: usize, char: char) -> Result<Self, Error> {
        if is_consume(char) {
            return Ok(Break::Ignore(index, char))
        }

        if is_break(char) {
            return Ok(Break::Token(index, char))
        }

        Err(Error::new(ErrorKind::Parse, format!("Character '{char} is not a break point'")))
    }

    fn end(index: usize) -> Self {
        Break::End(index)
    }

    fn index(self) -> usize {
        match self {
            Break::End(index) => index,
            Break::Ignore(index, _) => index,
            Break::Token(index, _) => index,
        }
    }
    
    #[inline]
    fn is_end(self) -> bool {
        matches!(self, Break::End(_))
    }

    #[inline]
    fn is_ignore(self) -> bool {
        matches!(self, Break::Ignore(_, _))
    }

    #[inline]
    fn is_token(self) -> bool {
        matches!(self, Break::Token(_, _))
    }
}

fn is_break(char: char) -> bool {
    if char.is_whitespace() {
        return true;
    }

    matches!(char, '<' | '>' | '-' | '+' | '(' | ')')

}

#[inline]
fn is_consume(char: char) -> bool {
    char.is_whitespace()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_break_on_ascii() {
        assert_eq!(next_break("", 0), Break::end(0));
        assert_eq!(next_break("  ", 0), Break::Ignore(0, ' '));
        assert_eq!(next_break("hello", 0), Break::end(5));

        assert_eq!(next_break("hello world!", 0), Break::Ignore(5, ' '));
        assert_eq!(next_break("hello world!", 6), Break::end(12));
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
        assert_eq!(iter.next().unwrap().as_ref(), "<");
        assert_eq!(iter.next().unwrap().as_ref(), "Two");
        assert_eq!(iter.next().unwrap().as_ref(), ">");
        assert_eq!(iter.next().unwrap().as_ref(), "Three");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("One-Two");
        assert_eq!(iter.next().unwrap().as_ref(), "One");
        assert_eq!(iter.next().unwrap().as_ref(), "-");
        assert_eq!(iter.next().unwrap().as_ref(), "Two");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("One+Two");
        assert_eq!(iter.next().unwrap().as_ref(), "One");
        assert_eq!(iter.next().unwrap().as_ref(), "+");
        assert_eq!(iter.next().unwrap().as_ref(), "Two");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("A->B");
        assert_eq!(iter.next().unwrap().as_ref(), "A");
        assert_eq!(iter.next().unwrap().as_ref(), "->");
        assert_eq!(iter.next().unwrap().as_ref(), "B");
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_token_breaks() {
        let mut iter = TokenIterator::new(">");
        assert_eq!(iter.next().unwrap().as_ref(), ">");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("-");
        assert_eq!(iter.next().unwrap().as_ref(), "-");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("->");
        assert_eq!(iter.next().unwrap().as_ref(), "->");
        assert!(iter.next().is_none());

        let mut iter = TokenIterator::new("-->");
        assert_eq!(iter.next().unwrap().as_ref(), "-->");
        assert!(iter.next().is_none());


    }

    #[test]
    fn as_array() {
        let iter = TokenIterator::new("F -> F F");
        let tokens : Vec<_> = iter.collect();
        let text : Vec<_> = tokens.iter().map(|t| t.text).collect();

        assert_eq!(text, ["F", "->", "F", "F"])
    }
}