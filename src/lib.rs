use rand::Rng;
use std::fmt;
use std::str::FromStr;

pub mod gen {
    use crate::{File, NoteLine, Pattern, Snap};
    use clipboard::{ClipboardContext, ClipboardProvider};
    pub fn gen_pattern(
        pattern: Pattern,
        snap: Snap,
        num_meas: usize,
        anchor_len: Option<usize>,
        super_shuffle: bool,
    ) {
        let mut total: usize = 0;
        let spacings: Vec<usize> = snap.get_192nds();
        let note_types = pattern.get_chordtypes();
        for num in &spacings {
            total += num;
        }
        let mut last = note_types[0].gen();

        let mut notes = File::new(snap);
        notes.push_noteline(last.clone()); //note 0

        let mut space_count = 0;
        let mut spacing_num = 0;
        let mut note_num = 1; //start with 1 since we did note 0 four lines above
        let num_runs = 192 / total;
        let num_runs = num_runs * num_meas;
        let a_len = match anchor_len {
            Some(val) => val,
            None => usize::MAX,
        };
        for _ in 0..num_runs {
            for _ in 0..total {
                if space_count == spacings[spacing_num] - 1 {
                    //if this line is where a note gets placed
                    let mut current = note_types[note_num % note_types.len()].gen();
                    let is_cj =
                        (pattern == Pattern::Chordjacks) || (pattern == Pattern::DenseChordjacks);
                    while !super_shuffle && !is_cj//not sure if this is really the best way to do this, as this prevents anchor limits, although enforcing those with chordjacks would be somewhat problematic as low anchor lengths would be very limiting
                        && (NoteLine::is_minijack(&last, &current)
                        || notes.current_anchor_length(&current) >= a_len)
                    {
                        //this will need to be changed in someway for chordjacks
                        current = note_types[note_num % note_types.len()].gen();
                    }
                    last = current.clone();
                    notes.push_noteline(current);
                    note_num += 1;
                    space_count = 0;
                    if spacing_num + 1 == spacings.len() {
                        //if we run out of spacings, repeat
                        spacing_num = 0;
                    } else {
                        spacing_num += 1;
                    }
                } else {
                    notes.push_noteline(NoteLine::gen_empty());
                    space_count += 1;
                }
            }
        }
        for _ in 0..3 {
            notes.push_noteline(NoteLine::gen_empty());
        }
        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
            Ok(c) => c,
            Err(e) => {
                println!("Error `{:?}` when getting clipboard access", e);
                return;
            }
        };
        match ctx.set_contents(format!("{}", notes)) {
            Ok(_) => {
                println!("Notedata copied to clipboard.");
            }
            Err(e) => {
                println!(
                    "Error `{:?}` when attempting to copy notedata to clipboard.",
                    e
                );
            }
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Note {
    None,
    Tap,
    /*
    Commenting these out for now. I kinda want them here for completeness' sake, but I'm not really sure how I'd use them
    HoldStart,
    RollStart,
    LNEnd,
    Mine,
    Fake, */
}

enum ChordType {
    Single,
    Jump,
    Hand,
    Quad,
}

#[derive(Debug)]
struct File {
    notes: Vec<Measure>,
    snap: Snap,
}

#[derive(Debug)]
struct Measure {
    notes: Vec<NoteLine>,
}

#[derive(Clone, Debug)]
struct NoteLine {
    notes: Vec<Note>,
}

struct NotePointer {
    pub pos: usize,
    pub spacings: Vec<usize>,
}

impl NotePointer {
    pub fn new(init_pos: usize, snap: Snap) -> Self {
        Self {
            pos: init_pos,
            spacings: snap.get_192nds(),
        }
    }

    pub fn move_pointer(&mut self, num_pos_to_move: usize) {
        let mut cur_space: usize;
        for _ in 0..num_pos_to_move {
            cur_space = self.pos % self.spacings.len();

            self.pos = if self.spacings[cur_space] < self.pos {
                self.pos - self.spacings[cur_space]
            } else {
                0
            };
            if self.pos == 0 {
                break;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Snap {
    //S at the beginning of each as enum name can't start with number
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

#[derive(Debug)]
pub enum PatternParseError {
    UnrecognizedPattern,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Stream,
    Jumpstream,
    LightJumpstream,
    Handstream,
    Chordjacks,
    DenseChordjacks,
}

impl Snap {
    fn get_192nds(&self) -> Vec<usize> {
        match self {
            Snap::S4th => {
                vec![48]
            }
            Snap::S8th => {
                vec![24; 2]
            }
            Snap::S12th => {
                vec![16; 3]
            }
            Snap::S16th => {
                vec![12; 4]
            }
            Snap::S20th => {
                vec![10, 9, 10, 9, 10]
            }
            Snap::S22nd => {
                vec![9, 9, 9, 8, 9, 9, 9, 8, 9, 9, 8]
            }
            Snap::S26th => {
                vec![7, 7, 8, 7, 8, 7, 7, 8, 7, 8, 7, 7, 8]
            }
            Snap::S28th => {
                vec![7, 7, 7, 7, 7, 7, 6]
            }
            Snap::S32nd => {
                vec![6; 8]
            }
            Snap::S36th => {
                vec![5, 5, 6, 5, 5, 6, 5, 5, 6]
            }
            Snap::S40th => {
                vec![5, 5, 5, 5, 4, 5, 5, 5, 5, 4]
            }
            Snap::S44th => {
                vec![4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5]
            }
            Snap::S48th => {
                vec![4; 12]
            }
            Snap::S64th => {
                vec![3; 16]
            }
            Snap::S70th => {
                vec![
                    3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3, 3, 2, 3, 3,
                    3, 2, 3, 3, 3, 2, 3, 3, 2,
                ]
            }
            Snap::S80th => {
                vec![2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2, 2, 3, 2, 3, 2]
            }
            Snap::S96th => {
                vec![2; 24]
            }
            Snap::S192nd => {
                vec![1; 48]
            }
        }
    }
}

impl FromStr for Pattern {
    type Err = PatternParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fmted_s = s.to_lowercase();
        let fmted_s = fmted_s.trim();
        let pattern = match &fmted_s[..] {
            "stream" => Pattern::Stream,
            "jumpstream" | "js" => Pattern::Jumpstream,
            "handstream" | "hs" => Pattern::Handstream,
            "chordjacks" | "chordjack" | "cjs" | "cj" => Pattern::Chordjacks,
            "light jumpstream" | "light js" | "ljs" => Pattern::LightJumpstream,
            "dense chordjacks" | "dense cjs" | "dcjs" | "dcj" => Pattern::DenseChordjacks,
            _ => {
                return Err(PatternParseError::UnrecognizedPattern);
            }
        };
        Ok(pattern)
    }
}
impl FromStr for Snap {
    type Err = SnapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fmted_s = s.to_lowercase();
        let fmted_s = fmted_s.trim();
        let snap = match fmted_s {
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

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Self::Stream => "Stream",
            Self::Jumpstream => "Jumpstream",
            Self::LightJumpstream => "Light Jumpstream",
            Self::Handstream => "Handstream",
            Self::Chordjacks => "Chordjacks",
            Self::DenseChordjacks => "Dense Chordjacks",
        };
        write!(f, "{}", result)
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

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: &str;
        result = match self {
            Note::None => "0",
            Note::Tap => "1",
            /*  Note::HoldStart => "2",
            Note::RollStart => "4",
            Note::LNEnd => "3",
            Note::Mine => "M",
            Note::Fake => "F", */
        };
        write!(f, "{}", result)
    }
}

impl fmt::Display for PatternParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            PatternParseError::UnrecognizedPattern => "Pattern not recognized!",
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

impl File {
    fn new(snap: Snap) -> Self {
        let mut notes = Vec::new();
        notes.push(Measure::new());
        Self { notes, snap }
    }

    fn get_noteline(&self, noteline_num: usize) -> NoteLine {
        let meas_num = noteline_num / 192;
        let nl_num = noteline_num % 192;
        self.notes[meas_num].notes[nl_num].clone()
    }

    fn total_notelines(&self) -> usize {
        let num_full_meas = self.notes.len();
        let last_meas_len = self.notes[num_full_meas - 1].notes.len();
        (192 * (num_full_meas - 1)) + last_meas_len
    }

    fn push_noteline(&mut self, noteline: NoteLine) {
        let current_meas_num = self.notes.len();
        if self.notes[current_meas_num - 1].notes.len() == 192 {
            //if current measure is full, add new measure
            self.notes.push(Measure::new());
            self.notes[current_meas_num].push(noteline);
        } else {
            self.notes[current_meas_num - 1].push(noteline);
        }
    }

    fn current_anchor_length(&self, noteline: &NoteLine) -> usize {
        let mut np = NotePointer::new(self.total_notelines(), self.snap);

        let mut a_len: usize = 1;
        np.move_pointer(2);
        while NoteLine::is_minijack(&noteline, &self.get_noteline(np.pos)) {
            np.move_pointer(2);
            a_len += 1;
            if np.pos == 0 {
                break;
            }
        }

        a_len
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

impl ChordType {
    fn gen(&self) -> NoteLine {
        match self {
            Self::Single => NoteLine::gen_single(),
            Self::Jump => NoteLine::gen_jump(),
            Self::Hand => NoteLine::gen_hand(),
            Self::Quad => NoteLine::gen_quad(),
        }
    }
}

impl Pattern {
    fn get_chordtypes(&self) -> Vec<ChordType> {
        match self {
            Self::Stream => vec![ChordType::Single],
            Self::Jumpstream => vec![ChordType::Jump, ChordType::Single],
            Self::LightJumpstream => vec![
                ChordType::Jump,
                ChordType::Single,
                ChordType::Single,
                ChordType::Single,
            ],
            Self::Handstream => vec![
                ChordType::Hand,
                ChordType::Single,
                ChordType::Jump,
                ChordType::Single,
            ],
            Self::Chordjacks => vec![
                ChordType::Hand,
                ChordType::Jump,
                ChordType::Jump,
                ChordType::Jump,
            ],
            Self::DenseChordjacks => vec![
                ChordType::Quad,
                ChordType::Hand,
                ChordType::Jump,
                ChordType::Hand,
            ],
        }
    }
}

impl NoteLine {
    const LENGTH: usize = 4;
    fn gen_quad() -> Self {
        Self {
            notes: vec![Note::Tap, Note::Tap, Note::Tap, Note::Tap],
        }
    }
    fn gen_hand() -> Self {
        let mut notes: Vec<Note> = Vec::new();
        let note = rand::thread_rng().gen_range(0..Self::LENGTH);
        for i in 0..Self::LENGTH {
            if i == note {
                notes.push(Note::None);
            } else {
                notes.push(Note::Tap);
            }
        }
        Self { notes }
    }
    fn gen_jump() -> Self {
        //pretty sure this could be done better, specifically by eliminating the loop but not sure how i want to right now
        let mut note1 = 0;
        let mut note2 = 0;

        while note1 == note2 {
            note1 = rand::thread_rng().gen_range(0..Self::LENGTH);
            note2 = rand::thread_rng().gen_range(0..Self::LENGTH);
        }
        let mut notes: Vec<Note> = Vec::new();
        for i in 0..Self::LENGTH {
            if i == note1 || i == note2 {
                notes.push(Note::Tap);
            } else {
                notes.push(Note::None);
            }
        }
        Self { notes }
    }
    fn gen_single() -> Self {
        let mut notes: Vec<Note> = Vec::new();
        let note_num = rand::thread_rng().gen_range(0..Self::LENGTH);
        for i in 0..Self::LENGTH {
            if i == note_num {
                notes.push(Note::Tap);
            } else {
                notes.push(Note::None);
            }
        }
        Self { notes }
    }
    fn gen_empty() -> Self {
        Self {
            notes: vec![Note::None, Note::None, Note::None, Note::None],
        }
    }
    fn is_minijack(line1: &NoteLine, line2: &NoteLine) -> bool {
        for i in 0..line1.notes.len() {
            if line1.notes[i] == Note::Tap && line2.notes[i] == Note::Tap {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");
        for meas in &self.notes {
            result.push_str(&format!("{}", meas));
        }
        write!(f, "{}", result)
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

#[cfg(test)]
mod tests {
    use crate::{gen::gen_pattern, File, Note, NoteLine, NotePointer, Pattern, Snap};
    #[test]
    fn js_gen() {
        //cargo test -- --nocapture
        //^^^ for printing output of tests
        gen_pattern(Pattern::Jumpstream, Snap::S16th, 1, None, false);
    }
    #[test]
    fn hs_gen() {
        //cargo test -- --nocapture
        //gen_pattern!(Pattern::Handstream, 12, 12, 12, 12);
    }
    #[test]
    fn note_pointer_test_16ths() {
        let mut np = NotePointer::new(192, Snap::S16th);
        np.move_pointer(2);
        assert_eq!(np.pos, 168);
    }
    #[test]
    fn note_pointer_test_40ths() {
        let mut np = NotePointer::new(192, Snap::S40th);
        np.move_pointer(3);
        assert_eq!(np.pos, 177);
    }
    #[test]
    fn anchor_count_test() {
        let mut notes = File::new(Snap::S16th);
        let tap1 = NoteLine {
            notes: vec![Note::Tap, Note::None, Note::None, Note::None],
        };
        let tap2 = NoteLine {
            notes: vec![Note::None, Note::Tap, Note::None, Note::None],
        };
        for i in 0..192 {
            if i % 24 == 0 {
                notes.push_noteline(tap1.clone());
            } else if i % 12 == 0 {
                notes.push_noteline(tap2.clone());
            } else {
                notes.push_noteline(NoteLine::gen_empty());
            }
        }
        let a_len = notes.current_anchor_length(&tap1);
        assert_eq!(a_len, 8);
    }
}
