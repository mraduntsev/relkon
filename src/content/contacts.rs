#[derive(Debug, Clone, PartialEq)]
pub struct ContactsData {
    pub contacts: &'static [Contact],
    pub points: &'static [OperatingPoints],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub title: &'static str,
    pub object: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPoints {
    pub number: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const CONTACTS: ContactsData = ContactsData {
    contacts: &[
        Contact {
            title: "Email · документы и официальные запросы",
            object: "3dstroyproekt@gmail.com",
        },
        Contact {
            title: "Телефон · консультации по проекту",
            object: "+7 982 7 111 000",
        },
        Contact {
            title: "Консультации по проектам — по будням",
            object: "ПН–ПТ 10:00–19:00 (МСК)",
        },
        Contact {
            title: "Договор · счёт · акт",
            object: "Формат работы",
        },
    ],

    points: &[
        OperatingPoints {
            number: "01",
            title: "Заявка",
            description: "Звоните или пишите на почту — как удобнее для первого разговора.",
        },
        OperatingPoints {
            number: "02",
            title: "КП за 1 день",
            description: "В течение рабочего дня — состав, сроки и стоимость по этапам.",
        },
        OperatingPoints {
            number: "03",
            title: "Договор",
            description: "Пакет документов, счёт, аванс",
        },
        OperatingPoints {
            number: "04",
            title: "Работа",
            description: "Статус каждую неделю, правки в один цикл, сопровождение до сдачи.",
        },
    ],
};
