
## Clinews

A simple cli news reader app in Rust.

## Video walkthrough

1. [Building a CLI app in Rust](https://www.youtube.com/watch?v=4km2UijVC3M)

2. [Refactoring the CLI app in Rust](https://www.youtube.com/watch?v=LHPV3z9OSic)

## How to install

#### Ubuntu

Use cargo-deb subcommand (`cargo install cargo-deb`) to generate a package and then install system wide using dpkg

#### Linux distribution
Either cargo install --path .

or if you

## Running clinews

Register on newsapi.org and get an API key.

Set the API key in your `.bashrc` or `.zshrc` like:

```bash
export API_KEY=xxx
```

Run `clinews` from terminal.
