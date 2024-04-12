# emmeffcee-api

__table of contents__

- [emmeffcee-api](#emmeffcee-api)
  - [overview](#overview)
  - [why?](#why)
  - [changelog and versions](#changelog-and-versions)
  - [contributing](#contributing)

## overview

This repository contains the API for the "**M**odular **F**an **C**ontroller" (_EmmEffCee_).

The modular fan controller is an ARGB and fan controller with these characteristics:
- independent from the computer's OS
- I²C-based
- multi-channel capable
- extendedable and customizable

It consists of a hardware- and a software-based component.

## why?

Because I can.

It's a challenge to myself how far I can take this project. It is born out of disgust for existing BIOS- and software-based solutions for controlling computer fans and ARGB lights. 

This implementation consists of a dedicated hardware component (Raspberry Pi + custom PCBs) and a software component (this code). Choosing this design will allow us to run completely independent of the host's operating system, specifically it can already be active during the boot process and after the computer has crashed. It will allow us to track and log temperature and fan speeds over time. We can do all this without stealing any compute cycles from the computer.

It is aimed at computer building enthusiasts and may be useful to competitive overclockers.

__pain points__

DC and PWM fans:

- BIOS typically does not allow to take measurements from multiple sources to control a specific fan. Using the BIOS you are limited to control a specific fan by CPU __or__ GPU.
- Mainboards typically have no or very limited capabilities for adding additional temperature sensors. Even if you can add sensors you may still be unable to use them to control fan speeds without using additional software.
- Software exists to replace the BIOS but this is then tied to the operating system. The best software out there [FanControl](https://github.com/Rem0o/FanControl.Releases) is still a pain if you daily-drive your computer as a non-privileged user. (as everybody should)

ARGB led strips:

- Vendor-provided solutions (BIOS and software) are atrocious. I'm specifically looking at you, Asus Armor Crate.
- The number of distinct ARGB channels is typically very low and can't be extended without using additional hardware.
- The number of LEDs per channel is typically limited. Connecting too many devices by daisy-chaining them or splitting a channel to multiple devices easily leads to color loss and/or flickering. This limit is arbitrary and does not need to exist. Every ATX power supply has a stabilized 5V power rail we could tap into.

## changelog and versions

Please see the [changelog](CHANGELOG.md).

## contributing

Please see the [contribution guide](docs/CONTRIBUTING.md).
