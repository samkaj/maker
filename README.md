# Maker

I don't find writing makefiles particularily enjoyable, and they usually end up looking exactly the same. Let's automate it with Rust :)

## Usage

Supply your project path to the CLI, and it creates a Makefile for you. The goal is that the defaults should work fine, but comments will exist to make it clear what you'll need to add (external libs, etc.).

#### Generate a makefile for a C project using defaults

```sh
cargo run -- src inc
make
./a.out
```

## Features

- [x] Generate Makefiles
    - Ignore directories
    - {ex|in}clude files
- [ ] Update Makefiles (if you add new sources)
    - It's possible to simply run `maker` again, however, custom rules added will not be preserved.

