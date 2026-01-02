use serde_json::Value;

pub(crate) fn redact_body_snippet(bytes: &[u8], byte_limit: usize) -> Option<String> {
    if byte_limit == 0 || bytes.is_empty() {
        return None;
    }

    let value = serde_json::from_slice::<Value>(bytes).ok();
    if let Some(mut value) = value {
        redact_json_value(&mut value);
        let json = serde_json::to_string(&value).ok()?;
        return Some(truncate_utf8_bytes(&json, byte_limit));
    }

    let text = String::from_utf8_lossy(bytes);
    Some(truncate_utf8_bytes(&redact_text(&text), byte_limit))
}

fn redact_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if should_redact_key(key) {
                    *value = Value::String("<redacted>".to_owned());
                } else {
                    redact_json_value(value);
                }
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                redact_json_value(item);
            }
        }
        _ => {}
    }
}

fn should_redact_key(key: &str) -> bool {
    matches!(
        key.to_ascii_lowercase().as_str(),
        "authorization"
            | "cookie"
            | "password"
            | "secret"
            | "client_secret"
            | "token"
            | "access_token"
            | "refresh_token"
            | "api_key"
            | "apikey"
    )
}

fn redact_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for line in text.lines() {
        if let Some(redacted) = redact_header_line(line, "authorization") {
            output.push_str(&redacted);
        } else if let Some(redacted) = redact_header_line(line, "cookie") {
            output.push_str(&redacted);
        } else {
            output.push_str(&redact_inline_token(line));
        }
        output.push('\n');
    }

    if output.ends_with('\n') && !text.ends_with('\n') {
        output.pop();
    }
    output
}

fn redact_header_line(line: &str, header_name: &str) -> Option<String> {
    let (name, value) = line.split_once(':')?;
    if name.trim().eq_ignore_ascii_case(header_name) {
        return Some(format!("{}: <redacted>", name.trim()));
    }
    let _ = value;
    None
}

fn redact_inline_token(line: &str) -> String {
    let line = redact_prefixed_token(line, "Bearer ");
    redact_prefixed_token(&line, "Basic ")
}

fn redact_prefixed_token(line: &str, prefix: &str) -> String {
    let Some(start) = line.find(prefix) else {
        return line.to_owned();
    };

    let token_start = start + prefix.len();
    let mut token_end = token_start;
    for (idx, ch) in line[token_start..].char_indices() {
        if ch.is_whitespace() {
            break;
        }
        token_end = token_start + idx + ch.len_utf8();
    }

    if token_end == token_start {
        return line.to_owned();
    }

    let mut out = String::with_capacity(line.len());
    out.push_str(&line[..token_start]);
    out.push_str("<redacted>");
    out.push_str(&line[token_end..]);
    out
}

fn truncate_utf8_bytes(input: &str, byte_limit: usize) -> String {
    if input.len() <= byte_limit {
        return input.to_owned();
    }

    let mut end = 0;
    for (idx, _) in input.char_indices() {
        if idx > byte_limit {
            break;
        }
        end = idx;
    }

    let mut truncated = input[..end].to_owned();
    truncated.push_str("...(truncated)");
    truncated
}
