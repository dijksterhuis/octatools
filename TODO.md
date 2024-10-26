# TODO
- Does the Octatrack create an OT file for samples which are lodaded into static / flex slots?
  Or is it just for sample chains with slices? --- Created when loaded.
- CI
- Release builds
- Get project sample slot relative paths to resolve to the absolute file path.
- Deal with over use of `.clone()` absolutely everywhere.
- Handle simple cli command options
- Tests, tests, tests!
- Stop overusing traits and methods -- remember, functional w/ dataclasses.
- Split up project settings into smaller structs. One struct for each settings page.
- Logging pass over.
- Inspect RIFF header issues with `hound`.
- `AIFF` file reading.
- Handle both `AIFF` and `WAV` writes at 16/24 bit depth & 24.1/28kHz sample rates.
- Minor audio edits in for sample chains (fade in 1%/2%/5%/100%, fade out, fade in/out type (linear/exp/etc), normalisation, 'fast-cuts' glitching, reverses?)
- Deconstruct a sample chain into individual samples (inverse of creating a sample chain)
- Deconstruct a bunch of sample chains + attribute files into a YAML config for offline editing (add more samples, chainge gain values, etc.)
- Random slice selection -- randonly create Nx slices for a single sample file.
- Mode parameter for how to do the sample chaining?
- Sample attribute gain noramlisation in YAML / CLI? Should it be 0.0 -> 100.0? Or Octatrack native machine values (-24.0dB -> +24.0db)?
- `ValuesFrom` Trait for ......? some non-OT enum...? Errors?
- Set up projects to be able to write to new project files. for  TEMPLATE projects (no overwriting existing projects!).
  e.g. fill static sample slots 001 through 032 with drum sample chains, 064-128 with field recordings etc.
- How to handle moving sample slots and associated bank data etc between projects?
- TODO: what about project.strd ??! which one of work/strd is the "active" un-saved/un-synced data? (work is current, strd is last version)
