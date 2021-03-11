pub use atty::Stream as AttyStream;

/// Possible stream sources.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Stream {
    /// The standard error stream.
    Stderr,
    /// The standard output stream.
    Stdout,
}

impl From<Stream> for AttyStream {
    fn from(s: Stream) -> Self {
        match s {
            Stream::Stderr => AttyStream::Stderr,
            Stream::Stdout => AttyStream::Stdout,
        }
    }
}

// Platforms that are based on Unix support Emojis if the system language is a UTF-8 one.
#[cfg(all(unix, not(target_os = "macos")))]
lazy_static::lazy_static! {
    static ref IS_LANG_UTF8: bool = {
        match std::env::var("LANG") {
            Ok(lang) => lang.to_uppercase().ends_with("UTF-8"),
            _ => false,
        }
    };
}

// The new Windows Terminal does support emojis. Currently, the terminal will
// set the environment variable `WT_SESSION`. This can be used to check if the
// user uses that specific app.
#[cfg(windows)]
fn platform_supports_emoji() -> bool {
    std::env::var("WT_SESSION").is_ok()
}

// macOS by default has emoji support.
#[cfg(target_os = "macos")]
fn platform_supports_emoji() -> bool {
    true
}

// On unix systems the enabled language decides whether emojis are supported or
// not.
#[cfg(all(unix, not(target_os = "macos")))]
fn platform_supports_emoji() -> bool {
    *IS_LANG_UTF8
}

#[cfg(all(not(unix), not(windows)))]
fn platform_supports_emoji() -> bool {
    false
}

/// Check if the given IO stream supports Emoji output.
///
/// Platforms support is determined by checking these conditions:
///     - macOS has Emoji support by default
///     - Unix systems have support if the active language supports them.
///     - Windows machines running the new Terminal app support Emojis.
///
/// # Arguments
///
/// - `stream`: The stream to check for.
///
/// # Returns
///
/// `true` if the stream and platform support Emoji output, `false` otherwise.
pub fn supports_emoji(stream: Stream) -> bool {
    platform_supports_emoji() && atty::is(stream.into())
}
