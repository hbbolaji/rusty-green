pub fn test_option() -> Option<u8> {
    let mut opt1 = None;
    println!("{:?}", opt1);
    opt1 = Some(10);
    opt1
}

#[derive(Debug)]
pub enum CharacterType {
  Archer,
  Warrior,
  Mage
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            Self::Archer => "Archer",
            Self::Mage => "Mage",
            Self::Warrior => "Warrior"
        }.into()
    }
}

pub fn test_option_chartype() -> Option<CharacterType> {
  let mut chartype: Option<CharacterType> = None;
  println!("{:?} character has been choosen", chartype);
  chartype = Some(CharacterType::Mage);
  chartype
}
