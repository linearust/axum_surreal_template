use std::collections::HashMap;

pub fn parse_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .filter_map(|(field, errs)| {
            errs.first()
                .and_then(|e| e.message.as_ref())
                .map(|msg| (field.to_string(), msg.to_string()))
        })
        .collect()
}
