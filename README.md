# `embedded-platform`

**NOTE**: This is currently a sketch work-in-progress set of libraries that should be treated as a proof-of-concept.

This is defines the `embedded-platform` set of crates.  The idea is to add device and peripheral support to complement
`embedded-hal`-based crates.  This makes it possible to plug-and-play and mix-and-match different crates that adhere to
common specs.  For example, if you have a `nrf52840`-based MCU as well as a `ili9341`-based device, and both adhere to
the Adafruit Feather spec (pin layout, voltage levels, ...), you can connect them up and all the wiring will be done for
you.

The ambition is that `embedded-platform` should be to `embedded-hal` what `tokio` is to `mio`.

Some design trade-offs that have been made:

  * `#![forbid(unsafe_code)]`; that belongs in `-pac` or `-hal` crates.
  * Do some compatibility checks at runtime during startup instead of at compile time, for example to check that a pin
    is used only once.  It turns out to be super tricky to do granular ownership mapping of device registers at compile
    time (this has been done in [`drone-os`](https://www.drone-os.com/)), and instead we opt to do some checks at
    runtime (e.g. `Option::take`).  This wastes a dozen or so instructions at startup, which is a one-time cost.
  * All APIs are async-first, so that code won't have to block and we can be power efficient.  This does require an
    executor, and one can be made that doesn't require `alloc`, yet to be written.
  * The crate uses its own HAL-like traits for e.g. `OutputPin` or `I2cRead` to enable async APIs as well as smooth
    over any incompatibilities between `embedded_hal::gpio::v1` and `embedded_hal::gpio::v2` etc.
  * All platform crates should be maintained in this repository so that changes like the last bullet point can be
    made in lock-step.
  * Don't expose interrupts to the user.  `mypin.changes()` should return an async `futures::Stream` when the pin
    changes.  In the background, we stash away a `Waker` that gets called from the interrupt handler.

You can think about the intended stack like this:

```text
┌─────────────────────────────────────────┐
│         Peripheral Access Crate         │
│            e.g. nrf52840-pac            │
├─────────────────────────────────────────┤
│        Hardware Abstraction Layer       │
│            e.g. nrf52840-hal            │
├─────────────────────────────────────────┤
│         Platform Implementation         │
│          e.g. platform-nrf52840         │
│ ┌─────────────────────────────────────┐ │
│ │          Specific Product           │ │
│ │         e.g. Particle Argon         │ │
│ ├─────────────────────────────────────┤ │
│ │            Common Spec              │ │
│ │        e.g. Adafruit Feather        │ │
│ │          or Arduino Shield          │ │
│ ├─────────────────────────────────────┤ │
│ │              Adapter                │ │
│ │        e.g. "Main SPI bus" on       │ │
│ │        specific Feather pins        │ │
│ └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│              Device Driver              │
│              e.g. ili9341               │
└─────────────────────────────────────────┘
```
