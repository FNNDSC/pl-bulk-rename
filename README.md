# Bulk Rename

[![Version](https://img.shields.io/docker/v/fnndsc/pl-bulk-rename?sort=semver)](https://hub.docker.com/r/fnndsc/pl-bulk-rename)
[![MIT License](https://img.shields.io/github/license/fnndsc/pl-bulk-rename)](https://github.com/FNNDSC/pl-bulk-rename/blob/main/LICENSE)
[![ci](https://github.com/FNNDSC/pl-bulk-rename/actions/workflows/ci.yml/badge.svg)](https://github.com/FNNDSC/pl-bulk-rename/actions/workflows/ci.yml)

`pl-bulk-rename` is a [_ChRIS_](https://chrisproject.org/)
_ds_ plugin which copies files from an input directory to an
output directory under different names using regular expressions.

## Installation

`pl-bulk-rename` is a _[ChRIS](https://chrisproject.org/) plugin_, meaning it can
run from either within _ChRIS_ or the command-line.

[![Get it from chrisstore.co](https://ipfs.babymri.org/ipfs/QmaQM9dUAYFjLVn3PpNTrpbKVavvSTxNLE5BocRCW1UoXG/light.png)](https://chrisstore.co/plugin/pl-bulk-rename)

## Usage

Regular expression syntax is based on the [regex](https://crates.io/crates/regex) crate.
See https://docs.rs/regex/1.5.5/regex/#grouping-and-flags

### Local Usage

To get started with local command-line usage, use [Apptainer](https://apptainer.org/)
(a.k.a. Singularity) to run `pl-bulk-rename` as a container.

To print its available options, run:

```shell
singularity exec docker://fnndsc/pl-bulk-rename bulkrename --help
```

## Examples

`bulkrename` copies data from an input directory to an output directory.

Consider the data in [`examples/input`](examples/input):

```
examples/input
├── a
│   ├── food.txt
│   └── log
├── b
│   ├── food.txt
│   └── log
└── c
    ├── food.txt
    └── log
```

To rename every `.txt` file to have the name of their parent directory:

```shell
bulkrename --filter '.*\.txt' \
           --expression '^(.*?)/(.*?)\.txt$' \
           --replace '$1.txt' \
           examples/input examples/filewise
```

To rename the subdirectories `a`, `b`, `c`, of the input directory to have a prefix `pear_`:

```shell
bulkrename --filter '^[abc]$' \
           --expression '([abc])' \
           --replace 'pear_$1' \
           examples/input examples/dirwise
```
