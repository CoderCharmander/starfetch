# starfetch

A rewrite of the command-line program [`starfetch`](https://github.com/Haruno19/starfetch) in Rust.

This utility displays constellations and some information about them in the command line.

## Why?

There is no real point to the rewrite. It's just a fun afternoon project.

## Installation

First, run this command:
```bash
cargo install --path=.
```
Then, run `starfetch -d` to display the data directory, and copy all files in `share/constellations` there.

## Usage

Use `starfetch --help` for a help page.

Use `starfetch -r` to display a random constellation.

Use `starfetch -l` to list all available constellations with their name, file name and quadrant.

Use `starfetch aries` to display Aries, or any other constellation.

## Constellations

All constellations are taken from [the original `starfetch`](https://github.com/Haruno19/starfetch) and
adapted with the `convert.py` script. I'd like to thank Stefano Gini (Haruno19) for creating the data files.

### File format

Constellation files are JSON, an example is:
```json
{
    "title": "───── ａｒｉｅｓ ─────",
    "graph": [
        [4, 2, "✦"],
        [14, 4, "✦"],
        [16, 6, "✦"],
        [15, 7, "✦"],
    ],
    "name": "Aries",
    "quadrant": "NQ1",
    "right_ascension": "01h 46m 37.3761s –03h 29m 42.4003s",
    "declination": "+31.2213154° to –10.3632069°",
    "area": "441 sq.deg. (39th)",
    "main_stars": "4, 9"
}
```

Most of the attributes are self-explanatory, and simply strings displayed literally in the output.
`graph` is a list of stars, with their X and Y coordinates, and the character used to draw them, in this
order.

## Screenshots

| ![starfetch -l](/images/Screenshot_20220408_210043.png) | ![starfetch ursa_major](/images/Screenshot_20220408_210222.png) |
| --- | --- |
| Listing constellations | Displaying a constellation |