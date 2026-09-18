# From idea to board

This page explains how an idea becomes running behavior on the Heltec board.

## The big picture

```text
Product idea
    ↓
Application design
    ↓
Rust program in main.rs
    ↓
Device and driver crates
    ↓
esp-hal
    ↓
ESP32-S3 machine code
    ↓
Firmware image
    ↓
USB flashing through /dev/ttyUSB0
    ↓
Program runs on the board
```

## 1. The application

The application describes what the device should do.

For example:

- read usage data
- decide what information matters
- show a value on the display
- wait and refresh the value
- handle missing data or a disconnected network

This behavior lives in our Rust code, mainly under `src/`.

## 2. Rust packages

`Cargo.toml` lists the packages used by the project.

Different packages solve different problems:

- `esp-hal` provides Rust access to the ESP32-S3 hardware.
- A display driver, such as an SSD1306 driver, knows the command language of the display controller.
- A network or sensor driver can provide access to another device or service.
- A board-support layer can collect Heltec-specific pin and hardware details.

The packages form layers. The application should not need to know every low-level register.

## 3. The hardware layers

The display usually works through several layers:

```text
Application: show "42%"
    ↓
Display driver: draw text and send display commands
    ↓
I2C driver from esp-hal: send bytes over I2C
    ↓
Heltec pin configuration: select SDA, SCL, and display address
    ↓
OLED controller and screen
```

`esp-hal` knows how the ESP32-S3 uses I2C. It does not automatically know every detail of the Heltec board.

The display driver knows the display controller protocol. It does not automatically know which Heltec pins are connected to it.

We connect these facts in the board configuration.

## 4. What we must learn about this board

Before choosing the final packages, we need to verify:

- the exact display controller
- whether the display uses I2C or another bus
- the SDA and SCL pins
- the display I2C address
- the ESP32-S3 pin and power restrictions
- the correct HTIT-WB32LAF V3.2 radio details

These facts come from the official schematic, datasheet, pin map, and careful hardware tests.

## 5. Building the firmware

The Rust compiler normally builds for the computer running it. For this project, Cargo must build for the ESP32-S3 target instead.

The build process is:

1. Cargo reads `Cargo.toml`.
2. Rust compiles our code and its dependencies.
3. The compiler targets the ESP32-S3 instruction set.
4. The linker places code and data into the ESP32-S3 memory layout.
5. The build creates a firmware image.

The firmware is not a Linux program. It is machine code for the microcontroller.

## 6. Putting the firmware on the board

The board is connected through a Silicon Labs CP2102 USB-to-UART bridge. Linux exposes this connection as `/dev/ttyUSB0`.

A flashing tool sends the firmware image through that serial connection. The ESP32-S3 stores it in flash memory and starts it after reset.

```text
Linux flashing tool
    → USB
    → CP2102 bridge
    → ESP32-S3 bootloader
    → flash memory
    → application starts
```

The USB device is only the transport path. After flashing, the program executes on the ESP32-S3.

## Main idea

We do not start by writing display code blindly.

We first build a hardware model:

- what the board contains
- how the parts are connected
- which protocol each part uses
- which Rust layer represents each fact

Then we choose packages and design the application around that model. The program design must be signed off before implementation begins.
