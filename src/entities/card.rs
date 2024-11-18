use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub r#type: CardType,
    pub image: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: String,
    pub provider_url: String,
    pub html: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub embed_url: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}
