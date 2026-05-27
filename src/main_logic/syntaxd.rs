use std::collections::HashMap;

pub enum Dictionaries {
    Russian,
    English,
    Emoji,
    Crab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWordType {
    Let,
    Print,
    Input,
    If,
    Then,
    Goto,
    Random,
    End,
}

pub struct SyntaxDict {
    pub keywords: HashMap<String, KeyWordType>
}

#[allow(dead_code)]
impl SyntaxDict {
    fn default_basic() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("LET".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("INPUT".to_string(), KeyWordType::Input);
        keywords.insert("IF".to_string(), KeyWordType::If);
        keywords.insert("THEN".to_string(), KeyWordType::Then);
        keywords.insert("GOTO".to_string(), KeyWordType::Goto);
        keywords.insert("RANDOM".to_string(), KeyWordType::Random);
        keywords.insert("END".to_string(), KeyWordType::End);
        Self { keywords }
    }

    // Кастомный русский синтаксис!
    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("ПУСТЬ".to_string(), KeyWordType::Let);
        keywords.insert("ПЕЧАТЬ".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        keywords.insert("ЕСЛИ".to_string(), KeyWordType::If);
        keywords.insert("ТО".to_string(), KeyWordType::Then);
        keywords.insert("ИДИ".to_string(), KeyWordType::Goto);
        keywords.insert("РАНДОМ".to_string(), KeyWordType::Random);
        keywords.insert("СТОП".to_string(), KeyWordType::End);
        Self { keywords }
    }

    fn emoji_style() -> Self {
    let mut keywords = HashMap::new();
        keywords.insert("✍".to_string(), KeyWordType::Let);
        keywords.insert("🖨".to_string(), KeyWordType::Print);
        keywords.insert("⌨".to_string(), KeyWordType::Input);
        keywords.insert("❓".to_string(), KeyWordType::If);
        keywords.insert("➡".to_string(), KeyWordType::Then);
        keywords.insert("🚀".to_string(), KeyWordType::Goto);
        keywords.insert("🎲".to_string(), KeyWordType::Random);
        keywords.insert("⛔".to_string(), KeyWordType::End);
        Self { keywords }
    }
    fn crab_style() -> Self {
    let mut keywords = HashMap::new();
        keywords.insert("🦀".to_string(), KeyWordType::Let);
        keywords.insert("📢".to_string(), KeyWordType::Print);
        keywords.insert("⚓".to_string(), KeyWordType::Input);
        keywords.insert("🌊".to_string(), KeyWordType::If);
        keywords.insert("🚢".to_string(), KeyWordType::Then);
        keywords.insert("🚀".to_string(), KeyWordType::Goto);
        keywords.insert("🎲".to_string(), KeyWordType::Random);
        keywords.insert("⛔".to_string(), KeyWordType::End);
        Self { keywords }
    }

    pub fn choose_dict(dicts: Dictionaries) -> SyntaxDict {
        use Dictionaries::*;
        match dicts {
            Russian => Self::russian_style(),
            English => Self::default_basic(),
            Emoji => Self::emoji_style(),
            Crab => Self::crab_style(),
        }
    }

    pub fn get_dict(name_of_dict: &str) -> Dictionaries {
        match name_of_dict {
            "RUSSIAN" => Dictionaries::Russian,
            "EMOJI" => Dictionaries::Emoji,
            "CRAB" => Dictionaries::Crab,
            _ => Dictionaries::English,
        }
    }
}