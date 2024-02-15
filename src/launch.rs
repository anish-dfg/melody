use derive_more::Display;

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum LaunchMode {
    #[display(fmt = "development")]
    Development,
    #[display(fmt = "testing")]
    Testing,
    #[display(fmt = "staging")]
    Staging,
    #[display(fmt = "production")]
    Production,
}
