## Hardware Overview

#### I2C Addresses:

* Si7021: 0x40
* SGP30: 0x58
* BME280:
    * 0x76 (when the SDO pin is grounded)
    * 0x77 (when the SDO pin is connected to VCC)
* SSD1306: 0x3C or 0x3D, depending on the PCB design.

## Tools

The PCB diagram was designed using KiCad.

## Links

* [Raspberry Pi pinout](https://www.raspberrypi.com/documentation/computers/images/GPIO-Pinout-Diagram-2.png?hash=df7d7847c57a1ca6d5b2617695de6d46)
