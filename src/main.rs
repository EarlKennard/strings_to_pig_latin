use std::io;

fn main() {
    println!("
    Hello, world! This program will convert any english string to pig latin. Strings in other languages may work to some degree but this program is undoubtedly not made for
    that.

    What is pig latin? To quote the Rust book:
        The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
        Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
        
    This program can handle multiple words, but for now, symbols such as apostrophes, commas and periods are not taken into account and may make the result
    look funny. Additionally, please don't make your input string too long, as pasting a long one, such as a movie script, may just crash your computer.\n");

    println!("Type the string that you want to convert to pig latin:");
    let users_string = user_input();
    let converted = pig_latin(&users_string);

    println!("Here is your original string: {}
Here is your string converted to pig latin: {}", &users_string, &converted);
}

// user types string
fn user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input = input.trim().to_lowercase();

    input
}

// convert string to pig latin
fn pig_latin(user_string: &String) -> String {
    let mut empty_str = String::from("");
    for word in user_string.split_whitespace() {
        empty_str = empty_str + &one_word(word) + " ";
    }

    empty_str.trim().to_string(); // bc there's a space at the end
    empty_str
}

// converts single words to pig latin
fn one_word(single_word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    
    if vowels.iter().any(|&vowel| single_word.starts_with(vowel)) {
        let mut new_word = String::from("");
        new_word = new_word + single_word + "-hay";
        new_word
    } else if consonants.iter().any(|&consonant| single_word.starts_with(consonant)) {
        let mut new_word = String::from("");
        new_word = new_word + &single_word[1..] + "-" + &single_word[0..1] + "ay";
        new_word
    } else { // needed bc if it doesn't start with a letter, then it's probably not a word
        single_word.to_string()
    }
}


/* Notes
1. has to handle spaces too
2. can maybe handle periods and commas at the end of words? i really don't know 
*/
