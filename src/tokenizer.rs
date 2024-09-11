use crate::str_checker;

pub struct TokensResult<'a> {
    pub is_valid_url: bool,

    pub protocol: Option<&'a str>,
    pub host: Option<&'a str>,
    pub directories: Vec<&'a str>,
    pub filename: Option<&'a str>,
    pub suffix: Option<&'a str>,

    pub words: Vec<&'a str>,
    pub numbers: Vec<&'a str>,
    pub alphanumeric: Vec<&'a str>,
    pub punctuations: Vec<&'a str>,

    pub tokens: Vec<&'a str>,
    pub granular_tokens: Vec<&'a str>,
}

impl TokensResult<'_> {
    pub fn granular(&self) -> String {
        let mut granular_string = String::from("[");
        let mut has_tokens = false;
        for token in &self.granular_tokens {
            granular_string.push_str(format!("\r\n    {} => ", token).as_str());
            for (index, c) in token.chars().enumerate() {
                granular_string.push_str(format!("'{}'", c).as_str());
                if index != token.len() - 1 {
                    granular_string.push_str(", ");
                }
            }
            has_tokens = true;
        }
        if has_tokens {
            granular_string.push_str("\r\n");
        }
        granular_string.push_str("]\r\n");
        granular_string
    }
    pub fn new<'a>(input_str: &'a str) -> TokensResult<'a> {
        let mut result = TokensResult {
            is_valid_url: false,

            protocol: None,
            host: None,
            directories: Vec::new(),
            filename: None,
            suffix: None,
            numbers: Vec::new(),

            words: Vec::new(),
            alphanumeric: Vec::new(),
            punctuations: Vec::new(),

            tokens: tokenize(input_str),
            granular_tokens: Vec::new(),
        };

        // Check if it's a URL for a website.
        result.is_valid_url = result.tokens.len() >= 3
            && is_valid_protocol(result.tokens.get(0).unwrap())
            && !result.tokens.get(0).unwrap().is_empty()
            && result.tokens.get(1).unwrap().is_empty();

        if result.is_valid_url {
            let protocol_str = result.tokens.get(0).unwrap();
            result.protocol = Some(&protocol_str[..protocol_str.len() - 1]);
            result.host = Some(result.tokens.get(2).unwrap());

            // [protocol:]/[empty]/[host]
            if result.tokens.len() > 3 {
                let it = &mut result.tokens[3..].into_iter().peekable();
                while let Some(token) = it.next() {
                    // Check if it's the last element in the list of tokens.
                    if it.peek().is_none() {
                        let (filename, suffix) = split_suffix(token);
                        result.filename = Some(filename);
                        result.suffix = Some(suffix);
                    } else {
                        result.directories.push(token);
                    }
                }
            } else {
                let (host, suffix) = split_suffix(result.tokens.get(2).unwrap());
                result.host = Some(host);
                result.suffix = Some(suffix);
            }
        }

        for token in &result.tokens {
            if token.is_empty() {
                continue;
            }
            let mut start_index = 0;
            for (index, c) in token.chars().enumerate() {
                // Split the token if a punctuation is encountered.
                if !c.is_alphanumeric() {
                    // Push the characters before the punctuation and the punctuation itself.
                    let before = &token[start_index..index];
                    if !before.is_empty() {
                        result.granular_tokens.push(before);
                    }
                    result.granular_tokens.push(&token[index..index + 1]);
                    start_index = index + 1;
                }
            }
            let last_part: &str = &token[start_index..];
            if !last_part.is_empty() {
                result.granular_tokens.push(last_part);
            }
        }

        for token in &result.granular_tokens {
            if str_checker::is_number(token) {
                result.numbers.push(token);
            } else if str_checker::is_word(token) {
                result.words.push(token);
            } else if str_checker::is_alphanumeric(token) {
                result.alphanumeric.push(token);
            // Discard empty tokens.
            } else if !token.is_empty() {
                result.punctuations.push(token);
            }
        }

        result
    }
}

pub fn is_valid_protocol(protocol_str: &str) -> bool {
    // Skip empty protocol string.
    if protocol_str.len() == 0 {
        return false;
    }

    let end_index: usize = protocol_str.len() - 1;
    for (current_index, c) in protocol_str.chars().enumerate() {
        if (current_index != end_index && !c.is_alphabetic())
            || (current_index == end_index && c != ':')
        {
            return false;
        }
    }
    true
}

// Some URLs have a suffix after the filename.
// --> for example: https://www.example.com/article/123?category=technology#section2
// Separation should be done to properly extract the filename.
pub fn split_suffix(mixed: &str) -> (&str, &str) {
    let first_part: &str;
    let suffix: &str;

    for (index, c) in mixed.chars().enumerate() {
        if !c.is_alphanumeric() && (c == '?' || c == '#') {
            first_part = &mixed[..index];
            suffix = &mixed[index + 1..];
            return (first_part, suffix);
        }
    }
    (mixed, "")
}

fn tokenize(input_str: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut start_index: usize = 0;
    for (index, c) in input_str.chars().enumerate() {
        if c == '/' {
            let extracted: &str = &input_str[start_index..index];
            tokens.push(extracted);
            start_index = index + 1;
        }
    }
    let last_part: &str = &input_str[start_index..];
    tokens.push(last_part);
    tokens
}
