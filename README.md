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


## Preview
(I have an alias `alias muz='muz_rs'` set up. Which is why I'm invoking the script through `muz`)

![2024-06-16_02-16](https://github.com/endvvell/muz.rs/assets/34137807/953a073b-3022-4f0c-b851-bf112b6e737c)

![2024-06-16_02-18](https://github.com/endvvell/muz.rs/assets/34137807/dddf91cf-d3a2-4a8e-8b48-87fc7ea1d705)

![2024-06-16_02-20](https://github.com/endvvell/muz.rs/assets/34137807/5f4e7e76-b2d4-43c3-9f99-0202351ffb5a)

![2024-05-24_13-05](https://github.com/endvvell/muz.rs/assets/34137807/677580a8-fd53-4bfa-ab5d-4f31e363ab19)

![2024-06-16_02-09](https://github.com/endvvell/muz.rs/assets/34137807/16bd75d9-2dd3-4db2-9061-2d82715e89ff)

<br/>

---
