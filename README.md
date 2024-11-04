# OctaTools

![CLI Tools for the Elektron Octatrack DPS-1](assets/logo.png "OctaTools")

CLI Tools for the [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer)

Only tested against the latest version of the Octatrack OS 1.40B (?).

**NOTE**: This is mostly a **learning** project for me to mess around and get to grips with Rust. 
**DO NOT EXPECT HIGH QUALITY RUST CODE HERE**.

### Current Features

- Create 1x Slice Sample Chain via CLI
- Create multiple Slice Sample Chains via YAML
- Find compatible WAV files in a local directory and write them as a list in a YAML file
- Scan a Compact Flash card and dump Project level information to YAML 

### Currently Working on

- Transferring a Bank from one Project/Set to another

### TODOs

- Deconstruct 1x Slice Sample Chain via CLI
- Deconstruct multiple Slice Sample Chains via YAML
- List all Sets, Projects, Sample Slots, Samples. 
- Reverse Engineer bank files (separate library crate for Octatrack file ser/de).
- Cross-compilation / CI builds on Windows 10/11 and macOS
- Consolidate all audio files from Projects within a Set into a Set's Audio Pool
- Consolidate relevant audio files from a Set's Audio Pool to other Project(s)
- Minor sample editing (normalisation, fades, reverses, etc).
- PyO3 bindings for creating a python interface.
- Actually make the rust code idiomatic and 'clean'.


### What this software is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

