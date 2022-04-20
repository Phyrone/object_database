pub trait StringParserT: ToString {
    fn skip_whitespace(self) -> Self;
    fn next_line(self) -> Self;
    fn next_word(self) -> Self;

}

struct StringParser {}



