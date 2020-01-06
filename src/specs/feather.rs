use crate::gpio;
use crate::i2c;
use crate::platform;

/// A platform that conforms to the [Adafruit Feather specification](https://learn.adafruit.com/adafruit-feather/feather-specification).
///
/// Any platform conforming to the specification must use 3.3V as the core voltage.
///
/// # Pins
///
///   * `D0-D8` are general purpose GPIO pins (digital input and output).  `D0` and `D1` double as
///     `SDA`/`SCL` so that is their primary name, but the trait will have associated type defaults
///     once that enters stable rust.
///   * `A0-A5` are capable of analog input in addition to GPIO.  The exception is the ESP8266
///     feather, which only has a working `A0`.  Hence analog traits are not required by the other
///     pins, while in practice most boards will actually implement them.
///   * `RX`/`TX` are bound to a hardware UART.
///   * `SDA`/`SCL` are bound to the main I²C bus.
///   * `SCK`/`MOSI`/`MISO` are bound to the main SPI bus.
///   * `P0` is mapped to something custom depending on the specific feather, usually a GPIO.
///
/// Additionally, it is guaranteed that there is one main LED bound to a pin, but which one it is
/// is left unspecified.
///
/// The pins are placed roughly according to this illustration:
///
/// ```text
///      ┌──────────────────┐
///  RST │ ○                │
///  3V3 │ ○                │
/// VREF │ ○                │
///  GND │ ○                │
///   A0 │ ○              ○ │ BAT
///   A1 │ ○              ○ │ EN
///   A2 │ ○              ○ │ USB
///   A3 │ ○              ○ │ D8
///   A4 │ ○              ○ │ D7
///   A5 │ ○              ○ │ D6
///  SCK │ ○              ○ │ D5
/// MOSI │ ○              ○ │ D4
/// MISO │ ○              ○ │ D3
///   RX │ ○              ○ │ D2
///   TX │ ○              ○ │ SCL / D1
///   P0 │ ○              ○ │ SDA / D0
///      └──────────────────┘
/// ```
pub trait Feather: platform::Platform {
    type MainLed: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type MainI2cMapping: i2c::I2cBusMapping<Self::SDA, Self::SCL>;

    type SDA: gpio::IntoOpenDrainOutputPin<Error = Self::Error>
        + gpio::IntoFloatingInputPin<Error = Self::Error>;
    type SCL: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D2: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D3: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D4: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D5: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D6: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D7: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type D8: gpio::IntoPushPullOutputPin<Error = Self::Error>;

    type P0;
    type TX: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type RX: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type MISO: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type MOSI: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type SCK: gpio::IntoPushPullOutputPin<Error = Self::Error>;
    type A5: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type A4: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type A3: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type A2: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type A1: gpio::IntoFloatingInputPin<Error = Self::Error>;
    type A0: gpio::IntoFloatingInputPin<Error = Self::Error>;

    fn take_main_led(&mut self) -> Self::MainLed;

    fn take_main_i2c(
        &mut self,
    ) -> <Self::MainI2cMapping as i2c::I2cBusMapping<Self::SDA, Self::SCL>>::Bus;

    fn take_sda(&mut self) -> Self::SDA;
    fn take_scl(&mut self) -> Self::SCL;
    fn take_d2(&mut self) -> Self::D2;
    fn take_d3(&mut self) -> Self::D3;
    fn take_d4(&mut self) -> Self::D4;
    fn take_d5(&mut self) -> Self::D5;
    fn take_d6(&mut self) -> Self::D6;
    fn take_d7(&mut self) -> Self::D7;
    fn take_d8(&mut self) -> Self::D8;
    fn take_p0(&mut self) -> Self::P0;
    fn take_tx(&mut self) -> Self::TX;
    fn take_rx(&mut self) -> Self::RX;
    fn take_miso(&mut self) -> Self::MISO;
    fn take_mosi(&mut self) -> Self::MOSI;
    fn take_sck(&mut self) -> Self::SCK;
    fn take_a5(&mut self) -> Self::A5;
    fn take_a4(&mut self) -> Self::A4;
    fn take_a3(&mut self) -> Self::A3;
    fn take_a2(&mut self) -> Self::A2;
    fn take_a1(&mut self) -> Self::A1;
    fn take_a0(&mut self) -> Self::A0;
}
