## Hardware Overview

The hardware is assembled on a soldered breadboard, but future versions will feature a dedicated PCB.

This project is straightforward and includes the following I2C components:

* **BME280:** Temperature, Humidity, and Pressure sensor.
* **Si7021:** Temperature and Humidity sensor.
* **SSD1306:** 0.91-inch 128x32 OLED display.

![electronic circuit of the device](../Documentation/Circuit/07_09_2024.pngg)

#### I2C Addresses:

* Si7021: 0x40
* BME280:
    * 0x76 (when the SDO pin is grounded)
    * 0x77 (when the SDO pin is connected to VCC)
* SSD1306: 0x3C or 0x3D, depending on the PCB design.

## Tools

The PCB diagram was designed using KiCad.

## Links

* [Raspberry Pi pinout](https://www.raspberrypi.com/documentation/computers/images/GPIO-Pinout-Diagram-2.png?hash=df7d7847c57a1ca6d5b2617695de6d46)
* [Guide: Using the BME280 on Linux](http://wiki.sunfounder.cc/index.php?title=BMP280_Pressure_Sensor_Module)
