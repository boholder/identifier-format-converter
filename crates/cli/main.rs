use clap::{App, Arg};

fn main() {
    App::new("naming")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(
            ("\n".to_string() +
                &[DESCRIPTION, LINKS, HELP_HINT, EXAMPLES].join("\n\n")
            ).as_str())
        .args(&args())
        .get_matches();
}

const DESCRIPTION: &str = "\
Extract and convert the naming format(case) of identifiers from files.
Use this tool to prepare identifier name strings for further operations
(matching, replacing...) on relative files.
It is recommended to use this tool in combination with \"xargs\".";

const HELP_HINT: &str = "\
Use -h for a brief help information, and --help for a more detailed version.";

const EXAMPLES: &str = "\
EXAMPLE:
    # 1. default output all 5 format conventions, starts with origin input
    $ echo \"pageSize\" | naming
    pageSize PAGE_SIZE page_size page-size pageSize PageSize

    # 2. search all positions of one identifier
    $ echo \"pageSize\" | naming | xargs -I {} -- grep -r src_dir

    # 3. change one identifier from camelCase to snake_case
    $ echo \"pageSize\" | naming --output=s | \\
      xargs -l -t -- bash -c 'sed -i \"s/$0/$1/g\" mapper.java'
    bash -c 'sed -i \"s/$0/$1/g\" mapper.java' pageSize page_size";

const LINKS: &str = "\
Check the [homepage] for more information:
-> https://github.com/boholder/naming

Feel free to submit new [issues] when you found a bug or have suggestions:
-> https://github.com/boholder/naming/issues/new";

fn args<'a, 'b>() -> Box<[Arg<'a, 'b>]> {
    vec![
        Arg::with_name("filter")
            .short("f")
            .long("filter")
            .help("Set formats to be extracted.")
            // TODO 给每个参数加长介绍。
            .long_help("aaaa")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true)
            // screaming-snake, snake, kebab, camel, pascal, hungarian-notation
            // S, s, k, c, p, h
            // TODO Notice that camel case is conflict with hungarian-notation option
            .possible_values(&["S", "s", "k", "c", "p", "h"])
            .hide_possible_values(true),
        Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Set formats to be converted to.")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true)
            // can't output hungarian notation format
            // so there is no "h" value
            .possible_values(&["S", "s", "k", "c", "p"])
            .hide_possible_values(true),
        Arg::with_name("eof")
            .short("e")
            .long("eof")
            .help("Set logical EOF string; if occurs, the rest of the input is ignored.")
            .takes_value(true),
        Arg::with_name("before")
            .short("b")
            .long("before")
            .help("Set non-whitespace characters before identifiers; default are(2): '(' ','")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true),
        Arg::with_name("after")
            .short("a")
            .long("after")
            .help("Set non-whitespace characters after identifiers; default are(3): '=' ')' ','")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true),
        Arg::with_name("json")
            .long("json")
            .help("Output in json format"),
        Arg::with_name("regex")
            .long("regex")
            .help("Output as an OR-joined (\"x|y|z\") regular expression (for further matching)."),
        Arg::with_name("files")
            .multiple(true),
    ].into_boxed_slice()
}