#[derive(Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub enum Endianness {
    Little,
    Big,
    #[default]
    Native,
}
