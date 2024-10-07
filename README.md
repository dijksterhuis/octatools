# Octatrack Sample Manager

Sample manager for the [Elektron Octatrack DPS-1](https://www.elektron.se/en/octratrack-mkii-explorer) seqeuncer based sampler.
Both MK1 and MKII should be supported.

TBD: I may turn this into a library to support the reading/writing of Octatrack sample chain files for use elsewhere.

### What this software aims to do

Doing sample preperation work for the Octatrack in a **more automated manner**, covering the following jobs:

1. Create `.ot` files for Octatrack sample 'chains' from a central CSV file
2. (Safely) Sync files between a Compact Flash card and a local directory.

The ideal aim is to have a master CSV file which can be edited to copy files to
various Audio Pools, or create various different various sample chain versions
based on the CSV's configured settings.

That way it's possible to copy and paste lines in a CSV file to do the work for
me, instead of repeatedly clicking on UI buttons in OctaChainer or DigiChain.
This is not meant as a knock against OctaChainer or DigiChain.
I just prefer command line interfaces as I find it easier to automate.

Ideally, it would be useful to **safely** sync changes onto a Compact Flash card
-- i.e. without destroying anything that changed in the interim.

Long term "dream" features also include the following, but they aren't a priority
- Graphical user interface.
- List samples used in each project (via scanning `project.strd` files).
- Determine if a sample chain file is already used in a project (via scanning `project.strd` files)
  and block editing of a sample chain, or only allow editing after providing an explicit warning.
- Bulk project sample consolidation
- Pull and **push** sync, so that everything can be done offline, without the CF card to hand.
- Minor sample editing (normalisation, fades, reverses, 
- Combinatorial drum pattern chain generator (probably a separate / standalone project).

### What this software is not
- A clone of DigiChain
- A clone of OctaEdit (whereforartthou Rusty :sad:)
