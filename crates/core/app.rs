use clap::{App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new("naming")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(ABOUT)
        .args(&args())
}

const ABOUT: &str = "\n\
Extract and convert the naming format(case) of identifiers from files.
Use this tool to prepare identifier name strings for further operations
(matching, replacing...) on relative files.
It is recommended to use this tool in combination with \"xargs\".

Check the [homepage] for more information:
-> https://github.com/boholder/naming

Feel free to submit new [issues] when you found a bug or have suggestions:
-> https://github.com/boholder/naming/issues/new

Use -h for a brief help information, and --help for a more detailed version.

EXAMPLE:
    # 1. default output all 5 format conventions, starts with origin input
    $ echo \"pageSize\" | naming
    pageSize PAGE_SIZE page_size page-size pageSize PageSize

    # 2. search all positions of one identifier
    $ echo \"pageSize\" | naming | xargs -n1 -I {} -- grep -r {} src_dir

    # 3. change one identifier from camelCase to snake_case
    $ echo \"pageSize\" | naming --output=s | \\
      xargs -l -t -- bash -c 'sed -i \"s/$0/$1/g\" IbatisMapper.xml'
    bash -c 'sed -i \"s/$0/$1/g\" mapper.java' pageSize page_size";

fn args<'a, 'b>() -> Box<[Arg<'a, 'b>]> {
    vec![
        Arg::with_name("filter")
            .short("f")
            .long("filter")
            .help("Set formats to be extracted.")
            .takes_value(true)
            .multiple(true)
            .use_delimiter(true)
            // screaming-snake, snake, kebab, camel, pascal, hungarian-notation
            // S, s, k, c, p, h
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
        Arg::with_name("locator")
            .short("l")
            .long("locator")
            .help("Set locator pairs around identifiers, \
                the separator between prefix and suffix is a space; \
                default(1): \"\\s \\s\"")
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