![Utilities for the Elektron Octatrack DPS-1](assets/logo-wide.png "octatools")

Utilities for [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer) binary data files.

Releases will be available on [GitHub](https://github.com/dijksterhuis/octatools).
Development happens on [GitLab](https://gitlab.com/dijksterhuis/octatools).

# Table of Contents

- [Warnings](./README.md#warnings)
- [octatools is not ...](./README.md#octatools-is-not-)
- [octatools is ...](./README.md#octatools-is-)
  - [`octatools-bin`](./README.md#octatools-bin----the-cli-binary)
  - [`octatools-lib`](./README.md#octatools-lib----the-readwrite-files-library)
  - [`octatools-derive`](./README.md#octatools-derive)
  - [`octatools-py`](./README.md#octatools-py----the-python-module-)
- [How to build packages](./README.md#how-to-build-packages)
- [How to run tests](./README.md#how-to-run-tests)
- [Credit](./README.md#how-to-run-tests)

# Warnings

- octatools only works with OS version 1.40B (the one I have installed)

- AIFF files are not currently supported for `octatools-bin samples` commands.
  Only WAV files are currently supported.

- Only Linux is fully supported at the moment.
  Most commands from `octatools-bin` can *probably* be run on windows.

- If you are worried about destroying your Octatrack projects / data files -- take a backup
copy of the compact flash card / set / project folder and work on that copy first.

- This has mostly been a **learning** project for me to get to grips with Rust. 
**Please do not expect high quality rust code**.


# octatools is not ...

- A replacement DigiChain
- A new OctaEdit
- Another Octachainer
- Tested against multiple version of Octatrack OS (only 1.40B)
- A GUI application with pretty buttons
- Supported on macOS
- Fully supported on windows (I'm trying my best with it)
- Currently stable
- Thoroughly tested for all possible edge cases
- Expertly written
- Paid-for software with a dedicated support team 

# octatools is ...

Multiple rust packages for doing 'stuff' with Octatrack binary data files ...

## `octatools-bin` -- the CLI binary

TL;DR version -- Perform various useful actions on data files 
like copying banks between projects or chain sample files with slices

- Copy banks from one project to another, moving relevant project sample slots with the
  bank
- Create slice sample chains from multiple WAV files
- Create a linear/random slice grid for an existing wav file
- Deconstruct a slice sample chain into component WAV files
- Write a new binary data file from a YAML/JSON source file (project/bank/sample)
- Dump binary data to a YAML/JSON file (project/bank/sample/part/pattern)
- Inspect various data files (project/arrangement/bank/part/pattern/sample)
- List samples slots being used in a project
- Find compatible WAV files in a local directory and write their file paths to a YAML file

Example usage:
- [Copying banks between projects in the same set](./README.md#example-copying-banks-between-projects-in-the-same-set)
- [Copying banks within the same project](./README.md#example-copying-banks-between-projects-in-the-same-set)
- [Copying banks with a YAML config](./README.md#example-copying-banks-with-a-yaml-config)
- [Slice based sample chaining with the CLI](./README.md#example-slice-based-sample-chaining-with-the-cli)
- [Slice based sample chaining with a YAML config](./README.md#example-slice-based-sample-chaining-with-a-yaml-config)
- [Creating a "god-chain" with a YAML config](./README.md#example-creating-a-god-chain-with-a-yaml-config)
- [Creating random/linear slice grids](./README.md#example-creating-randomlinear-slice-grids)
- [Deconstructing samples based on slices](./README.md#example-deconstructing-samples-based-on-slices)

### Example: Copying banks between projects in the same set

Here's an example of copying `Bank 1` from the `PROJECT_SOURCE` project to the 
`PROJECT_DEST` project.  The bank will be copied to `Bank 16` in the 
`PROJECT_DEST` project.

```bash
octatools-bin banks copy \
  ./path/to/SET/PROJECT_SOURCE \  # source project location 
  ./path/to/SET/PROJECT_DEST \    # destination project location
  1 \  # source bank number (bank to copy) 
  16   # destination bank number
```

The unsaved project/bank state should now be different when you load the project.
If you are happy with the changes, save the project and the bank with 
`PROJECT MENU > PROJECT > SAVE`.

If anything looks wrong, you should be able to use `PROJECT MENU > PROJECT > 
RELOAD` to undo all changes. Failing that, there are backup files created you 
can use to manually restore the project/bank. However, please be aware **sample 
files cannot be un-copied/un-transferred**, see the warning above as it is 
possible to have sample files be overwritten if you are not careful with file 
names.

**WARNING 1**: sample files are copied to the new project based on the sample
file names. You will encounter issues/breakage if you have sample files between
the projects that share the same file name but are actually different files. It
is **very** easy to get lazy naming your sample files (I'm just as guilty as you
are).

**WARNING 2**: the source project's sample slots are deduplicated in-memory
during the copy to try and add as few sample slots to the destination project as
possible (we only have 128 possible sample slots available per slot type). There
are no changes made to the source project files as a result of this
deduplication. However, if you purposefully duplicate sample slots in the source
project for some reason, these duplicate sample slots will be removed during
copying. Sample slots are unique based on their current slot settings, including
the file path of the sample loaded into the sample slot, so a difference in slot
settings (e.g. gain/tempo) means that slot is treated as unique.

### Example: Copying banks within the same project

You can also use this command to copy existing banks within the same project
```bash
octatools-bin banks copy \
  ./path/to/SET/PROJECT_SOURCE \  # source project location 
  ./path/to/SET/PROJECT_SOURCE \  # same project
  1 \  # source bank number (bank to copy) 
  16   # destination bank number
```

### Example: Copying banks with a YAML config

If you have a lot of banks to copy, you can speed things up by creating a YAML 
configuration file and using the `banks copy-n` command to copy each bank in 
series i.e. one after the other.

Example configuration file for copying the same bank multiple times to different projects
```yaml
# file saved at ./bank_copies.yaml
bank_copies:
  # copy the bank to a different project
  - src: 
      # I'm using Linux based relative file paths here
      # for windows you will need to change these to: ".\\path\\to\\SET\\PROJECT_SOURCE" 
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_A"
      bank_id: 1
  # copy the bank to another project
  - src:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_B"
      bank_id: 16
  # copy the bank within the same project
  - src:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 16
  # copy another bank from another source project to a project in a different set
  - src:
      project: "./path/to/SET/DIFFERENT_SOURCE"
      bank_id: 12
    dest:
      project: "./path/to/SET_B/PROJECT_C"
      bank_id: 6
```

Then run
```bash
octatools-bin banks copy-n ./bank_copies.yaml
```

**NOTE**: Because the bank copy operations are performed in series, you could do
weird stuff like copying `PROJECT_SRC` bank 5 to `PROJECT_A` bank 1, then 
copying `PROJECT_A` bank 1 to `PROJECT_B` bank 2, then copying `PROJECT_B` bank 
2 to `PROJECT_C` bank 15. 

I'm not really sure why you would want to do that, but you could :person-shrugging:



### Example: Slice based sample chaining with the CLI

Create new sample files `chained-1.wav` and `chained-1.ot` which chains together multiple wav files, 
all accessible in a single Octatrack sample slot using the slices
```bash
octatools-bin samples chain create \
  chained \         # base file name of the resulting sample files
  ./outdir \        # directory path the new sample chain will be written to
  ./sample_1.wav \  # file paths of the sample files to chain together
  ./sample_2.wav \
  ./sample_3.wav \
  ./sample_4.wav \
  # ... etc 
```
The output chains are always suffixed with a number to cover the case where more 
than 64 sample files are included in the chain. 100 input samples will create 2x
output chain file pairs: `chained_1.wav`/`chained_1.ot` and `chained_2.wav`/`chained_2.ot`.

So, you can include as many wav file paths as you want (sort of... memory limits apply).

### Example: Slice based sample chaining with a YAML config

Doing the same thing as the CLI, but for two different chains using a YAML 
config file, which also allows you to modify the other settings of a generated 
sample chain
```yaml
# YAML file written to `./chains.yaml`
global_settings:
  # currently has no effect -- i might remove the option.
  normalize: false
  # directory path where new sample chains will be written to
  # WARNING: Make sure `chain_name` is unique for the chains you want to generate
  out_dir_path: "./outdir" 
chains:
  # first chain to be created
  - chain_name: chain_1
    Octatrack_settings:
      bpm: 120.0
      # gain needs fixing as it's not working properly
      gain: 48.0
      # Time stretch options: "Off", "Normal" or "Beat"
      timestretch_mode: "Off"
      # Loop options: "Off", "Normal" or "PingPong"
      loop_mode: "Off"
      # Quantization options: "Direct", "PatternLength", "OneStep", "TwoSteps",
      # "ThreeSteps", "FourSteps"  ...  etc. etc.
      # For a complete list, see: ./octatools-lib/src/samples.options.rs
      quantization_mode: "Direct"
    sample_file_paths:
      - "./sample_1.wav"
      - "./sample_2.wav"
      - "./sample_3.wav"
      - "./sample_4.wav"
      - "./sample_5.wav"
  # a second chain, with different source samples and settings
  - chain_name: chain_2
    Octatrack_settings:
      bpm: 200.0
      gain: 36.0
      timestretch_mode: "Normal"
      loop_mode: "PingPong"
      quantization_mode: "PatternLength"
    sample_file_paths:
      - "./other_sample_1.wav"
      - "./other_sample_2.wav"
      - "./sample_1.wav"
      - "./other_sample_3.wav"
      - "./sample_2.wav"
```

Then run
```bash
octatools-bin samples chain create-n ./chains.yaml
```

### Example: Creating a "god-chain" with a YAML config
Let's say you have a bunch of favourite audio files that you usually use in a 
project. You can create a YAML config for these samples like so
```yaml
# YAML file written to `./godchain.yaml`
global_settings:
  normalize: false  # doesn't do anything at the moment
  out_dir_path: "./outdir" 
chains:
  - chain_name: godchain
    Octatrack_settings:
      bpm: 120.0
      gain: 24.0
      timestretch_mode: "Off"
      loop_mode: "Off"
      quantization_mode: "Direct"
    sample_file_paths:
      - "./favourite_1.wav"
      - "./favourite_2.wav"
      - "./favourite_3.wav"
      - "./favourite_4.wav"
      - "./favourite_5.wav"
```
Running the following command will create the files `./outdir/godchain-1.wav` and 
`./outdir/godchain-1.ot`, which you can load into your Octatrack projects
```bash
octatools-bin samples chain create-n ./godchain.yaml
```

If you find some new favourite samples at a later date, you can add them to the 
"godchain" by adding them to the end of the `sample_file_paths` section (example
YAML shortened for brevity)
```yaml
    sample_file_paths:
      - "./favourite_1.wav"
      - "./favourite_2.wav"
      - "./favourite_3.wav"
      - "./favourite_4.wav"
      - "./favourite_5.wav"
      - "./new_favourite_A.wav"
      - "./new_favourite_B.wav"
      - "./new_favourite_C.wav"
      - "./new_favourite_D.wav"
      - "./new_favourite_E.wav"
```
and then running the command again
```bash
octatools-bin samples chain create-n ./godchain.yaml
```
The "godchain" files will be recreated with the new samples added as slices 
after the existing slices. On the Octatrack, replace the existing `godchain-1.wav` 
and `godchain-1.ot` files with the new version to include the updated "godchain" 
in a project.

Any time you find new samples you like, just add them to the same config and 
regenerate the chain.

**WARNING**: Always add additional samples to the END of the `sample_file_paths` 
YAML list. The order of the list determines the order of slices. Adding files to
the start of the list will put your new samples at the start of your sample 
chain, potentially meaning existing projects will no longer be using the correct 
slices!

**NOTE**: The `projects dedupe` command idea mentioned in the bank copying 
example may cause issues here as it will likely rename all files based on the 
combined content hash of sample file pairs. Generating a new "godchain" will 
mean having different content hashes ... Need to think about this.

### Example: Creating random/linear slice grids

If you're like me, you like finding weird sounds within a much larger sample file 
to use within your music. Weird transients that can be used as drum hits, or 
strange tones that can be used as background ambience. I usually do this is by 
randomly seeking through a sample in the Octatrack audio editor menu and 
previewing the audio.

The `octatools-bin samples grid random` command pre-generates a bunch of random 
slices for a single audio file, to basically skip me having to do all this 
seeking business. Now I can just turn the SLICE knob and see what I get!
```bash
octatools-bin samples grid random <WAV_FILE_PATH> <N_SLICES>
```
**WARNING**: Unique sample file name conventions apply. If you want multiple 
random grids then you need to make copies of the files with different names and 
then run this command multiple times.

Or maybe you want to create a slice grid which is linear, i.e. all the slices 
are the same length and equally spaced apart. In which case, you can use:
```bash
octatools-bin samples grid linear <WAV_FILE_PATH> <N_SLICES>
```

### Example: Deconstructing samples based on slices
Let's say you've been creating slices in a sample on the Octatrack.
You found four or five sections of a long audio file that you really like.
You'd like to extract just those slices and add them to a "god-chain" that 
contains all of your favourite slices.

You can create new WAV files from the slices of a sample file pair like so:

```bash
octatools-bin samples chain deconstruct my_sample.ot my_sample.wav ./outdir
```
This will extract the slices and write them as new files in `./outdir`.
The file names will be: `my_sample_0.wav`, `my_sample_1.wav`, etc.

You can then add these files to an existing YAML config for your "god-chain" 
and recreate the sample chain with your newly discovered slices.

### Work in Progress features (need more work / need to start work on)
- Collect a project's sample files to the project directory
- Collect a project's sample files to the set Audio Pool
- Purge project directory of any sample files not present in sample slots
- Find compatible audio files on the local file system for using in the Octatrack
- Deduplicate project sample files with unique file names (content hash based)
- Deduplicate all set sample files with unique file names (content hash based)

### Help Wanted
- User guide / documentation
- Writing tests
- General testing of the software

## `octatools-lib` -- the read/write files library

Library with functions for reading/writing Octatrack binary data.
Most of this is just the [`serde` crate](https://serde.rs) with a bunch of function 
definitions for doing different things.

### Current Features (mostly working-ish)
- Deserialize Octatrack data files into rust types
- Serialize rust types into Octatrack data files
- Convert Octatrack data files into YAML (string or file)
- Convert Octatrack data files into JSON (string or file)
- Convert JSON (string or file) into Octatrack data files
- Convert YAML (string or file) into Octatrack data files

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

## `octatools-py` -- the python module 

Python extension module to allow the reading/writing of Octatrack binary data to/from 
JSON.

### Notes

The module can be used to turn Octatrack data files into json, and back again. 
It can write the JSON file to the filesystem, or convert to/from bytes. 

The purpose of this package is to provide non-rust devs with a mechanism to make their own 
software. Rust has (fairly) stable bindings for Python provided by [PyO3](https://pyo3.rs/v0.15.1/).


which might be useful for an 
application based on python HTTP APIs if someone is so inclined ;] -- read the file from 
your website into bytes, pass to `octatools_py`, get the json for it etc. etc..

### Current Features (mostly working-ish)
- Deserialize all Octatrack data structures from binarized data
- Serialize all Octatrack data files to rust types
- Serialize/deserialize to/from YAML and JSON

## `octatools-derive`

**If you don't write rust code you can ignore this**.

Used to create `#[derive(XXXX)]` macros for the following:
- `#[derive(Decodeable)]` for the `octatools-lib::Decode` trait
- `#[derive(Encodeable)]` for the `octatools-lib::Encode` trait
- `#[derive(DefaultsAsArray)]` for the `octatools-lib::DefaultsArray` trait
- `#[derive(DefaultsAsArrayBoxed)]` for the `octatools-lib::DefaultsArrayBoxed` trait

See the trait descriptions for more information.


# How to build packages
For a dev versions of `octatools-bin` and `octatools-lib`:
```bash
make build
```

For a release version of `octatools-bin` and `octatools-lib`:
```bash
make release
```

For a dev version of `octatools-py` (linux only)
```bash
# builds python extension module, 
# installs it in a local virtual environment,
# then perform some minimal smoke tests.
make build-py
```

For a release version of `octatools-py`  (linux only)
```bash
# not available yet
```

# How to run tests
Running tests
```bash
make test
```

To generate a test coverage report:
```bash
cargo install tarpaulin
make cov
```

The project is currently hanging around 50-60% test coverage. 

# Credit

The following projects were used a starting references for the initial
serialization/deserialization efforts of data files (mostly the `.ot` files). 
Without them, octatools probably wouldn't exist.

- Digichain by brian3kb -- https://github.com/brian3kb/digichain
- OctaChainer by KaiDrange -- https://github.com/KaiDrange/OctaChainer

A special shout out to OctaEdit by Rusty (no longer available http://www.octaedit.com)
which showed the community what it was possible to do. 
Hopefully there are lots of moonbeams and bunny rabbits wherever you are.

Other rust based credits:

- [PyO3](https://pyo3.rs/v0.15.1/) is basically the entire reason `octatools-py` can exist
- the [serde](https://serde.rs) framework made reverse engineering data files a lot easier, 
  `octatools-lib` probably wouldn't exist without serde.
- same with [bincode](https://github.com/bincode-org/bincode) for reading the binary data
