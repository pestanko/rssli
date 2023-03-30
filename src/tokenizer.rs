pub fn tokenize(orig: &str) -> anyhow::Result<Vec<String>> {
    let mut tokens = Vec::new();

    let mut buffer = String::new();
    let mut is_string = false;
    let mut escaped = false;

    for ch in orig.chars() {
        if is_string {
            if escaped {
                escaped = false;
                buffer.push(ch);
                continue;
            }

            match ch {
                '"' => {
                    tokens.push(buffer);
                    buffer = String::new();
                    is_string = false;
                }
                '\\' => {
                    escaped = true;
                }
                _ => {
                    buffer.push(ch);
                }
            }
            continue;
        }

        if ch == '(' || ch == ')' || ch == '\"' || ch.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push(buffer);
                buffer = String::new();
            }
        }

        match ch {
            '(' => {
                tokens.push("(".to_string());
            }
            ')' => {
                tokens.push(")".to_string());
            }
            '"' => {
                is_string = true;
                buffer.push('\"');
            }
            _ => {
                if !ch.is_whitespace() {
                    buffer.push(ch);
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_simple_expressions() {
        assert_eq!(tokenize("()").unwrap(), vec!["(".to_string(), ")".to_owned()]);
        assert_eq!(
            tokenize("( 15 )").unwrap(),
            vec!["(".to_string(), "15".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(15)").unwrap(),
            vec!["(".to_string(), "15".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(   15    )").unwrap(),
            vec!["(".to_string(), "15".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(   15  16  )").unwrap(),
            vec![
                "(".to_string(),
                "15".to_owned(),
                "16".to_owned(),
                ")".to_owned()
            ]
        );
    }

    #[test]
    fn test_tokens_string_expressions() {
        assert_eq!(
            tokenize("(\"ahoj\")").unwrap(),
            vec!["(".to_string(), "\"ahoj".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(\"\")").unwrap(),
            vec!["(".to_string(), "\"".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(\"ahoj svet\")").unwrap(),
            vec!["(".to_string(), "\"ahoj svet".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(\"ahoj\" \"svet\")").unwrap(),
            vec![
                "(".to_string(),
                "\"ahoj".to_string(),
                "\"svet".to_owned(),
                ")".to_owned()
            ]
        );
        assert_eq!(
            tokenize("(\"ahoj\\\\ svet\")").unwrap(),
            vec!["(".to_string(), "\"ahoj\\ svet".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize("(\"ahoj\\\\ svet\")").unwrap(),
            vec!["(".to_string(), "\"ahoj\\ svet".to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize(r#"("ahoj\\ svet")"#).unwrap(),
            vec!["(".to_string(), r#""ahoj\ svet"#.to_owned(), ")".to_owned()]
        );
        assert_eq!(
            tokenize(r#"("ahoj\"\"\\ svet")"#).unwrap(),
            vec![
                "(".to_string(),
                r#""ahoj""\ svet"#.to_owned(),
                ")".to_owned()
            ]
        );
    }
}
