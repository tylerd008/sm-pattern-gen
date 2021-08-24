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
}
fn main() {
    let opt = Opt::from_args();
    gen::gen_pattern(opt.pattern, opt.snap);
}
