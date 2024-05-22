# practical-runner

A practical application runner written in Rust. Inspired by [dmenu](https://tools.suckless.org/dmenu/)


### Screenshots
![Screenshot 1](screenshots/screenshot-1716407683.webp)
![Screenshot 2](screenshots/screenshot-1716407811.webp)
![Screenshot 3](screenshots/screenshot-1716408039.webp)


### Features
- theming (colors, font family, font size, line spacing, window border)
- custom row count
- smart row scrolling
- prompt message
- open menu on a specific display


### Installation
To use practical-runner run the following command:
```console
cargo install practicalrunner
```
Or clone this repository and run the following command in the repository's root directory:
```console
cargo install --path .
```


### Usage
```console
practicalrunner
```
For information about the various arguments run the following command:
```console
practicalrunner --help
```


### Controls
- [enter] to run the selected suggestion
- [up] / [down] arrow to scroll the suggestions
- [escape] / [ctrl-c] to quit
