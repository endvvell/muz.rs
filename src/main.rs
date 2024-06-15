#![allow(clippy::needless_return)] // I like the code to be explicit

mod colours;

use clap::Parser;
use colours::colours::{GREEN, RED, RESET_COLOR, YELLOW_WITH_UNDERLINE};
use indexmap::IndexMap;
use rand::Rng;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{self, BufWriter, Write},
    process::exit,
};

const NATURAL: &str = "\u{266E}";
const SHARP: &str = "\u{266F}";
const FLAT: &str = "\u{266D}";
const DDSHARP: &str = "\u{1D12A}";
const DDFLAT: &str = "\u{1D12B}";

#[derive(Debug, Default)]
struct AccidentalNotation<'a> {
    natural: &'a str,
    sharp: &'a str,
    flat: &'a str,
    ddsharp: &'a str,
    ddflat: &'a str,
}

#[derive(Parser, Debug)]
struct CommandArguments {
    #[clap(short = 'b')]
    /// enables basic notation of accidentals when specified
    basic_notation_enabled: bool,

    #[clap(short = 'c')]
    /// prints cheatsheet of scales
    get_cheats: bool,
}

#[derive(Debug, Default)]
struct Octave<'a> {
    complete_octave: IndexMap<String, Vec<String>>,
    accidental_notation: AccidentalNotation<'a>,
    exercise_type: i32,
    major: Vec<String>,
    minor: Vec<String>,

    maj_sharps: Vec<String>,
    maj_flats: Vec<String>,
    min_sharps: Vec<String>,
    min_flats: Vec<String>,

    all_sharps: Vec<String>,
    all_flats: Vec<String>,
    all_accidentals: Vec<String>,
    all_notes: Vec<String>,
}

impl<'a> Octave<'a> {
    fn init(mut self, cmd_args: &CommandArguments) -> Self {
        self.gen_accidental_notation(cmd_args);
        self.gen_complete_octave();
        self.gen_maj_sharp();
        self.gen_maj_flats();
        self.gen_min_sharp();
        self.gen_min_flats();
        self.gen_all_sharps();
        self.gen_all_flats();
        self.gen_all_accidentals();
        self.gen_all_notes();
        self
    }

    fn gen_accidental_notation(&mut self, cmd_args: &CommandArguments) {
        if cmd_args.basic_notation_enabled {
            self.accidental_notation = AccidentalNotation {
                natural: "",
                sharp: "#",
                flat: "b",
                ddsharp: "##",
                ddflat: "bb",
            }
        } else {
            self.accidental_notation = AccidentalNotation {
                natural: NATURAL,
                sharp: SHARP,
                flat: FLAT,
                ddsharp: DDSHARP,
                ddflat: DDFLAT,
            }
        }
    }

    fn gen_complete_octave(&mut self) {
        self.complete_octave = IndexMap::from([
            (
                "C".to_string(),
                vec![
                    "A".to_string()
                        + self.accidental_notation.sharp
                        + self.accidental_notation.ddsharp,
                    "B".to_string() + self.accidental_notation.sharp,
                    "D".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "C".to_string() + self.accidental_notation.sharp,
                vec![
                    "B".to_string() + self.accidental_notation.ddsharp,
                    "D".to_string() + self.accidental_notation.flat,
                    "E".to_string()
                        + self.accidental_notation.flat
                        + self.accidental_notation.ddflat,
                ],
            ),
            (
                "D".to_string(),
                vec![
                    "C".to_string() + self.accidental_notation.ddsharp,
                    "E".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "D".to_string() + self.accidental_notation.sharp,
                vec![
                    "C".to_string()
                        + self.accidental_notation.sharp
                        + self.accidental_notation.ddsharp,
                    "E".to_string() + self.accidental_notation.flat,
                    "F".to_string() + self.accidental_notation.ddflat,
                    "F".to_string()
                        + self.accidental_notation.flat
                        + self.accidental_notation.ddflat,
                ],
            ),
            (
                "E".to_string(),
                vec![
                    "D".to_string() + self.accidental_notation.ddsharp,
                    "F".to_string() + self.accidental_notation.flat,
                ],
            ),
            (
                "F".to_string(),
                vec![
                    "D".to_string()
                        + self.accidental_notation.sharp
                        + self.accidental_notation.ddsharp,
                    "E".to_string() + self.accidental_notation.sharp,
                    "G".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "F".to_string() + self.accidental_notation.sharp,
                vec![
                    "E".to_string() + self.accidental_notation.ddsharp,
                    "G".to_string() + self.accidental_notation.flat,
                    "A".to_string()
                        + self.accidental_notation.flat
                        + self.accidental_notation.ddflat,
                ],
            ),
            (
                "G".to_string(),
                vec![
                    "F".to_string() + self.accidental_notation.ddsharp,
                    "A".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "G".to_string() + self.accidental_notation.sharp,
                vec![
                    "F".to_string()
                        + self.accidental_notation.sharp
                        + self.accidental_notation.ddsharp,
                    "A".to_string() + self.accidental_notation.flat,
                    "B".to_string()
                        + self.accidental_notation.flat
                        + self.accidental_notation.ddflat,
                ],
            ),
            (
                "A".to_string(),
                vec![
                    "G".to_string() + self.accidental_notation.ddsharp,
                    "B".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "A".to_string() + self.accidental_notation.sharp,
                vec![
                    "G".to_string()
                        + self.accidental_notation.sharp
                        + self.accidental_notation.ddsharp,
                    "B".to_string() + self.accidental_notation.flat,
                    "C".to_string() + self.accidental_notation.ddflat,
                ],
            ),
            (
                "B".to_string(),
                vec![
                    "A".to_string() + self.accidental_notation.ddsharp,
                    "C".to_string() + self.accidental_notation.flat,
                ],
            ),
        ]);

        self.major = vec![
            "C major".to_string(),
            "D major".to_string(),
            "E major".to_string(),
            "F major".to_string(),
            "G major".to_string(),
            "A major".to_string(),
            "B major".to_string(),
        ];

        self.minor = vec![
            "C minor".to_string(),
            "D minor".to_string(),
            "E minor".to_string(),
            "F minor".to_string(),
            "G minor".to_string(),
            "A minor".to_string(),
            "B minor".to_string(),
        ];
    }

    fn gen_maj_sharp(&mut self) {
        self.maj_sharps =
            self.gen_min_maj_sharp_flat(self.major.clone(), self.accidental_notation.sharp);
    }

    fn gen_maj_flats(&mut self) {
        self.maj_flats =
            self.gen_min_maj_sharp_flat(self.major.clone(), self.accidental_notation.flat);
    }

    fn gen_min_sharp(&mut self) {
        self.min_sharps =
            self.gen_min_maj_sharp_flat(self.minor.clone(), self.accidental_notation.sharp);
    }

    fn gen_min_flats(&mut self) {
        self.min_flats =
            self.gen_min_maj_sharp_flat(self.minor.clone(), self.accidental_notation.flat);
    }

    fn gen_all_sharps(&mut self) {
        self.all_sharps =
            self.gen_combined_scales(&[self.maj_sharps.clone(), self.min_sharps.clone()])
    }

    fn gen_all_flats(&mut self) {
        self.all_flats = self.gen_combined_scales(&[self.maj_flats.clone(), self.min_flats.clone()])
    }

    fn gen_combined_scales(&self, slice_of_vecs: &[Vec<String>]) -> Vec<String> {
        slice_of_vecs.concat()
    }

    fn get_accidentals(&self) -> [&str; 5] {
        [
            self.accidental_notation.natural,
            self.accidental_notation.sharp,
            self.accidental_notation.flat,
            self.accidental_notation.ddsharp,
            self.accidental_notation.ddflat,
        ]
    }

    fn gen_all_accidentals(&mut self) {
        self.all_accidentals = self.gen_combined_scales(&[
            // not sure if this is the best solution, but I can't bother
            self.maj_flats.clone(),
            self.min_flats.clone(),
            self.maj_sharps.clone(),
            self.min_sharps.clone(),
        ]);
    }

    fn gen_all_notes(&mut self) {
        self.all_notes = self.gen_combined_scales(&[
            self.all_accidentals.clone(),
            self.major.clone(),
            self.minor.clone(),
        ])
    }

    fn gen_min_maj_sharp_flat(
        &self,
        min_or_maj_scale: Vec<String>,
        sharp_or_flat: &str,
    ) -> Vec<String> {
        let mut result = vec![];

        for accidental in self.get_accidentals() {
            if accidental == sharp_or_flat {
                for value in &min_or_maj_scale {
                    let split_note: Vec<&str> = value.split(' ').collect();
                    result.push(
                        [
                            split_note[0].to_string(),
                            accidental.to_string(),
                            " ".to_string(),
                            split_note[1].to_string(),
                        ]
                        .join(""),
                    )
                }
            }
        }

        result
    }

    // TODO:
    // * proper error handling
    // * introducing a BufWriter made this function faster, but still not as fast as python printing,
    // I think this has to do with all the things that get cloned in the process,
    // so go through the algo and use references where possible instead of cloning those objects
    fn give_cheats(&self) {
        let scale_types = ["major", "minor"];
        let stdout_lock = io::stdout().lock();
        let mut writer_buf = BufWriter::new(stdout_lock);

        for each_scale_type in scale_types {
            // scale name
            writer_buf.write_all(
                format!(
                    "\n\n{:>42}\n",
                    YELLOW_WITH_UNDERLINE.to_owned()
                        + &each_scale_type.to_uppercase()
                        + RESET_COLOR
                )
                .as_bytes(),
            ); // TODO: error handling

            for (primary_notation, alt_notation) in &self.complete_octave {
                let printable_answer = self.alight_answer(
                    self.find_answer(-1, primary_notation.trim(), each_scale_type)
                        .0,
                );

                let length_of_scale = format!(
                    "  {:<4}|  {}",
                    printable_answer[0],
                    printable_answer.join("-  ")
                )
                .chars()
                .count();

                // scale key header
                writer_buf.write_all(
                    format!(
                        "\n{:>width$} {} )\n",
                        "(",
                        printable_answer[0].trim(),
                        width = (length_of_scale as f32 / 2.25) as usize
                    )
                    .as_bytes(),
                );

                let mut ruler: Vec<String> = vec![];

                let mut ruler_counter = 0;

                // each number of ruler
                printable_answer.iter().enumerate().for_each(|(_, value)| {
                    ruler_counter += 1;
                    let note_char_length = value.chars().count();
                    ruler.push(format!(
                        "{:>width$}",
                        "(".to_owned() + &ruler_counter.to_string() + ")",
                        width = note_char_length
                    ));
                });

                let left_adjusted_padding = format!("{:>6}", printable_answer[0]).chars().count();

                // entire ruler of numbers
                writer_buf.write_all(
                    format!(
                        "  {:>width$}{}\n",
                        "|",
                        ruler.join("   "),
                        width = left_adjusted_padding - 1
                    )
                    .as_bytes(),
                );

                // primary notation
                writer_buf.write_all(
                    format!(
                        "  {:<width$}|  {}\n",
                        printable_answer[0],
                        printable_answer.join("-  "),
                        width = 4
                    )
                    .as_bytes(),
                );

                // alternative notations
                for each_note in alt_notation {
                    if each_note.contains(
                        &(self.accidental_notation.sharp.to_owned()
                            + self.accidental_notation.ddsharp),
                    ) || each_note.contains(
                        &(self.accidental_notation.flat.to_owned()
                            + self.accidental_notation.ddflat),
                    ) {
                        continue;
                    } else {
                        let printable_answer =
                            self.alight_answer(self.find_answer(-1, each_note, each_scale_type).0);
                        writer_buf.write_all(
                            format!(
                                "  {:<width$}|  {}\n",
                                printable_answer[0],
                                printable_answer.join("-  "),
                                width = 4
                            )
                            .as_bytes(),
                        );
                    }
                }
            }
            writer_buf.write_all("\n".as_bytes());
        }
    }

    fn askForAnswer(&self) -> Option<UserAnswer> {
        let mut user_input = String::new();
        if self.exercise_type == 1 {
            io::stdout().flush(); // TODO: error handling

            io::stdin().read_line(&mut user_input).unwrap();

            let user_answer = user_input.trim().to_owned();

            return Some(UserAnswer::SimpleString(user_answer));
        } else if self.exercise_type == 2 {
            io::stdout().flush(); // TODO: error handling

            io::stdin().read_line(&mut user_input).unwrap();

            let user_answer: Vec<String> =
                user_input.split(' ').map(|s| s.trim().to_owned()).collect();

            return Some(UserAnswer::VecOfStrings(
                if user_answer.len() == 1 && user_answer[0].is_empty() {
                    vec![]
                } else {
                    user_answer
                },
            ));
        } else {
            return None;
        }
    }

    fn ask_question(&self, selected_levels: &Vec<String>) -> Option<(i32, Vec<String>)> {
        let rnd_tone = rand::thread_rng().gen_range(2..=7); // range [2:7] because we don't want to include 1st and 8th tones
        let notes = selected_levels.clone();
        let rnd_note = notes
            .get(rand::thread_rng().gen_range(0..=notes.len()-1))
            .unwrap() as &str;
        if self.exercise_type == 1 {
            print!("\n\nQuestion: ");
            let num_suffix: &str;
            if rnd_tone == 1 {
                // "1st" tone is here only for completes instead this becomes useful in the future
                num_suffix = "st";
                let quest = format!(
                    "What is the {}{} tone of {}?",
                    rnd_tone, num_suffix, rnd_note
                );
                print!("{}", quest);
            } else if rnd_tone == 2 {
                num_suffix = "nd";
                let quest = format!(
                    "What is the {}{} tone of {}?",
                    rnd_tone, num_suffix, rnd_note
                );
                print!("{}", quest);
            } else if rnd_tone == 3 {
                num_suffix = "rd";
                let quest = format!(
                    "What is the {}{} tone of {}?",
                    rnd_tone, num_suffix, rnd_note
                );
                print!("{}", quest);
            } else {
                num_suffix = "th";
                let quest = format!(
                    "What is the {}{} tone of {}?",
                    rnd_tone, num_suffix, rnd_note
                );
                print!("{}", quest);
            }
            return Some((
                rnd_tone,
                rnd_note
                    .split(' ')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            ));
        } else if self.exercise_type == 2 {
            print!("\nWrite out {} scale:\n", rnd_note);
            return Some((
                rnd_tone,
                rnd_note
                    .split(' ')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            ));
        } else {
            return None;
        }
    }

    fn find_note(&self, root_note_of_scale: &str) -> Option<i32> {
        for (index, (primary_notation, alt_notations)) in
            (&self.complete_octave).into_iter().enumerate()
        {
            for alt_note in alt_notations {
                let note_rex_x =
                    Regex::new(format!(r"(^|[ ])({})([ ]|$)", primary_notation).as_str())
                        .unwrap()
                        .is_match(root_note_of_scale);
                let note_rex_y = Regex::new(format!(r"(^|[ ])({})([ ]|$)", alt_note).as_str())
                    .unwrap()
                    .is_match(root_note_of_scale);
                if note_rex_x || note_rex_y {
                    return Some(index as i32);
                }
            }
        }
        return None;
    }

    fn play(&self, selected_levels: Vec<String>) {
        println!("\n\nLet's begin:");
        loop {
            let question = self.ask_question(&selected_levels).unwrap();
            if self.exercise_type == 1 {
                print!("\n\nYour answer (optional): ");
            } else {
                print!("\n\nYour answer (space separated): ");
            }
            let user_answer = self.askForAnswer().unwrap(); // TODO: test with empty answer
            println!();
            if user_input_contains_exit_word(&user_answer) {
                exit(0);
            }
            self.give_answer(question, user_answer);
        }
    }

    fn find_answer(
        &self,
        tone_to_find: i32,
        root_note_of_scale: &str,
        maj_or_min: &str,
    ) -> (Vec<String>, Vec<String>) {
        let index_of_note_in_octave_map = self.find_note(root_note_of_scale).unwrap(); // returns an index of a root note in the octave
        let all_possible_notations = self.find_tone(
            root_note_of_scale,
            tone_to_find,
            index_of_note_in_octave_map,
            maj_or_min,
        );

        return all_possible_notations;
    }

    fn find_tone(
        &self,
        root_note_of_scale: &str,
        tone_to_find: i32,
        mut index_of_note_in_octave_map: i32,
        maj_or_min: &str,
    ) -> (Vec<String>, Vec<String>) {
        let scale_formula = match maj_or_min {
            "major" => "WWHWWWH",
            "minor" => "WHWWHWW",
            _ => "",
        };
        let mut all_tones: Vec<String> = vec![root_note_of_scale.to_string()];
        // This is retarded:
        let mut alphabet_list = vec![self
            .complete_octave
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .get(index_of_note_in_octave_map as usize)
            .unwrap()
            .to_string()]; // "alphabet_list" will keep track of actual "key(or string) that is played", while "all_tones" keeps correct notation for that key
        let mut tone_count = 1; // "1" to count in the starting tone
                                // let mut alphabet_answer = vec![];
        for step in scale_formula.chars().map(|some_char| some_char.to_string()) {
            let answer = self.tone_search(
                all_tones,
                &step,
                index_of_note_in_octave_map,
                tone_count,
                tone_to_find,
                &mut alphabet_list, // TODO: cloning here solves the problem of "use after borrow", but figure out the underlying issue and remove the `.clone`
            );
            index_of_note_in_octave_map = answer.0;
            all_tones = answer.1;
            tone_count = answer.2;
            // alphabet_answer = answer.3; // <--- "alphabet_list"
        }
        // `all_tones` contains the scale in the key used in the question,
        // `alphabet_list` contains the same scale but in the primary notation (only naturals and sharps)
        return (all_tones, alphabet_list);
    }

    fn tone_search(
        &self,
        mut all_tones: Vec<String>,
        step: &str,
        index_of_note_in_octave_map: i32,
        mut tone_count: i32,
        tone_to_find: i32,
        alphabet_list: &mut Vec<String>,
    ) -> (i32, Vec<String>, i32) {
        let primary_notes: Vec<&String> = self.complete_octave.keys().collect();
        let restart_list_w = index_of_note_in_octave_map + 2 - primary_notes.len() as i32;
        let restart_list_h = index_of_note_in_octave_map + 1 - primary_notes.len() as i32;
        if step == "W" {
            let step = 2;
            let made_step = self.make_step(
                all_tones,
                step,
                index_of_note_in_octave_map,
                tone_count,
                tone_to_find,
                alphabet_list, // TODO: cloning here solves the problem of "use after borrow", but figure out the underlying issue and remove the `.clone`
                restart_list_w,
            );
            let next_note = made_step.0;
            all_tones = made_step.1;
            tone_count = made_step.2;
            return (next_note, all_tones, tone_count);
        } else {
            // otherwise the step is always: step == "H"
            let step = 1;
            let made_step = self.make_step(
                all_tones,
                step,
                index_of_note_in_octave_map,
                tone_count,
                tone_to_find,
                alphabet_list, // TODO: cloning here solves the problem of "use after borrow", but figure out the underlying issue and remove the `.clone`
                restart_list_h,
            );
            let next_note = made_step.0;
            all_tones = made_step.1;
            tone_count = made_step.2;
            return (next_note, all_tones, tone_count);
        }
    }

    // TODO: replace `self.complete_octave.keys().collect::Vec<&String>()` with something like `self.primary_notes` where `primary_notes` is a vector of complete_octave.keys()
    fn make_step(
        &self,
        mut all_tones: Vec<String>,
        step: i32,
        index_of_note_in_octave_map: i32,
        mut tone_count: i32,
        tone_to_find: i32,
        alphabet_list: &mut Vec<String>,
        restart_list: i32,
    ) -> (i32, Vec<String>, i32) {
        if tone_to_find > 1 {
            tone_count += 1;
        }
        let mut next_note = index_of_note_in_octave_map + step;
        let right_adjusted_padding = "Relative Minor :".chars().count();

        // TODO: proper docs, I don't remember what exactly `restart_list` is for, but I think it indicates whether we reached an end in counting the tones of the octave and restarted the count from the beginning of it
        // the `restart_list` is negative if: index_of_note_in_octave_map + 1 - primary_notes.len() -- so it must be related to the "overflowing" over the end of the octave
        if restart_list >= 0 {
            next_note = restart_list;
            if next_note
                > self
                    .complete_octave
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .len() as i32
            {
                next_note -= self
                    .complete_octave
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .len() as i32;
            }
            let alphabet = self.alpha_search(&all_tones, next_note, alphabet_list); // ['B', 'C♯', ..., 'F♯'] , F♯
            all_tones.push(alphabet);
            if self.exercise_type == 1 {
                if tone_count == tone_to_find && tone_to_find > -1 {
                    println!("-----------------------------------");
                    println!(
                        "\n{:>width$} {}\n",
                        "Answer :",
                        all_tones[all_tones.len() - 1],
                        width = right_adjusted_padding
                    );
                }
            } else if self.exercise_type == 2 && tone_count == 8 {
                println!(
                    "\n{:>width$} {}\n",
                    "Answer :",
                    all_tones.join(" "),
                    width = right_adjusted_padding
                );
            }
            return (next_note, all_tones, tone_count);
        } else {
            let alphabet = self.alpha_search(&all_tones, next_note, alphabet_list); // ['B', 'C♯', ..., 'F♯'] , F♯
            all_tones.push(alphabet);
            if self.exercise_type == 1 {
                if tone_count == tone_to_find && tone_to_find > -1 && self.exercise_type == 1 {
                    println!("-----------------------------------");
                    println!(
                        "\n{:>width$} {}\n",
                        "Answer :",
                        all_tones[all_tones.len() - 1],
                        width = right_adjusted_padding
                    );
                }
            } else if self.exercise_type == 2 && tone_count == 8 {
                println!(
                    "\n{:>width$} {}\n",
                    "Answer :",
                    all_tones.join(" "),
                    width = right_adjusted_padding
                );
            }
            return (next_note, all_tones, tone_count);
        }
    }

    // TODO: this function is ridiculous, abstract all those waterfalls away as some `self` field and use it here
    fn alpha_search(
        &self,
        all_tones: &Vec<String>,
        next_note: i32,
        alphabet_list: &mut Vec<String>,
    ) -> String {
        let alphabet = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];

        // getting first character of the tone because some tones have accidentals
        if all_tones.last().unwrap().chars().nth(0).unwrap() == 'B' {
            for y in self
                .complete_octave
                .get(
                    self.complete_octave
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>()
                        .get(next_note as usize)
                        .unwrap(),
                )
                .unwrap()
            {
                if y.chars().nth(0).unwrap() == 'C' {
                    alphabet_list.push(
                        self.complete_octave
                            .keys()
                            .cloned()
                            .collect::<Vec<String>>()
                            .get(next_note as usize)
                            .unwrap()
                            .clone(),
                    );
                    return y.to_string();
                }
            }
        } else {
            for y in self
                .complete_octave
                .get(
                    self.complete_octave
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>()
                        .get(next_note as usize)
                        .unwrap(),
                )
                .unwrap()
            {
                if y.chars().nth(0).unwrap()
                    == alphabet[alphabet
                        .iter()
                        .position(|&x| x == all_tones.last().unwrap().chars().nth(0).unwrap())
                        .unwrap()
                        + 1]
                {
                    alphabet_list.push(
                        self.complete_octave
                            .keys()
                            .cloned()
                            .collect::<Vec<String>>()
                            .get(next_note as usize)
                            .unwrap()
                            .clone(),
                    );
                    return y.to_string();
                }
            }
        }
        alphabet_list.push(
            self.complete_octave
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .get(next_note as usize)
                .unwrap()
                .clone(),
        );
        return self
            .complete_octave
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .get(next_note as usize)
            .unwrap()
            .to_string();
    }

    fn give_answer(
        &self,
        question: (i32, Vec<String>), // TODO: I have a feeling that I should try to rewrite the code in such a way, so that the types specified here can be traced down to GameLevel-type primitives
        user_answer: UserAnswer,
    ) // -> Vec<String>
    {
        let tone_to_find = question.0;
        let root_note_of_scale = question.1.first().unwrap();
        let maj_or_min = question.1.get(1).unwrap();

        let found_answer = self.find_answer(tone_to_find, root_note_of_scale, maj_or_min); // returns: [['E♭', 'F', 'G♭',...], ['D♯', 'F', 'F♯',...]]

        self.check_answer(tone_to_find, user_answer, found_answer.clone());

        let printable_answer = self.alight_answer(found_answer.0);

        let mut ruler: Vec<String> = vec![];

        let mut ruler_counter = 0;

        for value in printable_answer.iter() {
            ruler_counter += 1;
            let note_char_length = value.chars().count();
            ruler.push(format!(
                "{:<width$}",
                ruler_counter,
                width = note_char_length
            ));
        }

        let right_adjusted_padding = "Relative Minor :".chars().count();

        println!(
            "{:>width$} {}",
            "|",
            ruler.join(""),
            width = right_adjusted_padding
        );
        println!(
            "{:>width$} {}",
            "Scale :",
            printable_answer.join(""),
            width = right_adjusted_padding
        );

        // finding relative minor and major
        if maj_or_min == "major" {
            let rel_minor = self.alight_answer(
                self.find_answer(
                    -1, // -1 is used in `makeStep` function to avoid printing the "Answer :" line // TODO: refactor to a better solution
                    printable_answer[printable_answer.len() - 3].trim(),
                    "minor",
                )
                .0,
            );

            println!(
                "{:>width$} {}",
                "Relative Minor :",
                rel_minor.join(""),
                width = right_adjusted_padding
            );
        } else {
            // maj_or_min == "minor"
            let rel_major = self.alight_answer(
                self.find_answer(
                    -1, // -1 is used in `make_step` function to avoid printing the "Answer :" line
                    printable_answer[printable_answer.len() - 6].trim(),
                    "major",
                )
                .0,
            );

            println!(
                "{:>width$} {}",
                "Relative Major :",
                rel_major.join(""),
                width = right_adjusted_padding
            );
        }
        println!("\n-----------------------------------");

        // let mut pause_input = String::new();
        // io::stdout().flush();
        // pause_input.clear();
        // io::stdin().read_line(&mut pause_input);

        println!("(Press \"enter\" to continue or type \"exit\" to quit)");
        // the
        let pause_input = self.askForAnswer().unwrap(); // this might fail? (only if exercise type is not set, or set illogically (to a number which does not represent any exercise))
        if user_input_contains_exit_word(&pause_input) {
            exit(0);
        }
        print!("{}", "\n".repeat(40));

        // print!("{}[2J", 27 as char);
    }

    fn check_answer(
        &self,
        tone_to_find: i32,
        user_answer: UserAnswer,
        found_answer: (Vec<String>, Vec<String>),
    ) {
        let right_adjusted_padding = "Relative Minor :".chars().count();

        match user_answer {
            // this would mean that exercise type is 1
            UserAnswer::SimpleString(answer_value) => {
                let alphabet_answer = found_answer.1;
                let simple_answer =
                    self.simplify_note(found_answer.0.get((tone_to_find - 1) as usize).unwrap());
                // "if the answer is not empty"
                if !answer_value.is_empty() {
                    if answer_value.to_lowercase() != simple_answer.to_lowercase()
                        && (
                            // alternative notations contain user answer
                            self.complete_octave.get(alphabet_answer.get((tone_to_find-1) as usize).unwrap())
                        .unwrap()
                        .clone()
                        .into_iter()
                        .map(|each_value| {self.simplify_note(&each_value.to_lowercase())}).collect::<Vec<String>>().contains(&answer_value.to_lowercase())
            // or TODO: docs, don't remember what this comparison is for (some comparison with "underlying" alphabetical notation of tone?)
            || answer_value.to_lowercase() == self.simplify_note(&alphabet_answer[(tone_to_find-1) as usize].to_lowercase())
                        )
                    {
                        println!(
                            "{:>width$} \"{}\" -- {}, but not notation-wise ({})\n",
                            "Your answer:",
                            answer_value,
                            GREEN.to_owned() + "Enharmonically correct" + RESET_COLOR,
                            found_answer.0[(tone_to_find - 1) as usize],
                            width = right_adjusted_padding
                        );
                    } else if answer_value.to_lowercase() == simple_answer.to_lowercase() {
                        println!(
                            "{:>width$} \"{}\" -- {}\n",
                            "Your answer :",
                            answer_value,
                            GREEN.to_string() + "correct" + RESET_COLOR,
                            width = right_adjusted_padding
                        );
                    } else {
                        println!(
                            "{:>width$} \"{}\" -- {}\n",
                            "Your answer :",
                            answer_value,
                            RED.to_string() + "nope" + RESET_COLOR,
                            width = right_adjusted_padding
                        );
                    }
                }
            }

            // this would mean that exercise type is 2
            UserAnswer::VecOfStrings(answer_value) => {
                // "if the answer is not empty"
                if !answer_value.is_empty() {
                    if answer_value
                        .iter()
                        .map(|each_value| self.simplify_note(&each_value.to_lowercase()))
                        .collect::<Vec<String>>()
                        == found_answer
                            .0
                            .into_iter()
                            .map(|each_value| self.simplify_note(&each_value.to_lowercase()))
                            .collect::<Vec<String>>()
                    {
                        println!(
                            "{:>width$} \"{}\" -- {}\n",
                            "Your answer :",
                            answer_value.join(" "),
                            GREEN.to_string() + "correct" + RESET_COLOR,
                            width = right_adjusted_padding
                        );
                    } else {
                        println!(
                            "{:>width$} \"{}\" -- {}\n",
                            "Your answer :",
                            answer_value.join(" "),
                            RED.to_string() + "nope" + RESET_COLOR,
                            width = right_adjusted_padding
                        );
                    }
                }
            }
        }
    }

    fn simplify_note(&self, given_note: &String) -> String {
        if given_note.contains(self.accidental_notation.ddsharp) {
            return given_note.chars().nth(0).unwrap().to_string() + "##";
        } else if given_note.contains(self.accidental_notation.ddflat) {
            return given_note.chars().nth(0).unwrap().to_string() + "bb";
        } else if given_note.contains(self.accidental_notation.sharp) {
            return given_note.chars().nth(0).unwrap().to_string() + "#";
        } else if given_note.contains(self.accidental_notation.flat) {
            return given_note.chars().nth(0).unwrap().to_string() + "b";
        }
        return given_note.to_string();
    }

    fn mut_askExercise(&mut self) -> i32 {
        let mut user_input = String::new();

        loop {
            println!("\nChoose exercise:\n1 - Find Tone\n2 - Write Out Scales");
            print!("\nExercise: ");
            io::stdout().flush(); // TODO: error handling
            user_input.clear();
            io::stdin().read_line(&mut user_input).unwrap();

            match user_input.trim() {
                "1" => {
                    self.exercise_type = 1;
                    return 1;
                }
                "2" => {
                    self.exercise_type = 2;
                    return 2;
                }
                "exit" => exit(0),
                "quit" => exit(0),
                "q" => exit(0),
                _ => {
                    println!("* Please enter a valid option");
                    continue;
                }
            };
        }
    }

    fn alight_answer(&self, found_answer: Vec<String>) -> Vec<String> {
        let mut result = vec![];

        for note in found_answer {
            result.push(format!("{:<4}", note));
        }

        return result;
    }
}

impl<'a> Game<'a> {
    fn init(mut self, o: &Octave) -> Self {
        self.levels = IndexMap::from([
            (
                "1",
                GameLevel {
                    level_name: "Major scales".to_string(),
                    notes_set: o.major.clone(),
                },
            ),
            (
                "2",
                GameLevel {
                    level_name: "Minor scales".to_string(),
                    notes_set: o.minor.clone(),
                },
            ),
            (
                "3",
                GameLevel {
                    level_name: [
                        "All sharp accidental scales (",
                        o.accidental_notation.sharp,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.all_sharps.clone(),
                },
            ),
            (
                "3a",
                GameLevel {
                    level_name: [
                        "Only major sharp scales (major ",
                        o.accidental_notation.sharp,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.maj_sharps.clone(),
                },
            ),
            (
                "3b",
                GameLevel {
                    level_name: [
                        "Only minor sharp scales (minor ",
                        o.accidental_notation.sharp,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.min_sharps.clone(),
                },
            ),
            (
                "4",
                GameLevel {
                    level_name: [
                        "All flat accidental scales (",
                        o.accidental_notation.flat,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.all_flats.clone(),
                },
            ),
            (
                "4a",
                GameLevel {
                    level_name: [
                        "Only major flat scales (major ",
                        o.accidental_notation.flat,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.maj_flats.clone(),
                },
            ),
            (
                "4b",
                GameLevel {
                    level_name: [
                        "Only minor flat scales (minor ",
                        o.accidental_notation.flat,
                        ")",
                    ]
                    .concat(),
                    notes_set: o.min_flats.clone(),
                },
            ),
            (
                "5",
                GameLevel {
                    level_name: "Everything".to_string(),
                    notes_set: o.all_notes.clone(),
                },
            ),
        ]);

        self.types = Game::<'a>::gen_game_types();
        self
    }

    fn gen_game_types() -> Vec<GameType<'a>> {
        vec![
            GameType {
                game_type_id: 1,
                game_type_name: "Find Tone",
            },
            GameType {
                game_type_id: 2,
                game_type_name: "Write the Scale",
            },
        ]
    }
}

fn init_game_resources<'a>(cmd_args: &CommandArguments) -> (Game<'a>, Octave) {
    let octave = Octave::default().init(cmd_args);
    let game = Game::default().init(&octave);
    (game, octave)
}

#[derive(Debug, Default)]
struct GameType<'a> {
    // TODO: either remove these because they are not needed, or rewrite code so that it uses these fields
    game_type_id: i32,
    game_type_name: &'a str,
}

#[derive(Debug, Default)]
struct GameLevel {
    level_name: String,
    notes_set: Vec<String>,
}

#[derive(Debug, Default)]
struct Game<'a> {
    types: Vec<GameType<'a>>,
    levels: IndexMap<&'a str, GameLevel>,
}

fn select_levels(all_levels: &IndexMap<&str, GameLevel>) -> Option<Vec<String>> {
    loop {
        print!("\n--------------------------------------------------\n");
        println!("\t\tChoose the difficulty:\n");
        println!(" Select what to include:");
        let alt_option_rex = Regex::new(r".*(.a|.b).*").unwrap();
        for (option_id, level) in all_levels {
            // there's a better way to re-write this
            if alt_option_rex.is_match(option_id) {
                print!("\n{:>5} - {}", option_id, level.level_name);
            } else {
                print!("\n{:>2} - {}", option_id, level.level_name);
            }
        }
        let mut user_input = String::new();
        print!("\n\nInclude: ");
        io::stdout().flush(); // TODO: error handling
        io::stdin().read_line(&mut user_input).unwrap();
        print!("\n--------------------------------------------------\n");

        // `user_input` can look like this "44a12"

        // checking validity of selected levels:
        let mut notes = Vec::new();
        let mut not_found = 0;

        for each_level_option_id in all_levels.keys().cloned().collect::<Vec<&str>>() {
            let primary_option_rex =
                Regex::new(format!(".*({}).*", each_level_option_id).as_str()).unwrap();

            // checking here if the user's input contains primary option id:
            let found_primary_option = primary_option_rex.is_match(&user_input);

            if found_primary_option {
                // if a primary option was found in the user input, check if there follows 'a' or 'b' right after
                // and if so, add that option from the game levels to the set of levels that his function returns
                let found_sub_option = Regex::new(
                    format!(r".*({}a|{}b).*", each_level_option_id, each_level_option_id).as_str(),
                )
                .unwrap()
                .find(&user_input);
                if found_sub_option.is_some() {
                    continue; // skipping this iteration because user's input was found to actually refer to a suboption which will be the `each_level_option_id` and therefore `primary_option_rex` of the next iteration where it will go into the `else` branch of this fork
                } else {
                    notes.extend(
                        all_levels
                            .get(each_level_option_id)
                            .unwrap()
                            .notes_set
                            .clone(),
                    );
                    print!(
                        "\n* {:?}",
                        all_levels.get(each_level_option_id).unwrap().level_name
                    );
                }
            } else {
                not_found += 1;
                if not_found > all_levels.len() - 1 {
                    println!("\nInvalid options specified.");
                    continue;
                }
            }
        }

        if notes.is_empty() {
            println!("\nNote sets are empty.");
            continue;
        } else {
            // if notes.len() > 7 {
            //     println!("\n\nRemoving duplicates..")
            // };
            let deduplicated_notes = HashSet::<String>::from_iter(notes)
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            return Some(deduplicated_notes);
        }
    }
}

#[derive(Debug)]
enum UserAnswer {
    SimpleString(String),
    VecOfStrings(Vec<String>),
}

fn user_input_contains_exit_word(user_answer: &UserAnswer) -> bool {
    match user_answer {
        UserAnswer::SimpleString(value) => {
            if value.eq("exit") || value.eq("quit") || value.eq("q") {
                return true;
            }
        }
        UserAnswer::VecOfStrings(value) => {
            if value.contains(&"exit".to_string())
                || value.contains(&"quit".to_string())
                || value.contains(&"q".to_string())
            {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let cmd_args = CommandArguments::parse();

    if cmd_args.basic_notation_enabled {
        println!("* Basic Notes - Enabled");
    }

    let (game, mut octave) = init_game_resources(&cmd_args);

    if cmd_args.get_cheats {
        octave.give_cheats();
        exit(0);
    }

    octave.mut_askExercise();
    let selected_levels = select_levels(&game.levels);

    octave.play(selected_levels.unwrap());
}

// TODO:
// * either find a lib wrapper that uses buffered writer or write your own,
// because the printing is slow not only in this function but also when asking for exercise
// * see the error dir from practice for insp on how to manage io errors and read files
// * -h option for hints? for exercise 1. "the X tone is between E# .. G", for 2. just give a scale with random spots in it being "_" blank
// * -s to see "string view" (with variable string count?) (could colour code it)
// * rewrite what you can with generics
// * spread the code by modules (mod octave, mod game, etc..)
