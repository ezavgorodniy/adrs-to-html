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
                mdast::Node::Paragraph(_) => {
                    result.push_str("<p>");
                    process_node(result, internal_child);
                    result.push_str("</p>");
                },
                mdast::Node::Heading(heading) => {
                    result.push_str(&format!("<h{}>", heading.depth));
                    process_node(result, internal_child);
                    result.push_str(&format!("</h{}>", heading.depth));
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
                mdast::Node::Code(_) => {
                    result.push_str(&format!("<code>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</code>"));
                },
                mdast::Node::Link(link) => {
                    result.push_str(&format!("<a href=\"{}\">", link.url));
                    process_node(result, internal_child);
                    result.push_str(&format!("</a>"));
                },
                mdast::Node::Image(image) => {
                    result.push_str(&format!("<img src=\"{}\">", image.url));
                    process_node(result, internal_child);
                },
                mdast::Node::Emphasis(_) => {
                    result.push_str(&format!("<em>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</em>"));
                },
                mdast::Node::Strong(_) => {
                    result.push_str(&format!("<strong>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</strong>"));
                },
                mdast::Node::ThematicBreak(_) => {
                    result.push_str(&format!("<hr>"));
                    process_node(result, internal_child);
                },
                mdast::Node::BlockQuote(_) => {
                    result.push_str(&format!("<blockquote>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</blockquote>"));
                },
                mdast::Node::Html(html) => {
                    result.push_str(&format!("{}", html.value));
                    process_node(result, internal_child);
                },
                mdast::Node::FootnoteReference(_) => {
                    result.push_str(&format!("<sup>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</sup>"));
                },
                mdast::Node::FootnoteDefinition(_) => {
                    result.push_str(&format!("<sup>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</sup>"));
                },
                mdast::Node::Table(_) => {
                    result.push_str(&format!("<table>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</table>"));
                },
                mdast::Node::TableRow(_) => {
                    result.push_str(&format!("<tr>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</tr>"));
                },
                mdast::Node::TableCell(_) => {
                    result.push_str(&format!("<td>"));
                    process_node(result, internal_child);
                    result.push_str(&format!("</td>"));
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
    fn generate_adr_html_heading_sixth_level_should_return_h4() {
        let result = generate_adr_html(&String::from("###### Test"));
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), String::from("<h6>Test</h6>"))
    }
}
