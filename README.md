# curther

A cursor-controlled theremin. Contrary to its pronunciation, curther is implemented in Rust, not Lisp.

Volume is controlled by moving your cursor up or down, and pitch is controlled by moving it left or right.

## Installation

Building from source requires Rust be installed (see [rustup.rs](https://rustup.rs/)).

```
git clone https://github.com/ryan-bickel/curther.git
cargo install --path curther
```

### Updating

If you deleted the source code, repeat in the installation above. Otherwise:

```
cd path/to/curther
git pull
cargo install --path .
```

## Usage

Run `curther` with your desired arguments (see below). Move your mouse up or down to increase or decrease the volume.
Move your mouse right or left to increase or decrease the pitch. Press escape to exit.

Beware high volumes. It gets surprisingly loud.

If running on MacOS, you will be prompted on the first run to allow your terminal to capture the mouse position in the system's accessibility settings.

### Arguments

| Option               | Description                            | Possible values                  | Default        |
|----------------------|----------------------------------------|----------------------------------|----------------|
| `-w, --waveform`     | Waveform function used by the theremin | square, sawtooth, sine, triangle | square         |
| `-f, --frequency`    | Maximum frequency in hertz             | 20 - 20000                       | 1600           |
| `-v, --volume`       | Volume percentage                      | 1 - 100                          | 20             |
| `-i, --intervals`    | Space-separated list of intervals      | 1 - ∞                            | N/A (disabled) |
| `-p, --polling-rate` | Mouse polling rate in hertz            | 1 - 1000                         | 1000           |
| `-h, --help`         | Print help                             | N/A                              | N/A            |

### Examples

`curther`: defaults to a square waveform with a maximum frequency of 1600 hz at 20% volume

`curther -w sawtooth -f 4000 -v 10`:
sawtooth waveform with a maximum frequency of 4000 hz at 10% volume

`curther -w sine -i 1.5 1.531`: intervals at $\frac{1}{1.5}$ and $\frac{1}{1.531}$ times the base frequency (known as the "wolf" interval)
