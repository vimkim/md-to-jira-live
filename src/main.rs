use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use std::fs;
use warp::Filter;

fn markdown_to_confluence(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut list_depth = 0;
    let mut list_stack: Vec<bool> = Vec::new(); // Stack to track list types (true for ordered, false for unordered)

    for event in parser {
        let event = dbg!(event);
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                } => {
                    // Convert the heading level to Confluence format (e.g., `h1.`, `h2.`, etc.)
                    let heading_level = match level {
                        HeadingLevel::H1 => "h1.",
                        HeadingLevel::H2 => "h2.",
                        HeadingLevel::H3 => "h3.",
                        HeadingLevel::H4 => "h4.",
                        HeadingLevel::H5 => "h5.",
                        HeadingLevel::H6 => "h6.",
                    };

                    // Start the heading in Confluence format
                    output.push_str(&format!("\n\n{} ", heading_level));

                    // Optionally include `id` if needed
                    if let Some(id_value) = id {
                        output.push_str(&format!("[ID: {}] ", id_value));
                    }

                    // Optionally include `classes` if needed
                    if !classes.is_empty() {
                        output.push_str(&format!("[Classes: {:?}] ", classes));
                    }

                    // Optionally include attributes if needed
                    if !attrs.is_empty() {
                        output.push_str(&format!("[Attrs: {:?}] ", attrs));
                    }
                }
                Tag::Emphasis => output.push('_'),
                Tag::Strong => output.push('*'),
                Tag::List(Some(_)) => {
                    // Ordered list: push `true` to stack
                    list_stack.push(true);
                    list_depth += 1;
                }
                Tag::List(None) => {
                    // Unordered list: push `false` to stack
                    list_stack.push(false);
                    list_depth += 1;
                }
                Tag::Item => {
                    // Use the last value in the stack to determine list type
                    if let Some(&is_ordered_list) = list_stack.last() {
                        if is_ordered_list {
                            output.push_str(&format!("\n{} ", "#".repeat(list_depth)));
                        } else {
                            output.push_str(&format!("\n{} ", "*".repeat(list_depth)));
                        }
                    }
                }
                Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                    output.push_str(&format!("\n{{code:{}}}\n", lang));
                }
                Tag::CodeBlock(CodeBlockKind::Indented) => {
                    // Start a generic code block for indented code
                    output.push_str("\n{code}\n");
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => output.push('\n'),
                TagEnd::Emphasis => output.push('_'),
                TagEnd::Strong => output.push('*'),
                TagEnd::List(_) => {
                    // Pop the stack to restore the previous list type
                    list_stack.pop();
                    list_depth -= 1;
                    if list_depth == 0 {
                        output.push('\n');
                    }
                }
                TagEnd::Item => {
                    // Add a line break after each list item
                }
                _ => {}
            },
            Event::Text(text) => {
                // Add text content
                output.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                // Add a line break
                output.push('\n');
            }
            Event::Code(text) => {
                // Inline code
                output.push_str(&format!(" {{{{{}}}}} ", text));
            }
            Event::Html(html) => {
                // Add raw HTML content
                output.push_str(&format!("{{html}}{}{{html}}", html));
            }
            _ => {}
        }
    }

    output
}

#[tokio::main]
async fn main() {
    // Define a warp route to serve the converted content
    let markdown_route = warp::path::end().and(warp::get()).map(|| {
        // Read `main.md`
        let markdown_content = fs::read_to_string("main.md").unwrap_or_else(|_| {
            "# Error\nCould not read `main.md`. Make sure the file exists.".to_string()
        });

        // Convert Markdown to Confluence Wiki format
        let confluence_content = markdown_to_confluence(&markdown_content);

        // Serve the result in an HTML textarea
        // Serve the result in an HTML textarea
        warp::reply::html(format!(
            r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Markdown to Confluence</title>
        <style>
            * {{
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }}
            body {{
                font-family: Arial, sans-serif;
                line-height: 1.6;
                background-color: #f4f4f9;
                color: #333;
                padding: 20px;
            }}
            h1 {{
                text-align: center;
                color: #444;
                margin-bottom: 20px;
            }}
            textarea {{
                width: 100%;
                height: 100vh;
                border: 1px solid #ccc;
                border-radius: 8px;
                padding: 10px;
                font-family: "Courier New", Courier, monospace;
                font-size: 14px;
                background-color: #fff;
                color: #333;
                resize: none;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            }}
            textarea:focus {{
                outline: none;
                border-color: #007bff;
                box-shadow: 0 0 8px rgba(0, 123, 255, 0.25);
            }}
        </style>
    </head>
    <body>
        <h1>Markdown to CUBRID Jira Confluence Wiki Style</h1>
        <textarea readonly>{}</textarea>
    </body>
    </html>
    "#,
            confluence_content
        ))
    });

    // Start the server
    let port = 3030;
    println!("Server running at http://localhost:{}/", port);
    warp::serve(markdown_route)
        .run(([127, 0, 0, 1], port))
        .await;
}
