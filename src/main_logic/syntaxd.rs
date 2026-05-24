use std::collections::HashMap;

pub enum Dictionaries {
    Russian,
    English,
    Emoji,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWordType {
    Let,
    Print,
    Input,
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
        Self { keywords }
    }

    // Кастомный русский синтаксис!
    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("ПУСТЬ".to_string(), KeyWordType::Let);
        keywords.insert("ПЕЧАТЬ".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        Self { keywords }
    }

    fn emoji_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("✍️".to_string(), KeyWordType::Let);
        keywords.insert("🖨".to_string(), KeyWordType::Print);
        keywords.insert("⌨️".to_string(), KeyWordType::Input);
        Self { keywords }
    }

    pub fn choose_dict(dicts: Dictionaries) -> Self {
        use Dictionaries::*;
        match dicts {
            Russian => Self::russian_style(),
            English => Self::default_basic(),
            Emoji => Self::emoji_style(),
        }
    }
}