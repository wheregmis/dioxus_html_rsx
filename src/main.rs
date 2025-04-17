use dioxus::prelude::*;
use dioxus_rsx_rosetta::Dom;
use std::borrow::Cow;

mod syntax_highlight;
use syntax_highlight::CodeBlock;

/// Copy text to clipboard (web only)
#[cfg(feature = "web")]
fn to_clipboard(text: &str) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    // Create a temporary textarea element
    let textarea: web_sys::HtmlTextAreaElement = document
        .create_element("textarea")
        .expect("failed to create textarea")
        .dyn_into()
        .expect("failed to convert to textarea");

    // Set its value to the text we want to copy
    textarea.set_value(text);

    // Add it to the DOM
    let body = document.body().expect("no body");
    let _ = body.append_child(&textarea);

    // Select the text
    textarea.select();

    // Execute the copy command using document.execCommand
    let _ = js_sys::Reflect::get(&document, &wasm_bindgen::JsValue::from_str("execCommand"))
        .expect("execCommand not found")
        .dyn_ref::<js_sys::Function>()
        .expect("execCommand is not a function")
        .call1(&document, &wasm_bindgen::JsValue::from_str("copy"))
        .expect("execCommand failed");

    // Remove the textarea
    let _ = body.remove_child(&textarea);
}

#[cfg(not(feature = "web"))]
fn to_clipboard(_text: &str) {
    // Do nothing on non-web platforms
}

/// Preprocesses HTML to convert React-style attributes to Dioxus RSX format and normalize whitespace
/// - Converts `className` to `class`
/// - Normalizes whitespace in text content while preserving attribute spacing
fn preprocess_html(html: &str) -> Cow<str> {
    // Start with a mutable string for multiple operations
    let mut processed = html.to_string();

    // Replace className with class - handle both quoted and unquoted attributes
    if html.contains("className") {
        // Handle className="value" (double quotes)
        processed = processed.replace("className=\"", "class=\"");

        // Handle className='value' (single quotes)
        processed = processed.replace("className='", "class='");

        // Handle className=value (no quotes)
        processed = processed.replace("className=", "class=");
    }

    // Only normalize whitespace in text content, not in attributes
    // We'll parse the HTML more carefully to preserve attribute spacing
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_quotes = false;
    let mut quote_char = '"'; // Default quote character
    let mut last_char = ' '; // Initialize with space

    for c in processed.chars() {
        // Track if we're inside a tag
        if c == '<' {
            in_tag = true;
            result.push(c);
        }
        // Track if we're at the end of a tag
        else if c == '>' {
            in_tag = false;
            result.push(c);
        }
        // Handle quotes - track if we're inside quoted attribute values
        else if (c == '"' || c == '\'') && in_tag {
            if !in_quotes {
                // Starting quotes
                in_quotes = true;
                quote_char = c;
            } else if c == quote_char {
                // Ending quotes (matching the opening quote type)
                in_quotes = false;
            }
            result.push(c);
        }
        // Handle whitespace
        else if c.is_whitespace() {
            // Inside a tag or quotes, preserve all whitespace for attributes
            if in_tag || in_quotes {
                result.push(c);
            }
            // In text content, normalize whitespace
            else {
                // Only add a space if the previous character wasn't whitespace
                if !last_char.is_whitespace() {
                    result.push(' ');
                }
            }
        }
        // All other characters
        else {
            result.push(c);
        }

        last_char = c;
    }

    // Final cleanup - remove spaces immediately after '>' and before '<'
    let mut final_result = String::new();
    let mut last_was_tag_end = false;
    let mut chars = result.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '>' {
            last_was_tag_end = true;
            final_result.push(c);
        } else if c == '<' {
            final_result.push(c);
            last_was_tag_end = false;
        } else if c.is_whitespace() {
            // Skip space after '>' or before '<'
            if last_was_tag_end {
                // Check if next char is '<'
                if let Some(&next) = chars.peek() {
                    if next != '<' {
                        final_result.push(c);
                    }
                }
            } else {
                final_result.push(c);
            }
        } else {
            final_result.push(c);
            last_was_tag_end = false;
        }
    }

    Cow::Owned(final_result)
}

fn main() {
    // Launch the app
    dioxus::launch(app);
}

fn app() -> Element {
    let mut html_input = use_signal(String::new);
    let mut rsx_output = use_signal(|| "Your Generated RSX".to_string());
    let mut copied = use_signal(|| false);

    // Add CSS for syntax highlighting
    let css = r#"/* Tailwind-like utility classes for syntax highlighting */
.text-blue-400 { color: #60a5fa; }
.text-blue-500 { color: #3b82f6; }
.text-blue-300 { color: #93c5fd; }
.text-green-400 { color: #4ade80; }
.text-green-500 { color: #22c55e; }
.text-green-300 { color: #86efac; }
.text-purple-400 { color: #c084fc; }
.text-purple-500 { color: #a855f7; }
.text-orange-400 { color: #fb923c; }
.text-yellow-500 { color: #eab308; }
.text-gray-500 { color: #9ca3af; }
.text-white { color: #ffffff; }

/* Code block styling */
.language-html, .language-rsx {
    background-color: transparent;
    color: #d4d4d4;
    border-radius: 0.375rem;
    overflow-x: auto;
    font-family: monospace;
    line-height: 1.5;
    font-size: 0.875rem;
}

pre {
    margin: 0;
    padding: 0;
}
"#;

    rsx! {
        style { dangerous_inner_html: "{css}" }
        div { style: "width: 100%; height: 100%; background-color: #1A1A1A; color: #FFFFFF; min-height: 100vh; display: flex; flex-direction: column;",
            // Navbar
            nav { style: "background-color: #222222; padding: 1rem 1.5rem; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #333333; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);",
                // Logo/Title
                div { style: "display: flex; align-items: center;",
                    // Rust-themed logo
                    div { style: "background-color: #CD7F32; width: 2rem; height: 2rem; border-radius: 0.25rem; margin-right: 0.75rem; display: flex; align-items: center; justify-content: center; font-weight: bold; color: #111111;",
                        "R"
                    }
                    h1 { style: "color: #CD7F32; font-family: monospace; margin: 0; font-size: 1.5rem;",
                        "HTML to RSX Converter"
                    }
                }

                // Links
                div { style: "display: flex; gap: 1rem;",
                    a {
                        href: "https://github.com/wheregmis/dioxus_html_rsx",
                        target: "_blank",
                        style: "color: #FFFFFF; text-decoration: none; display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; border-radius: 0.25rem; background-color: #333333; transition: background-color 0.2s ease-in-out; border: 1px solid #444444;",
                        onmouseenter: |_| {},

                        // GitHub icon (simplified SVG as text)
                        span { style: "font-size: 1.2rem;", "🔗" }
                        "GitHub"
                    }
                }
            }

            // Main content
            main { style: "flex: 1; padding: 1.5rem; max-width: 1400px; width: 100%; margin: 0 auto;",
                // Responsive grid - will stack on smaller screens
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(500px, 1fr)); gap: 1.5rem;",

                    // Input section
                    div { style: "background-color: #222222; border: 1px solid #333333; border-radius: 0.5rem; padding: 1.25rem; position: relative; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                        // Rust-themed accent line
                        div { style: "position: absolute; top: 0; left: 0; width: 4px; height: 100%; background: linear-gradient(to bottom, #CD7F32, #FFA07A); border-top-left-radius: 0.5rem; border-bottom-left-radius: 0.5rem;" }

                        h2 { style: "color: #FFFFFF; font-family: monospace; margin-bottom: 0.75rem; display: flex; align-items: center; gap: 0.5rem;",
                            span { style: "color: #CD7F32; font-size: 1.25rem;", "📄" }
                            "HTML Input"
                        }

                        // Simple textarea for HTML input
                        textarea {
                            value: "{html_input}",
                            oninput: move |e| html_input.set(e.value().clone()),
                            placeholder: "Paste your HTML code here...",
                            style: "width: 100%; height: 60vh; padding: 0.75rem; background-color: #333333; color: #FFFFFF; border: 1px solid #444444; border-radius: 0.25rem; font-family: monospace; resize: none; line-height: 1.5; font-size: 0.95rem; transition: border-color 0.2s ease-in-out; outline: none;",
                        }

                        div { style: "display: flex; justify-content: center; margin-top: 0.75rem;",
                            button {
                                onclick: move |_| {
                                    spawn(async move {
                                        let html_value = html_input();
                                        let preprocessed_html = preprocess_html(&html_value);
                                        let dom = Dom::parse(&preprocessed_html).unwrap();
                                        let rsx_response_callbody = dioxus_rsx_rosetta::rsx_from_html(&dom);
                                        let formatted = dioxus_autofmt::write_block_out(&rsx_response_callbody)
                                            .unwrap();
                                        rsx_output.set(formatted);
                                    });
                                },
                                style: "padding: 0.75rem 1.5rem; background-color: #CD7F32; color: #111111; border: none; border-radius: 0.25rem; cursor: pointer; font-weight: bold; display: flex; align-items: center; gap: 0.5rem; transition: background-color 0.2s ease-in-out; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);",

                                // Convert icon
                                span { style: "font-size: 1.2rem;", "⟳" }
                                "Convert to RSX"
                            }
                        }
                    }

                    // Output section
                    div { style: "background-color: #222222; border: 1px solid #333333; border-radius: 0.5rem; padding: 1.25rem; position: relative; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                        // Rust-themed accent line
                        div { style: "position: absolute; top: 0; left: 0; width: 4px; height: 100%; background: linear-gradient(to bottom, #CD7F32, #FFA07A); border-top-left-radius: 0.5rem; border-bottom-left-radius: 0.5rem;" }

                        h2 { style: "color: #FFFFFF; font-family: monospace; margin-bottom: 0.75rem; display: flex; align-items: center; gap: 0.5rem;",
                            span { style: "color: #CD7F32; font-size: 1.25rem;", "📝" }
                            "RSX Output"
                        }

                        div { style: "width: 100%; height: 60vh; padding: 0.5rem; background-color: #1A1A1A; color: #FFFFFF; border: 1px solid #333333; border-radius: 0.25rem; overflow: auto; position: relative;",
                            // Use the CodeBlock component for syntax highlighting
                            CodeBlock {
                                code: rsx_output().to_string(),
                                language: "rsx".to_string(),
                            }

                            // Copy button (positioned in the top-right corner)
                            button {
                                onclick: move |_| {
                                    let output = rsx_output();
                                    to_clipboard(&output);
                                    copied.set(true);
                                    #[cfg(feature = "web")]
                                    {
                                        use wasm_bindgen::prelude::*;
                                        let mut copied_clone = copied.clone();
                                        let closure = Closure::once_into_js(move || {
                                            copied_clone.set(false);
                                        });
                                        let _ = web_sys::window()
                                            .unwrap()
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                closure.as_ref().unchecked_ref(),
                                                2000,
                                            );
                                    }
                                },
                                style: "position: absolute; top: 0.5rem; right: 0.5rem; background-color: #333333; color: #FFFFFF; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; cursor: pointer; display: flex; align-items: center; gap: 0.25rem; font-size: 0.8rem; transition: background-color 0.2s ease-in-out;",

                                // Show different icon/text based on copied state
                                if copied() {
                                    span { style: "font-size: 1rem; color: #4ade80;",
                                        "✔️"
                                    }
                                    "Copied!"
                                } else {
                                    span { style: "font-size: 1rem;", "📋" }
                                    "Copy"
                                }
                            }
                        }
                    }
                }
            } // Close main tag

            footer { style: "margin-top: 2rem; padding: 1.5rem; color: #9CA3AF; text-align: center; border-top: 1px solid #333333; background-color: #222222; box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);",
                div { style: "margin-bottom: 1rem;",
                    "Built with "
                    a {
                        href: "https://dioxuslabs.com",
                        target: "_blank",
                        style: "color: #CD7F32; text-decoration: none; font-weight: bold; transition: color 0.2s ease-in-out;",
                        "Dioxus"
                    }
                    " ❤️ "
                }

                div { style: "font-size: 0.9rem;",
                    "Developed by "
                    a {
                        href: "https://github.com/wheregmis",
                        target: "_blank",
                        style: "color: #CD7F32; text-decoration: none; font-weight: bold; transition: color 0.2s ease-in-out;",
                        "Sabin Regmi (wheregmis)"
                    }
                }
            }
        }
    }
}
