extern crate sciter;
// https://github.com/sciter-sdk/rust-sciter
fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("../common-ui/index.html");
    frame.run_app();
}




