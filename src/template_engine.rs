use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    name: String,
    phone: String
}

static TEMPLATE : &'static str = "Hello {name}, We offer your number {phone} 50% percent discount";

pub fn create_message(name: &String, phone: &String) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("offer", TEMPLATE).expect("Error in adding the template");

    let context = Context {
        name: name.to_string(),
        phone: phone.to_string()
    };

    let rendered = tt.render("offer", &context).expect("Error in rendering the message");
    rendered
}