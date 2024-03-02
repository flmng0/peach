// use peach::prelude::*;

// fn main() {
//     let settings = Settings {
//         title: Some("Example Sketch"),
//         size: [512.0, 512.0].into(),
//         framerate: Some(1),
//         ..Default::default()
//     };

//     peach::run::<Example>(settings).unwrap();
// }

// #[derive(Default)]
// struct Example;

// #[allow(unused_variables)]
// impl Handler for Example {
//     fn setup(sketch: &mut Sketch) -> Self {
//         Self::default()
//     }

//     fn quit(&mut self) {
//         println!("Quitting!");
//     }

//     fn draw(&mut self, sketch: &mut Sketch, gfx: &mut
// Graphics) {         println!("Drew!");
//     }

//     fn key_pressed(&mut self, sketch: &mut Sketch, key:
// Key) {         println!("Key pressed: {:#?}", key);
//     }

//     fn key_released(&mut self, sketch: &mut Sketch, key:
// Key) {         println!("Key released: {:#?}", key);
//     }

//     fn mouse_moved(&mut self, sketch: &mut Sketch,
// position: Point) {         println!("Mouse moved: {:#?}",
// position);     }

//     fn mouse_pressed(&mut self, sketch: &mut Sketch,
// button: MouseButton) {         println!("Mouse button
// pressed: {:#?}", button);     }

//     fn mouse_released(&mut self, sketch: &mut Sketch,
// button: MouseButton) {         println!("Mouse button
// released: {:#?}", button);     }
// }

fn main() {}
