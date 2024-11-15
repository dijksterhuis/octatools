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

### TODOs

- List all Sets, Projects, Sample Slots, Samples. 
- Finish reverse engineering bank files.
- Cross-compilation / CI builds on Windows 10/11 and macOS.
- Consolidate all audio files from Projects within a Set into a Set's Audio Pool.
- Consolidate relevant audio files from a Set's Audio Pool to other Project(s).
- Minor sample editing (normalisation, fades, reverses, etc).
- PyO3 bindings for creating a python interface.
- Actually make the rust code idiomatic and 'clean' and optimised.
- Ser/De to Enum/String/etc types instead of u8.

### What this software is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

