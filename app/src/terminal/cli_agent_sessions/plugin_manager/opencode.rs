use std::sync::LazyLock;

use async_trait::async_trait;

use super::{CliAgentPluginManager, PluginInstructionStep, PluginInstructions};

// Keep in sync with the opencode-warp npm package version.
// This version is also hardcoded into UPDATE_INSTRUCTIONS below (so the update
// instructions tell users to pin to this specific version to force OpenCode's
// plugin cache to re-fetch). Update both together.
const MINIMUM_PLUGIN_VERSION: &str = "0.1.5";

pub(super) struct OpenCodePluginManager;

#[async_trait]
impl CliAgentPluginManager for OpenCodePluginManager {
    fn minimum_plugin_version(&self) -> &'static str {
        MINIMUM_PLUGIN_VERSION
    }

    fn can_auto_install(&self) -> bool {
        false
    }

    fn install_instructions(&self) -> &'static PluginInstructions {
        &INSTALL_INSTRUCTIONS
    }

    fn update_instructions(&self) -> &'static PluginInstructions {
        &UPDATE_INSTRUCTIONS
    }
}

// Localized slices cannot be const-promoted, so each LazyLock allocates them once.
static INSTALL_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: warp_i18n::t_static!("terminal-plugin-opencode-install-title"),
    subtitle: warp_i18n::t_static!("terminal-plugin-opencode-install-subtitle"),
    steps: Box::leak(Box::new([
        PluginInstructionStep {
            description: warp_i18n::t_static!("terminal-plugin-opencode-open-config"),
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: warp_i18n::t_static!("terminal-plugin-opencode-add-plugin"),
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp\"]",
            executable: false,
            link: None,
        },
    ])),
    post_install_notes: Box::leak(Box::new([warp_i18n::t_static!(
        "terminal-plugin-opencode-restart-activate"
    )])),
});

static UPDATE_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: warp_i18n::t_static!("terminal-plugin-opencode-update-title"),
    subtitle: warp_i18n::t_static!("terminal-plugin-opencode-update-subtitle"),
    steps: Box::leak(Box::new([
        PluginInstructionStep {
            description: warp_i18n::t_static!("terminal-plugin-opencode-open-config"),
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: warp_i18n::t_static!("terminal-plugin-opencode-replace-plugin"),
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp@0.1.5\"]",
            executable: false,
            link: None,
        },
    ])),
    post_install_notes: Box::leak(Box::new([warp_i18n::t_static!(
        "terminal-plugin-opencode-restart-update"
    )])),
});

#[cfg(test)]
#[path = "opencode_tests.rs"]
mod tests;
