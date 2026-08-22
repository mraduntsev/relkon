pub struct StatItem {
    pub value: u32,
    pub suffix: &'static str,
    pub label: &'static str,
}

pub const STATS: &[StatItem] = &[
    StatItem {
        value: 15,
        suffix: "+",
        label: "лет в проектировании",
    },
    StatItem {
        value: 200,
        suffix: "+",
        label: "выполненных проектов",
    },
    StatItem {
        value: 10000,
        suffix: "+",
        label: "тонн стали в КМ/КМД",
    },
    StatItem {
        value: 10000,
        suffix: "+",
        label: "м² монолита в КЖ",
    },
];

pub const AXIS_LABELS: &[&str] = &["А", "Б", "В"];

pub struct StampField {
    pub label: &'static str,
    pub value: &'static str,
}

pub const STAMP_FIELDS: &[StampField] = &[
    StampField {
        label: "Шифр:",
        value: "ВН-2026-08",
    },
    StampField {
        label: "Наименование:",
        value: "Лендинг конструктора",
    },
    StampField {
        label: "Стадия:",
        value: "Р · Масштаб: 1:100",
    },
    StampField {
        label: "Лист:",
        value: "1 · Листов: 1",
    },
    StampField {
        label: "Разработал:",
        value: "Радунцев М. В.",
    },
    StampField {
        label: "Проверил:",
        value: "—",
    },
    StampField {
        label: "Н. контр.:",
        value: "—",
    },
    StampField {
        label: "Дата:",
        value: "18.08.2026",
    },
];
