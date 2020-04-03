#!/bin/bash

qemu-system-x86_64 -display sdl -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin
