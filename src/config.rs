use crate::{sketch::Sketch, state::State, Key, Point, Size};

use std::fmt;

/// Function signature for a key press or release callback.
pub type KeyFn = fn(&mut Sketch, &State, Key);
/// Function signature for a mouse or window motion
/// callback.
pub type MoveFn = fn(&mut Sketch, &State, Point);
/// Function signature for a window resize callback.
pub type SizeFn = fn(&mut Sketch, &State, Size);

/// Event callbacks.
pub struct Callbacks {
    /// Callback for when a key is pressed.
    pub key_down: Option<KeyFn>,
    /// Callback for when a key is released.
    pub key_up: Option<KeyFn>,
    /// Callback for when the mouse cursor is moved.
    pub mouse_moved: Option<MoveFn>,
    /// Callback for when the window is moved.
    pub window_moved: Option<MoveFn>,
    /// Callback for when the window is resized.
    pub window_resized: Option<SizeFn>,
}

impl Default for Callbacks {
    fn default() -> Self {
        Self {
            key_down: None,
            key_up: None,
            mouse_moved: None,
            window_moved: None,
            window_resized: None,
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
///     callbacks: Callbacks::default(),
/// }
/// ```
///
/// [0]: ../sketch/struct.Sketch.html
pub struct Config {
    /// Name of the sketch, which is used for the title of
    /// its window.
    pub name: String,
    /// Size of the sketch's window.
    pub size: Size,
    /// Whether the sketch's window should be resizable.
    pub resizable: bool,
    /// Option exit key.
    pub exit_key: Option<Key>,
    /// Framerate of the sketch, or none for an un-capped
    /// framerate.
    pub framerate: Option<usize>,
    /// Other miscellaneous callback configurations.
    pub callbacks: Callbacks,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Config")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("resizable", &self.resizable)
            .field("exit_key", &self.exit_key)
            .field("framerate", &self.framerate)
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::from("peach sketch"),
            size: Size::new(640.0, 480.0),
            resizable: false,
            exit_key: None,
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
        self.size.width = width;
        self.size.height = height;

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
        self.size.width = width;

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
        self.size.height = height;

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

    /// Sets the exit key for the configuration.
    pub fn with_exit_key(mut self, exit_key: Key) -> Self {
        self.exit_key = Some(exit_key);

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

    /// Sets the mouse motion callback of the configuration.
    ///
    /// # See Also
    /// - [`Config::with_callbacks`][0].
    ///
    /// [0]: struct.Config.html#method.with_callbacks
    pub fn with_mouse_moved(mut self, mouse_moved: MoveFn) -> Self {
        self.callbacks.mouse_moved = Some(mouse_moved);

        self
    }

    /// Sets the window motion callback of the
    /// configuration.
    ///
    /// # See Also
    /// - [`Config::with_callbacks`][0].
    ///
    /// [0]: struct.Config.html#method.with_callbacks
    pub fn with_window_moved(mut self, window_moved: MoveFn) -> Self {
        self.callbacks.window_moved = Some(window_moved);

        self
    }

    /// Sets the window resize callback of the
    /// configuration.
    ///
    /// # See Also
    /// - [`Config::with_callbacks`][0].
    ///
    /// [0]: struct.Config.html#method.with_callbacks
    pub fn with_window_resized(mut self, window_resized: SizeFn) -> Self {
        self.callbacks.window_resized = Some(window_resized);

        self
    }
}
