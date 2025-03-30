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

static ICON_RANGE: [u32; 3] = [icons::ICON_MIN as u32, icons::ICON_MAX as u32, 0];

pub struct Fonts {
    pub header: imgui::FontId,
}

impl Fonts {
    pub fn setup(imgui: &mut imgui::Context) -> Fonts {
        let font_size = 24.0;
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
                // data: include_bytes!("../fonts/fa-solid-900.ttf"),
                // data: include_bytes!("../fonts/Font Awesome 6 Free-Regular-400.otf"),
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
        let header_font = imgui.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../fonts/static/Orbitron-Regular.ttf"),
            size_pixels: font_size * 4.0,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Orbitron Header")),
                ..imgui::FontConfig::default()
            }),
        }]);
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

#[derive(Default)]
enum BackgroundValue<T> {
    Handle(std::thread::JoinHandle<T>),
    Value(T),
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
    fn new(f: impl FnOnce() -> T + Send + 'static) -> Self {
        Self::Handle(std::thread::spawn(f))
    }

    fn new_async(f: impl Future<Output = T> + Send + 'static) -> Self {
        Self::Handle(std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(f)
        }))
    }
}

impl<T> BackgroundValue<T> {
    fn is_finished(&self) -> bool {
        match self {
            BackgroundValue::Handle(h) => h.is_finished(),
            BackgroundValue::Value(_) => true,
            BackgroundValue::Unset => unreachable!(),
        }
    }

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

    fn try_get(&mut self) -> Option<&T> {
        self.maybe_value();
        if let BackgroundValue::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn try_mut(&mut self) -> Option<&mut T> {
        self.maybe_value();
        if let BackgroundValue::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn try_take(&mut self) -> Option<T> {
        self.maybe_value();
        if let BackgroundValue::Value(_) = self {
            let v = std::mem::replace(self, BackgroundValue::Unset);
            Some(v.into_inner())
        } else {
            None
        }
    }

    fn into_inner(self) -> T {
        match self {
            BackgroundValue::Handle(h) => h.join().unwrap(),
            BackgroundValue::Value(v) => v,
            BackgroundValue::Unset => unreachable!(),
        }
    }
}

enum GameHookState {
    Resolved(Box<Addresses>),
    FileNotFound,
    Searching,
    Ignored,
    UnsupportedBinary,
    Failed(String),
}

struct GameHook {
    version: GameVersion,
    state: GameHookState,
    target_dir: PathBuf,
    background_search: BackgroundValue<GameHookState>,
}

impl GameHook {
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

    pub fn is_ready(&self) -> bool {
        matches!(self.state, GameHookState::Resolved(_))
    }

    pub fn search(&mut self) {
        self.state = GameHookState::Searching;
        let path = self.version.full_path(&self.target_dir);
        self.background_search = BackgroundValue::Handle(std::thread::spawn(move || {
            hooks_addresses::search_patterns(&path)
                .inspect_err(|e| println!("{e}"))
                .map_or_else(
                    |e| GameHookState::Failed(e.to_string()),
                    |a| GameHookState::Resolved(Box::new(a)),
                )
        }));
    }
}

pub struct GameHooks {
    games: HashMap<GameVersion, GameHook>,
}

impl GameHooks {
    fn new(games: HashMap<GameVersion, GameHook>) -> Self {
        Self { games }
    }

    fn has_only_unknown(&self) -> bool {
        !self.games.values().any(|g| {
            matches!(
                g.state,
                GameHookState::Resolved(_) | GameHookState::Ignored | GameHookState::Searching
            )
        })
    }

    fn is_searching(&self) -> bool {
        self.games.values().any(|g| matches!(g.state, GameHookState::Searching))
    }

    fn get(&self, gv: GameVersion) -> Option<&GameHook> {
        self.games.get(&gv)
    }

    fn iter_ready(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values().filter(|g| g.is_ready())
    }

    fn start_searching(&mut self) {
        for hook in self.games.values_mut() {
            if matches!(hook.state, GameHookState::UnsupportedBinary) {
                hook.search();
            }
        }
    }

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

    fn ignore_unknown_binaries(&mut self) {
        for hook in self.games.values_mut() {
            if !matches!(hook.state, GameHookState::Resolved(_)) {
                hook.state = GameHookState::Ignored;
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values()
    }
}

fn load_game_binaries(target_dir: &Path) -> BackgroundValue<GameHooks> {
    let target_dir = target_dir.to_path_buf();
    BackgroundValue::Handle(std::thread::spawn(move || {
        GameHooks::new(
            [
                GameVersion::SplinterCellBlacklistDx9,
                GameVersion::SplinterCellBlacklistDx11,
            ]
            .into_iter()
            .map(|gv| (gv, GameHook::new(gv, target_dir.clone())))
            .collect(),
        )
    }))
}

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

#[derive(Debug)]
struct AnimatedText<'a> {
    variants: &'a [&'a str],
    interval: Duration,
    current_variant: usize,
    last_instant: Instant,
}

impl<'a> AnimatedText<'a> {
    pub fn new(variants: &'a [&'a str], interval: Duration) -> Self {
        Self {
            variants,
            interval,
            current_variant: 0,
            last_instant: Instant::now(),
        }
    }

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

    pub fn text(&self) -> &str {
        self.variants[self.current_variant]
    }
}
