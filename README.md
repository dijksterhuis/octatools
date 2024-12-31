# OctaTools

![Utilities for the Elektron Octatrack DPS-1](assets/logo.png "OctaTools")

Utilities for the [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer).
Currently only tested against the latest version of the Octatrack OS 1.40B.

### Warning

This has mostly been a **learning** project for me to get to grips with Rust. 
**Please do not expect high quality rust code by default**.
Use at your own risk -- there are still edge cases and some jank. 
Only Linux OSs are supported at the moment.

### Repo structure

- `./assets/` -- project logo
- `./data/tests/` -- data used when running the tests, may also be useful when trying out `octatools-bin` commands
- `./examples/` -- examples of yaml data used / created by the `octatools-bin` binary executable
- `./octatools-bin` -- CLI binary executable code to interact with octatrack data files
- `./serde_octatrack` -- serialization/deserialization library for octatrack data files 

### What this is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer

## `octatools-bin` -- Octatools (CLI Binary Executable)

Command line binary executable to interact with Octatrack data files.

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

## `octatools-gui` -- Octatools (CLI Binary Executable)

Eventually I'd like to create a simple cross-platform GUI application for people to perform all the octatools tasks alongside a cli binary.
This package is a placeholder to act as a guilt trip every time I look at the repository.

## `serde_octatrack` -- Ser/De library

Library used for reading/writing octatrack binary data.
Most of this is just the [`serde` crate](https://serde.rs).

### Current Features (mostly working-ish)
- Deserialise all Octatrack data structures from binarized data
- Serialise most Octatrack data structures to binarized data (cannot serialise arrangements yet?)
- Serialize/deserialise to/from YAML and JSON

