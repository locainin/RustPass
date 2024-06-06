#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum CharClass {
    UpperLetters,
    LowerLetters,
    Numbers,
    SpecialCharacters,
    NoClass,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum GeneratorFlag {
    CharFromEveryGroup,
    NoFlags,
}

impl Default for CharClass {
    fn default() -> Self {
        CharClass::NoClass
    }
}

impl Default for GeneratorFlag {
    fn default() -> Self {
        GeneratorFlag::NoFlags
    }
}

