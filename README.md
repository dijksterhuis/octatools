# OctaTools

![Utilities for the Elektron OctaTrack DPS-1](assets/logo-wide.png "OctaTools")

Utilities for the [Elektron OctaTrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer).
Only tested against the latest version of the OctaTrack OS (1.40B).

### Warning

This has mostly been a **learning** project for me to get to grips with Rust. 
**Please do not expect high quality rust code**.

Only Linux OSs are fully supported at the moment.
Most things *can* run windows, but there is some jank I need to deal with first 
(e.g. the fixed 100MB stack size on windows causes problems when copying bank files).

Use at your own risk -- there are still edge cases and some jank.

If you are worried about destroying your Octatrack projects / data files -- take a backup
copy of the compact flash card / set / project folder and work on that copy first. 
I will not take any responsibility for irretrievable data loss that occurs 
(although I may feel bad about it and will likely try to fix whatever bug caused it, if it
was a bug).

### Repo structure

- `./assets/` -- project logo stuff
- `./data/tests/` -- data used when running the tests, may also be useful when trying out
  `octatools-bin` commands
- `./examples/` -- examples of yaml data used / created by the `octatools-bin` binary 
  executable. 
- `./octatools-bin` -- CLI binary executable code to interact with OctaTrack data files
- `./octatools-derive` -- Derive macros for adding specific traits to types
- `./octatools-gui` -- Placeholder for developing a GUI application (want `octatools-bin`
  confirmed working on Windows/OSX first)
- `./octatools-lib` -- Main library containing functions and all the 
  serialization/deserialization code for OctaTrack data files 
- `./octatools-lib` -- Python extension built with PyO3 to build a python module 
- `./octatools_py` -- which can turn octatrack data files into json, and back again. Can write 
  the file to disk or can translate to/from bytes (might be useful for an application 
  based on python HTTP APIs if someone is so inclined ;])

### What this is not
- A clone of DigiChain
- A clone of OctaEdit
- A clone of Octachainer
- Stable
- Thoroughly tested
- Expertly written

## `octatools-bin` -- CLI Executable

Command line binary executable to interact with OctaTrack data files.

### Current Features (mostly working-ish)
- Copy banks from one project to another, moving relevant project sample slots with the 
  bank
- Create slice sample chains from multiple WAV files
- Deconstruct a slice sample chain into multiple WAV files
- Create a linear/random slice grid for an existing wav file
- Consolidate Project sample slot files into Project audio folder
- Centralize Project sample slot files into Set's Audio pool folder
- Purge Project's audio folder of any audio files not present in Project sample slots
- Write a new binary data file from a YAML/JSON source file (project/bank/sample)
- Dump binary data to a YAML/JSON file (project/bank/sample/part/pattern)
- Inspect various data files (project/arrangement/bank/part/pattern/sample) 
- List samples slots being used in a project
- Find compatible WAV files in a local directory and write their file paths to a YAML file
- Scan a Compact Flash card and dump OctaTrack file information to YAML

## `octatools-derive` -- Macro definitions for derive traits

**If you don't write rust code you can ignore this**.

Used to create `#[derive(XXXX)]` macros for the following:
- `#[derive(Decodeable)]` for the `octatools-lib::Decode` trait
- `#[derive(Encodeable)]` for the `octatools-lib::Encode` trait
- `#[derive(DefaultsAsArray)]` for the `octatools-lib::DefaultsArray` trait
- `#[derive(DefaultsAsArrayBoxed)]` for the `octatools-lib::DefaultsArrayBoxed` trait

See the trait descriptions for more information.

## `octatools-gui` -- GUI Application

Eventually I'd like to create a simple cross-platform GUI application containing the 
various `octatools-bin` commands for people who don't know how to use the terminal.

### Notes

This package is a placeholder to act as a guilt trip every time I look at the repository.

Probably need to have everything tested and working on windows before I'm gonna be 
comfortable getting this sorted.

## `octatools-lib` -- Functions & Ser/De library

Library used for reading/writing octatrack binary data.
Most of this is just the [`serde` crate](https://serde.rs) with a bunch of function 
definitions for doing different things.

### Current Features (mostly working-ish)
- Deserialize OctaTrack data files into rust types
- Serialize rust types into OctaTrack data files
- Convert OctaTrack data files into YAML (string or file)
- Convert OctaTrack data files into JSON (string or file)
- Convert JSON (string or file) into OctaTrack data files
- Convert YAML (string or file) into OctaTrack data files

### Notes

There are a small number of fields in data files which I haven't worked out what they do / 
are used for yet. If anyone can help here then I would be very grateful 
(see the list in [./octatools-lib/TODO.md](./octatools-lib/TODO.md)).

Also, the JSON / YAML structures are a bit ... weird.
I have done my best to not convert the underlying data into a structures so that the 
library returns structures that are *as-close-to-the-raw-data-as-possible*.

Like, the arrangements data could do with some work to deal with all the `{"empty": ""}` 
arranger rows. Header fields *probably* don't need to be there and can be injected in 
during deserialization.

## `octatools-py` -- Python extension

Python extension module to allow the reading/writing of octatrack binary data to/from 
JSON.

### Notes

The module can be used to turn octatrack data files into json, and back again. 
It can write the JSON file to disk or convert to/from bytes, which might be useful for an 
application based on python HTTP APIs if someone is so inclined ;] -- read the file from 
your website into bytes, pass to `octatools_py`, get the json for it etc. etc..

### Current Features (mostly working-ish)
- Deserialize all OctaTrack data structures from binarized data
- Serialize all OctaTrack data files to rust types
- Serialize/deserialize to/from YAML and JSON

