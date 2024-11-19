# OctaTools

![CLI Tools for the Elektron Octatrack DPS-1](assets/logo.png "OctaTools")

CLI tools for the [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer)

Only tested against the latest version of the Octatrack OS 1.40B (?).

### Warnings

This has mostly been a **learning** project for me to mess around and get to grips with Rust. 
**Do not expect high quality rust code just now**.

Use at your own risk -- there are still edge cases and some jank.

Only Linux supported at the moment.


### Current Features (mostly working-ish)

- Copy 1x Bank to a new location via CLI.
```bash
octatools transfer bank <SRC_BANK_FILE_PATH> <DEST_BANK_FILE_PATH>
```

- Copy Nx Banks to new locations via YAML.
```bash
octatools transfer banks <YAML_CONFIG_FILE_PATH>
```

- Create 1x sliced Sample Chain via CLI
```bash
octatools chains create-chain <CHAIN_NAME> <OUT_DIR_PATH> [WAV_FILE_PATHS]...
```
- Create Nx sliced Sample Chains via YAML
```bash
octatools chains create-chains <YAML_CONFIG_FILE_PATH>
```

- Create an linear slice grid for an exisitng wav file via CLI
```bash
octatools chains grid-linear <WAV_FILE_PATH> <OUT_OT_PATH> <N_SLICES>
```

- Create an random slice grid for an exisitng wav file via CLI
```bash
octatools chains grid-random <WAV_FILE_PATH> <OUT_OT_PATH> <N_SLICES>
```

- Deconstruct 1x sliced Sample Chain into a WAV file per slice via the CLI
```bash
octatools chains deconstruct-chain <OT_FILE_PATH> <AUDIO_FILE_PATH> <OUT_DIR_PATH>
```

- Deconstruct Nx sliced Sample Chain into a WAV file per slice via YAML
```bash
octatools chains deconstruct-chains <YAML_CONFIG_FILE_PATH>
```

- Inspect various project data files (project/bank/parts/part/pattern/pattern/arrangement/sample) 
```bash
octatools inspect project <PATH_TO_PROJECT_FILE>
octatools inspect sample <PATH_TO_OT_FILE>
octatools inspect arrangement <PATH_TO_ARRANGEMENT_FILE>
octatools inspect bank <PATH_TO_BANK_FILE>
octatools inspect part-saved <PATH_TO_BANK_FILE> [<PART_NUMBER>...]
octatools inspect part-unsaved <PATH_TO_BANK_FILE> [<PART_NUMBER>...]
octatools inspect pattern <PATH_TO_BANK_FILE> [<PATTERN_NUMBER>...]
```

- List samples slots being used in a project
```bash
octatools list project-slots <PATH_TO_PROJECT_FILE>
```

- Find compatible WAV files in a local directory and write their file paths to a YAML file
```bash
# Output a simple YAML list of compatible files, 
# can be copied and pasted into the YAML config for
# the `octatools chains create-chains` command
octatools index samplesdir-simple <SAMPLES_DIR_PATH> [OUTPUT_YAML_FILE_PATH]

# Generate a more full analysis of available samples
octatools index samplesdir-full <SAMPLES_DIR_PATH> [OUTPUT_YAML_FILE_PATH]
```

- Scan a Compact Flash card and dump full Project information to YAML (warning, generates multi-GB YAML output!)
```bash
# Generate a YAML dump of all Octatrack data files on a Compact Flash card.
# CF_CARD_PATH is the path to the root of the Compact Flash Card
octatools index cfcard <CF_CARD_PATH> [OUTPUT_YAML_FILE_PATH]
```
### Repo structure

- `./assets/` contains the project logo.
- `./data/tests/` contains data for running the tests, or trying out some commands.
- `./examples/yaml/` contains some examples on how to do batch operations for copying banks and creating sample chains.
- `./serde_octatrack` contains the library for serialization and deserialization of octatrack files. 
- `./src` contains the CLI commands code.

### TODOs / Other Ideas

#### `serde_octatrack`
- Ser/De to Enum/String/etc types instead of u8 -- possible with `serde_repr`, but don't know if it's desirable to do this ... adds a lot of complexity for now.
- PyO3 bindings for creating a python interface to `serde_octatrack`
- Finish reverse engineering files 
  - Banks:
    - `MidiTrackParameterLocks.unknown` --> space for sample locks? but no samples for MIDI.
    - `AudioTrackTrigs.unknown_1`
    - `AudioTrackTrigs.unknown_2`
    - `AudioTrackTrigs.unknown_3` --> big 192 length block?!
    - `MidiTrackTrigMasks.unknown` --> looks like trig mask, but no trig trypes remain?
    - `MidiTrackTrigs.unknown_1`
    - `MidiTrackTrigs.unknown_2` --> no idea what this is, some kind of mask?
    - `MidiTrackTrigs.unknown_3` --> big 128 length block?!
    - `Pattern.unknown`
    - `AudioTrackSceneParams.unknown_1` --> seems the underlying machine OS code re-uses the same data structure in several places (this looks like sample locks?)
    - `AudioTrackSceneParams.unknown_2` --> seems the underlying machine OS code re-uses the same data structure in several places (this looks like sample locks?)
    - `SceneTrackXlv.unknown_` only 2 length
    - `MidiTrackParamsValues.unknown` --> space for sample select?
  - Projects:
    - `ProjectSettings.midi_soft_thru` --> what is this for? no menu option named like this.
    - `MidiControlMidiPage.midi_midi_track_cc_in` -- no menu option for this?
    - `ProjectStates.track_othermode` -- ??
    - `ProjectStates.midi_mode` -- no idea.
  - Arrangements:
    - `ArrangementBlock.unknown_1`
    - `ArrangementBlock.unknown_2`
    - `ArrangementFile.unknown_1`
    - `ArrangementFile.unknown_2`
- More tests.
- Even more tests.

### `octatools`

- Clean up CLI commands, sort out CLI options etc via CLAP.
  - go the `docker container/volume/network` route
    - `octatools project`
    - `octatools sample`
    - `octatools bank`
    - `octatools arrangement`
    - etc.
- Refactor the copy_bank code -- it is a mess. 
- Fixup the sample chain gain settings so they're easier to understand (not being translated properly for humans atm).
- Make the code more idiomatic / 'clean' / optimised.
- Cross-compilation / CI builds on Windows 10/11 and macOS.
- Consolidation:
  - Audio files from a Project into a Set's Audio Pool.
  - Audio files from a Set Audio Pool into a Project (only get what is needed).
  - Audio files from all Project into a Set Audio Pools.
  - Audio files from all Set Audio Pools into Projects.
- Minor sample editing for sample chains (normalisation, fades, reverses, etc).
- Handle AIFF files (and switching between AIFF and WAV within the code -- probably needs an abstraction).
- List all Sets, Projects, Samples. 
- Sane Logging messages
- Sane Error handling
- Work out some better chainer CLI command names.
- Templates
  - Projects -- YAML project spec -> Octatrack project file
  - Parts -- load the same template onto all parts in all banks in a project 
- Inspect RIFF header issues with `hound` on samples from mars files
- CI release builds ($$$$)
- Deal with over use of `.clone()` absolutely everywhere.
- More tests.
- Even more tests.

### What this software is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

