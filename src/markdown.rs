use pulldown_cmark::{html, Parser};
use serde::ser::{Serialize, Serializer};
use rocket::request::{FromFormValue};
use rocket::http::RawStr;

pub struct MarkdownText(pub String);

impl Serialize for MarkdownText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let parser = Parser::new(&self.0.as_str());
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);
        println!("formated : {}", html_buf);
        serializer.serialize_str(html_buf.as_str())
    }
}

impl<'v> FromFormValue<'v> for MarkdownText {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<MarkdownText, &'v RawStr> {
        Parser::new(&form_value.as_str()).for_each(|item| println!("parsed: {:?}", item));
        Ok(MarkdownText(form_value.to_string()))
    }
}
