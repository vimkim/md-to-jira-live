use std::net::Ipv4Addr;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd, html};

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
                Tag::Emphasis => output.push_str(" _"),
                Tag::Strong => output.push_str(" *"),
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
                TagEnd::Emphasis => output.push_str("_ "),
                TagEnd::Strong => output.push_str("* "),
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
                output.push_str(&format!("' {{{{{}}}}} '", text));
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
    use clap::{Arg, Command};
    use std::fs;
    use warp::Filter;

    // Parse command-line arguments
    let matches = Command::new("Markdown Server")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Serves rendered Markdown and Confluence content over HTTP")
        .arg(
            Arg::new("markdown")
                .short('m')
                .long("markdown")
                .value_name("MARKDOWN")
                .required(true)
                .help("Path to the Markdown file to render"),
        )
        .arg(
            Arg::new("html")
                .short('t')
                .long("template")
                .value_name("HTML_TEMPLATE")
                .default_value("index.html")
                .help("Path to the HTML template file"),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .default_value("127.0.0.1")
                .help("Server address to bind to"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .default_value("3030")
                .help("Port to run the server on"),
        )
        .get_matches();

    // Retrieve and define variables
    // Parse command-line arguments
    let address: Ipv4Addr = matches
        .get_one::<String>("address")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let port: u16 = matches.get_one::<String>("port").unwrap().parse().unwrap();

    use std::sync::Arc;

    // Parse command-line arguments
    let markdown_path = Arc::new(matches.get_one::<String>("markdown").unwrap().clone());
    let html_template_path = Arc::new(matches.get_one::<String>("html").unwrap().clone());

    // Define the route
    let markdown_route = warp::path::end().and(warp::get()).map({
        let markdown_path = Arc::clone(&markdown_path);
        let html_template_path = Arc::clone(&html_template_path);
        move || {
            // Read Markdown and HTML
            let markdown_content = fs::read_to_string(&*markdown_path).unwrap_or_else(|_| {
                "# Error\nCould not read the specified Markdown file.".to_string()
            });
            let rendered_html = markdown_to_html(&markdown_content);
            let confluence_content = markdown_to_confluence(&markdown_content);

            // Read the external HTML file
            let mut html_template = fs::read_to_string(&*html_template_path).unwrap_or_else(|_| {
                "Error: Could not read the specified HTML template file.".to_string()
            });

            // Replace placeholders with dynamic content
            html_template = html_template
                .replace("{{ rendered_html }}", &rendered_html)
                .replace("{{ confluence_content }}", &confluence_content);

            warp::reply::html(html_template)
        }
    });

    // Start the server
    warp::serve(markdown_route).run((address, port)).await;
}
