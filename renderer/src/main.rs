use window::Window;

mod buffer;
mod window;

fn main() {
    let window = Window::new("Hello, world!");

    window
        .run({
            let mut i = 0;
            move || {
                println!("Hello, world! {}", i);
                i += 1;
            }
        })
        .unwrap();
}
