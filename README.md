## naming

[![CI](https://github.com/boholder/naming/actions/workflows/ci.yml/badge.svg)](https://github.com/boholder/naming/actions/workflows/ci.yml)

naming is a command line tool that helps you extract and convert the naming format (case|notation)
of identifiers from files or stdin. Use this tool to prepare identifier name strings for further
operations (matching,replacing...) on relative files.

### Quick links

* [User guide](doc/USERGUIDE.md)
    * Help information with the explanation of design ideas.
    * Usage examples.
* [Installation](#installation)
* [Building & Running tests](#building)
* [Help us improve this tool](#contribution)

### What it can do?

You can convert identifiers to different naming cases, with multiple output formats.

```text
$ echo "userId" | naming
userId USER_ID user_id user-id userId UserId

$ echo "userId" | naming --json
{"result":[{"origin":"userId","screaming_snake":"USER_ID","snake":"user_id",
"kebab":"user-id","camel":"userId","pascal":"UserId"}]}

$ echo "userId" | naming --regex
userId USER_ID|user_id|user-id|userId|UserId
```

You can extract identifiers from files or stdin(via pipe, have shown above).

```text
$ cat a.txt
SCREAMING_SNAKE
snake_case
kebab-case
camelCase
PascalCase
$ naming a.txt
SCREAMING_SNAKE SCREAMING_SNAKE screaming_snake screaming-snake screamingSnake ScreamingSnake
snake_case SNAKE_CASE snake_case snake-case snakeCase SnakeCase
kebab-case KEBAB_CASE kebab_case kebab-case kebabCase KebabCase
camelCase CAMEL_CASE camel_case camel-case camelCase CamelCase
PascalCase PASCAL_CASE pascal_case pascal-case pascalCase PascalCase
```

The following example is more complex than entering commands manually, but it automates the process,
which is why we made this tool in the first place. By combining this tool with others, you can write
shell scripts about identifier format relative jobs.

```text
# Search all positions of an identifier in project directory
$ echo "pageSize" | naming | xargs -n1 -I {} -- grep -r {} src_dir

# Change one identifier from camelCase to snake_case
$ echo "pageSize" | naming --output=s | \
    xargs -l -t -- bash -c 'sed -i "s/$0/$1/g" src_file'
bash -c 'sed -i "s/$0/$1/g" src_file' pageSize page_size
(^-- `xargs -t` output) (run sed command...)
```

### Installation

We provide pre-compiled binaries for `x86_64-pc-windows-gnu`, `x86_64-unknown-linux-musl`
and `x86_64-apple-darwin` platforms, you can download them at
[release page](https://github.com/boholder/naming/releases). 

You can also build it (assuming you alreay have installed 
[Rust dev toolchain](https://www.rust-lang.org/tools/install)) directly from source code
("nightly" version?), following the [building](#building) progress.

Or just type:

```text
$ cargo install naming_clt
```

to install a stable version.

### Building

Just routine process will work:

```text
$ git clone https://github.com/boholder/naming
$ cd naming
$ cargo build --release
$ ./target/release/naming --help
$ (print help information)
```

After cloning the source to your local machine, you can run all tests via:

```text
$ cd naming
$ cargo test --all
```

There are
[two ignored tests](crates/naming_clt_lib/tests/identifying_ability_on_languages.rs)
, don't mind, these two tests made us to narrow down the capabilities of the tool, avoid duplicate
implementing grep's functionality.

### Contribution

* Does the [user guide](doc/USERGUIDE.md) and [help information](crates/core/app.rs) sound natural?
  Help us fix grammatical errors and polish the description.
* When you found a bug or have suggestions, feel free to submit
  new [issues](https://github.com/boholder/naming/issues/new).
* What command line tools is this tool suitable for use in combination with? Have you found any 
  useful ways to use this tool? Feel free to submit an issue or PR to share your findings.
