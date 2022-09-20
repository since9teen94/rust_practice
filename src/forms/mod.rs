use chrono::Datelike;
use serde::Serialize;

#[derive(Serialize)]
pub struct LogRegForm {
    title: String,
    action: String,
    method: String,
    fields: Vec<LogRegFormField>,
    year: i32,
}

#[derive(Serialize)]
struct LogRegFormField {
    id: String,
    text: String,
    field_type: String,
    placeholder: String,
}

impl LogRegFormField {
    pub fn new(id: &str, text: &str, field_type: &str, placeholder: &str) -> LogRegFormField {
        LogRegFormField {
            id: String::from(id),
            text: String::from(text),
            field_type: String::from(field_type),
            placeholder: String::from(placeholder),
        }
    }
}

impl LogRegForm {
    pub fn new(title: &str, action: &str, method: &str) -> LogRegForm {
        let mut form_fields = vec![
            LogRegFormField::new("email", "Email", "email", "Please enter a valid email."),
            LogRegFormField::new(
                "password",
                "Password",
                "password",
                "Please enter a valid password.",
            ),
        ];
        if title == "Register" {
            form_fields.insert(
                0,
                LogRegFormField::new(
                    "first_name",
                    "First Name",
                    "first_name",
                    "Please enter your first name.",
                ),
            );
            form_fields.insert(
                1,
                LogRegFormField::new(
                    "last_name",
                    "Last Name",
                    "last_name",
                    "Please enter your last name.",
                ),
            );
            form_fields.push(LogRegFormField::new(
                "confirm_password",
                "Confirm Password",
                "password",
                "Please confirm your password.",
            ));
        };
        let year = chrono::Utc::now().year();

        LogRegForm {
            title: String::from(title),
            action: String::from(action),
            method: String::from(method),
            year,
            fields: form_fields,
        }
    }
}
