extern crate serde_json;
extern crate stammw_blog;

// use serde;
use stammw_blog::markdown::MarkdownText;

#[test]
fn serializes_to_html() {
    let md_text = MarkdownText("# Hello\n\
                   I am some Markdown content `inline`\n\
                   ```\n\
                   code\n\
                   ```\n".to_string());

    let serialized = serde_json::to_string(&md_text).unwrap();
    assert_eq!(serialized.to_string(), "\"<h1>Hello</h1>\\n\
                            <p>I am some Markdown content <code>inline</code></p>\\n\
                            <pre><code>code\\n\
                            </code></pre>\\n\"");
}
