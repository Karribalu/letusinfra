pub enum SupportKind {
    Infra,
    App,
}
impl SupportKind {
    pub fn variants() -> [&'static str; 2] {
        ["Infra", "App"]
    }

    pub fn is_valid(kind: &str) -> bool {
        Self::variants().contains(&kind)
    }
}
