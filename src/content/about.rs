#[derive(Debug, Clone, PartialEq)]
pub struct ProfileData {
    pub full_name: &'static str,
    pub position: &'static str,
    pub quote: &'static str,
    pub src: &'static str,
    pub signature: Option<&'static str>,
    pub qualifications: &'static [Qualification],
    pub work_info: &'static [&'static str],
    pub about: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Qualification {
    pub title: &'static str,
    pub organization: Option<&'static str>,
    pub year: Option<u32>,
}

pub const PROFILE: ProfileData = ProfileData {
    full_name: "Радунцев Максим Владимирович",
    position: "Главный конструктор / ГК",
    quote: "Хороший чертёж — тот, по которому завод не звонит ни разу. Плохой — тот, из-за которого звонят каждый день",
    src: "./images/me.webp",
    signature: Some("ИП Радунцев"),

    qualifications: &[
        Qualification {
            title: "Специалист ПГС",
            organization: None,
            year: None,
        },
        Qualification {
            title: "ЕВРОСОФТ, STARK ES",
            organization: Some("ЕВРОСОФТ"),
            year: Some(2019),
        },
    ],

    work_info: &["Работа с РФ и ЕАЭС", "ИП, договор и счёт"],

    about: &[
        "Проектирую и управляю строительными проектами более 15 лет. Специализируюсь на промышленном и гражданском строительстве.",
        "Работаю с проектами любой сложности — от концепции до рабочей документации. Обеспечиваю полное сопровождение на всех этапах.",
        "Сотрудничаю с заказчиками из России и стран ЕАЭС. Работаю как ИП, что позволяет гибко выстраивать финансовые отношения.",
        "Главный принцип — ответственность за результат и соблюдение сроков. Каждый проект довожу до логического завершения.",
    ],
};
