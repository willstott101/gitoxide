use std::{borrow::Cow, convert::TryFrom};

use gix_config::parse::{Event, Events, Section};

mod error;
mod from_bytes;
mod key;
mod section;

#[test]
fn size_in_memory() {
    let actual = std::mem::size_of::<Section<'_>>();
    assert!(
        actual <= 6768,
        "{} <= 6768: This shouldn't change without us noticing",
        actual
    );
    let actual = std::mem::size_of::<Event<'_>>();
    assert!(
        actual <= 104,
        "{} <= 104: This shouldn't change without us noticing",
        actual
    );
    let actual = std::mem::size_of::<Events<'_>>();
    assert!(
        actual <= 872,
        "{} <= 872: This shouldn't change without us noticing",
        actual
    );
}

#[test]
fn empty() {
    assert_eq!(Events::from_str("").unwrap().into_vec(), vec![]);
}

#[test]
fn newlines_with_spaces() {
    assert_eq!(
        Events::from_str("\n   \n \n").unwrap().into_vec(),
        vec![newline(), whitespace("   "), newline(), whitespace(" "), newline()]
    )
}

#[test]
fn consecutive_newlines() {
    assert_eq!(
        Events::from_str("\n\n\n\n\n").unwrap().into_vec(),
        vec![newline_custom("\n\n\n\n\n")],
        "multiple newlines are merged into a single event"
    );
}

fn name(name: &'static str) -> Event<'static> {
    Event::SectionKey(gix_config::parse::section::Key::try_from(name).unwrap())
}

fn value(value: &'static str) -> Event<'static> {
    Event::Value(Cow::Borrowed(value.into()))
}

fn newline() -> Event<'static> {
    newline_custom("\n")
}

fn newline_custom(value: &'static str) -> Event<'static> {
    Event::Newline(Cow::Borrowed(value.into()))
}

fn whitespace(value: &'static str) -> Event<'static> {
    Event::Whitespace(Cow::Borrowed(value.into()))
}

fn separator() -> Event<'static> {
    Event::KeyValueSeparator
}
