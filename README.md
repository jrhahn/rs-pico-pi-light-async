# rs-pico-pi-light-async

Asynchronous light-switch firmware for the Raspberry Pi Pico (RP2040), built
with [Embassy](https://embassy.dev/).

It drives all 16 GPIO outputs via the RP2040's eight PWM slices, with one
async task per slice (two channels each). Every task runs the same PWM loop
but starts at a different time offset, producing a staggered "running light"
sweep across the pins. The code also includes (commented-out) variations for
traffic-light alternation, independent per-channel duty cycles, and a
random-flicker fire/glow effect.

A custom carrier PCB for the Pico W lives in [`pcb/`](pcb/) (Altium schematic,
Gerbers, and bill of materials).

## Hardware

- Raspberry Pi Pico / Pico W (RP2040)
- PWM frequency: 25 kHz (configurable in `get_pwm_config`)
- All 16 GPIOs (PIN_0–PIN_15) used as PWM outputs

## Building

Requires the Rust embedded toolchain and the `thumbv6m-none-eabi` target
(see [`rust-toolchain.toml`](rust-toolchain.toml)).

```sh
cargo build --release
```

## Flashing & running

The runner is configured for [`probe-rs`](https://probe.rs/) with an RP2040
target (see [`.cargo/config.toml`](.cargo/config.toml)). With a debug probe
attached:

```sh
cargo run --release
```

Log output is provided over RTT via `defmt`.

## License

MIT — see [LICENSE](LICENSE).
