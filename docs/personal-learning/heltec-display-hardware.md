# Heltec V3.2 display hardware

Source index: [Project sources](../sources.md)

## Evidence checked

- [HTIT-WB32LA V3.2 datasheet](https://s.heltec.cn/download/WiFi_LoRa_32_V3/HTIT-WB32LA_V3.2.pdf)
- [Heltec WiFi LoRa 32 V3 resource directory](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/)
- [HTIT-WB32LA(F) V3 schematic](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/HTIT-WB32LA(F)_V3_Schematic_Diagram.pdf)

## Confirmed

- The datasheet describes an onboard **0.96-inch, 128 × 64 dot-matrix OLED**.
- The official schematic labels the display connections `OLED_SCL` and `OLED_SDA`.
- The schematic connects `OLED_SCL` to ESP32-S3 **GPIO18**.
- The schematic connects `OLED_SDA` to ESP32-S3 **GPIO17**.
- The display reset line is `OLED_RST`, connected to ESP32-S3 **GPIO21**.
- The `SCL`/`SDA` names are consistent with an I2C connection.

## Confirmed by official Heltec code

The official [HeltecAutomation/Heltec_ESP32 repository](https://github.com/HelTecAutomation/Heltec_ESP32) contains board examples for `WiFi_LoRa_32_V3` and OLED examples that:

- include `HT_SSD1306Wire.h`
- instantiate `SSD1306Wire`
- use address `0x3c`
- use `GEOMETRY_128_64`
- pass the board's `SDA_OLED`, `SCL_OLED`, and `RST_OLED` definitions

The official [I2C scanner example](https://github.com/HelTecAutomation/Heltec_ESP32/blob/master/examples/ESP32/I2C_Scanner/I2C_Scanner.ino) explicitly states that the OLED is on I2C0 and that the scan should find address `0x3C`.

This is enough to choose an SSD1306-compatible Rust display driver and address for the first implementation. The physical board should still be tested once with an I2C scan or a minimal display firmware before treating the wiring as proven.
