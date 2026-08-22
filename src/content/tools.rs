pub struct Tools {
    pub title: &'static str,
    pub level: &'static str,
}

pub const TOOLS: &[Tools] = &[
    Tools {
        title: "Stark ES",
        level: "95",
    },
    Tools {
        title: "ЛИРА-САПР",
        level: "95",
    },
    Tools {
        title: "Tekla Structures / IFC-координация",
        level: "95",
    },
    Tools {
        title: "Автоматизация: Excel / Python",
        level: "95",
    },
    Tools {
        title: "AutoCAD / Advance Steel",
        level: "90",
    },
];

pub const CHIP: &[&'static str] = &[
    "Расчёты МКЭ",
    "Узлы на болтах и сварке",
    "Карты раскроя",
    "Авторский надзор",
    "Аудит чертежей",
    "Нагрузки и воздействия",
    "Серии и типовые конструкции",
];

pub const STANDARDS: &[&'static str] = &[
    "СП 16.13330.2017 — стальные конструкции",
    "СП 20.13330.2016 — нагрузки и воздействия",
    "СП 63.13330.2018 — бетонные и ж/б конструкции",
    "ГОСТ 27772-2021 — сталь для конструкций",
    "ГОСТ 23118-2019 — стальные конструкции, ТУ",
    "ГОСТ Р 21.101-2026 — правила выполнения рабочей документации",
    "Eurocode для зарубежных клиентов",
];
