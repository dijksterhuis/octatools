# OctaTools

![CLI Tools for the Elektron Octatrack DPS-1](assets/logo.png "OctaTools")

CLI Tools for the [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer)

Only tested against the latest version of the Octatrack OS 1.40B (?).

**NOTE**: This is mostly a **learning** project for me to mess around and get to grips with Rust. 
**DO NOT EXPECT HIGH QUALITY RUST CODE HERE**.

### Current Features

- Copy 1x Bank to a new location via CLI.
- Copy Nx Banks to new locations via YAML.
- Create 1x sliced Sample Chain via CLI
- Create Nx sliced Sample Chains via YAML
- Deconstruct 1x sliced Sample Chain into a WAV file per slice via the CLI
- Deconstruct Nx sliced Sample Chain into a WAV file per slice via YAML
- Inspect various project data files (project/bank/parts/part/pattern/pattern/arrangement/sample) 
- List samples slots being used in a project
- Find compatible WAV files in a local directory and write their file paths to a YAML file
- Scan a Compact Flash card and dump full Project information to YAML (warning, generates multi-GB YAML output!)

### Repo structure

- `./assets/` contains the logo thing.
- `./data/tests/` contains data for running the tests, or trying out some commands.
- `./examples/yaml/` contains some examples on how to do batch operations for copying banks and creating sample chains.
- `./serde_octatrack` contains the code for the serialization and deserialization of octatrack files. 
- `./src` contains the CLI commands code.

### TODOs / Other Ideas

- Clean up CLI commands, sort out CLI options etc via CLAP.
- Fixup the sample chain gain settings so they're easier to understand (not being translated properly for humans atm).
- List all Sets, Projects, Sample Slots, Samples. 
- Finish reverse engineering bank files (MIDI Track Pattern trigs, the massive block in Parameter Locks).
- Cross-compilation / CI builds on Windows 10/11 and macOS.
- Consolidate all audio files from Projects within a Set into a Set's Audio Pool.
- Consolidate relevant audio files from a Set's Audio Pool to other Project(s).
- Minor sample editing (normalisation, fades, reverses, etc).
- PyO3 bindings for creating a python interface to `serde_octatrack`
- Actually make the rust code idiomatic and 'clean' and optimised.
- Ser/De to Enum/String/etc types instead of u8.
- Handle AIFF files (and switching between AIFF and WAV within the code -- probably needs an abstraction).
- Logging pass over.
- Random slices for a long audio file.
- Template project command: YAML project -> Octatrack project files
- As above, but with Parts for a bank.
- Inspect RIFF header issues with `hound` on samples from mars files
- More tests.
- Even more tests.
- CI release builds
- Deal with over use of `.clone()` absolutely everywhere.

### What this software is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

