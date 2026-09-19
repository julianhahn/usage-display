# Battery selection for the Usage Display

## Recommendation

Use a **single-cell 3.7 V LiPo battery** with:

- nominal voltage: 3.7 V
- full voltage: 4.2 V
- protected cell with over-charge, over-discharge, and short-circuit protection
- 2-pin SH1.25 connector, with the polarity checked before plugging in
- starting capacity: **1,000 mAh** if it fits the wooden case
- smaller fallback: **500 mAh**

Do not use a 2-cell pack, a 9 V battery, or a bare lithium cell without protection.

## Why this fits the board

Heltec's V3.2 datasheet specifies a single 3.7 V lithium battery and a battery input range of 3.3–4.2 V. The V3.2 schematic labels the connector `1.25X2P-LiPo` and shows pin 1 as `GND` and pin 2 as `VBAT`.

The board includes battery charging and automatic USB/battery power handling. The V3.2 schematic uses an LGS4056HDA charger with a 2 kΩ programming resistor. The charger datasheet maps that resistor to about 500 mA charge current.

The battery does not need to provide a regulated 3.3 V output. The board's power circuit handles the battery voltage for the ESP32-S3 rail.

## Capacity choice

The exact average current of this firmware has not been measured. Wi-Fi transmit bursts matter more than the OLED alone. A 1,000 mAh cell gives useful margin for testing and should fit the planned enclosure more easily than a large power-bank-style pack. A 500 mAh cell is the safer physical fallback if the case becomes too small.

Capacity is not enough to prove runtime. Measure the real board current while Wi-Fi connects, while the HTTPS request runs, and while the device is idle. Then estimate runtime from the measured average current and leave margin for the battery protection cutoff.

## Mechanical fit

The board is about 75 × 30 × 20 mm including headers from the supplied photos. The first case model allows about 80 × 35 × 25 mm internally, but that space is shared with the battery, wiring, switch, and second button. Select a flat pouch cell after measuring the real free volume. Do not press the cell against pin headers, the OLED, or sharp wood edges.

The battery connector is on the back side and was not visible in the supplied photos. Before ordering, measure its exact position and confirm the cable exits toward the planned case side.

## Open checks before buying

- Confirm the exact board revision printed on the PCB.
- Confirm connector polarity with the V3.2 schematic and a multimeter.
- Measure the open space behind the board and the planned battery thickness.
- Confirm that the selected cell includes a protection circuit.
- Confirm the seller's connector pitch and wire polarity; do not trust listing photos alone.
- Measure current and charging temperature during the first charge.

## Sources

- [Heltec HTIT-WB32LA V3.2 datasheet](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/HTIT-WB32LA_V3.2.pdf)
- [Heltec WiFi LoRa 32 V3.2 schematic](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/WiFi_LoRa_32_V3.2_Schematic_Diagram.pdf)
- [LGS4056HDA datasheet](https://datasheet.lcsc.com/lcsc/2211231800_Legend-Si-LGS4056HDA-4-35_C5280698.pdf)
- [Espressif ESP32-S3 hardware design guidelines](https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32s3/schematic-checklist.html)
