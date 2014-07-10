pub fn pathmatch(pattern: &str, pathstring: &str) -> bool
{
    return pattern == pathstring
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

