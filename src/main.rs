use rand::Rng;
use std::fmt;

fn main() {
    gen_stream(Snap::S26th);
}

fn gen_stream(snap: Snap) {
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
                3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2,
                3, 3, 3, 2, 3, 3, 2
            );
        }
        Snap::S80th => {
            gen_stream!(2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2);
        }
        _ => println!("afdasdf"),
    }
}

#[derive(PartialEq)]
enum Note {
    None,
    Tap,
    HoldStart,
    RollStart,
    LNEnd,
    Mine,
    Fake,
}
struct NoteLine {
    notes: Vec<Note>,
}

enum Snap {
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
        println!("{}",last);

        let mut space_count = 0;
        let mut spacing_num = 0;
        let num_runs = 192 / total;
        for i in 0..num_runs{
            for j in 0..total{
                if i * j == (num_runs - 1)*(total - 1) {//the last note is the first note of the next measure, so make sure to indicate its a new measure before placing it
                    println!(",");
                }
                if space_count == spacings[spacing_num] - 1{//if this line is where a note gets placed
                    let mut current = NoteLine::gen_single(4);
                    while let Ok(true) = NoteLine::is_minijack(&last, &current){
                        current = NoteLine::gen_single(4);
                    }
                    println!("{}", current);
                    space_count = 0;
                    if spacing_num + 1 == spacings.len(){//if we run out of spacings, repeat
                        spacing_num = 0;
                    }else {
                        spacing_num += 1;
                    }
                    last = current;
                } else {
                    println!("{}", NoteLine::gen_empty(4));
                    space_count += 1;
                }
            }
        }
        println!("0000");//uhhhh change this uhhhhh
        println!("0000");
        println!("0000");
    };
}
