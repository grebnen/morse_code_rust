use std::collections::HashMap;
use std::*;

const UNKNOWN_CHARACTER: &str = "........"; //<HH> ERROR - Morse Standards

fn main() {
    let prompt = &"Enter a word or phrase to encode into Morse.";
    let response = input(prompt);
    let encoded_message = encode(&response.trim());
    println!("{:?}", encoded_message);
}

fn input (message: &'_ impl fmt::Display) -> String {
    println!("{}", message);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read from stdin");
    user_input
}

pub fn encode(message: &str) -> String {
    let dictionary = _string_to_morse();

    message
        .chars()
        .map(|char| char.to_uppercase().to_string())
        .map(|letter| dictionary.get(letter.as_str()))
        .map(|option| option.unwrap_or_else(|| &UNKNOWN_CHARACTER).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($key, $value),)*]))
    };
}

fn _string_to_morse() -> HashMap<&'static str, &'static str> {
    map! {
        "A" => ".-",
        "B" => "-...",
        "C" => "-.-.",
        "D" => "-..",
        "E" => ".",
        "F" => "..-.",
        "G" => "--.",
        "H" => "....",
        "I" => "..",
        "J" => ".---",
        "K" => "-.-",
        "L" => ".-..",
        "M" => "--",
        "N" => "-.",
        "O" => "---",
        "P" => ".--.",
        "Q" => "--.-",
        "R" => ".-.",
        "S" => "...",
        "T" => "-",
        "U" => "..-",
        "V" => "...-",
        "W" => ".--",
        "X" => "-..-",
        "Y" => "-.--",
        "Z" => "--..",
        "1" => ".----",
        "2" => "..---",
        "3" => "...--",
        "4" => "....-",
        "5" => ".....",
        "6" => "-....",
        "7" => "--...",
        "8" => "---..",
        "9" => "----.",
        "0" => "-----",
        "&" => ".-...",
        "@" => ".--.-.",
        ":" => "---...",
        "," => "--..--",
        "." => ".-.-.-",
        "'" => ".----.",
        "\"" => ".-..-.",
        "?" => "..--..",
        "/" => "-..-.",
        "=" => "-...-",
        "+" => ".-.-.",
        "-" => "-....-",
        "(" => "-.--.",
        ")" => "-.--.-",
        " " => " ",
        "!" => "-.-.--",
    }
}
