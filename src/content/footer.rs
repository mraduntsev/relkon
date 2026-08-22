#[derive(Debug, Clone, PartialEq)]
pub struct FooterData {
    pub copyright: &'static str,
    pub work_info: &'static [&'static str],
}

pub const FOOTER: FooterData = FooterData {
    copyright: "© 2026 ИП Радунцев М. В. Все листы прошиты.",
    work_info: &[
        "Шифр: ЛЕНД-2026",
        "Стадия: П",
        "Лист 1 / 1",
        "Формат A4 → Web",
    ],
};
