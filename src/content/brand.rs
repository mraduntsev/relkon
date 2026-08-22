#[derive(Clone, PartialEq)]
pub struct BrandData {
    pub name: &'static str,
    pub full_desc_line1: &'static str,
    pub full_desc_line2: &'static str,
    pub compact_desc: &'static str,
    pub initial: &'static str,
}

pub const BRAND: BrandData = BrandData {
    name: "РАДУНЦЕВ·КМ",
    full_desc_line1: "Инженер-конструктор ПГС.",
    full_desc_line2: "КМ · КЖ · КМД · расчёты · надзор.",
    compact_desc: "конструктор КМ / КЖ / КМД",
    initial: "Р",
};
