# usage-display

An embedded Rust project for a Heltec WiFi LoRa 32 V3 board.

The first goal is understanding and design. We will not start by writing device code.

## Device

The connected board is a **Heltec WiFi LoRa 32 V3.2**. It is exposed as `/dev/ttyUSB0` through a Silicon Labs CP2102 USB-to-UART bridge.

The board variant is **HTIT-WB32LAF**. We will use the matching LoRa frequency settings from the official V3.2 documentation.

Main parts of the V3 board:

- ESP32-S3FN8 microcontroller
- Wi-Fi and Bluetooth Low Energy
- Semtech SX1262 LoRa radio
- Small OLED display
- USB-C connection
- Battery connector and charging circuit
- GPIO pins for sensors and other hardware

Official documentation:

- [Heltec official resource directory](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/)
- [WiFi LoRa 32 V3.2 datasheet](https://s.heltec.cn/download/WiFi_LoRa_32_V3/HTIT-WB32LA_V3.2.pdf)
- [WiFi LoRa 32 V3 documentation](https://docs.heltec.org/en/node/esp32/wifi_lora_32/index.html)
- [Heltec ESP32 setup information](https://heltec.org/wifi_kit_install/)

## Rust target

We will write the application in Rust. Rust must be compiled for the ESP32-S3 target and linked with the embedded runtime and board hardware support. The result is firmware, usually a binary image that can be flashed to the board over USB.

The exact Rust framework and target setup are a design decision. We will choose them after we understand the hardware, runtime model, and required features.

## Design process

Each work session follows this order:

1. Understand one part of the device or product goal.
2. Record the shared system model in C4 documentation.
3. Ask the agent for a program design based on that model.
4. Review the design together.
5. Sign off the design before implementation starts.
6. Implement one small, reviewable slice.

The agent may explain, map, and propose. The human signs off on architecture and program design.

See [Architecture process](docs/architecture-process.md).
