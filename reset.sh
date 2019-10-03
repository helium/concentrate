#!/bin/sh

IOT_SK_SX1302_RESET_PIN=23
IOT_SK_SX1302_POWER_EN_PIN=18

echo "Accessing CoreCellSX1302 reset pin through GPIO$IOT_SK_SX1302_RESET_PIN..."
echo "Accessing CoreCellSX1302 power enable pin through GPIO$IOT_SK_SX1302_POWER_EN_PIN..."

WAIT_GPIO() {
    sleep 0.1
}

# set GPIOs as output
echo "out" > /sys/class/gpio/gpio$IOT_SK_SX1302_RESET_PIN/direction; WAIT_GPIO
echo "out" > /sys/class/gpio/gpio$IOT_SK_SX1302_POWER_EN_PIN/direction; WAIT_GPIO

# write output for SX1302 CoreCell power_enable and reset
echo "1" > /sys/class/gpio/gpio$IOT_SK_SX1302_POWER_EN_PIN/value; WAIT_GPIO

echo "1" > /sys/class/gpio/gpio$IOT_SK_SX1302_RESET_PIN/value; WAIT_GPIO
echo "0" > /sys/class/gpio/gpio$IOT_SK_SX1302_RESET_PIN/value; WAIT_GPIO
