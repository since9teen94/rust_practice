use serde::Serialize;

#[derive(Serialize)]
pub struct LogRegForm {
    title: String,
    action: String,
    fields: Vec<LogRegFormField>,
}

impl LogRegForm {
    pub fn new(title: &str, action: &str) -> LogRegForm {
        let mut form_fields = vec![
            LogRegFormField::new("email", "Email", "email"),
            LogRegFormField::new("password", "Password", "password"),
        ];
        if title == "Register" {
            form_fields.insert(
                0,
                LogRegFormField::new("first_name", "First Name", "first_name"),
            );
            form_fields.insert(
                1,
                LogRegFormField::new("last_name", "Last Name", "last_name"),
            );
            form_fields.push(LogRegFormField::new(
                "confirm_password",
                "Confirm Password",
                "confirm_password",
            ));
        }
        LogRegForm {
            title: String::from(title),
            action: String::from(action),
            fields: form_fields,
        }
    }
}

#[derive(Serialize)]
struct LogRegFormField {
    id: String,
    text: String,
    field_type: String,
}

impl LogRegFormField {
    pub fn new(id: &str, text: &str, field_type: &str) -> LogRegFormField {
        LogRegFormField {
            id: String::from(id),
            text: String::from(text),
            field_type: String::from(field_type),
        }
    }
}
