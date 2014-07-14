#![feature(macro_rules)]

macro_rules! return_if_some(
    ($arg:expr) => (
        match $arg {
            Some(val) => return val,
            None => {},
        }
    );
)

pub fn pathmatch(pattern: &str, pathstring: &str) -> bool
{
    use std::str::Chars;
    let pattern_chars = pattern.chars();
    let path_chars = pathstring.chars();
    return pathmatch_impl(pattern_chars, path_chars);

    fn pathmatch_impl(mut pattern_chars: Chars, mut path_chars: Chars) -> bool
    {
        if char_iter_equals(pattern_chars, "**/", true) {
            // How can I convert Skip<Chars> to Chars to make use of .skip(3) ?
            let mut pattern_chars_copy = pattern_chars;
            pattern_chars_copy.next();
            pattern_chars_copy.next();
            pattern_chars_copy.next();
            if pathmatch_impl(pattern_chars_copy, path_chars) {
                return true;
            }
        }
        loop {
            if char_iter_equals(pattern_chars, "/**", false) && path_chars.peekable().is_empty() {
                return true;
            }
            match pattern_chars.next() {
                None => return path_chars.next().is_none(),
                Some('?') => match path_chars.next() {
                    None | Some('/') => return false,
                    _ => (),
                },
                Some('*') => match pattern_chars.peekable().peek() {
                    Some(&'*') => {
                        pattern_chars.next();
                        return_if_some!(match_any(pattern_chars, &mut path_chars, true));
                    },
                    _ => return_if_some!(match_any(pattern_chars, &mut path_chars, false)),
                },
                Some('/') => {
                    return_if_some!(match_exact('/', path_chars.next()));
                    if char_iter_equals(pattern_chars, "**/", true) && pathmatch_impl(pattern_chars, path_chars) {
                        return true;
                    }
                },
                Some(pc) => return_if_some!(match_exact(pc, path_chars.next())),
            }
        }
    }

    fn match_any(pattern_chars: Chars, path_chars: &mut Chars, allow_pathsep: bool) -> Option<bool>
    {
        loop {
            if pathmatch_impl(pattern_chars, path_chars.clone()) {
                return Some(true);
            }
            match path_chars.next() {
                None => return None,
                Some('/') if !allow_pathsep => return Some(false),
                _ => {},
            }
        }
    }

    fn match_exact(pattern_char: char, path_char: Option<char>) -> Option<bool>
    {
        match path_char {
            None => return Some(false),
            Some(x) if x != pattern_char => return Some(false),
            _ => return None,
        }
    }

    fn char_iter_equals(mut char_iter: Chars, needle: &str, startswith: bool) -> bool
    {
        for needle_char in needle.chars() {
            match char_iter.next() {
                Some(x) if x == needle_char => (),
                _ => return false,
            }
        }
        return if startswith { true } else { char_iter.next().is_none() };
    }
}

#[test]
fn pathmatch_test_collapse()
{
    // `/**/` may match a single slash.
    assert!(pathmatch("foo/**/bar", "foo/bar"));
    assert!(pathmatch("foo/**/", "foo/"));
    assert!(!pathmatch("foo/**/", "foo"));
    assert!(pathmatch("/**/bar", "/bar"));
    assert!(!pathmatch("/**/bar", "bar"));
    // to prevent this behavior, you would use this
    assert!(!pathmatch("foo/*/**/bar", "foo/bar"));
    assert!(!pathmatch("foo/**/*/bar", "foo/bar"));
    assert!(pathmatch("foo/**/bar", "foo//bar"));
    assert!(pathmatch("foo/*/**/bar", "foo//bar"));
}

#[test]
fn pathmatch_test_anypath_leading()
{
    // `**/` at the beginning of pattern may match nothing.
    assert!(pathmatch("**/bar", "bar"));
    assert!(pathmatch("**/bar", "/bar"));
    assert!(!pathmatch("a**/bar", "abar"));
    assert!(!pathmatch("*/bar", "bar"));
}

#[test]
fn pathmatch_test_anypath_trailing()
{
    // `/**` at the end of pattern may match nothing.
    assert!(pathmatch("foo/**", "foo"));
    assert!(pathmatch("foo/**", "foo/"));
    assert!(!pathmatch("foo/**g", "foog"));
    assert!(!pathmatch("foo/*", "foo"));
}

#[test]
fn pathmatch_test_anypath()
{
    assert!(pathmatch("**", ""));
    assert!(pathmatch("a**", "a/b/c"));
    assert!(pathmatch("**c", "a/b/c"));
    assert!(pathmatch("a**c", "a/b/c"));
    assert!(pathmatch("a/**/c", "a/b/c"));
    assert!(pathmatch("**/c", "a/b/c"));
    assert!(!pathmatch("**/g", "e/fg"));
    assert!(pathmatch("a/**", "a/b/c"));
    assert!(!pathmatch("e/**", "ef/g"));
}

#[test]
fn pathmatch_test_pathsep()
{
    assert!(!pathmatch("a?b", "a/b"));
    assert!(!pathmatch("a*b", "a/b"));
    assert!(pathmatch("a*/b", "a/b"));
    assert!(pathmatch("a/*b", "a/b"));
}

#[test]
fn pathmatch_test_anyname()
{
    assert!(pathmatch("*", ""));
    assert!(pathmatch("*", "?"));
    assert!(pathmatch("*", "d?F"));
    assert!(pathmatch("a*", "a"));
    assert!(pathmatch("a*", "abcdef"));
    assert!(pathmatch("*f", "abcdef"));
    assert!(pathmatch("a*f", "abcdef"));
    assert!(pathmatch("*cd*", "abcdef"));
    assert!(pathmatch("a*cd*f", "abcdef"));
    assert!(pathmatch("acdf", "acdf"));
    assert!(!pathmatch("*a", "abc"));
    assert!(!pathmatch("c*", "abc"));
}

#[test]
fn pathmatch_test_anychar()
{
    assert!(pathmatch("?", "?"));
    assert!(pathmatch("?", "a"));
    assert!(pathmatch("??", "BC"));
    assert!(pathmatch("B?", "BC"));
    assert!(pathmatch("?C", "BC"));
    assert!(!pathmatch("?", ""));
    assert!(!pathmatch("D?", "BC"));
    assert!(!pathmatch("?E", "BC"));
}

#[test]
fn pathmatch_test_exact_equality()
{
    assert!(pathmatch("", ""));
    assert!(pathmatch("a", "a"));
    assert!(pathmatch("BC", "BC"));
    assert!(!pathmatch("a", ""));
    assert!(!pathmatch("", "BC"));
}

