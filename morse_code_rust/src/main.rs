use std::collections::HashMap;
use std::*;
use rodio::{Decoder, Source};
use std::io::{self, BufReader};
use std::fs::File;
use std::thread::sleep;

const UNKNOWN_CHARACTER: &str = "........"; //<HH> ERROR - Morse Standards

fn main() {
    let prompt = &"Enter a word or phrase to encode into Morse.";
    let response = input(prompt);
    let encoded_message = encode(&response.trim());
    println!("{:?}", encoded_message);

    //play encoded message
    play_morse(encoded_message);
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

pub fn play_morse(message: String){
    //audio file setup
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let morse_dot = BufReader::new(File::open("morse_dot.mp3").unwrap());
    let morse_dash = BufReader::new(File::open("morse_dash.mp3").unwrap());

    //add Decoder and buffers
    let decoder_dot = Decoder::new(morse_dot).unwrap();
    let buffered_dot = decoder_dot.buffered();
    let decoder_dash = Decoder::new(morse_dash).unwrap();
    let buffered_dash = decoder_dash.buffered();

    for index in message.chars(){
        if index == '.' {
            sink.append(buffered_dot.clone());
        } else if index == '-' {
            sink.append(buffered_dash.clone());
        } else {
            sleep(time::Duration::from_millis(200)); //add a sleep to handle space between letters and words
        }
    }

    sink.sleep_until_end();
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
