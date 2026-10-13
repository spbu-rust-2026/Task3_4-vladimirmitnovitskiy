use std::collections::HashMap;

use task3_4_template::word_frequencies;

fn frequencies(entries: &[(&str, usize)]) -> HashMap<String, usize> {
    entries
        .iter()
        .map(|(word, count)| ((*word).to_owned(), *count))
        .collect()
}

#[test]
fn empty_input_has_no_words() {
    assert_eq!(word_frequencies(""), HashMap::new());
}

#[test]
fn spaces_only_input_has_no_words() {
    assert_eq!(word_frequencies("   "), HashMap::new());
}

#[test]
fn one_word_has_frequency_one() {
    assert_eq!(word_frequencies("rust"), frequencies(&[("rust", 1)]));
}

#[test]
fn distinct_words_each_have_frequency_one() {
    assert_eq!(
        word_frequencies("rust memory safety"),
        frequencies(&[("rust", 1), ("memory", 1), ("safety", 1)])
    );
}

#[test]
fn repeated_word_frequency_is_counted() {
    assert_eq!(word_frequencies("rust rust"), frequencies(&[("rust", 2)]));
}

#[test]
fn several_word_frequencies_are_counted() {
    assert_eq!(
        word_frequencies("a b a c b a"),
        frequencies(&[("a", 3), ("b", 2), ("c", 1)])
    );
}

#[test]
fn words_with_different_spelling_are_distinct() {
    assert_eq!(
        word_frequencies("rust runner rust"),
        frequencies(&[("rust", 2), ("runner", 1)])
    );
}

#[test]
fn punctuation_is_part_of_the_word() {
    assert_eq!(
        word_frequencies("rust rust, rust"),
        frequencies(&[("rust", 2), ("rust,", 1)])
    );
}

#[test]
fn unicode_words_are_counted() {
    assert_eq!(
        word_frequencies("кот кот мир"),
        frequencies(&[("кот", 2), ("мир", 1)])
    );
}

#[test]
fn leading_and_trailing_spaces_do_not_add_words() {
    assert_eq!(
        word_frequencies("  rust memory  "),
        frequencies(&[("rust", 1), ("memory", 1)])
    );
}

#[test]
fn multiple_spaces_between_words_do_not_add_empty_words() {
    assert_eq!(
        word_frequencies("rust   rust  memory"),
        frequencies(&[("rust", 2), ("memory", 1)])
    );
}

#[test]
fn a_long_repetition_has_the_exact_count() {
    assert_eq!(
        word_frequencies("echo echo echo echo echo"),
        frequencies(&[("echo", 5)])
    );
}
