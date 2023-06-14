use serde_json::Value;

pub fn values_to_strings(option_values: Option<&Vec<Value>>) -> Vec<String> {
    option_values
        .map(|values| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| Vec::new())
}
