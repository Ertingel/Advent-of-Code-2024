// cargo test --lib parsing
// cargo test --lib parsing -- --nocapture

use std::{ops::RangeBounds, str::FromStr};

pub fn any<'a, T: 'a>(
    alternative1: impl Fn(&'a str) -> Option<(&'a str, T)>,
    alternative2: impl Fn(&'a str) -> Option<(&'a str, T)>,
) -> impl Fn(&'a str) -> Option<(&'a str, T)> {
    move |input: &str| {
        let res = alternative1(input);
        if res.is_some() {
            return res;
        }
        alternative2(input)
    }
}

#[macro_export]
macro_rules! any {
    ($($args:expr),*) => {{
        move |input: &str| {
            $(
                let res = $args(input);
                if res.is_some() {
                    return res;
                }
            )*
            None
        }
    }}
}

#[macro_export]
macro_rules! all {
    ($first:expr, $($args:expr),*) => {{
        move |input: &str| {

            let mut longest = $first(input)?;
            $(
                let res = $args(input)?;

                if res.0.len() < longest.0.len() {
                    longest = res;
                }
            )*

            let remaining = longest.0;
            let byte_count = input.len() - remaining.len();
            let taken = &input[..byte_count];

            Some((remaining, taken))
        }
    }}
}
/*
#[macro_export]
macro_rules! pattern {
    ($first:expr, $($args:expr),*) => {{

        move |input: &str| {
            let mut remaining = input;

            let out = (
                {
                    let res = $first(remaining)?;
                    remaining = res.0;
                    res.1
                }
                $(
                    , {
                        let res = $args(remaining)?;
                        remaining = res.0;
                        res.1
                    }
                )*
            );

            let byte_count = input.len() - remaining.len();
            let taken = &input[..byte_count];

            Some((taken, out))
        }
    }}
}
*/

pub fn take<'a>(count: usize) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let mut itter = input.chars();
        let mut byte_count: usize = 0;

        for _ in 0..count {
            let c = itter.next()?;
            byte_count += c.len_utf8();
        }

        let taken = &input[..byte_count];
        let remaining = &input[byte_count..];
        Some((remaining, taken))
    }
}

pub fn char<'a>(char: char) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let c = input.chars().next()?;
        if c == char {
            let len = c.len_utf8();
            return Some((&input[len..], &input[..len]));
        }
        None
    }
}

pub fn not_char<'a>(char: char) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let c = input.chars().next()?;
        if c != char {
            let len = c.len_utf8();
            return Some((&input[len..], &input[..len]));
        }
        None
    }
}

pub fn newline<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    char('\n')
}

pub fn range<'a>(
    range: impl RangeBounds<char> + 'static,
) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let c = input.chars().next()?;
        if range.contains(&c) {
            let len = c.len_utf8();
            return Some((&input[len..], &input[..len]));
        }
        None
    }
}

pub fn not_range<'a>(
    range: impl RangeBounds<char> + 'static,
) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let c = input.chars().next()?;
        if !range.contains(&c) {
            let len = c.len_utf8();
            return Some((&input[len..], &input[..len]));
        }
        None
    }
}

pub fn numeric<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    range('0'..='9')
}

pub fn not_numeric<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    not_range('0'..='9')
}

pub fn lowercase<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    range('a'..='z')
}

pub fn not_lowercase<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    not_range('a'..='z')
}

pub fn uppercase<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    range('A'..='Z')
}

pub fn not_uppercase<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    not_range('A'..='Z')
}

pub fn alpha<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    //any::<& str>(&[lowercase(), uppercase()])
    any!(lowercase(), uppercase())
}

pub fn not_alpha<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    all!(not_lowercase(), not_uppercase())
}

pub fn alpha_numeric<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    any!(lowercase(), numeric(), uppercase())
}

pub fn not_alpha_numeric<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    all!(not_lowercase(), not_numeric(), not_uppercase())
}

pub fn string<'a>(pattern: &'a str) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let len = pattern.len();

        if len > input.len() {
            return None;
        }

        let taken = &input[..len];
        let remaining = &input[len..];

        if pattern == taken {
            return Some((remaining, taken));
        }
        None
    }
}

pub fn not_string<'a>(pattern: &'a str) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let len = pattern.len();

        if len > input.len() {
            return None;
        }

        let taken = &input[..len];
        let remaining = &input[len..];

        if pattern != taken {
            return Some((remaining, taken));
        }
        None
    }
}

pub fn whitespace<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    any!(char(' '), string(" "), newline())
}

pub fn optional<'a, T>(
    pattern: impl Fn(&'a str) -> Option<(&'a str, T)>,
) -> impl Fn(&'a str) -> Option<(&'a str, Option<T>)>
where
    T: 'a,
{
    move |input: &str| {
        let res = pattern(input);

        if let Some((remaining, val)) = res {
            return Some((remaining, Some(val)));
        }

        Some((input, None))
    }
}

pub fn repeating<'a, T>(
    min: usize,
    max: usize,
    pattern: impl Fn(&'a str) -> Option<(&'a str, T)>,
) -> impl Fn(&'a str) -> Option<(&'a str, Vec<T>)>
where
    T: 'a,
{
    move |input: &str| {
        let mut remaining = input;
        let mut acc: Vec<T> = Vec::new();

        for _ in 0..min {
            let res = pattern(remaining)?;
            remaining = res.0;
            acc.push(res.1);
        }

        for _ in min..max {
            let res = pattern(remaining);

            if let Some(res) = res {
                remaining = res.0;
                acc.push(res.1);
            } else {
                return Some((remaining, acc));
            }
        }

        Some((remaining, acc))
    }
}

pub fn continuous<'a>(
    min: usize,
    max: usize,
    pattern: impl Fn(&'a str) -> Option<(&'a str, &'a str)>,
) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let mut remaining = input;

        for _ in 0..min {
            let res = pattern(remaining)?;
            remaining = res.0;
        }

        for _ in min..max {
            let res = pattern(remaining);

            if let Some(res) = res {
                remaining = res.0;
            } else {
                let byte_count = input.len() - remaining.len();
                let taken = &input[..byte_count];
                return Some((remaining, taken));
            }
        }

        let byte_count = input.len() - remaining.len();
        let taken = &input[..byte_count];
        Some((remaining, taken))
    }
}

pub fn number<'a, T>() -> impl Fn(&'a str) -> Option<(&'a str, T)>
where
    T: 'a + FromStr,
{
    move |input: &str| {
        let (remaining, minus) = optional(char('-'))(input)?;
        let (remaining, num) = continuous(1, usize::MAX, numeric())(remaining)?;

        let minus = minus.unwrap_or("");
        let num = (minus.to_owned() + num).parse::<T>().ok()?;

        Some((remaining, num))
    }
}

pub fn take_untill<'a, T>(
    pattern: impl Fn(&'a str) -> Option<(&'a str, T)>,
) -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    move |input: &str| {
        let mut remaining = input;

        while !remaining.is_empty() {
            let res = pattern(remaining);
            if res.is_some() {
                let byte_count = input.len() - remaining.len();
                let taken = &input[..byte_count];
                return Some((remaining, taken));
            }

            let res = take(1)(remaining);
            if let Some(res) = res {
                remaining = res.0;
            } else {
                return Some(("", input));
            }
        }

        Some(("", input))
    }
}

pub fn find_all<'a, T>(
    pattern: impl Fn(&'a str) -> Option<(&'a str, T)>,
) -> impl Fn(&'a str) -> Vec<T>
where
    T: 'a,
{
    move |input: &str| {
        let mut remaining = input;
        let mut out: Vec<T> = Vec::new();

        while !remaining.is_empty() {
            let res = pattern(remaining);
            if let Some(res) = res {
                remaining = res.0;
                out.push(res.1);
                continue;
            }

            let res = take(1)(remaining);
            if let Some(res) = res {
                remaining = res.0;
            } else {
                return out;
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_tests() {
        assert_eq!(alpha()("abc"), Some(("bc", "a")));
        assert_eq!(alpha()("ABC"), Some(("BC", "A")));
        assert_eq!(alpha()("123"), None);

        assert_eq!(alpha_numeric()("abc"), Some(("bc", "a")));
        assert_eq!(alpha_numeric()("ABC"), Some(("BC", "A")));
        assert_eq!(alpha_numeric()("123"), Some(("23", "1")));
    }

    #[test]
    fn all_tests() {
        assert_eq!(not_alpha()("abc"), None);
        assert_eq!(not_alpha()("ABC"), None);
        assert_eq!(not_alpha()("123"), Some(("23", "1")));

        assert_eq!(not_alpha_numeric()("abc"), None);
        assert_eq!(not_alpha_numeric()("ABC"), None);
        assert_eq!(not_alpha_numeric()("123"), None);
        assert_eq!(not_alpha_numeric()("{[|"), Some(("[|", "{")));
    }

    /*
    #[test]
    fn pattern_tests() {
        assert_eq!(
            pattern!(lowercase(), numeric())("a1b2c3"),
            Some(("b2c3", ("a", "b")))
        );
    }
     */

    #[test]
    fn take_tests() {
        assert_eq!(take(1)("abc"), Some(("bc", "a")));
        assert_eq!(take(1)("bc"), Some(("c", "b")));
        assert_eq!(take(1)("c"), Some(("", "c")));
        assert_eq!(take(1)(""), None);
    }

    #[test]
    fn char_tests() {
        assert_eq!(char('a')("abc"), Some(("bc", "a")));
        assert_eq!(char('b')("abc"), None);

        assert_eq!(not_char('a')("abc"), None);
        assert_eq!(not_char('b')("abc"), Some(("bc", "a")));
    }

    #[test]
    fn range_tests() {
        assert_eq!(range('a'..='z')("abc"), Some(("bc", "a")));
        assert_eq!(range('A'..='Z')("abc"), None);

        assert_eq!(not_range('a'..='z')("abc"), None);
        assert_eq!(not_range('A'..='Z')("abc"), Some(("bc", "a")));
    }

    #[test]
    fn str_tests() {
        assert_eq!(string("abc")("abcdefg"), Some(("defg", "abc")));
        assert_eq!(string("abc")("bcdefg"), None);
        assert_eq!(string("abc")("ab"), None);
    }

    #[test]
    fn optional_tests() {
        assert_eq!(
            optional(string("abc"))("abcdefg"),
            Some(("defg", Some("abc")))
        );
        assert_eq!(optional(string("abc"))("bcdefg"), Some(("bcdefg", None)));
    }

    #[test]
    fn repeating_tests() {
        assert_eq!(
            repeating(0, 10, numeric())("123abc456"),
            Some(("abc456", vec!["1", "2", "3"]))
        );
        assert_eq!(
            repeating(0, 2, numeric())("123abc456"),
            Some(("3abc456", vec!["1", "2"]))
        );
        assert_eq!(
            repeating(0, 10, alpha())("123abc456"),
            Some(("123abc456", vec![]))
        );
        assert_eq!(repeating(1, 10, alpha())("123abc456"), None);
        assert_eq!(
            repeating(0, 10, numeric())("123"),
            Some(("", vec!["1", "2", "3"]))
        );
    }

    #[test]
    fn continuous_tests() {
        assert_eq!(
            continuous(0, 10, numeric())("123abc456"),
            Some(("abc456", "123"))
        );
        assert_eq!(
            continuous(0, 2, numeric())("123abc456"),
            Some(("3abc456", "12"))
        );
        assert_eq!(
            continuous(0, 10, alpha())("123abc456"),
            Some(("123abc456", ""))
        );
        assert_eq!(continuous(1, 10, alpha())("123abc456"), None);
        assert_eq!(continuous(0, 10, numeric())("123"), Some(("", "123")));
    }

    #[test]
    fn number_tests() {
        assert_eq!(number::<i32>()("abc"), None);
        assert_eq!(number::<i32>()("ABC"), None);
        assert_eq!(number::<i32>()("123"), Some(("", 123)));
        assert_eq!(number::<i32>()("123abc"), Some(("abc", 123)));
        assert_eq!(number::<i32>()("-123"), Some(("", -123)));
    }

    #[test]
    fn take_untill_tests() {
        assert_eq!(
            take_untill(numeric())("abc1de23fg"),
            Some(("1de23fg", "abc"))
        );
        assert_eq!(take_untill(numeric())("1de23fg"), Some(("1de23fg", "")));
        assert_eq!(take_untill(numeric())("de23fg"), Some(("23fg", "de")));
        assert_eq!(take_untill(numeric())("fg"), Some(("", "fg")));
        assert_eq!(take_untill(numeric())("1"), Some(("1", "")));
    }

    #[test]
    fn find_all_tests() {
        assert_eq!(find_all(numeric())("abc1de23fg4"), vec!["1", "2", "3", "4"]);
        assert_eq!(find_all(number::<i32>())("abc1de23fg4"), vec![1, 23, 4]);
    }
}
