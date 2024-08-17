use webserver::App;

fn main() {
    let app = App::new();

    // app.get("/", Box::new(|req| {
    //     print!("Incoming request: {req:?}");
    // }));

    app.listen("0.0.0.0:8080").unwrap();
}
