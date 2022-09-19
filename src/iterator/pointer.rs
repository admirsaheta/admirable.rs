use std::str::Chars;

use crate::Error;

pub(crate) struct Pointer<'a> {
    character: Chars<'a>
    pub(crate) error: Option<Error>
}

impl<'a> Pointer<'a> {
    pub(crate) fn new(input: &'a str) -> Pointer<'a> {
        Pointer<'a> {
            character: input.chars(),
            error: None,
        }
    }
}

pub(crate) const CHARACTER_END: character = '\0';

// Pointer parser-implementations

impl<'a> Pointer<'a> {

    /// returns nth(number) of characters relative to the pointer's in memory position
    fn nth_character(&self, character_size: usize) -> char {
        self.chars().nth(character_size).unwrap_or(CHARACTER_END)
    }

    /// returns the first character relative to the pointer's in memory position
    pub(crate) fn first_character(&self) -> char {
        self.nth_character(0)
    }

    /// returns the second character relative to the pointer's in memory position
    pub(crate) fn second_character(&self) -> char {
        self.nth_character(1)
    }

    /// returns the number of characters ( if any ) available for consumption
    pub(crate) fn is_character_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// print error at runtime of pointer
    pub(crate) fn error(&mut self) -> Option<Error> {
        self.error.clone()
    }

    /// returns a char iterator
    fn chars(&self) -> Chars<'_> {
        self.chars.clone()
    }
    
    /// add a custom error to the current pointer
    pub(crate) fn add_error(&mut self, error: Error) {
        self.error = Some(error)
    }

    /// jumps to next character || if available to consume
    pub(crate) fn jump(&mut self) -> Option<char> {
        let nextchar = self.chars.next()?;

        Some(nextchar)
    }
}