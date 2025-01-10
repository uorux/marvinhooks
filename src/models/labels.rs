use serde::{Deserialize, Serialize};

/// Represents a single label.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub group_id: Option<String>,
    pub created_at: i64,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub show_as: Option<String>,
    #[serde(default)]
    pub is_action: Option<bool>,
    #[serde(default)]
    pub is_hidden: Option<bool>,
}

/// Represents a group of labels.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelGroup {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub rank: i64,
    pub created_at: i64,
    #[serde(default)]
    pub is_exclusive: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub is_menu: Option<bool>,
}
