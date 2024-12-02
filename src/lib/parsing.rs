//pub fn matches(char: char) -> impl Fn(dyn Iterator<Item = char>) -> Option<char> {}
/* pub fn matches<I>(iter: I)
where
    I: IntoIterator<Item = char>,
{
} */

/* type CharIter = dyn IntoIterator<Item = char>;

pub fn matches<I, F>(char: char) -> impl Fn(&dyn IntoIterator<Item = char>) -> Option<char> {
    return |iter| None;
} */

/* pub fn char(char: char) -> fn(&str) -> Option<(&str, char)> {
    let char = char;
    |input: &str| {
        if input.starts_with(char) {
            Some((&input[1..], '#'))
        } else {
            None
        }
    }
} */

/* use crate::reader::Reader;

fn char(char: char) -> fn(&mut Reader) -> Option<char> {
    let char = char;
    |input: &mut Reader| {
        if input.get_char() == char {
            Some(char)
        } else {
            None
        }
    }
}
 */
