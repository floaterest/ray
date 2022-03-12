use std::io::{BufReader, Result, Read, BufRead};
use std::iter::Peekable;
use std::mem::transmute;
use std::str::{FromStr, SplitWhitespace};

pub struct Scanner<R: Read> {
    pub reader: BufReader<R>,
    tokens: Peekable<SplitWhitespace<'static>>,
    line: Box<str>,
}

impl<R: Read> Scanner<R> {
    pub fn new(r: R) -> Self {
        Self {
            tokens: "".split_whitespace().peekable(),
            line: "".to_string().into_boxed_str(),
            reader: BufReader::new(r),
        }
    }

    pub fn usize(&mut self) -> usize {
        self.next::<usize>()
    }

    pub fn u8(&mut self) -> u8 {
        self.next::<u8>()
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        self.prepare().unwrap();
        self.tokens.next().unwrap().parse().ok().unwrap()
    }

    fn prepare(&mut self) -> Result<()> {
        //! read line if needed
        while self.tokens.peek().is_none() {
            let mut line = String::new();
            let n = self.reader.read_line(&mut line)?;
            if n == 0 { return Ok(()); /* EOF */ }

            self.line = line.into_boxed_str();
            self.tokens = unsafe { transmute::<_, &'static str>(&*self.line) }.split_whitespace().peekable();
        }
        Ok(())
    }
}