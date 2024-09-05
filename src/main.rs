use std::error::Error;

use eframe::egui;

#[derive(Debug)]
struct Token<'a> {
    part: &'a str,
}

impl Token<'_> {
    fn display_granular(&self) {
        println!(
            "Token: {:?} -> {:?}",
            self.part,
            self.part.chars().into_iter().collect::<Vec<char>>()
        );
    }
}

fn is_alphabetic(character: char) -> bool {
    (character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z')
}

fn is_valid_domain_char(character: char) -> bool {
    is_alphabetic(character) || (character >= '0' && character <= '9') || character == '-'
}

fn validate_protocol(protocol_str: &str) -> Result<&str, Box<dyn Error>> {
    let end_index: usize = protocol_str.len() - 1;
    for (current_index, c) in protocol_str.chars().enumerate() {
        if (current_index != end_index && !is_alphabetic(c))
            || (current_index == end_index && c != ':')
        {
            return Err("This is not a valid protocol.".into());
        }
    }
    return Ok(&protocol_str[..end_index - 1]);
}

fn validate_hostname(hostname_str: &str) -> Result<&str, Box<dyn Error>> {
    if hostname_str.len() == 0 {
        return Err("This is not a valid hostname.".into());
    }
    let mut periods: usize = 0;
    let mut previous_is_period = false;
    for (current_index, c) in hostname_str.chars().enumerate() {
        if c == '.' {
            if previous_is_period {
                return Err("This is not a valid hostname.".into());
            }
            periods += 1;
            previous_is_period = true;
            continue;
        }
        previous_is_period = false;
        if (current_index == 0 && c == '-') || (current_index != 0 && !is_valid_domain_char(c)) {
            return Err("This is not a valid hostname.".into());
        }
    }
    if periods > 2 {
        return Err("This is not a valid hostname.".into());
    }
    Ok(&hostname_str)
}

fn validate_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, Box<dyn Error>> {
    if tokens.len() < 2 {
        return Err("Invalid URL.".into());
    }
    for token in &tokens[2..] {
        println!("Files / Directories: {:?}", token);
    }
    Ok(tokens)
}

fn extract_tokens(input_str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut start_index: usize = 0;
    for (index, c) in input_str.chars().enumerate() {
        if c == '/' {
            let extracted: &str = &input_str[start_index..index];
            // skip empty tokens
            if !extracted.is_empty() {
                tokens.push(Token { part: extracted });
            }
            start_index = index + 1;
        }
    }
    let last_part: &str = &input_str[start_index..];
    if !last_part.is_empty() {
        tokens.push(Token { part: last_part });
    }
    tokens
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://mcm.edu.ph/test_directory/test_subdirectory/test_page.php";
    let tokens: Vec<Token> = validate_tokens(extract_tokens(url))?;

    let protocol: &str = validate_protocol(tokens.get(0).unwrap().part)?;
    let hostname: &str = validate_hostname(tokens.get(1).unwrap().part)?;

    println!("Tokens: {:#?}", tokens);

    println!("Protocol: {}", protocol);
    println!("Hostname: {}", hostname);

    for token in tokens.iter() {
        token.display_granular();
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([853.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui app!",
        options,
        Box::new(|_cc| Ok(Box::<GraphicApp>::default())),
    )?;

    Ok(())
}

struct GraphicApp {}

impl Default for GraphicApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for GraphicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hi sir josh and josh lechoncito");
        });
    }
}
