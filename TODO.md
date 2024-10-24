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
- ...?
