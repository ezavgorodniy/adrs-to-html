use crate::files;
use markdown::{mdast, to_mdast, ParseOptions};

const LIST_ADRS_PLACEHOLDER: &str = "{{LIST_ADRS}}";
const ADR_PLACEHOLDER: &str = "{{ADR_CONTENT}}";


struct ParsedFile {
    name: String,
    content: String,
    status: String,
    date: String,
}

pub fn process_files(files: &Vec<files::File>, template: &str) -> Vec<files::File> {
    let parsed_files = parse_files(files, template);

    let list_adrs = generate_list_adrs(&parsed_files);
    parsed_files
        .iter()
        .map(|file| {
            let html_content = generate_adr_html(&file.content).unwrap();
            files::File {
                name: file.name.replace(".md", ".html"),
                content: template
                    .replace(ADR_PLACEHOLDER, &html_content)
                    .replace(LIST_ADRS_PLACEHOLDER, &list_adrs),
            }
        })
        .collect()
}

fn parse_files(files: &Vec<files::File>, template: &str) -> Vec<ParsedFile> {
    let mut parsed_files: Vec<ParsedFile> = files
        .iter()
        .map(|file| {
            let (status, content) = extract_status_from_adr_content(&file.content);
            let (date, content) = extract_date_from_adr_content(&content);
            ParsedFile {
                name: file.name.replace(".md", ".html"),
                content: content,
                status: status.unwrap_or_else(|| String::from("")),
                date: date.unwrap_or_else(|| String::from("")),
            }
        })
        .collect();
    parsed_files.sort_by(|a, b| a.name.cmp(&b.name));
    parsed_files
}

fn generate_list_adrs(adrs: &Vec<ParsedFile>) -> String {
    let mut result = String::from("<ul>");
    adrs.iter().for_each(|adr| {
        if adr.name == "index.md" {
            return;
        }
        result.push_str(&format!(
            "<li><a href=\"{}\">[{}]{}</a></li>",
            adr.name.replace(".md", ".html"),
            adr.status,
            adr.name.replace(".md", "")
        ));
    });
    result.push_str("</ul>");
    result
}

fn generate_adr_html(md: &str) -> Result<String, &'static str> {
    let mdast = to_mdast(md, &ParseOptions::default());

    let mut result = String::from("");
    match mdast {
        Ok(val) => {
            process_node(&mut result, &val);
        }
        Err(_err) => return Err("Error while parsing md file"),
    }

    Ok(result)
}

fn process_node(result: &mut String, node: &mdast::Node) {
    node.children().iter().for_each(|child| {
        child
            .as_slice()
            .iter()
            .for_each(|internal_child| match internal_child {
                mdast::Node::Text(text) => {
                    result.push_str(&text.value);
                }
                mdast::Node::Heading(heading) => {
                    result.push_str(&format!("<h{}>", heading.depth));
                    process_node(result, internal_child);
                    result.push_str(&format!("</h{}>", heading.depth));
                }
                mdast::Node::Link(link) => {
                    result.push_str(&format!("<a href=\"{}\">", link.url));
                    process_node(result, internal_child);
                    result.push_str(&format!("</a>"));
                }
                mdast::Node::Code(code) => {
                    result.push_str(&format!("<code>"));
                    result.push_str(&code.value);
                    result.push_str(&format!("</code>"));
                }
                mdast::Node::List(_) => {
                    result.push_str(&format!("<ul>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</ul>"));
                }
                mdast::Node::ListItem(_) => {
                    result.push_str(&format!("<li>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</li>"));
                }
                mdast::Node::Paragraph(_) => {
                    result.push_str(&format!("<p>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</p>"));
                }
                mdast::Node::Emphasis(_) => {
                    result.push_str(&format!("<span class=\"fst-italic\">"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</span>"));
                }
                mdast::Node::Strong(_) => {
                    result.push_str(&format!("<strong>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</strong>"));
                }
                mdast::Node::BlockQuote(_) => {
                    result.push_str(&format!("<blockquote class=\"blockquote\">"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</blockquote>"));
                }
                mdast::Node::ThematicBreak(_) => {
                    result.push_str(&format!("<hr>"));
                }
                _ => {
                    result.push_str(&format!("<pre>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</pre>"));
                }
            });
    });
}

fn extract_status_from_adr_content(content: &str) -> (Option<String>, String) {
    let mut modified_content = content.to_string();
    let re = regex::Regex::new(r"(?i)status:\s*(.*)").unwrap();
    if let Some(captures) = re.captures(&modified_content) {
        if let Some(status_value) = captures.get(1) {
            let status_value_str = status_value.as_str().trim().to_string();
            let escaped_status_value = regex::escape(&status_value_str);
            let pattern = regex::Regex::new(&format!(r#"(?i)status:\s*{}"#, escaped_status_value)).unwrap();
            modified_content = pattern.replace(&modified_content, "").trim().to_string();
            return (Some(status_value_str), modified_content);
        }
    }
    (None, modified_content)
}

fn extract_date_from_adr_content(content: &str) -> (Option<String>, String) {
    let mut modified_content = content.to_string();
    let re = regex::Regex::new(r"(?i)date:\s*(.*)").unwrap();
    if let Some(captures) = re.captures(&modified_content) {
        if let Some(date_value) = captures.get(1) {
            let date_value_str = date_value.as_str().trim().to_string();
            let escaped_status_value = regex::escape(&date_value_str);
            let pattern = regex::Regex::new(&format!(r#"(?i)date:\s*{}"#, escaped_status_value)).unwrap();
            modified_content = pattern.replace(&modified_content, "").trim().to_string();
            return (Some(date_value_str), modified_content);
        }
    }
    (None, modified_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_adr_html_heading_first_level_should_return_h1() {
        let result = generate_adr_html(&String::from("# Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h1>Test</h1>"))
    }

    #[test]
    fn generate_adr_html_heading_second_level_should_return_h2() {
        let result = generate_adr_html(&String::from("## Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h2>Test</h2>"))
    }

    #[test]
    fn generate_adr_html_heading_third_level_should_return_h3() {
        let result = generate_adr_html(&String::from("### Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h3>Test</h3>"))
    }

    #[test]
    fn generate_adr_html_heading_fourth_level_should_return_h4() {
        let result = generate_adr_html(&String::from("#### Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h4>Test</h4>"))
    }

    #[test]
    fn generate_adr_html_heading_fifth_level_should_return_h5() {
        let result = generate_adr_html(&String::from("##### Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h5>Test</h5>"))
    }

    #[test]
    fn generate_adr_html_heading_sixth_level_should_return_h6() {
        let result = generate_adr_html(&String::from("###### Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h6>Test</h6>"))
    }

    #[test]
    fn generate_adr_html_link_should_return_anchor_tagged_text() {
        let result = generate_adr_html(&String::from("[Test](https://test.com)"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<p><a href=\"https://test.com\">Test</a></p>")
        )
    }

    #[test]
    fn generate_adr_html_code_example_should_return_code() {
        let result = generate_adr_html(&String::from(
            "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```",
        ));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<code>fn main() {\n    println!(\"Hello, world!\");\n}</code>")
        )
    }

    #[test]
    fn generate_adr_html_list_should_return_bullet_list() {
        let result = generate_adr_html(&String::from(
            "- Item1
        - Item2",
        ));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<ul><li><p>Item1\n- Item2</p></li></ul>")
        )
    }

    #[test]
    fn generate_adr_html_paragraphs_should_return_paragraph_tag() {
        let result = generate_adr_html(&String::from("First Paragraph\n\nSecond Paragraph"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<p>First Paragraph</p><p>Second Paragraph</p>")
        )
    }

    #[test]
    fn generate_adr_html_italic_should_return_italic() {
        let result = generate_adr_html(&String::from("*italic*"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<p><span class=\"fst-italic\">italic</span></p>")
        )
    }

    #[test]
    fn generate_adr_html_italic_should_return_bold() {
        let result = generate_adr_html(&String::from("**bold**"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<p><strong>bold</strong></p>")
        )
    }

    #[test]
    fn generate_adr_html_italic_bold_should_return_bold_italic_span() {
        let result = generate_adr_html(&String::from("***bold***"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<p><span class=\"fst-italic\"><strong>bold</strong></span></p>")
        )
    }

    #[test]
    fn generate_adr_html_blockquote_should_return_blockquote() {
        let result = generate_adr_html(&String::from("> the quote"));
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            String::from("<blockquote class=\"blockquote\"><p>the quote</p></blockquote>")
        )
    }

    #[test]
    fn generate_adr_html_three_underscores_should_return_hr_tag() {
        let result = generate_adr_html(&String::from("___"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<hr>"))
    }

    #[test]
    fn generate_adr_html_three_dashes_should_return_hr_tag() {
        let result = generate_adr_html(&String::from("---"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<hr>"))
    }

    #[test]
    fn generate_adr_html_three_asteriks_should_return_hr_tag() {
        let result = generate_adr_html(&String::from("***"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<hr>"))
    }

    #[test]
    fn generate_list_adrs_should_return_ul_list() {
        let files = vec![ParsedFile {
            name: String::from("test.md"),
            content: String::from(""),
            status: String::from("Accepted"),
            date: String::from("26-03-2024"),
        }];
        let result = generate_list_adrs(&files);
        assert_eq!(
            result,
            String::from("<ul><li><a href=\"test.html\">[Accepted]test</a></li></ul>")
        )
    }

    #[test]
    fn generate_list_adrs_should_return_ul_list_sorted_by_name() {
        let files = vec![
            ParsedFile {
                name: String::from("a.md"),
                content: String::from(""),
                status: String::from("Accepted"),
                date: String::from("26-03-2024"),
            },
            ParsedFile {
                name: String::from("b.md"),
                content: String::from(""),
                status: String::from("Superseded"),
                date: String::from("26-03-2024"),
            },
            ParsedFile {
                name: String::from("c.md"),
                content: String::from(""),
                status: String::from("Rejected"),
                date: String::from("26-03-2024"),
            },
        ];
        let result = generate_list_adrs(&files);
        assert_eq!(result, String::from("<ul><li><a href=\"a.html\">[Accepted]a</a></li><li><a href=\"b.html\">[Superseded]b</a></li><li><a href=\"c.html\">[Rejected]c</a></li></ul>"))
    }

    #[test]
    fn generate_list_adrs_should_ignore_index_md() {
        let files = vec![
            ParsedFile {
                name: String::from("index.md"),
                content: String::from(""),
                status: String::from("Accepted"),
                date: String::from("26-03-2024"),
            },
            ParsedFile {
                name: String::from("test.md"),
                content: String::from(""),
                status: String::from("Accepted"),
                date: String::from("26-03-2024"),
            },
        ];
        let result = generate_list_adrs(&files);
        assert_eq!(
            result,
            String::from("<ul><li><a href=\"test.html\">[Accepted]test</a></li></ul>")
        )
    }

    #[test]
    fn extract_status_from_content() {
        let content = "some values status:     Accepted";
        let (status, modified_content) = extract_status_from_adr_content(content);
        assert_eq!(status, Some(String::from("Accepted")));
        assert_eq!(modified_content, "some values");
    }

    #[test]
    fn extract_status_from_content_case_insensitive() {
        let content = "some values StaTus:     Accepted";
        let (status, modified_content) = extract_status_from_adr_content(content);
        assert_eq!(status, Some(String::from("Accepted")));
        assert_eq!(modified_content, "some values");
    }
    
    #[test]
    fn extract_status_from_content_expect_escape_characters() {
        // inspired by adr 007 from TM CSRE
        let content = "some values Status: superseded by [0015-slo-as-code-usage-revised](0015-slo-as-code-usage-revised)";
        let (status, modified_content) = extract_status_from_adr_content(content);
        assert_eq!(status, Some(String::from("superseded by [0015-slo-as-code-usage-revised](0015-slo-as-code-usage-revised)")));
        assert_eq!(modified_content, "some values");
    }

    #[test]
    fn extract_date_from_content() {
        let content = "some values date: 26-03-2024";
        let (status, modified_content) = extract_date_from_adr_content(content);
        assert_eq!(status, Some(String::from("26-03-2024")));
        assert_eq!(modified_content, "some values");
    }

    #[test]
    fn extract_date_from_content_case_insensitive() {
        let content = "some values Date:     26-03-2024";
        let (status, modified_content) = extract_date_from_adr_content(content);
        assert_eq!(status, Some(String::from("26-03-2024")));
        assert_eq!(modified_content, "some values");
    }
}
