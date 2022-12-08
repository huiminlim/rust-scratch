// Super is relative to path
use super::mp3;

pub fn sample() {
    mp3::some_fn();

    // Super is relative to path
    super::mp3::some_fn();
    super::super::video::h264::some_fn();

    // Advisable to use crate to prevent
    // changing root directories
    crate::codec::audio::mp3::some_fn();
}
