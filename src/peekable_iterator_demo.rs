#[cfg(test)]
mod test {
    use std::{iter::Peekable, str::Chars};

    #[test]
    fn peekable_char_iterator() {
        fn next_if<F: Fn(&char) -> bool>(iter: &mut Peekable<Chars>, predicate: F) -> Option<char> {
            iter.next().filter(|c| predicate(c))
        }

        let is_vowel = |c: &char| "aeiou".contains(c.clone());

        let mut iter = "hello".chars().peekable();
        assert!(next_if(&mut iter, is_vowel).is_none());
        assert!(next_if(&mut iter, is_vowel).is_some());
        assert!(next_if(&mut iter, is_vowel).is_none());
        assert!(next_if(&mut iter, is_vowel).is_none());
        assert!(next_if(&mut iter, is_vowel).is_some());
    }
}
