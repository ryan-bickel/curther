# curther

A cursor-controlled theremin implemented in Rust.

Volume is controlled by moving your cursor up or down, and pitch is controlelled by moving it left or right.

## Installation

Building from source requires Rust be installed (see [rustup.rs](https://rustup.rs/)).

```
git clone https://github.com/ryan-bickel/curther.git
cd churther
cargo install --path .
```

## Usage


| Option            | Description                            | Possible values                  | Default |
|-------------------|----------------------------------------|----------------------------------|---------|
| `-w, --waveform`  | Waveform function used by the theremin | square, sawtooth, sine, triangle | square  |
| `-f, --frequency` | Maximum frequency in hertz             | 20 - 20000                       | 1600    |
| `-a, --amplitude` | Maximum amplitude                      | 0 - 1                            | 0.2     |
| `-h, --help`      | Print help                             | N/A                              | N/A     |



### Examples

`curther -w sawtooth -f 4000 -a 0.1`:
sawtooth waveform with a maximum pitch of 4000 hz at 10% maximum volume

`curther`: defaults to a square waveform with a maximum ptich of 1600 hz at 20% maximum volume

