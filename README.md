# roast :fire:

Command line app to deep sort JSON files.

roast is an implementation of [codesen/jsonsort-cli](https://github.com/codsen/codsen/tree/main/packages/json-sort-cli) written in Rust using [serde](https://github.com/serde-rs/json).

## Installation

Download the latest build for your platform from [Releases](https://github.com/kressnick25/roast/releases). Simply drop the executable into a location in your PATH.

### MacOS

MacOS Gatekeeper will prevent you from running the binary from the _Releases_ page. This is because I don't have $99 a year to throw away on the Apple Developer Program for my cli tool with no users.

Either trust me and [ignore the warning](https://support.apple.com/en-au/guide/mac-help/mh40616/mac)
or clone the project and build it yourself (rust toolchain required):
```sh
cargo build
```

## Usage

```sh
$ roast file1.json folder1/folder2/**/*.* folder3 -s
$ roast -t -n -s *
$ roast -s yourspecialfolder

$ roast -v
$ roast --version
$ roast -h
$ roast --help
```

### Ignored files/directories

The following will be not be processed:
- `node_modules/`
- `package.json`
- `package_lock.json`
- `package-lock.json`
- `npm-debug.log`
- `npm-shrinkwrap.json`
- `config.gypi`
- `.lock-wscript`
- `.DS_Store`
- `.svn/`
- `CVS/`

## Flags

Use `roast --help` to list available flags.
| short | long | description |
|---|---|---|
| -a | --arrays | Also sort any arrays if they contain only string elements |
| -d | --dry | Only list all the files to be processed |
| -g | --git | Sort any JSON files tracked by git, that have a modified status. Will not modify any untracked, staged, or ignored files |
| -i | --indentationCount | How many spaces/tabs to use (default: 2 -> spaces, 1 -> tabs) |
| -l | --lineEnding | Set to "cr", "crlf" or "lf". Otherwise, the original line ending of the file is used |
|   | --silent | Suppress output |
| -s | --spaces | Use spaces for JSON file indentation (default uses tabs) |
| -v | --verbose | Enable verbose output for debugging |
| -h | --help | Print help |
| -V | --version | Print version |

## Roadmap

See [enhancement](https://github.com/kressnick25/roast/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement) Issues

## Acknowledgements

 - [Codesen Home](https://codsen.com/os/json-sort-cli)
 - [Codesen Github](https://github.com/codsen/codsen/tree/main/packages/json-sort-cli)


## License

[MIT](https://choosealicense.com/licenses/mit/)

