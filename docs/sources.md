# Project sources

These sources support the current hardware and usage assumptions.

## Heltec board

- [Heltec WiFi LoRa 32 V3 resource directory](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/)
  - Official downloads for datasheets, schematics, pin maps, and examples.
- [Heltec WiFi LoRa 32 V3.2 datasheet](https://s.heltec.cn/download/WiFi_LoRa_32_V3.2/HTIT-WB32LA_V3.2.pdf)
  - Board specifications, ESP32-S3, OLED, USB-C, battery connection, and radio information.
- [Heltec WiFi LoRa 32 documentation](https://docs.heltec.org/en/node/esp32/wifi_lora_32/index.html)
  - Official product documentation.
- [Heltec ESP32 setup guide](https://heltec.org/wifi_kit_install/)
  - Official development setup information.

## Heltec hardware code

- [HeltecAutomation/Heltec_ESP32](https://github.com/HelTecAutomation/Heltec_ESP32)
  - Official Arduino examples and board definitions.
- [WiFi LoRa 32 V3 OLED examples](https://github.com/HelTecAutomation/Heltec_ESP32/tree/master/examples/ESP32)
  - Uses an SSD1306-compatible OLED driver and the V3 board pin definitions.
- [Official I2C scanner example](https://github.com/HelTecAutomation/Heltec_ESP32/blob/master/examples/ESP32/I2C_Scanner/I2C_Scanner.ino)
  - Documents checking the OLED I2C bus and address `0x3C`.

## ChatGPT subscription usage

- [OpenAI Codex repository](https://github.com/openai/codex)
  - First-party client source that reads ChatGPT subscription rate limits.
- [OpenAI Codex app-server documentation](https://developers.openai.com/codex/app-server/)
  - Documents the local `account/rateLimits/read` interface.
- [ChatGPT usage endpoint used by the local test](https://chatgpt.com/backend-api/wham/usage)
  - Internal endpoint. It is not a public OpenAI API and may change without notice.

The local proof is implemented in [`scripts/check_usage.py`](../scripts/check_usage.py). It reads the Codex OAuth credentials from `~/.codex/auth.json` and never stores them in this repository.
