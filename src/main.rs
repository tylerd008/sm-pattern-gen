use sm_pattern_gen::{gen, Pattern, Snap};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sm-pattern-gen",
    about = "Program for generating .sm file format patterns of a given snap."
)]
struct Opt {
    #[structopt(
        help = "Specifies the snap for the given pattern. Acceptable snaps are: 4th, 8th, 12th, 16th, 20th, 22nd, 26th, 28th, 32nd, 36th, 40th, 44th, 48th, 64th, 70th, 80th, 96th, and 192nd."
    )]
    snap: Snap,
    #[structopt(
        help = "Specifies the pattern. Acceptable patterns are: stream, light jumpstream, jumpstream, handstream, and chordjacks."
    )]
    pattern: Pattern,
    #[structopt(
        default_value = "1",
        help = "Specify the number of measures of notes to generate."
    )]
    num_measures: usize,
    #[structopt(
        short,
        help = "Specify the maximum length for anchors in the generated pattern. If specified, value must be at least 1 in general, 2 for jumpstream, and 3 for handstream. Does not affect chordjack output. Do note that low anchor values can be very restricting for pattern generation, and the program may hang during generation as a result. "
    )]
    anchor_length: Option<usize>,
    #[structopt(
        short,
        help = "Removes the minijack check for generation, resulting in patterns similar to super shuffle."
    )]
    super_shuffle: bool,
}
fn main() {
    let opt = Opt::from_args();
    if let Some(a_len) = opt.anchor_length {
        match opt.pattern {
            Pattern::Stream => assert!(a_len >= 1, "Anchor length must be at least 1 for streams."),
            Pattern::Jumpstream => assert!(
                a_len >= 2,
                "Anchor length must be at least 2 for jumpstream."
            ),
            Pattern::LightJumpstream => {
                assert!(
                    a_len >= 1,
                    "Anchor length must be at least 1 for light jumpstream."
                )
            }
            Pattern::Handstream => {
                assert!(
                    a_len >= 3,
                    "Anchor length must be at least 3 for handstream."
                )
            }
            Pattern::Chordjacks => {}
            Pattern::DenseChordjacks => {}
        }
    }
    gen::gen_pattern(
        opt.pattern,
        opt.snap,
        opt.num_measures,
        opt.anchor_length,
        opt.super_shuffle,
    );
}
