pub struct ASCIIChar {
    character : char,
}

impl ASCIIChar {
    fn new(character : char) -> Option<Self> {
        if character.is_ascii() {
            Option::Some(Self {character : character} )
        } else {
            Option::None
        }
    }

    pub fn get_char(&self) -> char {
        self.character
    }
}
