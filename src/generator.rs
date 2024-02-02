use markdown::{mdast, to_mdast, ParseOptions};

pub fn generate_adr_html(md: &str) -> Result<String, &'static str> {
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
        assert_eq!(result.unwrap(), String::from("<pre><a href=\"https://test.com\">Test</a></pre>"))
    }

    #[test]
    fn generate_adr_html_code_example_should_return_code() {
        let result = generate_adr_html(&String::from("```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<code>fn main() {\n    println!(\"Hello, world!\");\n}</code>"))
    }
}
