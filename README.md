
## Clinews

A simple cli news reader app in Rust.

## Video walkthrough

1. [Building a CLI app in Rust](https://www.youtube.com/watch?v=4km2UijVC3M)

2. [Refactoring the CLI app in Rust](https://www.youtube.com/watch?v=LHPV3z9OSic)

3. [Library API design overhaul, async and more](https://youtu.be/J_yGWdgeGQM)

## How to install

#### Ubuntu

Install the cargo-deb subcommand:
`cargo install cargo-deb`

generate a deb package by running:

`cargo deb` in clinews repository

install the .deb package from target/debian using `dpkg -i <clinews.deb>`


#### Other linux distribution

`cargo install --path .` - You will need rust toolchain setup for this.

## Running clinews

Register on newsapi.org and get an API key.

Set the API key in your `.bashrc` or `.zshrc` like:

```bash
export API_KEY=xxx
```

then

Run `clinews` from terminal.

## Contributions

Feel free to file issues and PRs
