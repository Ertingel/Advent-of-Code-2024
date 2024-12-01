//pub fn matches(char: char) -> impl Fn(dyn Iterator<Item = char>) -> Option<char> {}
/* pub fn matches<I>(iter: I)
where
    I: IntoIterator<Item = char>,
{
} */

type CharIter = IntoIterator<Item = char>;

pub fn matches<I, F>(char: char) -> impl Fn(&dyn IntoIterator<Item = char>) -> Option<char> {
    return |iter| None;
}
