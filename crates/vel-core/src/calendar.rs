use serde::{Deserialize, Serialize};

use crate::CalendarId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarVisibility {
    Private,
    Shared,
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Calendar {
    pub id: CalendarId,
    pub display_name: String,
    pub timezone: String,
    pub visibility: CalendarVisibility,
    pub is_default: bool,
    pub description: Option<String>,
    pub color: Option<String>,
}

impl Calendar {
    pub fn validate(&self) -> Result<(), String> {
        if self.display_name.trim().is_empty() {
            return Err("calendar display_name must not be empty".to_string());
        }
        if self.timezone.trim().is_empty() {
            return Err("calendar timezone must not be empty".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Calendar, CalendarVisibility};
    use crate::CalendarId;

    #[test]
    fn calendar_requires_display_name_and_timezone() {
        let invalid = Calendar {
            id: CalendarId::new(),
            display_name: String::new(),
            timezone: String::new(),
            visibility: CalendarVisibility::Private,
            is_default: false,
            description: None,
            color: None,
        };

        assert!(invalid.validate().unwrap_err().contains("display_name"));

        let valid = Calendar {
            id: CalendarId::new(),
            display_name: "Personal".to_string(),
            timezone: "America/Denver".to_string(),
            visibility: CalendarVisibility::Shared,
            is_default: true,
            description: None,
            color: Some("#2274a5".to_string()),
        };

        valid.validate().unwrap();
    }
}
