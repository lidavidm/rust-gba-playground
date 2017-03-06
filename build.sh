#!/bin/sh

set -e

xargo build --target gba --release
arm-none-eabi-objcopy -O binary target/gba/release/gba target/rom.gba
gbafix target/rom.gba

mgba-qt target/rom.gba
