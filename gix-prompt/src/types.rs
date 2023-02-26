use std::{borrow::Cow, convert::TryFrom, path::Path};

/// The error returned by [ask()][crate::ask()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Terminal prompts are disabled")]
    Disabled,
    #[error("The current platform has no implementation for prompting in the terminal")]
    UnsupportedPlatform,
    #[error(
        "Failed to open terminal at {:?} for writing prompt, or to write it",
        crate::unix::TTY_PATH
    )]
    TtyIo(#[from] std::io::Error),
    #[cfg(unix)]
    #[error("Failed to obtain or set terminal configuration")]
    TerminalConfiguration(#[from] nix::errno::Errno),
}

/// The way the user is prompted.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    /// Visibly show user input.
    Visible,
    /// Do not show user input, suitable for sensitive data.
    Hidden,
    /// Do not prompt the user at all but rather abort with an error. This is useful in conjunction with [Options::askpass].
    Disable,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Hidden
    }
}

/// The options used in `[ask()]`.
#[derive(Default, Clone)]
pub struct Options<'a> {
    /// The path or name (for lookup in `PATH`) to the askpass program to call before prompting the user.
    ///
    /// It's called like this `askpass <prompt>`, but note that it won't know if the input should be hidden or not.
    pub askpass: Option<Cow<'a, Path>>,
    /// The way the user is prompted.
    pub mode: Mode,
}

impl Options<'_> {
    /// Change this instance to incorporate information from the environment.
    ///
    /// - if `use_git_askpass` is true, use `GIT_ASKPASS` to override any existing [`askpass`][Options::askpass] program
    /// - otherwise fall back to the [`askpass`][Options::askpass] program already set
    /// - or try to use the `SSH_ASKPASS` if `use_ssh_askpass` is true
    ///
    /// At the and of this process, the `askpass` program may be set depending on the rules above.
    ///
    /// Lastly, if `use_git_terminal_prompt` is set, use the `GIT_TERMINAL_PROMPT` environment variable and evaluate it as boolean,
    /// and if false, set [`mode`][Options::mode] to `disable`.
    pub fn apply_environment(
        mut self,
        use_git_askpass: bool,
        use_ssh_askpass: bool,
        use_git_terminal_prompt: bool,
    ) -> Self {
        if let Some(askpass) = use_git_askpass.then(|| std::env::var_os("GIT_ASKPASS")).flatten() {
            self.askpass = Some(Cow::Owned(askpass.into()))
        }
        if self.askpass.is_none() {
            if let Some(askpass) = use_ssh_askpass.then(|| std::env::var_os("SSH_ASKPASS")).flatten() {
                self.askpass = Some(Cow::Owned(askpass.into()))
            }
        }
        self.mode = use_git_terminal_prompt
            .then(|| {
                std::env::var_os("GIT_TERMINAL_PROMPT")
                    .and_then(|val| gix_config_value::Boolean::try_from(val).ok())
                    .and_then(|allow| (!allow.0).then_some(Mode::Disable))
            })
            .flatten()
            .unwrap_or(self.mode);
        self
    }
}
