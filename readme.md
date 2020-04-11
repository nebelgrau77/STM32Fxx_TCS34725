# TCS34725 color sensor - DRAFT

Currently just prints raw readings to the console with semihosting.

https://docs.rs/tcs3472/0.1.1/tcs3472/ - crate description

https://ams.com/documents/20143/36005/TCS3472_DS000390_2-00.pdf - sensor datasheet

TO DO:

* conversion to useful RGB values
* connect either a color display (ST7735) or a WS2812 LED to show the color read by the sensor
* log the readings to console or SD card