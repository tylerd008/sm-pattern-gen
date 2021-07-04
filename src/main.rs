use pattern_gen::{gen, Pattern, Snap};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sm-pattern-gen",
    about = "Program for generating .sm file format streams of a given snap."
)]
struct Opt {
    snap: Snap,
    pattern: Pattern,
}
fn main() {
    let opt = Opt::from_args();
    gen::gen_pattern(opt.pattern, opt.snap);
}
