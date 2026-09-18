# ESP32 build and flash path

## Project-local commands

The local build reads Wi-Fi credentials from the gitignored file `secrets/build.env` and builds for the ESP32-S3:

```text
./scripts/build-local.sh
```

The reproducible flash command is:

```text
./scripts/flash-local.sh
```

It uses `/dev/ttyUSB0` by default. Override the port with `ESPFLASH_PORT`.

## Bootloader and flasher compatibility

The project uses `esp-bootloader-esp-idf::esp_app_desc!()` in `src/main.rs`. This puts the ESP-IDF application descriptor into the firmware image. The linker configuration keeps the descriptor reachable:

```text
-C link-arg=-Wl,--undefined=esp_app_desc
```

The project pins `espflash 4.4.0` for flashing. The stock `espflash 4.6.0` ESP32-S3 bootloader is generated from ESP-IDF 6.1 beta and rejected this board's efuse revision (`v1.3`). The project stores the compatible ESP32-S3 bootloader generated from the ESP-IDF 5.5.1 line at `tools/bootloaders/esp32s3-bootloader.bin`. The project-local flash script passes it explicitly with `--bootloader` and checks `tools/espflash-version.txt`.

Install the pinned flasher when needed:

```text
cargo install espflash --version 4.4.0 --locked
```

Earlier local `espflash 3.x` tooling also had installation/compatibility problems. The project-local flash script selects the pinned flasher and bootloader and flashes with `sg dialout`, so the current shell does not need a new login session after joining the `dialout` group.

The firmware uses `esp-bootloader-esp-idf 0.6.0` and the linker configuration resolved by Cargo. The checked-in bootloader is a flasher input, not a replacement for the Rust bootloader crate.

## Current proof

The Wi-Fi firmware has been built and flashed with `espflash 4.6.0`. The board was identified as an ESP32-S3 revision v0.2 with 8 MB flash.

Embassy Net now runs its network runner with DHCPv4. The board acquired `192.168.178.70/24` from the home router and reported gateway `192.168.178.1` over serial.
