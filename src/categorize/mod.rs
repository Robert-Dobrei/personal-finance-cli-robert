use regex::Regex;

pub fn auto_categorize(description: &str) -> Option<String> {
    let desc = description.to_lowercase();

    if Regex::new(r"(mcdonalds|kfc|pizza|restaurant|food|burger)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Food".to_string());
    }

    if Regex::new(r"(uber|bolt|taxi|transport|bus|train)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Transport".to_string());
    }

    if Regex::new(r"(kaufland|lidl|carrefour|mega|grocery|market)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Groceries".to_string());
    }

    if Regex::new(r"(netflix|cinema|spotify|game|entertainment)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Entertainment".to_string());
    }

    if Regex::new(r"(electricity|gas|water|internet|bill)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Utilities".to_string());
    }

	if Regex::new(r"(salary|transfer|income)")
        .unwrap()
        .is_match(&desc)
    {
        return Some("Income".to_string());
    }

    None
}
