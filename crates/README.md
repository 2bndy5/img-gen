# Crates

The directory of all crates in this cargo workspace.

## `img-gen-spec`

The specifications used to generate images.

## `img-gen-renderer`

The engine used to generate images from a given specification (`Layout`).

## `img-gen`

A convenience crate that rexport public-facing API from both\
`img-gen-spec` and `img-gen-renderer` crates.

This crate also facilitates the python binding distributed as
the python package `img-gen`.
