use hound::Sample;
// use std::error::Error;
use serde_yml::Error as SerdeYmlError;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, collections::HashMap};
use log::{error, info, warn, debug};
// use hex_literal::hex;


use crate::otsample::options::{
    SampleLoopModes,
    SampleTrigQuantizationModes,
    SampleTimestrechModes,
};
use crate::otsample::{
    SampleChain,
    SampleLoopConfig,
    SampleTrimConfig,
    Slices,
    get_otsample_n_bars_from_wavfiles,
};
use crate::wavfile::{
    WavFile,
    chain_wavfiles_64_batch,
};


// TODO: normalization
// TODO: should normalization be per chain? separate settings struct?
/// This YAML section affects all chains being created with this config.
#[derive(Debug, Serialize, Deserialize)]
struct YamlChainConfigGlobalSettings {
    normalize: bool,
    out_dir_path: PathBuf,
}


// Deliberately does not include the trim / loop length settings
// as they are mostly irrelevant for creating sample chains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainConfigChainsOctatrackSettings {
    pub bpm: f32,  // this will get multiplied by 24
    pub gain: u8,  // default is 48, minimum is 24, maximum is 72
    pub timestretch_mode: SampleTimestrechModes,  // needs to be one of the enum values
    pub loop_mode: SampleLoopModes,
    pub quantization_mode: SampleTrigQuantizationModes,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct YamlChainConfigSamplechain {
    pub octatrack_settings: YamlChainConfigChainsOctatrackSettings,
    pub sample_file_paths: Vec<PathBuf>,
    // use this for the .wav + .ot file names
    pub chain_name: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct YamlChainConfig {
    global_settings: YamlChainConfigGlobalSettings,
    chains: Vec<YamlChainConfigSamplechain>,
}


#[derive(Debug, Serialize, Deserialize)]
struct YamlSamplechains {
    conf: YamlChainConfig,
    chains: Vec<SampleChain>,
}

impl YamlSamplechains {
    pub fn from_pathbuf(yaml_file_path: &PathBuf) -> Result<Self, ()> {
        debug!("Reading YAML config file: {:#?}", &yaml_file_path);

        let f = std::fs::File
            ::open(yaml_file_path)
            .unwrap()
            ;

        let chain_config_data_load: Result<YamlChainConfig, SerdeYmlError> = serde_yml
            ::from_reader(f)
            ;

        if chain_config_data_load.is_err() {
            error!("Could not load YAML config: {:#?}", chain_config_data_load.err().unwrap().to_string());
            panic!();
        }

        let chain_config_data = chain_config_data_load.unwrap();

        debug!("Loaded YAML config file: {:#?}", &chain_config_data);

        debug!("Starting chain building ...");

        let chains : HashMap<PathBuf, SampleChain> = HashMap::new();
        for chain_config in chain_config_data.chains {

            let mut wavfiles: Vec<WavFile> = Vec::new();
            for wav_file_path in chain_config.sample_file_paths {
                let wavfile = WavFile::from_file(&wav_file_path).unwrap();
                wavfiles.push(wavfile);
            };

            // Vec<WavFile> of Wavs, with 64 samples per wavfile
            // Need this to be a hashmap (or struct) for the 64Batched data
            let wavfiles_batched: HashMap<WavFile, Vec<WavFile>> = chain_wavfiles_64_batch(&wavfiles).unwrap();

            for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
                debug!(
                    "Created {:1?} WAV chains from {:2?} samples",
                    &wavfiles_batched.len(),
                    &wavfiles.len()
                );
    
                let slices = Slices::from_wavfiles(&vec_wavs, &0).unwrap();
    
                // let chain = SampleChain::from_yaml_conf(&chain_config).unwrap();
                // chains.insert(chain);
    
                // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
                let bars = get_otsample_n_bars_from_wavfiles(&vec_wavs, &125.0).unwrap();
    
                let trim_config = SampleTrimConfig {
                    start: 0,
                    end: single_wav.len,
                    length: bars,
                };
        
                let loop_config = SampleLoopConfig {
                    start: 0,
                    length: bars,
                    mode: 0,
                };
        
                debug!("Converting config values for OT file: {:#?}", slices.count);
        
                let quantization_value: u8 = chain_config.octatrack_settings.quantization_mode.value().unwrap() as u8;
                let timestretch_value: u32 = chain_config.octatrack_settings.timestretch_mode.value().unwrap();
                let tempo_u32 = chain_config.octatrack_settings.bpm as u32;
                let gain_u16 = chain_config.octatrack_settings.gain as u16;
        
                // TODO: Why is this unused?
                let loopmode_value: u32 = chain_config.octatrack_settings.loop_mode.value().unwrap();
        
                // tempo: &u32,
                // stretch: &u32,
                // quantization: &u8,
                // gain: &u16,
                // trim_config: &SampleTrimConfig,
                // loop_config: &SampleLoopConfig,
                // slices: &Slices,
        
                debug!("Creating sample chain struct: {:#?}", slices.count);
        
                let chain_data = SampleChain
                    ::new(
                        &tempo_u32,
                        &timestretch_value,
                        &quantization_value,
                        &gain_u16,
                        &trim_config,
                        &loop_config,
                        &slices
                    )
                    .unwrap()
                    ;
        
                debug!("Sample chain struct: {:#?}", chain_data);

                let fstem = chain_config.chain_name + &format!("{:?}", idx);

                let base_outchain_path = chain_config_data
                    .global_settings
                    .out_dir_path
                    .join(fstem);
        
                let mut ot_outpath = base_outchain_path.clone();
                let mut wav_sliced_outpath = base_outchain_path.clone();
        
                ot_outpath.set_extension("ot");
                wav_sliced_outpath.set_extension("wav");
        
                let _chain_res = chain_data.to_file(&ot_outpath);
        
                debug!("Wrote Chain.");
        
                let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
        
                debug!("Wrote Wav Sliced: {:#?}", single_wav);

                // TODO: Won't work as need to append chains to a vector first.
                chains.insert(ot_outpath, chain_data);

            };


        };


        

    }
}


pub fn create_chains_from_yaml(yaml_file_path: &PathBuf) -> Result<(), ()> {

    debug!("Reading YAML config file: {:#?}", &yaml_file_path);

    let f = std::fs::File
        ::open(yaml_file_path)
        .unwrap()
        ;

    let chain_config_data_load: Result<YamlChainConfig, SerdeYmlError> = serde_yml
        ::from_reader(f)
        ;

    if chain_config_data_load.is_err() {
        error!("Could not load YAML config: {:#?}", chain_config_data_load.err().unwrap().to_string());
        panic!();
    }

    let chain_config_data = chain_config_data_load.unwrap();

    debug!("Loaded YAML config file: {:#?}", &chain_config_data);

    debug!("Starting chain building ...");

    for chain_config in chain_config_data.chains {
        debug!("Processing chain data: {:#?}", chain_config);

        debug!("Loading Nx WAV files: {:#?}", chain_config.sample_file_paths.len());

        let mut wavfiles: Vec<WavFile> = Vec::new();
        for wav_file_path in chain_config.sample_file_paths {
            let wavfile = WavFile::from_file(&wav_file_path).unwrap();
            wavfiles.push(wavfile);
        };

        debug!("Loaded Nx WAV files: {:#?}", wavfiles.len());

        if wavfiles.len() > 64 {
            warn!("More than 64 samples -- need to overflow here! TODO!");
        };

        debug!("Creating slices from WAV files: {:#?}", wavfiles.len());

        let slices = Slices::from_wavfiles(&wavfiles, &0).unwrap();

        debug!("Creating sliced WAV file: {:#?}", &wavfiles.clone().into_iter().map(|x: WavFile| x.len).collect::<Vec<u32>>());

        let wav_sliced = chain_wavfiles_64_batch(&wavfiles).unwrap();

        debug!("Sliced WAV file sample len: {:#?}", &wav_sliced.len);

        debug!("Working out bar length / trim config / loop config from slices: {:#?}", slices.count);

        let bars = get_otsample_n_bars_from_wavfiles(&wavfiles, &125.0).unwrap();

        let trim_config = SampleTrimConfig {
            start: 0,
            end: wavfiles.iter().map(|x: &WavFile| x.len as u32).sum(),
            length: bars,
        };

        let loop_config = SampleLoopConfig {
            start: 0,
            length: bars,
            mode: 0,
        };

        debug!("Converting config values for OT file: {:#?}", slices.count);

        let quantization_value: u8 = chain_config.octatrack_settings.quantization_mode.value().unwrap() as u8;
        let timestretch_value: u32 = chain_config.octatrack_settings.timestretch_mode.value().unwrap();
        let tempo_u32 = chain_config.octatrack_settings.bpm as u32;
        let gain_u16 = chain_config.octatrack_settings.gain as u16;

        // TODO: Why is this unused?
        let loopmode_value: u32 = chain_config.octatrack_settings.loop_mode.value().unwrap();

        // tempo: &u32,
        // stretch: &u32,
        // quantization: &u8,
        // gain: &u16,
        // trim_config: &SampleTrimConfig,
        // loop_config: &SampleLoopConfig,
        // slices: &Slices,

        debug!("Creating sample chain struct: {:#?}", slices.count);

        let chain_data = SampleChain
            ::new(
                &tempo_u32,
                &timestretch_value,
                &quantization_value,
                &gain_u16,
                &trim_config,
                &loop_config,
                &slices
            )
            .unwrap()
            ;

        debug!("Sample chain struct: {:#?}", chain_data);

        let base_outchain_path = chain_config_data
            .global_settings
            .out_dir_path
            .join(chain_config.chain_name);

        let mut ot_outpath = base_outchain_path.clone();
        let mut wav_sliced_outpath = base_outchain_path.clone();

        ot_outpath.set_extension("ot");
        wav_sliced_outpath.set_extension("wav");

        let _chain_res = chain_data.to_file(&ot_outpath);

        debug!("Wrote Chain.");

        let _wav_slice_res = wav_sliced.to_file(&wav_sliced_outpath);

        debug!("Wrote Wav Sliced: {:#?}", wav_sliced);

    };

    Ok(())

}

