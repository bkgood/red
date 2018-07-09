// a place to read or write to.
enum Target<'a> {
    File(&'a str),
    Command(&'a str),
}

impl<'a> Target<'a> {
    fn from_str(s: &'a str) -> Target<'a> {
        if s.starts_with("!") {
            Target::Command(&s[1..])
        } else if s.starts_with("\\!") {
            Target::File(&s[2..])
        } else {
            Target::File(s)
        }
    }
}

struct Args<'a> {
    prompt: Option<&'a str>,
    diagnostics: bool,
    file: Option<Target<'a>>,
}

impl<'a> Args<'a> {
    fn parse(arg_strs: &'a Vec<String>) -> Args<'a> {
        let mut args = Args {
            prompt: None,
            diagnostics: false,
            file: None,
        };

        let mut next_is_prompt = false;

        for arg in arg_strs {
            if next_is_prompt {
                args.prompt = Some(arg);
                next_is_prompt = false;
            } else if arg == "-" || arg == "-s" {
                args.diagnostics = true;
            } else if arg == "-p" {
                next_is_prompt = true;
            } else {
                args.file = Some(Target::from_str(&arg));
            }
        }

        args
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed = Args::parse(&args);
}
