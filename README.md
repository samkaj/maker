# Maker

I don't find writing makefiles particularily enjoyable, and they usually end up looking exactly the same. Let's automate it with Rust :)

## Usage

Supply your project path to the CLI, and it creates a Makefile for you. The goal is that the defaults should work fine, but comments will exist to make it clear what you'll need to add (external libs, etc.).

## Structure 

- [ ] `Maker` - generates the Makefile
- [ ] `CLI` - access Maker from the command line, supply a path and get a Makefile
- [ ] `HTTP` - access Maker from the web, supply your directory structure in a request and get a Makefile as a response

## Features

- Generate Makefiles
- Update Makefiles (if you add new sources)

