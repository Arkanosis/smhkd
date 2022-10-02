use serde_derive::Deserialize;

const USAGE: &str = "
Usage: smhkd list
       smhkd -h | --help
       smhkd --version

Commands:
    list                     List available MIDI controllers.

Arguments:

Options:
    -h, --help               Show this screen.
    --version                Show version.
";

#[derive(Deserialize)]
struct Args {
    cmd_list: bool,
    flag_version: bool,
}

fn main() {
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .deserialize()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("smhkd v{}", smhkd::version());
    } else {
        if args.cmd_list {
            smhkd::list_controllers();
        }
    }
}
