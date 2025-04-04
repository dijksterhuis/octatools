# ot-tools

Various Rust binaries/libraries for use with [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer)
binary data files.

![GitHub Release](https://img.shields.io/github/v/release/dijksterhuis/ot-tools?sort=semver&display_name=release&style=flat&label=Latest%20GitHub%20release&link=https%3A%2F%2Fgithub.com%2Fdijksterhuis%2Fot-tools%2Freleases%2Flatest)
[![pipeline status](https://gitlab.com/ot-tools/ot-tools/badges/main/pipeline.svg)](https://gitlab.com/ot-tools/ot-tools/-/commits/main)
[![coverage report](https://gitlab.com/ot-tools/ot-tools/badges/main/coverage.svg)](https://gitlab.com/ot-tools/ot-tools/-/commits/main)

# Table of Contents

- [Warnings](./README.md#warnings)
- [License](./README.md#license)
- [ot-tools is ...](./README.md#ot-tools-is-)
  - [`ot-tools` -- CLI binary](./README.md#ot-tools----the-cli-binary)
  - [`ot-tools-ops` -- operations library](./README.md#ot-tools-ops----the-projectssets-operations-library)
  - [`ot-tools-io` -- file read/write library](./README.md#ot-tools-io----the-readwrite-files-library)
  - [`ot-tools-derive` -- rust derive macros](./README.md#ot-tools-derive)
  - [`ot-tools-py` -- python extension module](./README.md#ot-tools-py----the-python-module-)
- [How to build packages](./README.md#how-to-build-packages)
- [How to run tests](./README.md#how-to-run-tests)
- [Credit](./README.md#credits)

# Warnings

- There will be bugs.
- ot-tools only works with OS version 1.40B.
- AIFF files are not currently supported for `ot-tools sample-files` commands.
  Only WAV files are currently supported.
- Most commands from `ot-tools` can *probably* be run on Windows/macOs with 
  release builds, but I mainly develop on Linux so I might miss some issues.
- If you are worried about destroying your Octatrack projects / data files -- 
  take a backup copy of the compact flash card / set / project folder and work 
  on that copy.
- This has mostly been a **learning** project for me to get to grips with Rust. 
  **Please do not expect high quality or reliable rust code**.
- **Name your sample files uniquely**. I cannot stress this enough. Especially
  with respect to the copying banks between projects command.
- Every function/module/command is currently in an 'unstable' state and will 
  possibly (probably) change in the future.
- There will be bugs.

# License
The ot-tools project is licensed under the 
[GNU GPL v3.0 license](https://www.gnu.org/licenses/gpl-3.0.en.html).

# ot-tools is ...

Multiple rust binaries/crates for doing 'stuff' with Octatrack binary data files
in different ways.

## `ot-tools` -- the CLI binary

Perform various useful actions to create and/or modify Octatrack projects/files, 
like copying banks between projects or chaining sample files together with 
slices.

The following features are mostly working, but still need thorough testing, i.e.
they need examples created and loaded onto the Octatrack for IRL on-machine 
confirmation that they work; plus probably a bunch more automated test cases ...
see [Help Wanted](./README.md#help-wanted).

**WARNING**: Before using any of the binary data file actions -- make sure you 
`SAVE PROJECT` in the octatrack project menu. Make sure there are both `.work` 
and `.strd` files in your project directory. ot-tools does not create `.strd` 
files for your projects (yet). If you make changes after only doing 
`SYNC TO CARD` then you will not be able to restore your project's previous 
state (`.work` files are the current state, `.strd` are the saved version which 
are used in a reload operation).

### Examples
- [Copying a bank to a project in the same set](./README.md#example-copying-a-bank-to-a-project-in-the-same-set)
- [Copying a bank within the same project](./README.md#example-copying-a-bank-within-the-same-project)
- [Copying multiple banks with a YAML config](./README.md#example-copying-multiple-banks-with-a-yaml-config)
- [Slice based sample chaining with the CLI](./README.md#example-slice-based-sample-chaining-with-the-cli)
- [Slice based sample chaining with a YAML config](./README.md#example-slice-based-sample-chaining-with-a-yaml-config)
- [Creating a "god-chain" with a YAML config](./README.md#example-creating-a-god-chain-with-a-yaml-config)
- [Creating random/linear slice grids](./README.md#example-creating-randomlinear-slice-grids)
- [Splitting samples based on slices](./README.md#example-splitting-samples-based-on-slices)
- [Converting data files to YAML/JSON](./README.md#example-converting-data-files-to-yamljson)
- [Writing YAML/JSON files as new data files](./README.md#example-writing-yamljson-files-as-new-binary-data-files)
- [Creating default project data files](./README.md#example-creating-default-project-data-files)

#### Example: Copying a bank to a project in the same set

Here's an example of copying `Bank 1` from the `PROJECT_SOURCE` project to the 
`PROJECT_DEST` project.  The bank will be copied to `Bank 16` in the 
`PROJECT_DEST` project.

```bash
ot-tools operations copy bank \
  ./path/to/SET/PROJECT_SOURCE \
  1 \ 
  ./path/to/SET/PROJECT_DEST \
  16
```

The unsaved project/bank state should now be different when you load the project.
If you are happy with the changes, save the project and the bank with 
`PROJECT MENU > PROJECT > SAVE`.

If the destination bank is not empty -- i.e. you have modified it previously 
-- then you will see this error:
```txt
ERROR: destination bank has been modified, but no force flag provided
```

If you are **absolutely sure** that you want to overwrite that bank, provide the 
`--force` flag
```bash
ot-tools operations copy bank \
  ./path/to/SET/PROJECT_SOURCE \
  1 \ 
  ./path/to/SET/PROJECT_DEST \
  16 \
  --force
```

This operation is only performed on the `*.work` files. So if anything looks 
wrong, you should be able to use `PROJECT MENU > PROJECT > RELOAD` to undo all 
changes (untested!). Failing that, there are backup files created you can use to 
manually restore the project/banks. However, please be aware **sample files 
cannot be un-copied/un-transferred**, see the warning below as it is possible to
have sample files be overwritten if you are not careful with file names.

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

#### Example: Copying a bank within the same project

You can also use this command to copy existing banks within the same project
```bash
ot-tools operations copy bank \
  ./path/to/SET/PROJECT_SOURCE \ 
  1 \ 
  ./path/to/SET/PROJECT_SOURCE \
  16
```

#### Example: Copying multiple banks with a YAML config

If you have a lot of banks to copy, you can speed things up by creating a YAML 
configuration file and using the `copying bank-yaml` command to copy each bank 
in series i.e. one after the other.

Example configuration file for copying the same bank multiple times to different 
projects
```yaml
# file saved at ./bank_copies.yaml
bank_copies:
  # copy the bank to a different project
  - src: 
      # I'm using Linux based relative file paths here
      # for windows you will need to change these to: 
      #   project: ".\\path\\to\\SET\\PROJECT_SOURCE" 
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_A"
      bank_id: 1
  # copy the bank to another project, overwriting the destination bank
  - src:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_B"
      bank_id: 16
    force: true
  # copy the bank within the same project
  - src:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 1
    dest:
      project: "./path/to/SET/PROJECT_SOURCE"
      bank_id: 16
  # copy another bank from another source project 
  # to a project in a different set
  - src:
      project: "./path/to/SET/DIFFERENT_SOURCE"
      bank_id: 12
    dest:
      project: "./path/to/SET_B/PROJECT_C"
      bank_id: 6
```

Then run
```bash
ot-tools operations copy bank-yaml ./bank_copies.yaml
```

**NOTE**: Because the bank copy operations are performed in series, you could do
weird stuff like copying `PROJECT_SRC` bank 5 to `PROJECT_A` bank 1, then 
copying `PROJECT_A` bank 1 to `PROJECT_B` bank 2, then copying `PROJECT_B` bank 
2 to `PROJECT_C` bank 15. 

I'm not really sure why you would want to do that, but you could :shrug:



#### Example: Slice based sample chaining with the CLI

Create new sample files `chained-1.wav` and `chained-1.ot` which chains together 
multiple wav files, all accessible in a single Octatrack sample slot using the 
slices
```bash
ot-tools sample-files chain \
  chained \
  ./outdir \
  ./sample_1.wav \
  ./sample_2.wav \
  ./sample_3.wav \
  ./sample_4.wav
```
The output chains are always suffixed with a number to cover the case where more 
than 64 sample files are included in the chain. 100 input samples will create 2x
output chain file pairs: `chained-1.wav`/`chained-1.ot` and `chained-2.wav`/
`chained-2.ot`.

So, you can include as many wav file paths as you want (sort of... memory limits 
apply).

#### Example: Slice based sample chaining with a YAML config

Doing the same thing as the CLI, but for two different chains using a YAML 
config file, which also allows you to modify the other settings of a generated 
sample chain
```yaml
# YAML file written to `./chains.yaml`
global_settings:
  # directory path where new sample chains will be written to
  # WARNING: Make sure `chain_name` is unique for the chains you want to generate
  out_dir_path: "./outdir" 
chains:
  # first chain to be created
  - chain_name: chain_1
    octatrack_settings:
      bpm: 120.0
      # between -24.0 and 24.0
      gain: 0.0
      # Time stretch options: "Off", "Normal" or "Beat"
      timestretch_mode: "Off"
      # Loop options: "Off", "Normal" or "PingPong"
      loop_mode: "Off"
      # Quantization options: "Direct", "PatternLength", "OneStep", "TwoSteps",
      # "ThreeSteps", "FourSteps"  ...  etc. etc.
      # For a complete list, see: ./ot-tools-io/src/samples/options.rs
      quantization_mode: "Direct"
    audio_file_paths:
      - "./sample_1.wav"
      - "./sample_2.wav"
      - "./sample_3.wav"
      - "./sample_4.wav"
      - "./sample_5.wav"
  # a second chain, with different source samples and settings
  - chain_name: chain_2
    octatrack_settings:
      bpm: 200.0
      gain: -12.0
      timestretch_mode: "Normal"
      loop_mode: "PingPong"
      quantization_mode: "PatternLength"
    audio_file_paths:
      - "./other_sample_1.wav"
      - "./other_sample_2.wav"
      - "./sample_1.wav"
      - "./other_sample_3.wav"
      - "./sample_2.wav"
```

Then run
```bash
ot-tools sample-files chain-yaml ./chains.yaml
```
There will be 4x files in the `./outdir` directory, an `.ot` and a `.wav` file 
for each chain.

See the [chain-create.yaml example](./examples/confs/chain-create.yaml) for more 
details on all the available configuration options when creating sample chains.

#### Example: Creating a "god-chain" with a YAML config
Let's say you have a bunch of favourite audio files that you usually use in a 
project. You can create a YAML config for these samples like so
```yaml
# YAML file written to `./godchain.yaml`
global_settings:
  out_dir_path: "./outdir" 
chains:
  - chain_name: godchain
    audio_file_paths:
      - "./favourite_1.wav"
      - "./favourite_2.wav"
      - "./favourite_3.wav"
      - "./favourite_4.wav"
      - "./favourite_5.wav"
```
Running the following command will create the files `./outdir/godchain-1.wav` and 
`./outdir/godchain-1.ot`, which you can load into your Octatrack projects
```bash
ot-tools sample-files chain-yaml ./godchain.yaml
```

If you find some new favourite samples at a later date, you can add them to the 
"godchain" by adding them to the end of the `audio_file_paths` section (example
YAML shortened for brevity)
```yaml
    audio_file_paths:
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
ot-tools sample-files chain-yaml ./godchain.yaml
```
The "godchain" files will be recreated with the new samples added as slices 
after the existing slices. On the Octatrack, replace the existing `godchain-1.wav` 
and `godchain-1.ot` files with the new version to include the updated "godchain" 
in a project.

Any time you find new samples you like, just add them to the same config and 
regenerate the chain.

**WARNING**: Always add additional samples to the END of the `audio_file_paths` 
YAML list. The order of the list determines the order of slices. Adding files to
the start of the list will put your new samples at the start of your sample 
chain, potentially meaning existing projects will no longer be using the correct 
slices!

#### Example: Creating random/linear slice grids

If you're like me, you like finding weird sounds within a much larger sample file 
to use within your music. Weird transients that can be used as drum hits, or 
strange tones that can be used as background ambience. I usually do this is by 
randomly seeking through a sample in the Octatrack audio editor menu and 
previewing the audio.

The `ot-tools samples grid random` command pre-generates a bunch of random 
slices for a single audio file, to basically skip me having to do all this 
seeking business. Now I can just turn the SLICE knob and see what I get!
```bash
ot-tools sample-files grid-random <WAV_FILE_PATH> <N_SLICES>
```
Or maybe you want to create a slice grid which is linear, i.e. all the slices 
are the same length and equally spaced apart. In which case, you can use:
```bash
ot-tools sample-files grid-linear <WAV_FILE_PATH> <N_SLICES>
```

Unique sample file name conventions apply. If you want multiple
random/linear grids then you need to make copies of the files with different 
names and then run this command multiple times, like in this `bash` example
```bash
for i in $(seq 1 10)
do
  cp ./my_sample_file.wav "./my_sample_file-rand-${i}.wav"
  ot-tools sample-files grid-random "./my_sample_file-rand-${i}.wav" 64
done  
```

#### Example: Splitting samples based on slices
Let's say you've been creating slices in a sample on the Octatrack.
You found four or five sections of a long audio file that you really like.
You'd like to extract just those slices and add them to a "god-chain" that 
contains all of your favourite slices.

You can create new WAV files from the slices of a sample file pair like so:

```bash
ot-tools sample-files split-slices my_sample.ot my_sample.wav ./outdir
```
This will extract the slices and write them as new files in `./outdir`.
The file names will be: `my_sample-0.wav`, `my_sample-1.wav`, etc.

You can then add these files to an existing YAML config for your "god-chain" 
and recreate the sample chain with your newly discovered slices.

You can also split samples in bulk using the `split-slices-yaml` command. See
the [split-by-slices.yaml example](./examples/confs/split-by-slices.yaml) for 
more details on the required YAML options/format.

#### Example: Converting data files to YAML/JSON
Let's say you wanted to inspect all the settings and sample slots for a project
without having to navigate through all the menus on the Octatrack

```bash
ot-tools bin-files bin-to-human \
  project \
  ./path/to/SET/PROJECT/project.work \
  yaml \
  ./project.yaml
```

This writes the `project.work` data file for a project to `./project.yaml`, 
where you can now inspect all the settings for the project.

See the [project.yaml example](./examples/human-readable/yaml/project.yaml) for
an example of this command's output for a default `project.work` file.

#### Example: Writing YAML/JSON files as new binary data files
Maybe I want to I can edit some of the settings for the project in the above 
example?

```yaml
# example truncated for brevity
settings:
  # ...
  control:
    audio:
      master_track: false  # change this to `true`
      cue_studio_mode: false  # change this to `true`
  # ...
```

I can convert this to a new binary data file
```bash
ot-tools bin-files human-to-bin \
  yaml \
  ./project.yaml \
  project \
  ./new_project.work
```

I can create a backup copy of `./path/to/SET/PROJECT/project.work` and then make 
the project settings change to match the edited YAML config by replacing the 
existing `project.work` file with the newly generated binary file
```bash
# make a backup in case i set inappropriate values
cp ./path/to/SET/PROJECT/project.work ./path/to/SET/PROJECT/project.work.backup
# replace the project file
cp ./new_project.work ./path/to/SET/PROJECT/project.work
```

**WARNING**: ot-tools will not perform validation when converting 
human-readable values back to the binary data formats, except for basic type 
overflows (e.g. a setting that cannot be negative, but you provided a negative 
value). Check the comments and documentation in `ot-tools-io` documentation to 
get an idea of appropriate values. This is also why I explicitly mentioned 
creating a backup in this example!

#### Example: Creating default project data files
Maybe I want to create a new Octatrack project, but I don't have access to my 
machine, or my compact flash card?

Well, I can run these commands and will end up with a complete project, ready to 
convert to YAML and start editing settings
```bash
mkdir ./NEW_PROJECT/
# new project file
ot-tools bin-files create-default project ./NEW_PROJECT/project.work

# bank files 1 to 16, inclusive
for i in `seq 1 16` 
do 
  ./ot-tools bin-files create-default bank ./NEW_PROJECT/bank$(printf "%02d\n" $i).work
done
# arrangements files 1 to 8, inclusive
for i in `seq 1 8` 
do 
  ot-tools bin-files create-default arrangement ./NEW_PROJECT/arr$(printf "%02d\n" $i).work
done
```

Running `ls ./NEW_PROJECT` gives
```bash
$ ls NEW_PROJECT/
arr01.work  arr03.work  arr05.work  arr07.work  bank01.work  bank03.work  bank05.work  bank07.work  bank09.work  bank11.work  bank13.work  bank15.work  project.work
arr02.work  arr04.work  arr06.work  arr08.work  bank02.work  bank04.work  bank06.work  bank08.work  bank10.work  bank12.work  bank14.work  bank16.work
```

The only thing missing is a `markers.work` file ... which *seems* to only be 
used to keep track of state within the sample editing UI on the Octatrack.

**WARNING**: Creating a completely new project without a `markers.work` file is 
currently untested behaviour. I'm just using it as an example to show what you 
can do with the `create-default` commands.

### Work in Progress features (need more work / need to start work on / need to emotionally let go of)
- Copy parts from one project/bank to another
- Copy patterns from one project/bank to another
- Collect a project's sample files to the project directory
- Collect a project's sample files to the set Audio Pool
- Purge project directory of any sample files not present in sample slots
- Find compatible audio files on the local file system for using in the Octatrack
- Deduplicate project sample files with unique file names (content hash based)
- Deduplicate all set sample files with unique file names (content hash based)
- Clearing all patterns within a project (leaving parts alone)
- Clearing all parts within a project (leaving patterns alone) ... is this even 
  useful? o_O
- Clearing all patterns within a bank (leaving parts alone)
- Clearing all parts within a bank (leaving patterns alone) ... is this even
  useful? o_O

### Help Wanted
- User guide / documentation
- Writing tests
- General testing of the software

## `ot-tools-ops` -- the projects/sets operations library
Rust library with a bunch of functions for inspecting and modifying files 
contained within Octatrack Set and Project directories. The code for things like
copying banks between projects or generating sample chains (see examples above) 
lives in this package.

The `ot-tools` CLI is basically a wrapper on top of this and the `ot-tools-io` 
package.

### Current Features
- Copy a bank to a project in the same set/in a different set
- Copying a bank within the same project
- Copying multiple banks with a YAML config
- Slice based sample chaining with the CLI
- Slice based sample chaining with a YAML config
- Creating a "god-chain" with a YAML config
- Creating random/linear slice grids
- Splitting samples based on slices
- Deduplicate a project's sample slots (needs more testing)
- Purge a project's sample slots (needs more testing)
- Consolidate project samples to project directory (needs more testing)
- Centralize project samples to the set's audio pool directory (needs more testing)
- Purge project directory samples that are not in use (needs more testing)

## `ot-tools-io` -- the read/write files library

Library with functions for reading/writing Octatrack binary data.
Most of this is just the [`serde`](https://serde.rs) and 
[`bincode`](https://github.com/bincode-org/bincode) crates with a bunch of 
function definitions for reading/writing different files or creating new types.

### Current Features (mostly working-ish)
- Deserialize Octatrack data files into rust types
- Serialize rust types into Octatrack data files
- Convert Octatrack data files into YAML (string or file)
- Convert Octatrack data files into JSON (string or file)
- Convert JSON (string or file) into Octatrack data files
- Convert YAML (string or file) into Octatrack data files

### Notes

There are a small number of fields in data files which I haven't worked out what 
they do / are used for yet. If anyone can help here then I would be very 
grateful (see the list in [./ot-tools-io/TODO.md](ot-tools-io/TODO.md)).

The JSON / YAML data structures are a bit ... weird. I have done my best to not 
parse the underlying data into new structures, keeping it so that the library 
returns data that is *as-close-to-the-raw-data-as-possible*.

Like, the arrangements data could do with some work to deal with all the 
`{"empty": ""}` arranger rows. Header fields *probably* don't need to be there
and can be injected in during deserialization.

## `ot-tools-py` -- the python module 

Python extension module for reading/writing of Octatrack binary data to/from 
YAML or JSON string data.

```python
# python
import json
from pathlib import Path
from ot_tools import bank_file_to_json

json_data: dict = json.loads(
    bank_file_to_json(
        Path("./PROJECT/bank01.work"),
    ),
)

print(json_data.keys())
# prints: dict_keys(['header_data', 'patterns', 'parts_unsaved', 'parts_saved', 'unknown', 'part_names', 'remainder'])
```

### Notes

The purpose of this package is to provide non-rust devs with a mechanism to make
their own software. Rust has (fairly) stable bindings for Python provided by 
[PyO3](https://pyo3.rs/v0.15.1/).

Might be useful for an application based on python HTTP APIs if someone is so 
inclined -- read the file from your website into bytes, pass to the `ot_tools` 
python module, get the json for it etc. etc.

I'm tempted to include some of the `ot-tools` functions in this at some 
point. But that's something I'll worry about much later.

### Current Features (mostly working-ish)
- Deserialize all Octatrack data structures from binarized data
- Serialize all Octatrack data files to rust types
- Serialize/deserialize to/from YAML and JSON

## `ot-tools-derive`

**If you don't write rust code you can ignore this**.

Used to create `#[derive(XXXX)]` macros for the following:
- `#[derive(Decodeable)]` for the `ot_tools_io::Decode` trait
- `#[derive(Encodeable)]` for the `ot_tools_io::Encode` trait
- `#[derive(DefaultsAsArray)]` for the `ot_tools_io::DefaultsArray` trait
- `#[derive(DefaultsAsArrayBoxed)]` for the `ot_tools_io::DefaultsArrayBoxed` trait

See the trait descriptions for more information.

# How to build packages
For a dev versions of `ot-tools` and `ot-tools-io`:
```bash
make build
```

For a release version of `ot-tools` and `ot-tools-io`:
```bash
make release
```

For a dev version of `ot-tools-py` (linux only)
```bash
# builds python extension module, 
# installs it in a local virtual environment,
# then perform some minimal smoke tests.
make build-py
```

For a release version of `ot-tools-py`  (linux only)
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

The project is currently hanging around 70% test coverage. 

# Credits

The following projects were used a starting references for the initial
serialization/deserialization efforts of data files (mostly the `.ot` files). 
Without them, ot-tools probably wouldn't exist.

- Digichain by brian3kb -- https://github.com/brian3kb/digichain
- OctaChainer by KaiDrange -- https://github.com/KaiDrange/OctaChainer

A special shout out to OctaEdit by Rusty (no longer available http://www.octaedit.com)
which showed the community what it was possible to do. 
Hopefully there are lots of moonbeams and bunny rabbits wherever you are.

Other rust based credits:

- [PyO3](https://pyo3.rs/v0.15.1/) is basically the entire reason `ot-tools-py` can exist
- the [serde](https://serde.rs) framework made reverse engineering data files a lot easier, 
  `ot-tools-io` probably wouldn't exist without serde.
- same with [bincode](https://github.com/bincode-org/bincode) for reading the binary data
