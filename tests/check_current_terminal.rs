use terminal_supports_emoji::{supports_emoji, Stream};

#[test]
fn check_current_terminal() {
    assert_eq!(true, supports_emoji(Stream::Stdout));
}
