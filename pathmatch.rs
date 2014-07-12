pub fn pathmatch(pattern: &str, pathstring: &str) -> bool
{
    use std::str::Chars;
    let pattern_chars = pattern.chars();
    let path_chars = pathstring.chars();
    return pathmatch_impl(pattern_chars, path_chars);

    fn pathmatch_impl(mut pattern_chars: Chars, mut path_chars: Chars) -> bool
    {
        loop {
            match pattern_chars.next() {
                None => return path_chars.next().is_none(),
                Some('?') => match path_chars.next() {
                    None | Some('/') => return false,
                    _ => (),
                },
                Some('*') => loop {
                    if pathmatch_impl(pattern_chars, path_chars) {
                        return true;
                    }
                    match path_chars.next() {
                        None => break,
                        Some('/') => return false,
                        _ => (),
                    }
                },
                Some(pc) => {
                    match path_chars.next() {
                        None => return false,
                        Some(sc) => if pc != sc { return false },
                    }
                }
            }
        }
    }
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

