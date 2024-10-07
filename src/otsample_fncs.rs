extern crate bincode;
extern crate serde;
extern crate serde_big_array;

use crate::results::*;
use crate::constants::SAMPLE_RATE;
use crate::wavfile::WavFile;

fn get_sample_length_as_bars(tempo: f32, sample_count: u32) -> F32Result {
    let beats_per_bar: f32 = 4.0;
    let duration_minutes: f32 = (sample_count as f32) / ((SAMPLE_RATE as f32) * 60.0);
    let beats = duration_minutes * tempo;
    let bars = beats / beats_per_bar;
    Ok(bars.floor())
}

fn get_u32_normalized_sample_bars_from_length(tempo: f32, sample_count: u32) -> U32Result {
    let bars = get_sample_length_as_bars(tempo, sample_count).unwrap();
    Ok(bars as u32)
}

// convert gain from -1.0 <= x <= 1.0
fn get_scaled_u16_gain(gain: f32) -> U16Result {
    Ok(((gain * 48.0) - 24.0) as u16)
}

fn len_into_seconds_f32(n_samples: u32, tempo: &f32) -> F32Result {
    Ok(tempo * (n_samples as f32) / SAMPLE_RATE as f32)
}

fn len_into_minutes_f32(n_samples: u32, tempo: &f32) -> F32Result {
    let seconds = len_into_seconds_f32(n_samples, tempo).unwrap();
    Ok(seconds / 60.0)
}

/// Convert number of audio samples into number of f32 "bars" when given a tempo
// NOTE: Assumes 4 beats to a bar.
fn len_into_bars_f32(n_samples: u32, tempo: &f32) -> F32Result {
    let bars = len_into_minutes_f32(n_samples, tempo).unwrap();
    Ok(bars / 4.0)
}

/// Convert number of audio samples into number of u32 "bars" when given a tempo
// NOTE: Assumes 4 beats to a bar.
fn len_into_bars_u32(n_samples: u32, tempo: &f32) -> U32Result {
    // let bars = self.len_into_bars_f32(tempo).unwrap();
    // Ok(bars as u32)
    let beats_per_bar: f32 = 4.0;
    let duration_minutes: f32 = (n_samples as f32) / ((SAMPLE_RATE as f32) * 60.0);
    let beats = duration_minutes * tempo;
    let bars = beats / beats_per_bar;
    Ok((bars * 100.0).floor() as u32)
}

// used for botth 
pub fn get_ot_sample_n_bars_from_wavfiles(wavs: &Vec<WavFile>, tempo_bpm: &f32) -> U32Result {
    let total_samples: u32 = wavs.iter().map(|x| x.len as u32).sum();
    let beats = total_samples as f32 / (SAMPLE_RATE as f32 * 60.0 * 4.0);
    let mut bars = ((tempo_bpm * 4.0 * beats) + 0.5) * 0.25;
    bars -= bars % 0.25;
    Ok((bars * 100.0) as u32)
}

// convert gain to: -1.0 <= x <= 1.0
// (0x30 = 0, 0x60 = 24 (max), 0x00 = -24 (min))
pub fn get_scaled_u16_gain_as_f32(gain: u16) -> F32Result {

    // // out of bounds error
    // if 24 > gain && gain > -24 {
    //     return Err(())
    // }
    
    Ok(gain as f32 + 24.0 / 48.0)
}

// convert gain from -1.0 <= x <= 1.0
pub fn gain_scaled_u16(gain: f32) -> U16Result {
    Ok(((gain * 48.0) - 24.0) as u16)
}

#[cfg(test)]
mod test_fncs {
    use super::*;
    
    // #[ignore]
    // #[test]
    // fn test_do_thing() {
    //     assert_eq!(get_ot_sample_n_bars_from_wavfiles(96536, &125.0), Ok(125));
    // }
}

