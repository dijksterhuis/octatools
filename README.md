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
octatools chains create-chain  <CHAIN_NAME> <OUT_DIR_PATH> [WAV_FILE_PATHS]...
```
- Create Nx sliced Sample Chains via YAML
```bash
octatools chains create-chains  <YAML_CONFIG_FILE_PATH>
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
octatools inspect arrangement <PATH_TO_ARRANGEMENT_FILE>
octatools inspect bank <PATH_TO_BANK_FILE>
octatools inspect parts <PATH_TO_BANK_FILE>
# note: there are actually 8 PART sections in the data files
# the extra 4 are for storing previous states for reloads
# (I'm not sure what order the saved parts are in just yet)
octatools inspect part <PATH_TO_BANK_FILE> <PART_NUMBER>
octatools inspect patterns <PATH_TO_BANK_FILE>
octatools inspect pattern <PATH_TO_BANK_FILE> <PATTERN_NUMBER>
octatools inspect project <PATH_TO_PROJECT_FILE>
octatools inspect sample <PATH_TO_OT_FILE>
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

