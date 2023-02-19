use std::io::{Result, Read, BufReader};
use std::io::prelude::*;

pub struct LineProcessor<R: Read> {
    /// Reader to get lines from
    reader: BufReader<R>,
    /// Line number starting at 0
    lno: usize,
    /// Are we currently in a comment (specifically multi-line)
    in_comment: bool,
    /// Buffer for use in the comment code
    buf: String,
}

impl<R: Read> LineProcessor<R> {
    pub fn new(reader: R) -> LineProcessor<R> {
        let reader = BufReader::new(reader);
        LineProcessor {
            reader,
            lno: 0,
            in_comment: false,
            buf: String::new(),
        }
    }

    fn read_line_strip_comments(
        &mut self,
        line: &mut String,
    ) -> Result<usize> {
        let is_comment_start = |b: &[u8]| b.len() >= 2 && b[0] == b'/' && b[1] == b'*';
        let is_comment_end = |b: &[u8]| b.len() >= 2 && b[0] == b'*' && b[1] == b'/';
        let is_single_line_comment = |b: &[u8]| b.len() >= 2 && b[0] == b'/' && b[1] == b'/';
        self.buf.clear();
        let count = self.reader.read_line(&mut self.buf)?;
        let bytes = self.buf.as_bytes();
        let mut i = 0;
        let mut chars = vec![];
        while i < bytes.len() {
            if self.in_comment {
                if is_comment_end(&bytes[i..]) {
                    self.in_comment = false;
                    i += 2;
                } else {
                    i += 1;
                }
            } else {
                if is_single_line_comment(&bytes[i..]) {
                    // Skip the rest of the line
                    break;
                } else if is_comment_start(&bytes[i..]) {
                    self.in_comment = true;
                    i += 2;
                } else {
                    chars.push(bytes[i]);
                    i += 1;
                }
            }
        }
        line.push_str(&unsafe { String::from_utf8_unchecked(chars) });
        Ok(count)
    }

    fn read_line(&mut self, line: &mut String) -> Result<usize> {
        let mut total_read = 0;
        loop {
            let n = self.read_line_strip_comments(line)?;
            if n == 0 {
                break;
            }
            total_read += n;
            // Check for the continuation
            let trimmed = line.trim_end();
            if !trimmed.ends_with("\\") {
                break;
            }
            // Strip the line continuation
            line.truncate(trimmed.len() - 1);
        }
        Ok(total_read)
    }
}

impl<R: Read> Iterator for LineProcessor<R> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some(Ok(line)),
            Err(err) => Some(Err(err)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let mut lp = LineProcessor::new("".as_bytes());
        assert!(lp.next().is_none());
    }

    #[test]
    fn single_line_comments() {
        let mut lp = LineProcessor::new("abc // test".as_bytes());
        assert_eq!(lp.next().unwrap().unwrap(), "abc ".to_string());
        assert!(lp.next().is_none());
    }

    #[test]
    fn multi_line_comments() {
        let mut lp = LineProcessor::new("abc /* xyz \n \n \n    */ abc".as_bytes());
        assert_eq!(lp.next().unwrap().unwrap(), "abc ".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), "".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), "".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), " abc".to_string());
        assert!(lp.next().is_none());
    }

    #[test]
    fn multiple_comments() {
        let mut lp = LineProcessor::new("123 xyz /* xyz \n */1023\nciao // xyz\n /* done */\n".as_bytes());
        assert_eq!(lp.next().unwrap().unwrap(), "123 xyz ".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), "1023\n".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), "ciao ".to_string());
        assert_eq!(lp.next().unwrap().unwrap(), " \n".to_string());
        assert!(lp.next().is_none());
    }
}
