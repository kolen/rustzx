//! Module implements emulation of sound chip AY, Spectrum Beeper and Mixer
pub mod beeper;
pub mod ay;
pub mod mixer;
pub mod sample;

use zx::constants::FPS;

pub const SAMPLE_RATE: usize = 44100;
/// samples per frame
pub const SAMPLES: usize = SAMPLE_RATE / FPS;
pub const CHANNELS: usize = 2;

/// Returns, which must be already processed at this time
pub fn samples_from_time(times: (usize, usize)) -> usize {
    let (time, max) = times;
    SAMPLES * time / max
}
