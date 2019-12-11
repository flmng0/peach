use crate::{
    sketch::{Sketch, State},
    VirtualKeyCode,
};

/// Function signature for a key press or release callback.
pub type KeyFn = fn(&mut Sketch, &State, VirtualKeyCode);

/// Event callbacks.
pub struct Callbacks {
    /// Callback for when a key is pressed.
    pub key_down: Option<KeyFn>,
    /// Callback for when a key is released.
    pub key_up: Option<KeyFn>,
}

impl Default for Callbacks {
    fn default() -> Self {
        Self {
            key_down: None,
            key_up: None,
        }
    }
}

/// Configuration for a [`Sketch`][0].
///
/// # Usage
/// A `Config` also provides builder pattern methods, for
/// example:
/// ```
/// let config = Config::new()
///     .with_name("Hello, Peach!")
///     .with_size(800.0, 600.0)
///     .with_framerate(60);
/// ```
///
/// # Defaults
/// ```
/// Config {
///     name: String::from("peach sketch"),
///     width: 640,
///     height: 480,
///     resizable: false,
///     framerate: None,
/// }
/// ```
///
/// [0]: ../sketch/struct.Sketch.html
pub struct Config {
    /// Name of the sketch, which is used for the title of
    /// its window.
    pub name: String,

    /// Width of the sketch's window.
    pub width: f32,

    /// Height of the sketch's window.
    pub height: f32,

    /// Whether the sketch's window should be resizable.
    pub resizable: bool,

    /// Framerate of the sketch, or none for an un-capped
    /// framerate.
    pub framerate: Option<usize>,

    /// Other miscellaneous callback configurations.
    pub callbacks: Callbacks,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::from("peach sketch"),
            width: 640.0,
            height: 480.0,
            resizable: false,
            framerate: None,
            callbacks: Callbacks::default(),
        }
    }
}

impl Config {
    /// Create a new, default sketch configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name for the config.
    pub fn with_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = name.into();

        self
    }

    /// Sets the size for the config.
    ///
    /// # See Also
    /// - [`Config::with_width`][0].
    /// - [`Config::with_height`][1].
    ///
    /// [0]: struct.Config.html#method.with_width
    /// [1]: struct.Config.html#method.with_height
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;

        self
    }

    /// Sets the width for the config.
    ///
    /// # See Also
    /// - [`Config::with_size`][0].
    /// - [`Config::with_height`][1].
    ///
    /// [0]: struct.Config.html#method.with_size
    /// [1]: struct.Config.html#method.with_height
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;

        self
    }

    /// Sets the height for the config.
    ///
    /// # See Also
    /// - [`Config::with_size`][0].
    /// - [`Config::with_width`][1].
    ///
    /// [0]: struct.Config.html#method.with_size
    /// [1]: struct.Config.html#method.with_width
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;

        self
    }

    /// Sets the resize-ability for the config.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;

        self
    }

    /// Sets the framerate for the config to
    /// `Some(framerate)`.
    ///
    /// # See Also
    /// - [`Config::without_framerate`][0].
    ///
    /// [0]: struct.Config.html#method.without_framerate
    pub fn with_framerate(mut self, framerate: usize) -> Self {
        self.framerate = Some(framerate);

        self
    }

    /// Sets the framerate for the config to `None`.
    ///
    /// # See Also
    /// - [`Config::with_framerate`][0].
    ///
    /// [0]: struct.Config.html#method.with_framerate
    pub fn without_framerate(mut self) -> Self {
        self.framerate = None;

        self
    }

    /// Sets the callbacks for the configuration.
    pub fn with_callbacks(mut self, callbacks: Callbacks) -> Self {
        self.callbacks = callbacks;

        self
    }

    /// Sets the key pressed callback of the configuration.
    ///
    /// # See Also
    /// - [`Config::with_callbacks`][0].
    ///
    /// [0]: struct.Config.html#method.with_callbacks
    pub fn with_key_down(mut self, key_down: KeyFn) -> Self {
        self.callbacks.key_down = Some(key_down);

        self
    }

    /// Sets the key released callback of the configuration.
    ///
    /// # See Also
    /// - [`Config::with_callbacks`][0].
    ///
    /// [0]: struct.Config.html#method.with_callbacks
    pub fn with_key_up(mut self, key_up: KeyFn) -> Self {
        self.callbacks.key_up = Some(key_up);

        self
    }
}
