pub fn pathmatch(pattern: &str, pathstring: &str) -> bool
{
    let mut pattern_chars = pattern.chars();
    let mut path_chars = pathstring.chars();
    loop {
        match pattern_chars.next() {
            None => return path_chars.next().is_none(),
            Some('?') => return path_chars.next().is_some(),
            Some(pc) => {
                match path_chars.next() {
                    None => return false,
                    Some(sc) => if pc != sc { return false },
                }
            }
        }
    }
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

