pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
}

pub const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        label: "Услуги",
        href: "#services",
    },
    NavItem {
        label: "Проекты",
        href: "#projects",
    },
    NavItem {
        label: "Процесс",
        href: "#process",
    },
    NavItem {
        label: "Обо мне",
        href: "#about",
    },
    NavItem {
        label: "Контакты",
        href: "#contact",
    },
];

pub fn nav_ids() -> Vec<&'static str> {
    NAV_ITEMS.iter().map(|item| &item.href[1..]).collect()
}
