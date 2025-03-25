//! Functions for CLI actions related to chaining samples into sliced sample chains.

#[cfg(test)]
#[allow(unused_imports)]
mod test;
mod yaml;

use log::trace;
use ot_tools_lib::samples::{
    configs::{SampleLoopConfig, SampleTrimConfig},
    options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    },
    slices::{Slice, Slices},
    SampleAttributes, SampleAttributesRawBytes,
};

use crate::{
    audio::wav::{WavFile, ALLOWED_BIT_DEPTHS, ALLOWED_SAMPLE_RATE},
    utils::{get_bin_nbars_ileaved_wavfiles, get_otsample_nbars_from_wavfile},
    RBoxErr,
};
use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::array::from_fn;
use std::path::{Path, PathBuf};

use ot_tools_lib::{
    get_bytes_slice, read_type_from_bin_file, type_to_yaml_file, write_type_to_bin_file,
    yaml_file_to_type,
};
use yaml::{
    create::YamlChainCreate,
    deconstruct::YamlChainDeconstruct,
    samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple},
};

const STDOUT_SECTION_SEPARATOR: &str =
    "============================================================";

fn prnt_stdout_new_section() {
    println!("{}", STDOUT_SECTION_SEPARATOR);
}

#[derive(Debug)]
enum CliSampleErrors {
    InvalidOptBitDepth,
    TooManySlices,
    AudioTooShort,
    NotADirectory,
}
impl std::fmt::Display for CliSampleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidOptBitDepth => write!(
                f,
                "invalid bit depth setting, only '16' or '24' are supported"
            ),
            Self::TooManySlices => write!(f, "maximum number of slices in a sample file is 64"),
            Self::AudioTooShort => write!(
                f,
                "audio file too short, must have minimum 128 discrete samples"
            ),
            Self::NotADirectory => {
                write!(f, "provided path does not point to an existing directory")
            }
        }
    }
}
impl std::error::Error for CliSampleErrors {
    fn description(&self) -> &str {
        match *self {
            CliSampleErrors::InvalidOptBitDepth => {
                "invalid bit depth setting, only '16' or '24' are supported"
            }
            CliSampleErrors::TooManySlices => "maximum number of slices in a sample file is 64",
            CliSampleErrors::AudioTooShort => {
                "audio file too short, must have minimum 128 discrete samples"
            }
            CliSampleErrors::NotADirectory => {
                "provided path does not point to an existing directory"
            }
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CliSampleErrors::InvalidOptBitDepth => None,
            CliSampleErrors::TooManySlices => None,
            CliSampleErrors::AudioTooShort => None,
            CliSampleErrors::NotADirectory => None,
        }
    }
}

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
pub fn show_ot_file_bytes(
    path: &Path,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let raw = read_type_from_bin_file::<SampleAttributesRawBytes>(path)?;

    let bytes = get_bytes_slice(raw.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

// todo: tests
// TODO: maybe options for setting stuff like tempo to non-default? change to a
//       "new" command?
/// Create a default OctaTrack sample attributes file for some wav file
pub fn create_default_ot_file_for_wav_file(path: &Path) -> RBoxErr<()> {
    let mut ot_path = path.to_path_buf();
    ot_path.set_extension("ot");

    let wavfile = WavFile::from_path(path)?;

    let ot_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &SampleTrimConfig {
            start: 0,
            end: wavfile.len,
            length: wavfile.len,
        },
        &SampleLoopConfig {
            start: 0,
            length: wavfile.len,
            mode: SampleAttributeLoopMode::default(),
        },
        &Slices {
            slices: from_fn(|_| Slice {
                trim_end: 0,
                trim_start: 0,
                loop_start: 0,
            }),
            count: 0,
        },
    );

    type_to_yaml_file::<SampleAttributes>(&ot_data?, &ot_path)?;

    Ok(())
}

// todo: test
/// Create Nx default OctaTrack sample attributes file for Nx wav files
pub fn create_default_ot_files_for_wav_files(paths: &[PathBuf]) -> RBoxErr<()> {
    for path in paths {
        create_default_ot_file_for_wav_file(path)?;
    }
    Ok(())
}

/// Create Octatrack samplechain file-pairs from a loaded yaml config.
pub fn create_samplechains_from_yaml(yaml_conf_fpath: &Path) -> RBoxErr<()> {
    let chain_conf = yaml_file_to_type::<YamlChainCreate>(yaml_conf_fpath)?;
    println!("Creating sample chains from yaml config.");
    prnt_stdout_new_section();
    trace!("Yaml contents: {chain_conf:#?}");

    for chain_config in &chain_conf.chains {
        println!("Creating chain: name={:#?}", &chain_config.chain_name);
        let mut ot_opts = None;
        if chain_config.octatrack_settings.is_some() {
            ot_opts = chain_config.clone().octatrack_settings;
        }

        let mut audio_opts = None;
        if chain_config.audio_processing.is_some() {
            audio_opts = chain_config.clone().audio_processing;
        }

        let mut format_opts = None;
        if chain_config.audio_format.is_some() {
            format_opts = chain_config.clone().audio_format;
        }

        batch_create_samplechains(
            &chain_config.audio_file_paths,
            &chain_conf.global_settings.out_dir_path,
            &chain_config.chain_name,
            ot_opts,
            audio_opts,
            format_opts,
        )?;
        prnt_stdout_new_section();
    }

    Ok(())
}

// Deliberately does not include the trim / loop length settings
// as they are mostly irrelevant for creating sample chains
/// Options that can be provided to `create_samplechain_from_pathbufs_only` for
/// controlling some of the global Octatrack sample attributes settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SampleChainOpts {
    bpm: Option<f32>,
    gain: Option<f32>,
    timestretch_mode: Option<SampleAttributeTimestrechMode>,
    trig_quantization_mode: Option<SampleAttributeTrigQuantizationMode>,
    loop_mode: Option<SampleAttributeLoopMode>,
}

/// Options that control audio processing of each slice in a sample chain
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SliceProcOpts {
    /// normalize individual slices
    pub normalize: Option<bool>,
    /// %-age length of linear fade in to apply to slices
    pub fade_in_percent: Option<f32>,
    /// %-age length of linear fade out to apply to slices
    pub fade_out_percent: Option<f32>,
    /// Resampled time stretch factor
    pub time_stretch: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub(crate) enum FileFormat {
    Wav,
    // TODO
    // Aiff
}

/// Options that control the output file formats
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct FileFormatOpts {
    /// normalize individual slices
    pub bit_depth: Option<u16>,
    /// %-age length of linear fade in to apply to slices
    pub format: Option<f32>,
}

/// Get a new `Slices` struct, given an arbitrary length `Vec` of `WavFile`s.
pub fn create_slices_from_wavfiles(wavfiles: &[WavFile], offset: u32) -> RBoxErr<Slices> {
    let mut new_slices: Vec<Slice> = Vec::new();
    let mut off = offset;

    if wavfiles.len() > 64 {
        return Err(CliSampleErrors::TooManySlices.into());
    }

    for w in wavfiles.iter() {
        new_slices.push(Slice::new(off, off + w.len, None)?);
        off += w.len;
    }

    let default_slice = Slice::new(0, 0, None)?;

    let mut slices_arr: [Slice; 64] = [default_slice; 64];
    for (i, slice_vec) in new_slices.iter().enumerate() {
        slices_arr[i] = *slice_vec;
    }

    Ok(Slices {
        slices: slices_arr,
        count: wavfiles.len() as u32,
    })
}

/// Create sample chains
pub fn batch_create_samplechains(
    wav_fps: &[PathBuf],
    outdir_path: &Path,
    outchain_name: &String,
    ot_options: Option<SampleChainOpts>,
    audio_options: Option<SliceProcOpts>,
    format_options: Option<FileFormatOpts>,
) -> RBoxErr<()> {
    // defaults for octatrack options
    let mut bpm: f32 = 120.0;
    let mut gain: f32 = 0.0;
    let mut tsmode = SampleAttributeTimestrechMode::default();
    let mut tqmode = SampleAttributeTrigQuantizationMode::default();
    let mut lpmode = SampleAttributeLoopMode::default();

    // parse options
    if let Some(opts) = ot_options {
        if let Some(opt) = opts.bpm {
            bpm = opt;
        }
        if let Some(opt) = opts.gain {
            gain = opt;
        }
        if let Some(opt) = opts.timestretch_mode {
            tsmode = opt;
        }
        if let Some(opt) = opts.trig_quantization_mode {
            tqmode = opt;
        }
        if let Some(opt) = opts.loop_mode {
            lpmode = opt;
        }
    };

    // defaults for output file format options
    let mut bit_depth = 16;
    // TODO: AIFF not currently supported
    // let mut format = FileFormat::Wav;

    if let Some(opts) = format_options {
        if let Some(opt) = opts.bit_depth {
            if !ALLOWED_BIT_DEPTHS.contains(&opt) {
                return Err(CliSampleErrors::InvalidOptBitDepth.into());
            }
            bit_depth = opt;
        }
    };

    println!(
        "Using sample settings: bpm={} gain={} tsmode={:?} tqmode={:?} loopmode={:?}",
        bpm, gain, tsmode, tqmode, lpmode,
    );

    for (idx, fps) in wav_fps.chunks(64).enumerate() {
        let mut wavfiles: Vec<WavFile> = vec![];
        for fp in fps {
            let w = WavFile::from_path(fp)?;
            wavfiles.push(w);
        }

        // at least one file is stereo, so all files must be converted to stereo
        if !wavfiles.iter().map(|x| x.spec.channels).all_equal() {
            // convert any mono files to interleaved stereo samples
            for w in wavfiles.iter_mut() {
                if w.spec.channels == 1 {
                    w.mono_to_stereo_interleaved()?;
                }
            }
        }

        // modify each slice's audio samples according to any options
        if let Some(opts) = &audio_options {
            if let Some(opt) = opts.time_stretch {
                for w in wavfiles.iter_mut() {
                    w.resample_time_stretch(opt)?;
                }
            }
            if let Some(opt) = opts.fade_in_percent {
                for w in wavfiles.iter_mut() {
                    w.linear_fade_in(opt)?;
                }
            }
            if let Some(opt) = opts.fade_out_percent {
                for w in wavfiles.iter_mut() {
                    w.linear_fade_out(opt)?;
                }
            }
            if opts.normalize.is_some() {
                for w in wavfiles.iter_mut() {
                    w.normalize()?;
                }
            }
        };

        let chain_channels = wavfiles[0].spec.channels;

        let chain_len_interleaved = wavfiles.iter().map(|x| x.len).sum::<u32>();
        let chain_samples = wavfiles
            .iter()
            .flat_map(|x| x.samples.clone())
            .collect::<Vec<_>>();

        // mixed channels issue should be dealt with, so can use first audio
        // file as an indicator of whether we're mono or stereo
        let wavspec = hound::WavSpec {
            channels: chain_channels,
            sample_rate: ALLOWED_SAMPLE_RATE,
            bits_per_sample: bit_depth,
            sample_format: hound::SampleFormat::Int,
        };

        let chain_wav = WavFile {
            spec: wavspec,
            len: chain_len_interleaved,
            samples: chain_samples,
            file_path: Default::default(),
        };

        trace!("Making chain: {}", idx + 1);
        let slices = create_slices_from_wavfiles(&wavfiles, 0)?;

        trace!("Calculating bar length: chainIdx={}", idx + 1);
        let bars = get_bin_nbars_ileaved_wavfiles(&wavfiles, &bpm, chain_channels)?;

        trace!("Setting up sample attributes data: chainIdx={}", idx + 1);
        let trim_config = SampleTrimConfig {
            start: 0,
            end: chain_wav.len,
            length: bars,
        };

        let loop_config = SampleLoopConfig {
            start: 0,
            length: bars,
            mode: lpmode,
        };

        let chain_data = SampleAttributes::new(
            &bpm,
            &tsmode,
            &tqmode,
            &gain,
            &trim_config,
            &loop_config,
            &slices,
        )?;

        trace!("Modifying file paths: chainIdx={}", idx + 1);

        // always suffix the batch number onto the same chain
        // if someone generates a sample chain with 64 files the first time,
        // then attempts to create a second version later on with 65 files then
        // they'll end up with different file names and have to manually load the
        // samples to every slot where they've been used.
        //
        // example use-case: users creating "godchains" containing all their
        // favourite samples. If they start with <64 samples, subsequently
        // adding more and regenerating their "godchain" will mean they have to
        // edit the existing sample slot in all their projects.
        //
        // so just always suffix `-{idx}` to the filenames.
        let base_outchain_path =
            outdir_path
                .to_path_buf()
                .join(format!["{}-{}", outchain_name, idx + 1]);

        let mut wav_sliced_outpath = base_outchain_path;
        wav_sliced_outpath.set_extension("wav");
        chain_wav.to_path(&wav_sliced_outpath)?;
        println!("Creating chain audio file: {wav_sliced_outpath:#?}");

        let mut ot_outpath = wav_sliced_outpath.clone();
        ot_outpath.set_extension("ot");

        write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath)?;
        println!("Created chain attributes file: {ot_outpath:#?}");
    }

    println!("Created sample chain: name={outchain_name:#?}");
    Ok(())
}

// todo: needs tests
/// Extract a slices from a sliced sample chain into individual samples.
pub fn deconstruct_samplechain_from_paths(
    audio_fpath: &Path,
    attributes_fpath: &Path,
    out_dirpath: &Path,
) -> RBoxErr<Vec<PathBuf>> {
    if !out_dirpath.is_dir() {
        return Err(CliSampleErrors::NotADirectory.into());
    }

    let wavfile = WavFile::from_path(audio_fpath)?;

    let attrs = read_type_from_bin_file::<SampleAttributes>(attributes_fpath)?;

    // todo: this feels fragile
    let base_sample_fname = audio_fpath
        .file_stem()
        .unwrap_or(std::ffi::OsStr::new("deconstructed_samplechain"))
        .to_str()
        .unwrap_or("deconstructed_samplechain");

    let mut out_fpaths: Vec<PathBuf> = vec![];

    for i in 0..attrs.slices_len {
        let slice = attrs.slices[i as usize];
        // wavs/hound interleaves channel data into a single vector
        // so need to multiply slices starts/ends by n_channels to get the actual
        // start position
        // TODO: Why do i not do this for creating sample chains?
        let strt_ileave = slice.trim_start * (wavfile.spec.channels as u32);
        let end_ileave = slice.trim_end * (wavfile.spec.channels as u32);
        let w = wavfile.samples[(strt_ileave as usize)..(end_ileave as usize)].to_vec();
        let wav_len = slice.trim_end - slice.trim_start;

        let wavslice = WavFile {
            spec: wavfile.spec,
            len: wav_len,
            samples: w,
            file_path: std::env::temp_dir().join("dummy.wav"),
        };

        let sample_fname = format!("{base_sample_fname}-{i:#?}");
        let mut out_fpath = out_dirpath.to_path_buf().join(sample_fname);
        out_fpath.set_extension("wav");
        wavslice.to_path(&out_fpath)?;
        out_fpaths.push(out_fpath);
    }
    Ok(out_fpaths)
}

pub fn deconstruct_samplechains_from_yaml(yaml_conf_fpath: &Path) -> RBoxErr<()> {
    let chain_conf = yaml_file_to_type::<YamlChainDeconstruct>(yaml_conf_fpath)?;

    println!("Deconstructing sample chains from yaml config.");
    trace!("Yaml contents: {chain_conf:#?}");

    for chain_config in &chain_conf.chains {
        deconstruct_samplechain_from_paths(
            &chain_config.sample,
            &chain_config.otfile,
            &chain_conf.global_settings.out_dir_path,
        )?;
    }

    Ok(())
}

/// Given a wavfile, create Nx random slices stored in a sample attributes file.
pub fn create_randomly_sliced_sample(wav_fp: &Path, n_slices: usize) -> RBoxErr<()> {
    if n_slices > 64 {
        return Err(CliSampleErrors::TooManySlices.into());
    };

    let wavfile = WavFile::from_path(wav_fp)?;

    if wavfile.len < 128 {
        return Err(CliSampleErrors::AudioTooShort.into());
    };

    let mut rng = rand::thread_rng();

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };

    let mut slices_arr: [Slice; 64] = [default_slice; 64];

    #[allow(clippy::needless_range_loop)]
    for i in 0..n_slices {
        let trim_start: u32 = rng.gen_range(0..=(wavfile.len - 64));
        // clipped random length so we don't always end up with long slices
        // at the start and shorter ones at the end
        let rndlen = trim_start + (wavfile.len / n_slices as u32).max(64);
        let trim_end: u32 = rng.gen_range(trim_start..=rndlen);
        let loop_start: u32 = 0xFFFFFFFF;

        let slice = Slice {
            trim_start,
            trim_end,
            loop_start,
        };

        slices_arr[i] = slice;
    }

    let slices = Slices {
        slices: slices_arr,
        count: n_slices as u32,
    };

    let bars = get_otsample_nbars_from_wavfile(&wavfile, &120.0)?;

    let trim_config = SampleTrimConfig {
        start: 0,
        end: wavfile.len,
        length: bars,
    };

    let loop_config = SampleLoopConfig {
        start: 0,
        length: bars,
        mode: SampleAttributeLoopMode::default(),
    };

    let chain_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &trim_config,
        &loop_config,
        &slices,
    )?;

    let mut ot_outpath = wav_fp.to_path_buf();
    ot_outpath.set_extension("ot");

    write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath)?;
    println!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

/// Given a wavfile, create Nx equal length slices stored in a sample attributes file.
pub fn create_equally_sliced_sample(wav_fp: &Path, n_slices: usize) -> RBoxErr<()> {
    if n_slices > 64 {
        return Err(CliSampleErrors::TooManySlices.into());
    };

    let wavfile = WavFile::from_path(wav_fp)?;

    if wavfile.len < 128 {
        return Err(CliSampleErrors::AudioTooShort.into());
    };

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };
    let mut slices_arr: [Slice; 64] = [default_slice; 64];
    let len = wavfile.len / (n_slices as u32);

    #[allow(clippy::needless_range_loop)]
    for i in 0..n_slices {
        let trim_start: u32 = (i as u32) * len;
        let trim_end: u32 = trim_start + len;
        let loop_start: u32 = 0xFFFFFFFF;

        let slice = Slice {
            trim_start,
            trim_end,
            loop_start,
        };

        slices_arr[i] = slice;
    }

    let slices = Slices {
        slices: slices_arr,
        count: n_slices as u32,
    };

    let bars = get_otsample_nbars_from_wavfile(&wavfile, &120.0)?;

    let trim_config = SampleTrimConfig {
        start: 0,
        end: wavfile.len,
        length: bars,
    };

    let loop_config = SampleLoopConfig {
        start: 0,
        length: bars,
        mode: SampleAttributeLoopMode::default(),
    };

    let chain_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &trim_config,
        &loop_config,
        &slices,
    )?;

    let mut ot_outpath = wav_fp.to_path_buf();
    ot_outpath.set_extension("ot");

    write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath)?;
    println!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

pub fn create_index_samples_dir_simple(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    println!("Indexing samples directory with 'simple' output: path={samples_dir_path:#?}");
    let sample_index = SamplesDirIndexSimple::new(samples_dir_path)?;

    if let Some(yaml_fp) = yaml_file_path {
        type_to_yaml_file(&sample_index, yaml_fp)?;
    };

    Ok(())
}

pub fn create_index_samples_dir_full(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    let sample_index = SamplesDirIndexFull::new(samples_dir_path)?;

    if let Some(yaml_fp) = yaml_file_path {
        type_to_yaml_file(&sample_index, yaml_fp)?;
    };

    Ok(())
}
