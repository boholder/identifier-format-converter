## User Guide

Sorry! Only the help information is available now. More content will be added later.

```text
Extract and convert the naming format(case) of identifiers from files.
Use this tool to prepare identifier name strings for further operations
(matching, replacing...) on relative files.
It is recommended to use this tool in combination with `xargs`.

Check the [homepage] for more information:
-> https://github.com/boholder/naming

Feel free to submit new [issues] when you found a bug or have suggestions:
-> https://github.com/boholder/naming/issues/new

Use -h for a brief help information, and --help for a more detailed version.

USAGE:
    naming.exe [FLAGS] [OPTIONS] [--] [files]...

FLAGS:
    -h, --help
            Prints help information

        --json
            Output result in json format. the output looks like:

                {"result":[
                    {"origin":"<origin matched string>",
                    "screaming_snake":"...",
                    "snake":"...",
                    "kebab":"...",
                    "camel":"...",
                    "pascal":"..."},
                    ...
                ]}

            A "result" array field under the root, one element for
            one match's result inside array. Json above is beautified,
            the real output doesn't contain spaces, looks like:
            "{"result":[{...},...]}".

            NOTE: The presence of the format fields in result
            depends on whether it is present in `--output` option.

        --regex
            Output result as an OR-joined (e.g. "x|y|z") regular expression.
            This is useful when you want to perform further operations involving
            "any given format of an identifier".

            This option will replace target output formats in result
            with a regex, which makes output looks like:

                <origin match 1> <OR-regex of match 1>
                <origin match 2> <OR-regex of match 2>
                ...

            Combine with `--json` and `--regex` options will
            makes output looks like (beautified):

                {"result":[
                    {"origin":"<origin matched string>",
                    "regex":"..."},
                    ...
                ]}

            NOTE: Same as `--json` option, the presence of the formats inside
            regex depends on whether it is present in `--output` option.

    -V, --version
            Prints version information


OPTIONS:
    -e, --eof <eof>
            Set logical EOF string, if occurs, the rest of the input is ignored.
            This will actually be done *after reading the whole content*, so
            it is recommended to use the following command to actually intercept
            large files (to prevent long processing time):

                $ sed '/<eof-word>/q' file_name | naming

            Command above means that output content from first line
            to the line contains given eof-word, to this tool.

    -f, --filter <filter>...
            Set which formats will be matched and extracted from input,
            discard other format matches. Default add all formats into
            filter, which means do not discard any match that can be
            matched by one of the formats.

            There are 6 available formats:
                * S -> SCREAMING_SNAKE_CASE
                * s -> snake_case
                * k -> kebab-case
                * c -> camelCase
                * P -> PascalCase
                * h -> hungarian notation

            The last format, hungarian notation means that let the tool
            recognize camel case matches as hungarian notation style,
            strip the first lowercase word (as this identifier's type prefix)
            and keep the remain part for further converting.
            example: "iPageSize" --strip"i"--> "PageSize" --> ...

            [!]NOTE: Due to the implementation of hungarian notation's matching,
            at most one of the two, hungarian notation (h) and camel case (c)
            can appear in `--filter` option. Pass both of them will let the tool
            exits with non-zero signal.

    -l, --locator <locator>...
            Set locator pairs around identifiers, in each pair value,
            the delimiter between prefix and suffix is a space.

            [!] Although this tool provides this option for convenience,
            be careful when using it. See warning information below for detail.

            The two parts of a pair are inserted directly into the
            regular pattern, so users need to manually escape the characters
            that need to be escaped. For [the regex syntax], check this document:

                https://docs.rs/regex/1.5.4/regex/index.html#syntax

            Default(1): "\b \b", which will match the valid identifiers
            separated by "Unicode word boundary (\w on one side and \W,
            \A, or \z on other)" positions.

            Each value passed to this option will be transformed to an regex
            pattern looks like: "(?:<prefix>|\A)(identifier)(?:<suffix>|\z)",
            where \A for matching the start of file position and
            \z for matching the end of file position.

            So there is no need to worry about having a match right next to
            the start or end of the file. If the input is only one valid word,
            that word will also be matched because of the hardcoded logic
            described above.

            [!]WARNING:
            Sorry for letting you know these implementation details.
            The rust regex library "regex" will matches text with a
            "non-overlapping" way, while it doesn't support lookaround syntax.
            So for the implementation, I used non-capture tuples to
            match prefixes and suffixes, which have mentioned above.

            An unintentional locator value, for example "\s \s",
            will generates a pattern "(?:\s|\A)([a-zA-Z0-9_-]+)(?:\s|\z)".
            This pattern will only matches "a" and "c" on text "a b c",
            while "b" shares two space symbols with "a" and "c".
            First matches "<start of file>a ", remain "b c<eof>",
            then matches " c<eof>", further extracting via group number
            gets "a" and "c" as final result.

            As you can see, locators that represent characters rather than
            positions may let to this unwanted result. Please use this option
            carefully and make sure the output of this tool is as expected,
            or use the grep method mentioned below as an alternative to
            extracting words from input text.

            [!]NOTE: Any incomplete pair value that couldn't be split
            into two part by space delimiter like "a"," a","a "
            will let the tool outputs nothing and exits with non-zero signal.

            NOTE:
            Due to the technical limitation, there is no guarantee that
            the order of matches in output (in normal output format, that
            means the order of lines) will be same as the order in
            origin file content.

            Each generated pattern will matches the input independently
            and all matches will be concatenated according to the order of
            values passed to this `--locator` option.

            If you want to keep this order, you can use tools like `grep`
            with a complex regex that contains lookaround syntax and OR operator
            to match all patterns at once, then extract the wanted part.
            For example:

                $ echo "@first@ #second#" | \
                 grep -oP "(?<=@)[a-z]+(?=@)|(?<=#)[a-z]+(?=#)" | naming

            Commands above piped "first\nsecond" to this tool.


    -o, --output <output>...
            Set which naming cases that matches will be converted to.

            There are 5 available formats:
                * S -> SCREAMING_SNAKE_CASE
                * s -> snake_case
                * k -> kebab-case
                * c -> camelCase
                * P -> PascalCase

            Default output all formats in a fix order --
            6 words separated by spaces, one line per match in output,
            origin match followed with naming cases of it:

                <origin match 1> <SCREAMING_SNAKE_CASE> <snake_case> \
                <kebab-case> <camelCase> <PascalCase>\n
                ...

            The order of target formats in the output is sames as
            the order of the corresponding values that passed to
            this option, so you can arrange the order of formats
            (in normal output format, that means the order of words
            in line) in output as you wish.
            This will be useful when you want to process the output
            of this tool, like, pass them to `xargs`.


ARGS:
    <files>...
            pass file names, or directly pass text via shell pipe


EXAMPLE:
    1. Default output all 5 format conventions, starting with origin input
        $ echo "pageSize" | naming
        pageSize PAGE_SIZE page_size page-size pageSize PageSize

    2. Search all positions of one identifier
        $ echo "pageSize" | naming | xargs -n1 -I {} -- grep -r {} src_dir

    3. Change one identifier from camelCase to snake_case
        $ echo "pageSize" | naming --output=s | \
            xargs -l -t -- bash -c 'sed -i "s/$0/$1/g" src_file'
        bash -c 'sed -i "s/$0/$1/g" src_file' pageSize page_size
        (^-- `xargs -t` output) (run sed command...)
```