# minesweeper-rs  [![Build Status](https://travis-ci.org/Vinatorul/minesweeper-rs.svg)](https://travis-ci.org/Vinatorul/minesweeper-rs) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)
Simple minesweeper in Rust

## License
`minesweeper` is licensed under the MIT license. Please read the LICENSE file in this repository for more information.

## Compiling

Follow these instructions to compile `cargo-count`, then skip down to Installation.

 1. Clone the project `$ git clone https://github.com/Vinatorul/minesweeper-rs && cd minesweeper-rs`
 2. Build the project `$ cargo build --release` (**NOTE:** There is a large performance differnce when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
 3. Once complete, the binary will be located at `target/release/minesweeper`

## Options

Pass `-h` to arguments or look here: 

```
USAGE:
    minesweeper [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --height <height>    window height
        --width <width>      window width
```

# How to Contribute

Contributions are always welcome! Please use the following guidelines when contributing to `minesweeper`

1. Fork `minesweeper`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/minesweeper-rs && cd minesweeper-rs`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog format so I can update my changelog using [clog](https://github.com/thoughtram/clog)
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. Run the tests (`cargo test`)
6. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
7. Push your changes back to your fork (`git push origin $your-branch`)
8. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)
