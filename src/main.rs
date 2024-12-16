use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use std::fs;
use warp::Filter;

fn markdown_to_confluence(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut is_list_item = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading(level, _, _) => {
                    // Convert heading levels to `h1.`, `h2.`, etc.
                    let heading_level = match level {
                        HeadingLevel::H1 => "h1.",
                        HeadingLevel::H2 => "h2.",
                        HeadingLevel::H3 => "h3.",
                        HeadingLevel::H4 => "h4.",
                        HeadingLevel::H5 => "h5.",
                        HeadingLevel::H6 => "h6.",
                    };
                    output.push_str(&format!("{}\n", heading_level));
                }
                Tag::Emphasis => output.push('_'),
                Tag::Strong => output.push('*'),
                Tag::Link(_, href, _) => {
                    output.push_str(&format!("[{}", href));
                }
                Tag::Item => {
                    // Start a new list item
                    if !is_list_item {
                        is_list_item = true;
                    }
                    output.push_str("- ");
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                Tag::Heading(_, _, _) => output.push('\n'),
                Tag::Emphasis => output.push('_'),
                Tag::Strong => output.push('*'),
                Tag::Link(_, _, title) => {
                    output.push(']');
                    if !title.is_empty() {
                        output.push_str(&format!(" {}", title));
                    }
                }
                Tag::Item => {
                    // Add a line break after each list item
                    output.push('\n');
                    is_list_item = false;
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
                output.push_str(&format!("`{}`", text));
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
        warp::reply::html(format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Markdown to Confluence</title>
                </head>
                <body>
                    <h1>Markdown to Confluence Wiki Style</h1>
                    <textarea style="width: 100%; height: 80vh;" readonly>{}</textarea>
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
