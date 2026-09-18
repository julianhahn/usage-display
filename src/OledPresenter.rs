use core::fmt::Write;

use display_interface::DisplayError;
use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_6X10, FONT_10X20},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use esp_hal::{Blocking, gpio::Output, i2c::master::I2c};
use heapless::String;
use ssd1306::{I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode, prelude::*};

type Display = Ssd1306<
    I2CInterface<I2c<'static, Blocking>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

/// Heltec V3.2 OLED. Keep the power and reset pins owned while it is in use.
pub struct OledPresenter {
    display: Display,
    _power: Output<'static>,
    _reset: Output<'static>,
}

impl OledPresenter {
    pub async fn new(
        i2c: I2c<'static, Blocking>,
        mut power: Output<'static>,
        mut reset: Output<'static>,
    ) -> Result<Self, DisplayError> {
        power.set_low(); // Vext is active-low on the V3.2 board.
        Timer::after_millis(50).await;
        reset.set_low();
        Timer::after_millis(10).await;
        reset.set_high();
        Timer::after_millis(20).await;
        let mut display = Ssd1306::new(
            I2CDisplayInterface::new(i2c), // address 0x3c
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        display.init()?;
        let mut presenter = Self {
            display,
            _power: power,
            _reset: reset,
        };
        presenter.show_status("Connecting...")?;
        Ok(presenter)
    }

    pub fn show_status(&mut self, status: &str) -> Result<(), DisplayError> {
        self.display.clear_buffer();
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        // Buffered drawing is infallible; only flushing can fail.
        let _ = Text::with_alignment(
            "ChatGPT weekly",
            Point::new(64, 12),
            style,
            Alignment::Center,
        )
        .draw(&mut self.display);
        let _ = Text::with_alignment(status, Point::new(64, 38), style, Alignment::Center)
            .draw(&mut self.display);
        self.display.flush()
    }

    pub fn show_remaining(&mut self, remaining: u8) -> Result<(), DisplayError> {
        self.display.clear_buffer();
        let mut value = String::<4>::new();
        // Every u8 followed by '%' fits the four-byte buffer.
        let _ = write!(value, "{}%", remaining);
        let small = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let large = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let _ = Text::with_alignment(
            "ChatGPT weekly",
            Point::new(64, 11),
            small,
            Alignment::Center,
        )
        .draw(&mut self.display);
        let _ = Text::with_alignment(value.as_str(), Point::new(64, 38), large, Alignment::Center)
            .draw(&mut self.display);
        let _ = Text::with_alignment("remaining", Point::new(64, 56), small, Alignment::Center)
            .draw(&mut self.display);
        self.display.flush()
    }
}
