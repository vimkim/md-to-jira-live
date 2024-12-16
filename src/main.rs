use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd, html};
use std::fs;
use warp::Filter;

fn markdown_to_confluence(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut list_depth = 0;
    let mut list_stack: Vec<bool> = Vec::new(); // Stack to track list types (true for ordered, false for unordered)

    for event in parser {
        // let event = dbg!(event);
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
                    let l = if lang.as_ref() == "plaintext" {
                        CowStr::from("sh")
                    } else {
                        lang // Use the original CowStr
                    };
                    output.push_str(&format!("\n{{code:language={}}}\n", l));
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
                TagEnd::CodeBlock => {
                    // Write the Confluence code block end marker
                    output.push_str("{code}");
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

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[tokio::main]
async fn main() {
    // Define a warp route to serve the converted content
    let markdown_route = warp::path::end().and(warp::get()).map(|| {
        // Read `main.md`
        let markdown_content = std::fs::read_to_string("main.md").unwrap_or_else(|_| {
            "# Error\nCould not read `main.md`. Make sure the file exists.".to_string()
        });

        // Convert Markdown to Confluence Wiki format
        let confluence_content = markdown_to_confluence(&markdown_content);
        let html_content = markdown_to_html(&markdown_content);

        // Serve the result in an HTML textarea and side-by-side layout
        warp::reply::html(format!(
            r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Markdown to CUBRID Jira Confluence Wiki Style</title>
        <style>
            * {{
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }}
            body {{
                font-family: Arial, sans-serif;
                line-height: 1.6;
                background-color: #f8f9fa;
                color: #333;
                padding: 20px;
            }}
            h1 {{
                text-align: center;
                color: #343a40;
                margin-bottom: 20px;
                font-size: 24px;
            }}
            .container {{
                max-width: 1500px;
                margin: 0 auto;
                display: flex;
                flex-direction: column;
                gap: 20px;
            }}
            .content {{
                display: flex;
                gap: 20px;
                flex-wrap: wrap;
            }}
            .rendered-html, .wiki-content {{
                flex: 1;
                min-width: 48%; /* Ensure proper wrapping for smaller screens */
                height: 70vh;
                padding: 15px;
                background-color: #fff;
                border: 1px solid #ced4da;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                overflow: auto;
            }}
            .rendered-html {{
                font-size: 14px;
            }}
            textarea {{
                width: 100%;
                height: 96%;
                border: none;
                font-family: "Courier New", Courier, monospace;
                font-size: 14px;
                resize: none;
                background-color: #fff;
                color: #495057;
                box-shadow: none;
            }}
            textarea:focus {{
                outline: none;
            }}
            button {{
                align-self: flex-end;
                padding: 10px 20px;
                font-size: 16px;
                font-weight: bold;
                color: #fff;
                background-color: #007bff;
                border: none;
                border-radius: 5px;
                cursor: pointer;
                transition: background-color 0.3s ease;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            }}
            button:hover {{
                background-color: #0056b3;
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>Markdown to Confluence Wiki Style</h1>
            <div class="content">
                <!-- Rendered HTML -->
                <div class="rendered-html">
                    {html_content}
                </div>
                <!-- Confluence Wiki -->
                <div class="wiki-content">
                    <textarea id="confluence-content" readonly>{confluence_content}</textarea>
                </div>
            </div>
            <button id="copy-button">Copy Confluence Wiki</button>
        </div>
        <script>
            document.getElementById('copy-button').addEventListener('click', function () {{
                const textarea = document.getElementById('confluence-content');
                textarea.select();
                textarea.setSelectionRange(0, textarea.value.length); // Ensure selection for mobile devices

                try {{
                    if (document.execCommand('copy')) {{
                        // Show a non-intrusive notification
                        const notification = document.createElement('div');
                        notification.textContent = 'Copied to clipboard!';
                        notification.style.position = 'fixed';
                        notification.style.bottom = '20px';
                        notification.style.right = '20px';
                        notification.style.backgroundColor = '#007bff';
                        notification.style.color = '#fff';
                        notification.style.padding = '10px 15px';
                        notification.style.borderRadius = '5px';
                        notification.style.boxShadow = '0 2px 5px rgba(0, 0, 0, 0.2)';
                        notification.style.fontSize = '14px';
                        notification.style.zIndex = '1000';
                        document.body.appendChild(notification);

                        // Fade out the notification
                        setTimeout(() => {{
                            notification.style.transition = 'opacity 0.5s';
                            notification.style.opacity = '0';
                            setTimeout(() => {{
                                notification.remove();
                            }}, 500); // Allow fade-out to complete
                        }}, 500);
                    }}
                }} catch (err) {{
                    console.error('Copy to clipboard failed', err);
                }}

                // Deselect the text
                textarea.setSelectionRange(0, 0);
                textarea.blur();
            }});
        </script>
    </body>
    </html>
    "#,
            html_content = html_content,
            confluence_content = confluence_content
        ))
    });

    // Start the server
    let port = 3030;
    println!("Server running at http://localhost:{}/", port);
    warp::serve(markdown_route)
        .run(([127, 0, 0, 1], port))
        .await;
}
