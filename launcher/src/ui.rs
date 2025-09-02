//! Contains shared UI components and logic used across different UI versions.
//!
//! This module provides common functionality such as font loading, background
//! task management, and game binary hooking. It also defines utility structs
//! for UI elements.

use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use hooks_addresses::Addresses;

use crate::games::GameVersion;

mod icons;
pub mod new;
pub mod old;

/// The range of icons to load from the Font Awesome font.
static ICON_RANGE: [u32; 3] = [icons::ICON_MIN as u32, icons::ICON_MAX as u32, 0];

/// A struct to hold the fonts used in the UI.
pub struct Fonts {
    pub header: imgui::FontId,
}

impl Fonts {
    /// Sets up the fonts for the UI.
    ///
    /// This function loads the fonts from the file system and adds them to the
    /// `imgui` context.
    pub fn setup(imgui: &mut imgui::Context) -> Fonts {
        let font_size = 24.0;
        // Load the main font and the icon font.
        imgui.fonts().add_font(&[
            imgui::FontSource::TtfData {
                data: include_bytes!("../fonts/static/Orbitron-Regular.ttf"),
                size_pixels: font_size,
                config: Some(imgui::FontConfig {
                    name: Some(String::from("Orbitron")),
                    ..imgui::FontConfig::default()
                }),
            },
            imgui::FontSource::TtfData {
                data: include_bytes!("../fonts/Font Awesome 6 Free-Solid-900.otf"),
                size_pixels: font_size,
                config: Some(imgui::FontConfig {
                    name: Some(String::from("Font Awesome")),
                    pixel_snap_h: true,
                    glyph_ranges: imgui::FontGlyphRanges::from_slice(&ICON_RANGE),
                    ..imgui::FontConfig::default()
                }),
            },
        ]);
        // Load the header font.
        let header_font = imgui.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../fonts/static/Orbitron-Regular.ttf"),
            size_pixels: font_size * 4.0,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Orbitron Header")),
                ..imgui::FontConfig::default()
            }),
        }]);
        // Load additional fonts.
        imgui.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../fonts/static/SpaceGrotesk-Regular.ttf"),
            size_pixels: font_size,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Space Grotesk")),
                ..imgui::FontConfig::default()
            }),
        }]);
        imgui.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../fonts/Silkscreen-Regular.ttf"),
            size_pixels: font_size,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Silkscreen")),
                ..imgui::FontConfig::default()
            }),
        }]);
        Fonts { header: header_font }
    }
}

/// A generic wrapper for values that are computed in the background.
///
/// This enum allows for starting a background task and then later retrieving
/// its value without blocking the main thread.
#[derive(Default)]
enum BackgroundValue<T> {
    /// The handle to the background thread.
    Handle(std::thread::JoinHandle<T>),
    /// The computed value.
    Value(T),
    /// The initial state before the background task is started.
    #[default]
    Unset,
}

impl<T: std::fmt::Debug> std::fmt::Debug for BackgroundValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundValue::Handle(_) => write!(f, "BackgroundValue::Handle"),
            BackgroundValue::Value(v) => write!(f, "BackgroundValue::Value({:?})", v),
            BackgroundValue::Unset => write!(f, "BackgroundValue::Unset"),
        }
    }
}

impl<T> BackgroundValue<T>
where
    T: Send + 'static,
{
    /// Creates a new `BackgroundValue` from a synchronous closure.
    fn new(f: impl FnOnce() -> T + Send + 'static) -> Self {
        Self::Handle(std::thread::spawn(f))
    }

    /// Creates a new `BackgroundValue` from an asynchronous future.
    fn new_async(f: impl Future<Output = T> + Send + 'static) -> Self {
        Self::Handle(std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(f)
        }))
    }
}

impl<T> BackgroundValue<T> {
    /// Checks if the background task has finished.
    fn is_finished(&self) -> bool {
        match self {
            BackgroundValue::Handle(h) => h.is_finished(),
            BackgroundValue::Value(_) => true,
            BackgroundValue::Unset => unreachable!(),
        }
    }

    /// If the background task has finished, this function retrieves the value
    /// and transitions the state to `Value(T)`.
    fn maybe_value(&mut self) {
        if let BackgroundValue::Handle(h) = self {
            if h.is_finished() {
                let h = std::mem::replace(self, BackgroundValue::Unset);
                let val = if let BackgroundValue::Handle(h) = h {
                    h.join().unwrap()
                } else {
                    unreachable!();
                };
                let _ = std::mem::replace(self, BackgroundValue::Value(val));
            }
        }
    }

    /// Tries to get a reference to the value.
    ///
    /// Returns `None` if the value is not yet available.
    fn try_get(&mut self) -> Option<&T> {
        self.maybe_value();
        if let BackgroundValue::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Tries to get a mutable reference to the value.
    ///
    /// Returns `None` if the value is not yet available.
    fn try_mut(&mut self) -> Option<&mut T> {
        self.maybe_value();
        if let BackgroundValue::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Tries to take the value, consuming the `BackgroundValue`.
    ///
    /// Returns `None` if the value is not yet available.
    fn try_take(&mut self) -> Option<T> {
        self.maybe_value();
        if let BackgroundValue::Value(_) = self {
            let v = std::mem::replace(self, BackgroundValue::Unset);
            Some(v.into_inner())
        } else {
            None
        }
    }

    /// Consumes the `BackgroundValue` and returns the inner value, blocking
    /// if the background task has not yet finished.
    fn into_inner(self) -> T {
        match self {
            BackgroundValue::Handle(h) => h.join().unwrap(),
            BackgroundValue::Value(v) => v,
            BackgroundValue::Unset => unreachable!(),
        }
    }
}

/// Represents the state of hooking a game binary.
enum GameHookState {
    /// The binary has been successfully hooked and the addresses are resolved.
    Resolved(Box<Addresses>),
    /// The binary file was not found.
    FileNotFound,
    /// A search for the addresses is in progress.
    Searching,
    /// The binary is not hooked, and no search will be performed.
    Ignored,
    /// The binary is not supported.
    UnsupportedBinary,
    /// An error occurred while hooking the binary.
    Failed(String),
}

/// Represents a hook for a specific game version.
struct GameHook {
    version: GameVersion,
    state: GameHookState,
    target_dir: PathBuf,
    background_search: BackgroundValue<GameHookState>,
}

impl GameHook {
    /// Creates a new `GameHook` for the given game version and target directory.
    pub fn new(gv: GameVersion, target_dir: PathBuf) -> Self {
        let res = hooks_addresses::get_from_path(&gv.full_path(&target_dir)).inspect_err(|e| println!("{e}"));
        let state = match res {
            Ok(addrs) => GameHookState::Resolved(Box::new(addrs)),
            Err(hooks_addresses::Error::UnknownBinary(_)) => GameHookState::FileNotFound,
            Err(hooks_addresses::Error::BinaryMismatch(_, _)) => GameHookState::UnsupportedBinary,
            Err(hooks_addresses::Error::NoFileName(_)) => GameHookState::FileNotFound,
            Err(hooks_addresses::Error::IdFailed) => GameHookState::Failed("couldn't identify".to_string()),
            Err(hooks_addresses::Error::IO(e)) => GameHookState::Failed(format!("{e}")),
        };
        Self {
            version: gv,
            state,
            target_dir,
            background_search: BackgroundValue::Unset,
        }
    }

    /// Returns `true` if the hook is ready (i.e., the addresses are resolved).
    pub fn is_ready(&self) -> bool {
        matches!(self.state, GameHookState::Resolved(_))
    }

    /// Starts a background search for the hook addresses.
    pub fn search(&mut self) {
        self.state = GameHookState::Searching;
        let path = self.version.full_path(&self.target_dir);
        self.background_search = BackgroundValue::Handle(std::thread::spawn(move || {
            hooks_addresses::search_patterns(&path)
                .inspect_err(|e| println!("{e}"))
                .map_or_else(|e| GameHookState::Failed(e.to_string()), |a| GameHookState::Resolved(Box::new(a)))
        }));
    }
}

/// A collection of `GameHook`s for all supported game versions.
pub struct GameHooks {
    games: HashMap<GameVersion, GameHook>,
}

impl GameHooks {
    /// Creates a new `GameHooks` collection.
    fn new(games: HashMap<GameVersion, GameHook>) -> Self {
        Self { games }
    }

    /// Returns `true` if all game binaries are unknown.
    fn has_only_unknown(&self) -> bool {
        !self
            .games
            .values()
            .any(|g| matches!(g.state, GameHookState::Resolved(_) | GameHookState::Ignored | GameHookState::Searching))
    }

    /// Returns `true` if any game hook is currently being searched.
    fn is_searching(&self) -> bool {
        self.games.values().any(|g| matches!(g.state, GameHookState::Searching))
    }

    /// Gets a reference to a `GameHook` for a specific game version.
    fn get(&self, gv: GameVersion) -> Option<&GameHook> {
        self.games.get(&gv)
    }

    /// Returns an iterator over all ready game hooks.
    fn iter_ready(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values().filter(|g| g.is_ready())
    }

    /// Starts searching for addresses for all unsupported binaries.
    fn start_searching(&mut self) {
        for hook in self.games.values_mut() {
            if matches!(hook.state, GameHookState::UnsupportedBinary) {
                hook.search();
            }
        }
    }

    /// Checks the status of background searches and updates the hook states.
    ///
    /// Returns `true` if all searches have finished.
    fn search_status(&mut self) -> bool {
        let mut finished = true;
        for hook in self.games.values_mut() {
            if let GameHookState::Searching = hook.state {
                if let Some(new_state) = hook.background_search.try_take() {
                    hook.state = new_state;
                } else {
                    finished = false;
                }
            }
        }
        finished
    }

    /// Ignores all unknown binaries, preventing further searches.
    fn ignore_unknown_binaries(&mut self) {
        for hook in self.games.values_mut() {
            if !matches!(hook.state, GameHookState::Resolved(_)) {
                hook.state = GameHookState::Ignored;
            }
        }
    }

    /// Returns an iterator over all game hooks.
    fn iter(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values()
    }
}

/// Loads the game binaries and creates `GameHook`s for them in the background.
fn load_game_binaries(target_dir: &Path) -> BackgroundValue<GameHooks> {
    let target_dir = target_dir.to_path_buf();
    BackgroundValue::Handle(std::thread::spawn(move || {
        GameHooks::new(
            [GameVersion::SplinterCellBlacklistDx9, GameVersion::SplinterCellBlacklistDx11]
                .into_iter()
                .map(|gv| (gv, GameHook::new(gv, target_dir.clone())))
                .collect(),
        )
    }))
}

/// A simple struct for representing a 2D size.
#[derive(Debug, Clone, Copy)]
struct Size {
    w: f32,
    h: f32,
}

impl From<[f32; 2]> for Size {
    fn from(s: [f32; 2]) -> Self {
        Self { w: s[0], h: s[1] }
    }
}

impl From<Size> for [f32; 2] {
    fn from(s: Size) -> Self {
        [s.w, s.h]
    }
}

impl From<Size> for mint::Vector2<f32> {
    fn from(s: Size) -> Self {
        mint::Vector2 { x: s.w, y: s.h }
    }
}

/// A simple struct for representing a color with RGBA components.
#[derive(Debug, Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Self {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

/// A struct for creating animated text that cycles through a list of variants.
#[derive(Debug)]
struct AnimatedText<'a> {
    variants: &'a [&'a str],
    interval: Duration,
    current_variant: usize,
    last_instant: Instant,
}

impl<'a> AnimatedText<'a> {
    /// Creates a new `AnimatedText`.
    pub fn new(variants: &'a [&'a str], interval: Duration) -> Self {
        Self {
            variants,
            interval,
            current_variant: 0,
            last_instant: Instant::now(),
        }
    }

    /// Updates the animation, changing the text if the interval has passed.
    ///
    /// Returns `true` if the text was updated.
    pub fn update(&mut self) -> bool {
        let inst = Instant::now();
        if inst.duration_since(self.last_instant) >= self.interval {
            self.last_instant = inst;
            self.current_variant = (self.current_variant + 1) % self.variants.len();
            true
        } else {
            false
        }
    }

    /// Returns the current text variant.
    pub fn text(&self) -> &str {
        self.variants[self.current_variant]
    }
}
