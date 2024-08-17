use webserver::{App, Handle, Return};

fn main() {
    let mut app = App::new();

    app.listen("0.0.0.0:8080").unwrap();
}
