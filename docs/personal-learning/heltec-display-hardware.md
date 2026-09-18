# Heltec V3.2 display hardware

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
- The `SCL`/`SDA` names are consistent with an I2C connection. The documents do not explicitly name the bus protocol in the extracted text, so this remains a connection-level conclusion to verify in the first hardware test.

## Not confirmed by these sources

- The OLED controller part number is not stated. The schematic labels the module only as `0.96 OLED` and the controller block as `U6`.
- The I2C address is not stated.

Before selecting a display-driver crate, check the controller and address with the board's official example/library or an I2C scan. Do not assume `SSD1306` or `0x3C` from the board name alone.
