#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use typed_builder::TypedBuilder;
use serde:: {Serialize, Deserialize};

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Color id given by Vinted
    pub id: i32,
    /// Color name in French
    pub title: String,
    /// Color hex representation with format `\#[0-9A-Fa-f]{6}`
    pub hex: String,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for Color {
    fn from(row: Row) -> Self {
        Color::builder()
            .id(row.get("id"))
            .title(row.get("title"))
            .hex(row.get("hex"))
            .build()
    }
}
