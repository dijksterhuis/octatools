# TODO
- Make the code more idiomatic / 'clean' / optimised.
  - TODO: Follow the operations structure in `ot-tools-cli`
- Consolidation:
  - Audio files from a Project into a Set's Audio Pool.
  - Audio files from a Set Audio Pool into a Project (only get what is needed).
  - Audio files from all Project into a Set Audio Pools.
  - Audio files from all Set Audio Pools into Projects.
- Handle AIFF files (and switching between AIFF and WAV within the code -- probably needs an abstraction).
- Inspect RIFF header issues with `hound` on samples from mars files
- Sane Logging messages?
- Sane Error handling?
- Improve test coverage and test cases (negative tests).
- examples directory showing rust usage of stuff like bank copying