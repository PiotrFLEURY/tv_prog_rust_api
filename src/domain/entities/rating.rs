use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Rating {
    /// The system of the rating (e.g., "MPAA", "TVPG")
    pub system: Option<String>,

    /// The value of the rating (e.g., "PG-13", "R")
    pub value: Option<String>,

    /// The icon associated with the rating, if any
    pub icon: Option<String>,
}