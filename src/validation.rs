pub fn is_email(value: &str) -> bool {
    if value.chars().any(char::is_whitespace) {
        return false;
    }

    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty()
        && !domain.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

#[cfg(test)]
mod tests {
    use super::is_email;

    #[test]
    fn email_validation_rejects_obviously_invalid_addresses() {
        for email in [
            "",
            "plain",
            "@example.com",
            "a@b",
            "a b@example.com",
            "a@example.",
        ] {
            assert!(!is_email(email), "{email}");
        }

        for email in ["a@example.com", "first.last+tag@sub.example.com"] {
            assert!(is_email(email), "{email}");
        }
    }
}
