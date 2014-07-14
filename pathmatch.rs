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
    let pattern_chars = cursor(pattern);
    let mut path_chars = cursor(pathstring);
    return pathmatch_impl(pattern_chars, &mut path_chars, false);

    #[deriving(Clone)]
    struct Cursor<'a>
    {
        chars: Chars<'a>,
        at_the_beginning: bool,
    }

    fn cursor<'a>(arg: &'a str) -> Cursor<'a>
    {
        Cursor { chars: arg.chars(), at_the_beginning: true }
    }

    impl<'a> Iterator<char> for Cursor<'a>
    {
        fn next<'a>(&mut self) -> Option<char>
        {
            self.at_the_beginning = false;
            return self.chars.next();
        }
    }

    fn pathmatch_impl(mut pattern_chars: Cursor, path_chars: &mut Cursor, alt_branch_mode: bool) -> bool
    {
        if path_chars.at_the_beginning && char_iter_equals(pattern_chars, "**/", alt_branch_mode, true) {
            // How can I convert Skip<Cursor> to Cursor to make use of .skip(3) ?
            let mut pattern_chars_copy = pattern_chars;
            pattern_chars_copy.next();
            pattern_chars_copy.next();
            pattern_chars_copy.next();
            if !path_chars.peekable().is_empty() && pathmatch_try(pattern_chars_copy, path_chars, alt_branch_mode) {
                return true;
            }
        }
        loop {
            if !path_chars.at_the_beginning && char_iter_equals(pattern_chars, "/**", alt_branch_mode, false) && path_chars.peekable().is_empty() {
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
                        return_if_some!(match_any(pattern_chars, path_chars, alt_branch_mode, true));
                    },
                    _ => return_if_some!(match_any(pattern_chars, path_chars, alt_branch_mode, false)),
                },
                Some('/') => {
                    return_if_some!(match_exact('/', path_chars.next(), alt_branch_mode));
                    if char_iter_equals(pattern_chars, "**/", alt_branch_mode, true) {
                        let mut pattern_chars_copy = pattern_chars;
                        pattern_chars_copy.next();
                        pattern_chars_copy.next();
                        pattern_chars_copy.next();
                        if pathmatch_try(pattern_chars_copy, path_chars, alt_branch_mode) {
                            return true;
                        }
                    }
                },
                Some('{') => {
                    let mut matched_something = false;
                    loop {
                        if !matched_something && pathmatch_try(pattern_chars, path_chars, true) {
                            matched_something = true;
                        }
                        match skip_alt_branch(&mut pattern_chars) {
                            Some(false) => break,
                            Some(true) => {},
                            None => /* error in pattern */ return false,
                        }
                    }
                    if !matched_something {
                        return false;
                    }
                },
                Some(',') | Some('}') if alt_branch_mode => return true,
                Some(pc) => return_if_some!(match_exact(pc, path_chars.next(), alt_branch_mode)),
            }
        }
    }

    fn pathmatch_try(pattern_chars: Cursor, path_chars: &mut Cursor, alt_branch_mode: bool) -> bool
    {
        let mut path_chars_copy = path_chars.clone();
        if pathmatch_impl(pattern_chars, &mut path_chars_copy, alt_branch_mode) {
            *path_chars = path_chars_copy;
            return true;
        }
        return false;
    }

    fn skip_alt_branch(pattern_chars: &mut Cursor) -> Option<bool>
    {
        loop {
            match pattern_chars.next() {
                Some(',') => return Some(true),
                Some('}') => return Some(false),
                Some('{') => if skip_alt_branch(pattern_chars).is_none() { return None; },
                None => return None,
                _ => {},
            }
        }
    }

    fn match_any(pattern_chars: Cursor, path_chars: &mut Cursor, alt_branch_mode: bool, allow_pathsep: bool) -> Option<bool>
    {
        loop {
            if pathmatch_try(pattern_chars, path_chars, alt_branch_mode) {
                return Some(true);
            }
            match path_chars.next() {
                None => return None,
                Some('/') if !allow_pathsep => return Some(false),
                _ => {},
            }
        }
    }

    fn match_exact(pattern_char: char, path_char: Option<char>, alt_branch_mode: bool) -> Option<bool>
    {
        match path_char {
            None => return Some(false),
            Some(',') | Some('}') if alt_branch_mode => return Some(false),
            Some(x) if x != pattern_char => return Some(false),
            _ => return None,
        }
    }

    fn char_iter_equals(mut char_iter: Cursor, needle: &str, alt_branch_mode: bool, startswith: bool) -> bool
    {
        for needle_char in needle.chars() {
            match char_iter.next() {
                Some(x) if x == needle_char => (),
                _ => return false,
            }
        }
        if startswith { true }
        else if alt_branch_mode {
            match char_iter.next() {
                Some(',') | Some('}') => true,
                _ => false,
            }
        } else { char_iter.next().is_none() }
    }
}

#[cfg(test)]
fn assert_pathmatch_many(pattern: &str, paths_yes: &[&str], paths_no: &[&str])
{
    for path in paths_yes.iter() {
        assert!(pathmatch(pattern, *path), "should match: pattern:'{}' path:'{}'", pattern, path);
    }
    for path in paths_no.iter() {
        assert!(!pathmatch(pattern, *path), "must not match: pattern:'{}' path:'{}'", pattern, path);
    }
}

#[test]
fn pathmatch_test_alt_combos()
{
    assert_pathmatch_many("a{?,/}c", ["abc", "a/c"], ["ac", "abbc"]);
    assert_pathmatch_many("{foo/**,**/bar}", ["foo", "bar"], ["foobar"]);
    assert_pathmatch_many("{foo/**,bar}baz", ["barbaz", "foo/baz"], ["foobaz"]);
    assert_pathmatch_many("foo{bar,**/baz}", ["foobar", "foo/baz"], ["foobaz"]);
    assert!(!pathmatch("{**/}", ""));
    assert!(!pathmatch("{{**/}}", ""));
}

#[test]
fn pathmatch_test_alt()
{
    assert!(pathmatch("{foo}", "foo"));
    assert_pathmatch_many("{foo,bar,baz}", ["foo", "bar", "baz"], ["", "foobarbaz", "qux"]);
    assert_pathmatch_many("{foo,bar}", ["foo", "bar"], ["", "foobar"]);
    assert!(pathmatch("{}", ""));
    assert!(!pathmatch("{}", "{}"));
    assert!(pathmatch("{foo}{bar}", "foobar"));
    assert!(pathmatch("{foo}*{bar}", "foobar"));
    assert!(pathmatch("{foo}*{bar}", "fooXXbar"));
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
    assert!(!pathmatch("**/", ""));
}

#[test]
fn pathmatch_test_anypath_trailing()
{
    // `/**` at the end of pattern may match nothing.
    assert!(pathmatch("foo/**", "foo"));
    assert!(pathmatch("foo/**", "foo/"));
    assert!(!pathmatch("foo/**g", "foog"));
    assert!(!pathmatch("foo/*", "foo"));
    assert!(!pathmatch("/**", ""));
    assert!(pathmatch("/**", "/"));
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

