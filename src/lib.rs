use std::fs;

// declare modules and decide if they are public API
mod mlcomment;
mod ascii;
mod dfa;
mod token;
mod lexeme;

fn test() {
    // read the whole file into a string and move on from there
    fs::read_to_string("test.hs");
}
