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
- Copy banks from one project to another, moving relevant project sample slots with the bank
- Create slice sample chains from multiple WAV files
- Deconstruct a slice sample chain into multiple WAV files
- Create a linear/random slice grid for an exisitng wav file
- Consolidate Project sample slot files into Project audio folder
- Centralize Project sample slot files into Set's Audio pool folder
- Purge Project's audio folder of any audio files not present in Project sample slots
- Write a new data file from YAML (project/bank/sample)
- Dump data to YAML (project/bank/sample/part/pattern)
- Inspect various data files (project/arrangement/bank/part/pattern/sample) 
- List samples slots being used in a project
- Find compatible WAV files in a local directory and write their file paths to a YAML file
- Scan a Compact Flash card and dumpy Octatrack file information to YAML

### Repo structure

- `./assets/` contains the project logo.
- `./data/tests/` contains data for running the tests, or trying out some commands.
- `./examples/yaml/confs` contains some examples on how to do batch operations for copying banks and creating sample chains.
- `./examples/yaml/dumps` contains some example yaml dumps of Octatrack data structures
- `./serde_octatrack` contains the library for serialization and deserialization of octatrack files. 
- `./src` contains the CLI commands code.

### TODOs

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

- Fixup the sample chain gain settings so they're easier to understand (not being translated properly for humans atm).
- Sort out CLI optional arguments via CLAP.
- Make the code more idiomatic / 'clean' / optimised.
- Cross-compilation / CI builds on Windows 10/11 and macOS.
- Consolidation:
  - Audio files from a Project into a Set's Audio Pool.
  - Audio files from a Set Audio Pool into a Project (only get what is needed).
  - Audio files from all Project into a Set Audio Pools.
  - Audio files from all Set Audio Pools into Projects.
- Handle AIFF files (and switching between AIFF and WAV within the code -- probably needs an abstraction).
- Inspect RIFF header issues with `hound` on samples from mars files
- Sane Logging messages
- Sane Error handling
- CI release builds ($$$$)
- Improve test coverage and test cases (negative tests).

### What this software is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

