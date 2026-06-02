use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// 言語種別。設定ファイルに小文字 ("en" / "ja") で保存される
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    En,
    Ja,
}

impl Lang {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Some(Lang::En),
            "ja" | "jp" | "japanese" => Some(Lang::Ja),
            _ => None,
        }
    }
}

// プロセス起動時に一度だけ設定する
static LANG: OnceLock<Lang> = OnceLock::new();

pub fn init(lang: Lang) {
    let _ = LANG.set(lang);
}

pub fn current() -> Lang {
    LANG.get().copied().unwrap_or(Lang::En)
}

// マクロで「英語 / 日本語」のペアから現在言語の文字列を返す
macro_rules! tr {
    ($en:expr, $ja:expr) => {
        match $crate::i18n::current() {
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Ja => $ja,
        }
    };
}
pub(crate) use tr;
