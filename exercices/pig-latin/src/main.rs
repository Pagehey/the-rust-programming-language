// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

const VOWELS: &str = "aeiouy";

fn pig_latinize_word(word: &str) -> String {
    // extract first_letter slice
    let first_letter = &word[..1];

    // if string slice is in VOWELS
    if VOWELS.contains(first_letter) {
        format!("{}{}", word, "-hay")
    } else {
        format!("{}-{}{}", &word[1..], first_letter, "ay")
    }
}

fn main() {
    let words = ["first", "apple", "page", "elephant"];

    // should return irst-fay, apple-hay, age-pay, elephant-hay
    let words = words.iter().map(|word| pig_latinize_word(word));

    for word in words { println!("{}", word) };
}
