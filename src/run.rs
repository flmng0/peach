use crate::{
    config::Config,
    sketch::{Sketch, State},
};

use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run<DrawFn>(draw: DrawFn, config: Config) -> !
where
    DrawFn: 'static + Fn(&mut Sketch, &State),
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(&config.name) // Reference prevents partial move
        .with_inner_size(LogicalSize::new(config.width as _, config.height as _))
        .with_resizable(config.resizable)
        .build(&event_loop)
        .unwrap();

    let mut sketch = Sketch::new(&window);
    let mut state = State::new(config.width, config.height);
    let callbacks = config.callbacks;

    event_loop.run(move |event, _, control_flow| match event {
        Event::EventsCleared => {
            draw(&mut sketch, &state);

            window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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
                state.modifiers = modifiers;

                let key_callback = match event_state {
                    ElementState::Pressed => callbacks.key_down,
                    ElementState::Released => callbacks.key_up,
                };

                if let Some(callback) = key_callback {
                    callback(&mut sketch, &state, key);
                }
            }
            _ => {}
        },
        _ => {}
    })
}
