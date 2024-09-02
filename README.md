[![Build](https://github.com/ztroop/btlescan/actions/workflows/build.yml/badge.svg)](https://github.com/ztroop/btlescan/actions/workflows/build.yml)

# btlescan

## Summary

This tool provides a cross-platform CLI with an interactive way to view Bluetooth Low Energy (BTLE) devices, showcasing their Address/UUID, Name, TX Power, and RSSI (Received Signal Strength Indicator) in a table format. It also provides functionality to inspect GATT information.

![demo](./assets/demo.png)

## Features

- Real-Time Discovery: Continuously scans for Bluetooth devices, updating the list in real-time as new devices appear or existing devices become unavailable.
- Device Information: Displays detailed information about each detected Bluetooth device, including:
    - **Address/UUID**: The unique address or UUID of the Bluetooth device.
    - **Name**: The name of the Bluetooth device, if available.
    - **TX Power**: The transmission power level, indicating the strength at which the device is broadcasting its signal.
    - **RSSI**: Received Signal Strength Indicator, a measure of the power present in the received signal, indicating how close or far the device is.
- Interactive UI: The terminal-based user interface allows users to scroll through the list of discovered devices, providing an easy way to browse and select devices of interest.
- Keyboard Navigation: Supports simple keyboard controls for navigation:
    - **Up/Down Arrows**: Scroll through the list of devices.
    - **Q**: Quit the application.
    - **S**: Toggle scanning.
    - **E**: Export CSV data to current directory.
    - **ENTER**: Open or close widget.

## Installation

```sh
git clone git@github.com:ztroop/btlescan.git && cd ./btlescan
cargo install --path .
```

### Arch Linux (AUR)

You can install `btlescan` from the [AUR](https://aur.archlinux.org/packages/btlescan) with using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers).

```shell
paru -S btlescan
```

## Alternatives

If you're looking to manage or pair Bluetooth devices, check out [bluetui](https://github.com/pythops/bluetui)!