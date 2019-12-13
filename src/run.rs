use crate::{config::Config, sketch::Sketch, state::State, Point, Size};

use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Run a sketch, provided a draw callback and a
/// configuration.
///
/// # Usage
/// ```
/// use peach::prelude::*;
///
/// fn main() {
///     peach::run(draw, Config::default());
/// }
///
/// fn draw(sketch: &mut Sketch, state: State) {
///     sketch.clear(0x282A36FF);
///
///     sketch.rotate(state.frame as f32 / 1000.0);
///
///     sketch.anchor(Anchor::Center);
///     sketch.fill(Color::RED);
///     sketch.rect(state.cursor, [100.0, 100.0]);
/// }
/// ```
pub fn run<DrawFn>(draw: DrawFn, config: Config) -> !
where
    DrawFn: 'static + Fn(&mut Sketch, &State),
{
    // Window set up.
    let size = config.size;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(&config.name)
        .with_inner_size(LogicalSize::new(size.width as _, size.height as _))
        .with_resizable(config.resizable)
        .build(&event_loop)
        .unwrap();

    // State set up.
    let mut sketch = Sketch::new(&window);
    let mut state = State::new(size);
    let callbacks = config.callbacks;
    let exit_key = config.exit_key;

    let fps = config
        .framerate
        .map(|framerate| Duration::new(1, 0) / framerate as _);

    sketch.push();
    if let Some(callback) = callbacks.setup {
        callback(&mut sketch);
    }

    // Event loop, doesn't return.
    event_loop.run(move |event, _, control_flow| match event {
        Event::EventsCleared => {
            let should_update = if let Some(fps) = fps {
                Instant::now().duration_since(state.last_frame) > fps
            } else {
                true
            };

            if should_update {
                state.update();

                sketch.push();
                draw(&mut sketch, &state);
                sketch.pop();

                window.request_redraw();
            }
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                let physical = size.to_physical(window.hidpi_factor());
                let size = Size::new(physical.width as _, physical.height as _);

                sketch.resize(size);
                state.resize(size);

                if let Some(callback) = callbacks.window_resized {
                    callback(&mut sketch, &state, size);
                }
            }
            WindowEvent::Moved(position) => {
                let position = Point::new(position.x as _, position.y as _);

                state.window_moved(position);

                if let Some(callback) = callbacks.window_moved {
                    callback(&mut sketch, &state, position);
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::CursorMoved {
                position,
                modifiers,
                ..
            } => {
                let physical = position.to_physical(window.hidpi_factor());
                let position = Point::new(physical.x as _, physical.y as _);

                state.mouse_moved(position);
                state.modifiers = modifiers;

                if let Some(callback) = callbacks.mouse_moved {
                    callback(&mut sketch, &state, position);
                }
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: event_state,
                        virtual_keycode: Some(key),
                        modifiers,
                        ..
                    },
                ..
            } => {
                if let Some(exit_key) = exit_key {
                    if key == exit_key {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                state.modifiers = modifiers;

                let key_callback = match event_state {
                    ElementState::Pressed => callbacks.key_down,
                    ElementState::Released => callbacks.key_up,
                };

                if let Some(callback) = key_callback {
                    callback(&mut sketch, &state, key);
                }
            }
            WindowEvent::RedrawRequested => {
                sketch.finish();
            }
            _ => {}
        },
        _ => {}
    })
}
