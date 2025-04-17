use dioxus::prelude::*;

/// Highlights HTML source code by applying spans for syntax elements such as tags, attributes, and content.
///
/// This function processes the provided HTML code, identifying tags, attributes, and content.
/// It wraps detected elements in appropriately colored spans for syntax highlighting.
fn highlight_html_syntax(code: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_attr_value = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();
    let mut quote_char = '"'; // Default quote character

    for i in 0..chars.len() {
        // Handle HTML comments
        if !in_tag
            && !in_comment
            && i + 3 < chars.len()
            && chars[i] == '<'
            && chars[i + 1] == '!'
            && chars[i + 2] == '-'
            && chars[i + 3] == '-'
        {
            // Start of comment
            if token_start < i {
                result.push_str(&code[token_start..i]);
            }
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        if in_comment
            && i + 2 < chars.len()
            && chars[i] == '-'
            && chars[i + 1] == '-'
            && chars[i + 2] == '>'
        {
            // End of comment
            result.push_str(&code[token_start..i + 3]);
            result.push_str("</span>");
            token_start = i + 3;
            in_comment = false;
            continue;
        }

        if in_comment {
            continue;
        }

        // Handle tag opening
        if !in_tag && chars[i] == '<' && i + 1 < chars.len() && chars[i + 1] != '!' {
            if token_start < i {
                // Text content before tag
                let content = &code[token_start..i];
                if !content.trim().is_empty() {
                    result.push_str("<span class='text-white'>");
                    result.push_str(content);
                    result.push_str("</span>");
                } else {
                    result.push_str(content);
                }
            }
            result.push_str("<span class='text-blue-400'>&lt;");
            in_tag = true;
            token_start = i + 1;
            continue;
        }

        // Handle tag closing
        if in_tag && chars[i] == '>' {
            if token_start < i {
                let tag_content = &code[token_start..i];
                result.push_str(&highlight_tag_content(tag_content));
            }
            result.push_str("&gt;</span>");
            in_tag = false;
            in_attr_value = false;
            token_start = i + 1;
            continue;
        }

        // Handle attribute values
        if in_tag && (chars[i] == '"' || chars[i] == '\'') {
            if !in_attr_value {
                // Start of attribute value
                if token_start < i {
                    let attr_name = &code[token_start..i];
                    result.push_str(&highlight_tag_content(attr_name));
                }
                quote_char = chars[i];
                result.push_str("<span class='text-green-400'>");
                result.push(chars[i]);
                in_attr_value = true;
                token_start = i + 1;
            } else if chars[i] == quote_char {
                // End of attribute value
                result.push_str(&code[token_start..i]);
                result.push(chars[i]);
                result.push_str("</span>");
                in_attr_value = false;
                token_start = i + 1;
            }
            continue;
        }

        // Handle whitespace in tags (not in attribute values)
        if in_tag && !in_attr_value && chars[i].is_whitespace() {
            if token_start < i {
                let tag_name = &code[token_start..i];
                if tag_name.starts_with('/') {
                    // Closing tag
                    result.push_str("<span class='text-blue-400'>");
                    result.push_str(tag_name);
                    result.push_str("</span>");
                } else {
                    // Opening tag
                    result.push_str("<span class='text-blue-400'>");
                    result.push_str(tag_name);
                    result.push_str("</span>");
                }
            }
            result.push(chars[i]);
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let remaining = &code[token_start..];
        if in_tag {
            result.push_str(&highlight_tag_content(remaining));
        } else if in_comment {
            result.push_str(remaining);
            result.push_str("</span>");
        } else {
            // Text content
            if !remaining.trim().is_empty() {
                result.push_str("<span class='text-white'>");
                result.push_str(remaining);
                result.push_str("</span>");
            } else {
                result.push_str(remaining);
            }
        }
    }

    result
}

/// Highlights the content inside an HTML tag, such as tag names and attributes.
fn highlight_tag_content(content: &str) -> String {
    let mut result = String::new();
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.is_empty() {
        return content.to_string();
    }

    // First part is the tag name
    let tag_name = parts[0];
    if tag_name.starts_with('/') {
        // Closing tag
        result.push_str(&format!("<span class='text-blue-400'>{}</span>", tag_name));
    } else {
        // Opening tag
        result.push_str(&format!("<span class='text-blue-400'>{}</span>", tag_name));
    }

    // Rest are attributes
    let mut current_pos = tag_name.len();
    for i in 1..parts.len() {
        // Find the position of this part in the original content
        let part_pos = content[current_pos..].find(parts[i]).unwrap() + current_pos;

        // Add any content between the last part and this one
        result.push_str(&content[current_pos..part_pos]);
        current_pos = part_pos + parts[i].len();

        // Highlight attribute name
        if parts[i].contains('=') {
            let attr_parts: Vec<&str> = parts[i].split('=').collect();
            result.push_str(&format!(
                "<span class='text-purple-400'>{}</span>=",
                attr_parts[0]
            ));

            // If there's a value part after the equals
            if attr_parts.len() > 1 {
                result.push_str(attr_parts[1]);
            }
        } else {
            result.push_str(&format!(
                "<span class='text-purple-400'>{}</span>",
                parts[i]
            ));
        }
    }

    // Add any remaining content
    if current_pos < content.len() {
        result.push_str(&content[current_pos..]);
    }

    result
}

/// Highlights RSX syntax by applying spans for syntax elements.
fn highlight_rsx_syntax(code: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();

    for i in 0..chars.len() {
        // Handle comments first
        if !in_string && i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            // Add any accumulated token before the comment
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_rsx_token(token, false));
            }

            // Start the comment span
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        // If we're in a comment and hit a newline, close the comment span
        if in_comment && chars[i] == '\n' {
            result.push_str(&code[token_start..=i]);
            result.push_str("</span>");
            token_start = i + 1;
            in_comment = false;
            continue;
        }

        // If we're in a comment, continue to next character
        if in_comment {
            continue;
        }

        // Handle string literals
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            if !in_string {
                // Start of string
                if token_start < i {
                    let token = &code[token_start..i];
                    result.push_str(&highlight_rsx_token(token, false));
                }
                result.push_str("<span class='text-green-400'>\"");
                token_start = i + 1;
                in_string = true;
            } else {
                // End of string
                result.push_str(&code[token_start..i]);
                result.push_str("\"</span>");
                token_start = i + 1;
                in_string = false;
            }
            continue;
        }

        // If we're in a string, continue to next character
        if in_string {
            continue;
        }

        // Handle whitespace and separators
        if chars[i].is_whitespace()
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '('
            || chars[i] == ')'
            || chars[i] == ':'
            || chars[i] == ','
        {
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_rsx_token(token, false));
            }

            // Add the separator character with special coloring for braces
            if chars[i] == '{' || chars[i] == '}' {
                result.push_str(&format!(
                    "<span class='text-yellow-500'>{}</span>",
                    chars[i]
                ));
            } else {
                result.push(chars[i]);
            }
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let token = &code[token_start..];
        if in_string {
            result.push_str(token);
        } else if in_comment {
            result.push_str(token);
            result.push_str("</span>");
        } else {
            result.push_str(&highlight_rsx_token(token, false));
        }
    }

    result
}

/// Highlights an RSX token by applying appropriate styling.
fn highlight_rsx_token(token: &str, in_string: bool) -> String {
    if in_string {
        return token.to_string();
    }

    // Clean the token
    let clean_token = token.trim();

    if clean_token.is_empty() {
        return token.to_string();
    }

    // Handle RSX keywords
    let keywords = [
        "rsx", "div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6", "a", "button", "input",
        "textarea", "form", "img", "nav", "footer", "header", "main", "section", "article",
    ];

    if keywords.contains(&clean_token) {
        return format!("<span class='text-blue-400'>{}</span>", token);
    }

    // Handle attributes
    if clean_token.ends_with(':') {
        return format!("<span class='text-purple-400'>{}</span>", token);
    }

    // Handle class and style attributes
    if clean_token == "class" || clean_token == "style" {
        return format!("<span class='text-purple-400'>{}</span>", token);
    }

    // Handle numbers
    if clean_token.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    token.to_string()
}

#[component]
/// Renders a syntax-highlighted code block as a Dioxus component.
///
/// This component applies syntax highlighting to the provided code snippet based on the specified language.
/// Currently supports "html" and "rsx" languages.
///
/// # Arguments
///
/// * `code` - The code snippet to highlight.
/// * `language` - The language identifier (e.g., "html", "rsx"). This value is case-insensitive.
pub fn CodeBlock(code: String, language: String) -> Element {
    let highlighted = match language.to_lowercase().as_str() {
        "html" => highlight_html_syntax(&code),
        "rsx" => highlight_rsx_syntax(&code),
        _ => code.clone(),
    };

    rsx! {
        pre {
            class: format!(
                "language-{} overflow-x-auto rounded-lg bg-dark-300/50 p-4 font-mono text-sm",
                language,
            ),
            style: "white-space: pre;", // Ensure whitespace is preserved
            dangerous_inner_html: "{highlighted}",
        }
    }
}
