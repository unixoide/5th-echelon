use std::cell::RefCell;
use std::rc::Rc;

use crate::config::ConfigMut;
use crate::config::{self};

#[derive(Debug)]
pub struct SettingsMenu {
    cfg: Rc<RefCell<ConfigMut>>,
}

impl SettingsMenu {
    pub fn new(cfg: Rc<RefCell<ConfigMut>>) -> Self {
        Self { cfg }
    }
    pub fn render(&mut self, ui: &imgui::Ui) -> bool {
        let go_back = ui.arrow_button("Back", imgui::Direction::Left);

        self.cfg.borrow_mut().update(|cfg| {
            imgui::TabBar::new("Settings").build(ui, || {
                imgui::TabItem::new("Main").build(ui, || {
                    ui.checkbox("Enable Overlay", &mut cfg.hook_config.enable_overlay);
                    let ui_versions = [None, Some(config::UIVersion::Old), Some(config::UIVersion::New)];
                    let mut selected_ui_version = ui_versions.iter().position(|v| v == &cfg.ui_version).unwrap_or(0);

                    if ui.combo("UI Version to use\n(requires launcher restart)", &mut selected_ui_version, &ui_versions, |uv| {
                        std::borrow::Cow::Borrowed(match uv {
                            Some(config::UIVersion::Old) => "Old",
                            Some(config::UIVersion::New) => "New",
                            None => "Choose on next start",
                        })
                    }) {
                        cfg.ui_version = ui_versions[selected_ui_version];
                    }
                });
                imgui::TabItem::new("Advanced").build(ui, || {
                    static LOG_LEVELS: [hooks_config::LogLevel; 5] = [
                        hooks_config::LogLevel::Trace,
                        hooks_config::LogLevel::Debug,
                        hooks_config::LogLevel::Info,
                        hooks_config::LogLevel::Warning,
                        hooks_config::LogLevel::Error,
                    ];
                    let mut current_item = LOG_LEVELS.binary_search(&cfg.hook_config.logging.level).unwrap();
                    ui.combo_simple_string("Log Level", &mut current_item, &LOG_LEVELS);
                    cfg.hook_config.logging.level = LOG_LEVELS[current_item];

                    ui.checkbox("Automatically join invites", &mut cfg.hook_config.auto_join_invite);
                    ui.input_text("Unreal Engine command line", &mut cfg.hook_config.internal_command_line).build();
                    ui.checkbox("Enable All Hooks", &mut cfg.hook_config.enable_all_hooks);
                    if !cfg.hook_config.enable_all_hooks && ui.collapsing_header("Individual Hooks", imgui::TreeNodeFlags::FRAME_PADDING) {
                        ui.indent();
                        for (variant, label) in hooks_config::Hook::VARIANTS.iter().zip(hooks_config::Hook::LABELS.iter()) {
                            // TODO
                            // if addr.hook_addr(*variant).is_none() {
                            //     continue;
                            // }
                            let found = cfg.hook_config.enable_hooks.contains(variant);
                            let mut enabled = found;
                            ui.checkbox(*label, &mut enabled);
                            match (enabled, found) {
                                (true, true) => {}
                                (true, false) => {
                                    cfg.hook_config.enable_hooks.insert(*variant);
                                }
                                (false, true) => {
                                    cfg.hook_config.enable_hooks.remove(variant);
                                }
                                (false, false) => {}
                            }
                        }
                        ui.unindent();
                    }
                });
            });
        });
        // if ui.button(format!("{} Save", ICON_FLOPPY_DISK)) {}

        !go_back
    }
}
