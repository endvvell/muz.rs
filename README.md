# muz.rs

Anki-like program for learning music theory.

The values for tones, notes, and scales are derived algorithmically, meaning there is no stored value for, say, 5th tone of D♯ Minor scale (A♯), but instead the algorithm goes through the notes in an octave and determines that value.

Right now the script is useful primarily for learning the major and minor scales. This helps build intuition when you are playing "formulaically", so the response to "ok, now I want to play 4th tone of G♯ major scale" is immediate. It also helps when you are using an instrument at the same time so the idea is associated with the movement you have to perform.

Hopefully I will get to adding other scales and aspects of music theory later on.

## Usage

```bash
muz_rs -<argument>
```

```text
Arguments:

-b    - replaces UTF-8 symbols for sharp and flat with '#' and 'b'

-c    - prints out a cheat sheet for major and minor scales.
        Can be used with '-b' to print the notes with 'basic' notation,
        like so:

            -cb | -bc | -c -b
```

## Installation

Either download the compiled binary from the `/bin` directory or clone this repo and compile the files with either `cargo` or `rustc`.
