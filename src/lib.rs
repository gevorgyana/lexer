// public API is marked as pub
mod mlcomment;
mod ascii;
mod dfa;
mod token;
mod lexeme;
mod pda;
mod driver;
mod qident;
mod reserved;
mod regex;

// the application follows the following conventions
// for dealing with errors
// https://github.com/gevorgyana/rust_conventions

// This project uses the following non-obvious features of Rust:
// raw string literals, r'\', those are literals that are not considered to
// have escape sequences. Ex.: ```if r'\' == '\\' { true } else { false } ```
// evaluates to true.

// It works with ASCII by
// - reading the input from file into a string. In Rust, a string is UTF-8-encoded
// sequence of chaaracters (char), 1 char takes 4 bytes, so it is represented by
// u32 type.
// - then collecting the bytes it contains. One byte takes 8 bits and is represented
// by u8.
//
// Of course, in case the file was UTF-8 encoded and had unicode characters, the
// bytes are not guaranteed to correspond to what we call letters.
