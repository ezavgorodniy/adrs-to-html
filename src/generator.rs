use markdown::{mdast, to_mdast, ParseOptions};
use crate::files;

const LIST_ADRS_PLACEHOLDER: &str = "{{LIST_ADRS}}";
const ADR_PLACEHOLDER: &str = "{{ADR_CONTENT}}";

pub fn process_files(files: &Vec<files::File>, template: &str) -> Vec<files::File> {
    let list_adrs = generate_list_adrs(files);
    files.iter().map(|file| {
        let html_content = generate_adr_html(&file.content).unwrap();
        files::File {
            name: file.name.replace(".md", ".html"),
            content: template.replace(ADR_PLACEHOLDER, &html_content).replace(LIST_ADRS_PLACEHOLDER, &list_adrs),
        }
    }).collect()
}

fn generate_list_adrs(adrs: &Vec<files::File>) -> String {
    let mut result = String::from("<ul>");
    adrs.iter().for_each(|adr| {
        if adr.name == "index.md" {
            return;
        }
        result.push_str(&format!("<li><a href=\"{}\">{}</a></li>", adr.name.replace(".md", ".html"), adr.name.replace(".md", "")));
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
        },
        Err(_err) => {
            return Err("Error while parsing md file")
        }
    }

    Ok(result)
}

fn process_node(result: &mut String, node: &mdast::Node) {
    node.children().iter().for_each(|child| {
        child.as_slice().iter().for_each(|internal_child| {
            match internal_child {
                mdast::Node::Text(text) => {
                    result.push_str(&text.value);
                },
                mdast::Node::Heading(heading) => {
                    result.push_str(&format!("<h{}>", heading.depth));
                    process_node(result, internal_child);
                    result.push_str(&format!("</h{}>", heading.depth));
                },
                mdast::Node::Link(link) => {
                    result.push_str(&format!("<a href=\"{}\">", link.url));
                    process_node(result, internal_child);
                    result.push_str(&format!("</a>"));
                },
                mdast::Node::Code(code) => {
                    result.push_str(&format!("<code>"));
                    result.push_str(&code.value);
                    result.push_str(&format!("</code>"));
                },
                mdast::Node::List(_) => {
                    result.push_str(&format!("<ul>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</ul>"));
                },
                mdast::Node::ListItem(_) => {
                    result.push_str(&format!("<li>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</li>"));
                },
                mdast::Node::Paragraph(_) => {
                    result.push_str(&format!("<p>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</p>"));
                },
                mdast::Node::Emphasis(_) => {
                    result.push_str(&format!("<span class=\"fst-italic\">"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</span>"));
                },
                mdast::Node::Strong(_) => {
                    result.push_str(&format!("<strong>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</strong>"));
                },
                _ => {
                    result.push_str(&format!("<pre>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</pre>"));
                },
            }
        });
    });
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
        assert_eq!(result.unwrap(), String::from("<p><a href=\"https://test.com\">Test</a></p>"))
    }

    #[test]
    fn generate_adr_html_code_example_should_return_code() {
        let result = generate_adr_html(&String::from("```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<code>fn main() {\n    println!(\"Hello, world!\");\n}</code>"))
    }

    #[test]
    fn generate_adr_html_list_should_return_bullet_list() {
        let result = generate_adr_html(&String::from("- Item1
        - Item2"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<ul><li><p>Item1\n- Item2</p></li></ul>"))
    }

    #[test]
    fn generate_adr_html_paragraphs_should_return_paragraph_tag() {
        let result = generate_adr_html(&String::from("First Paragraph\n\nSecond Paragraph"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<p>First Paragraph</p><p>Second Paragraph</p>"))
    }

    #[test]
    fn generate_adr_html_italic_should_return_italic() {
        let result = generate_adr_html(&String::from("*italic*"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<p><span class=\"fst-italic\">italic</span></p>"))
    }

    #[test]
    fn generate_adr_html_italic_should_return_bold() {
        let result = generate_adr_html(&String::from("**bold**"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<p><strong>bold</strong></p>"))
    }

    #[test]
    fn generate_adr_html_italic_bold_should_return_bold_italic_span() {
        let result = generate_adr_html(&String::from("***bold***"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<p><span class=\"fst-italic\"><strong>bold</strong></span></p>"))
    }

    #[test]
    fn generate_list_adrs_should_return_ul_list() {
        let files = vec![
            files::File { name: String::from("test.md"), content: String::from("") },
        ];
        let result = generate_list_adrs(&files);
        assert_eq!(result, String::from("<ul><li><a href=\"test.html\">test</a></li></ul>"))
    }
    #[test]
    fn generate_list_adrs_should_ignore_index_md() {
        let files = vec![
            files::File { name: String::from("index.md"), content: String::from("") },
            files::File { name: String::from("test.md"), content: String::from("") },
        ];
        let result = generate_list_adrs(&files);
        assert_eq!(result, String::from("<ul><li><a href=\"test.html\">test</a></li></ul>"))
    }
}
