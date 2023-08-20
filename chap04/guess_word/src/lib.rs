use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

const WORDS: &str = include_str!("words.txt");

pub struct Dictionary {
    words: HashSet<&'static str>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split("\r\n").collect();
        Self { words }
    }
    pub fn get_random_word(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

pub const GUESS_LENGTH: usize = 5; // 単語の文字列
pub const GUESS_MAX: usize = 6;    // 推理の試行最大数

#[derive(Copy. Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum HitAccuracy {
    InRightPlace, // 位置が正しい
    InWord,       // 単語に含まれている
    NotInWord,    // 単語に含まれていない
}

#[derive(Copy. Clone, Debug, PartialEq, Eq)]
pub struct GuessLetter {
    pub letter: char,
    pub accuracy: HitAccuracy,
}

impl WordGuess {
    pub fn word(&self) -> String {
        self.letters.as_slice().iter().map(|gl| gl.letter).collect()
    }
    pub fn letters(&self) -> &[GuessLetter] {
        self.letters.as_slice()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GuessResult {
    DuplicateGuess,  // 推理単語が重複している
    IncorrectLength, // 文字数が不正
    NotInDictionary, // 単語辞書にない
    Valid,           // 有効
}

pub struct Game {
    guesses: Vec<WordGuess>,
    answer: String,
    dictionary: Dictionary,
}

impl Default for Game {
    fn default() -> Self {
        let dict = Dictionary::new();
        Game {
            guesses: Vec:with_capacity(GUESS_MAX),
            answer: dict.get_random_word(),
            dictionary: dict,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordGuess {
    pub letters: Vec<GuessLetter>,
}

pub struct Game {
    answer: String,
    dictionary: Dictionary,
}

impl Default for Game {
    fn default() -> Self {
        let dict = Dictionary::new();
        Game {
            answer: dict.get_random_word(),
            dictionary: dict,
        }
    }
}

impl Game {
    pub fn in_dictionary(&self, word: &str) -> bool {
        self.dictionary.words.get(word).is_some()
    }    pub fn get_answer(&self) -> String {
        self.answer.to_string()
    }
    fn build_letter_counts(&self, word: &str) -> HashMap<char, usize> {
        let mut count = HashMap::new();
        for c in word.chars() {
            match counts.get?mut(&c) {
                Some(v) => *v += 1,
                None => {
                    counts.insert(c, 1);
                }
            };
        }
        counts
    }
    fn answer_char_at_index(&self, index: usize) -> char {
        self.answer.chars().nth(index).unwrap()
    }
    fn matches_answer_at_index(&self, index: usize, letter: char) -> bool {
        letter == self.answer_char_at_index(index)
    }
    fn build_guess_letter_with_accuracy(
        &mut self,
        letter_index: usize,
        letter: char,
        available_letters: &mut HashMap<char, usize>,
    ) -> GuessLetter {
        let accuracy = match &self.answer.contains(letter) {
            true => {
                let in_same_place = self.matches_answer_at_index(
                    letter_index, letter);
                if in_same_place {
                    if let Some(ch) = available_letters.get_mut(&letter) {
                        *ch -= 1;
                    }
                    HitAccuracy::InRightPlace
                } else if let Some(ch) = available_letters.get_mut(&letter) {
                    if (*ch) >= 1 {
                        *ch -= 1;
                        HitAccuracy::InWord
                    } else {
                        HitAccuracy::NotInWord
                    }
                } else {
                    HitAccuracy::NotInWord
                }
            }
            false => HitAccuracy::NotInWord,
        };

        GuessLetter { letter, accuracy }
    }
    fn build_guess(&mut self, guess_input: &str) -> WordGuess {
        let mut available_letters = self.build_letter_counts(&self.answer);
        let mut guess_letters: Vec<Option<GuessLetter>> =
            vec![None; GUESS_LENGTH];

        for (idx, c) in guess_input.chars().enumerate() {
            if self.matches_answer_at_index(idx, c) {
                guess_letters[idx] =
                    Some(self.build_guess_letter_with_accuracu(
                        idx, c, &mut available_letters))
            }
        }

        for (idx, c) in guess_input.chars().enumerate() {
            if guess_letters[idx].is_none() {
                guess_letters[idx] =
                    Some(self.build_guess_letter_with_accuracy(
                        idx, c, &mut available_letters))
            }
        }

        WordGuess {
            letters: guess_letters.iter().map(|o| o.unwrap()).collect(),
        }
        pub fn guess(&mut self, guess_input: &str) {
            let guess = self.build_guess(guess_input);
            self.guess.push(guess);
        }
    }
}
