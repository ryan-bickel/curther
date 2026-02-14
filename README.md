# curther

A cursor-controlled theremin. Contrary to its pronunciation, curther is implemented in Rust, not Lisp.

Volume is controlled by moving your cursor up or down, and pitch is controlled by moving it left or right.

## Installation

Building from source requires Rust be installed (see [rustup.rs](https://rustup.rs/)).

```
git clone https://github.com/ryan-bickel/curther.git
cargo install --path curther
```

## Usage

Run `curther` with your desired arguments (see below). Move your mouse up or down to increase or decrease the volume.
Move your mouse right or left to increase or decrease the pitch. Press escape to exit.

Beware high volumes. It gets surprisingly loud.

If running on MacOS, you will be prompted on the first run to allow your terminal to capture the mouse position in the system's accessibility settings.

### Arguments

| Option               | Description                                                         | Possible values                  | Default |
|----------------------|---------------------------------------------------------------------|----------------------------------|---------|
| `-w, --waveform`     | Waveform function used by the theremin                              | square, sawtooth, sine, triangle | square  |
| `-f, --frequency`    | Maximum frequency in hertz                                          | 20 - 20000                       | 1600    |
| `-v, --volume`       | Maximum volume percentage                                           | 1 - 100                          | 20      |
| `-p, --polling-rate` | Mouse polling rate in hertz. Reduce if you have performance issues. | 1 - 1000                         | 1000    |
| `-h, --help`         | Print help                                                          | N/A                              | N/A     |

### Examples

`curther -w sawtooth -f 4000 -a 0.1`:
sawtooth waveform with a maximum pitch of 4000 hz at 10% maximum volume

`curther`: defaults to a square waveform with a maximum pitch of 1600 hz at 20% maximum volume

