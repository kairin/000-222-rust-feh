// SPDX-License-Identifier: MIT

pub mod image_proc;
pub mod scanner;
pub mod tool_caps;
pub mod types;
pub mod ui_logic;

#[cfg(test)]
pub(crate) mod test_support {
    /// Serializes tests that read or mutate the process-wide PATH.
    pub static PATH_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
}
