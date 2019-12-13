//! Examples for each different event callback available for
//! a Peach sketch.
//!
//! All event signatures can be found at the module level
//! documentation of `peach::config`.
//!
//! Besides Setup, which only takes a `&mut Sketch`, all
//! events take a `&mut Sketch`, and a `&State`, along with
//! other event information.
use peach::prelude::*;

fn main() {
    peach::run(
        draw,
        // Alternatively, use the with_<event> builder methods on a Config.
        Config::default()
            .with_callbacks(Callbacks {
                setup: Some(setup),
                key_down: Some(key_down),
                key_up: Some(key_up),
                button_down: Some(button_down),
                button_up: Some(button_up),
                mouse_wheel: Some(mouse_wheel),
                mouse_moved: Some(mouse_moved),
                window_moved: Some(window_moved),
                window_resized: Some(window_resized),
            })
            .with_exit_key(Key::Escape)
            .with_resizable(true),
    );
}

fn draw(_sketch: &mut Sketch, _state: &State) {}

// Called at the start of the sketch
fn setup(_sketch: &mut Sketch) {
    println!("Sketch starting");
}

// Called when a key is pressed.
fn key_down(_sketch: &mut Sketch, _state: &State, key: Key) {
    println!("Key down: {:?}", key);
}

// Called when a key is released.
fn key_up(_sketch: &mut Sketch, _state: &State, key: Key) {
    println!("Key up: {:?}", key);
}

// Called when a mouse button is pressed.
fn button_down(_sketch: &mut Sketch, _state: &State, button: Button) {
    println!("Button down: {:?}", button);
}

// Called when a mouse button is released.
fn button_up(_sketch: &mut Sketch, _state: &State, button: Button) {
    println!("Button up: {:?}", button);
}

// Called when the mouse is scrolled up or down, depending
// on the provided delta.
fn mouse_wheel(_sketch: &mut Sketch, _state: &State, delta: f32) {
    println!("Wheel scrolled: {:?}", delta);
}

// Called when the mouse is moved.
fn mouse_moved(_sketch: &mut Sketch, _state: &State, pos: Point) {
    println!("Mouse moved: {:?}", pos);
}

// Called when the window is moved
fn window_moved(_sketch: &mut Sketch, _state: &State, pos: Point) {
    println!("Window moved: {:?}", pos);
}

// Called when the window is reized.
fn window_resized(_sketch: &mut Sketch, _state: &State, size: Size) {
    println!("Window resized: {:?}", size);
}
