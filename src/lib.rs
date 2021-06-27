use rand::Rng;
use std::fmt;
use std::str::FromStr;

pub mod gen {
    use crate::gen_stream;
    use crate::{Measure, NoteLine, Snap};
    pub fn gen_stream(snap: Snap) {
        match snap {
            Snap::S16th => {
                gen_stream!(12, 12, 12, 12);
            }
            Snap::S20th => {
                gen_stream!(10, 9, 10, 9, 10);
            }
            Snap::S22nd => {
                gen_stream!(9, 9, 9, 8, 9, 9, 9, 8, 9, 9, 8);
            }
            Snap::S26th => {
                gen_stream!(7, 7, 8, 7, 8, 7, 7, 8, 7, 8, 7, 7, 8);
            }
            Snap::S28th => {
                gen_stream!(7, 7, 7, 7, 7, 7, 6);
            }
            Snap::S36th => {
                gen_stream!(5, 5, 6, 5, 5, 6, 5, 5, 6);
            }
            Snap::S40th => {
                gen_stream!(5, 5, 5, 5, 4, 5, 5, 5, 5, 4);
            }
            Snap::S44th => {
                gen_stream!(4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5);
            }
            Snap::S70th => {
                gen_stream!(
                    3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3,
                    3, 2, 3, 3, 3, 2, 3, 3, 2
                );
            }
            Snap::S80th => {
                gen_stream!(2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2);
            }
            _ => println!("afdasdf"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Note {
    None,
    Tap,
    HoldStart,
    RollStart,
    LNEnd,
    Mine,
    Fake,
}

#[derive(Debug)]
struct Measure {
    notes: Vec<NoteLine>,
}

#[derive(Clone, Debug)]
struct NoteLine {
    notes: Vec<Note>,
}

#[derive(Debug)]
pub enum Snap {
    //cant start enum name with number zzzzz
    S4th,
    S8th,
    S12th,
    S16th,
    S20th,
    S22nd,
    S26th,
    S28th,
    S32nd,
    S36th,
    S40th,
    S44th,
    S48th,
    S64th,
    S70th,
    S80th,
    S96th,
    S192nd,
}

#[derive(Debug)]
pub enum SnapParseError {
    UnrecognizedSnap,
}

impl FromStr for Snap {
    type Err = SnapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let snap = match s {
            "4th" | "4" => Snap::S4th,
            "8th" | "8" => Snap::S8th,
            "12th" | "12" => Snap::S12th,
            "16th" | "16" => Snap::S16th,
            "20th" | "20" => Snap::S20th,
            "22th" | "22" => Snap::S22nd,
            "26th" | "26" => Snap::S26th,
            "28th" | "28" => Snap::S28th,
            "32th" | "32" => Snap::S32nd,
            "36th" | "36" => Snap::S36th,
            "40th" | "40" => Snap::S40th,
            "44th" | "44" => Snap::S44th,
            "48th" | "48" => Snap::S48th,
            "64th" | "64" => Snap::S64th,
            "70th" | "70" => Snap::S70th,
            "80th" | "80" => Snap::S80th,
            "96th" | "96" => Snap::S96th,
            "192th" | "192" => Snap::S192nd,
            _ => {
                return Err(SnapParseError::UnrecognizedSnap);
            }
        };
        Ok(snap)
    }
}

impl fmt::Display for Snap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Self::S4th => "4th",
            Self::S8th => "8th",
            Self::S12th => "12th",
            Self::S16th => "16th",
            Self::S20th => "20th",
            Self::S22nd => "22nd",
            Self::S26th => "26th",
            Self::S28th => "28th",
            Self::S32nd => "32nd",
            Self::S36th => "36th",
            Self::S40th => "40th",
            Self::S44th => "44th",
            Self::S48th => "48th",
            Self::S64th => "64th",
            Self::S70th => "70th",
            Self::S80th => "80th",
            Self::S96th => "96th",
            Self::S192nd => "192nd",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
enum NoteGenErr {
    DiffLengths, //maybe this is unnecessary given the keycount will probably always be hardcoded to 4 but nothing wrong with keeping the door open for expansion later i guess
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: &str;
        result = match self {
            Note::None => "0",
            Note::Tap => "1",
            Note::HoldStart => "2",
            Note::RollStart => "4",
            Note::LNEnd => "3",
            Note::Mine => "M",
            Note::Fake => "F",
        };
        write!(f, "{}", result)
    }
}

impl fmt::Display for SnapParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            SnapParseError::UnrecognizedSnap => "Snap not recognized!",
        };

        write!(f, "{}", result)
    }
}

impl Measure {
    fn new() -> Self {
        Self { notes: Vec::new() }
    }

    fn push(&mut self, notes: NoteLine) {
        self.notes.push(notes);
    }
}

impl NoteLine {
    fn gen_single(key_num: usize) -> Self {
        let mut notes: Vec<Note> = Vec::new();
        let note_num = rand::thread_rng().gen_range(0..key_num);
        for i in 0..key_num {
            if i == note_num {
                notes.push(Note::Tap);
            } else {
                notes.push(Note::None);
            }
        }
        Self { notes }
    }
    fn gen_empty(key_num: usize) -> Self {
        let mut notes: Vec<Note> = Vec::new();
        for _ in 0..key_num {
            notes.push(Note::None);
        }
        Self { notes }
    }
    fn is_minijack(line1: &NoteLine, line2: &NoteLine) -> Result<bool, NoteGenErr> {
        if line1.notes.len() != line2.notes.len() {
            return Err(NoteGenErr::DiffLengths);
        }
        for i in 0..line1.notes.len() {
            if line1.notes[i] == Note::Tap && line2.notes[i] == Note::Tap {
                return Ok(true);
            }
        }
        return Ok(false);
    }
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");
        for nl in &self.notes {
            result.push_str(&format!("{}\n", nl));
        }

        write!(f, "{},\n", result)
    }
}

impl fmt::Display for NoteLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");

        for note in &self.notes {
            result.push_str(&format!("{}", note));
        }

        write!(f, "{}", result)
    }
}

#[macro_export]
macro_rules! gen_stream {
    ($ ($spacing:expr),*) => {
        let mut total: usize = 0;
        let mut spacings: Vec<usize> = Vec::new();
        $(spacings.push($spacing);)*
        for num in &spacings{
            total += num;
        }
        println!("tot: {}", total);
        let mut last = NoteLine::gen_single(4);

        let mut notes: Vec<Measure> = Vec::new();
        notes.push(Measure::new());
        notes[0].push(last.clone());

        let mut space_count = 0;
        let mut spacing_num = 0;
        let mut meas_num = 0;
        let num_runs = 192 / total;
        println!("num runs: {}", num_runs);
        for i in 0..num_runs{
            for j in 0..total{
                if i * j == (num_runs - 1)*(total - 1) {//the last note is the first note of the next measure, so make sure to indicate its a new measure before placing it
                    meas_num += 1;
                    notes.push(Measure::new());
                }
                if space_count == spacings[spacing_num] - 1{//if this line is where a note gets placed
                    let mut current = NoteLine::gen_single(4);
                    while let Ok(true) = NoteLine::is_minijack(&last, &current){
                        current = NoteLine::gen_single(4);
                    }
                    last = current.clone();
                    notes[meas_num].push(current);
                    space_count = 0;
                    if spacing_num + 1 == spacings.len(){//if we run out of spacings, repeat
                        spacing_num = 0;
                    }else {
                        spacing_num += 1;
                    }
                } else {
                    notes[meas_num].push(NoteLine::gen_empty(4));
                    space_count += 1;
                }
            }
        }
        for _ in 0..3{
            notes[meas_num].push(NoteLine::gen_empty(4));
        }
        for i in 0..notes.len(){
            print!("{}", notes[i]);
        }
    };
}
