#!/bin/sh

# Low, pause
echo "0"   > /dev/gpio/SX1301_RESET/value
sleep 0.1
# High, pause
echo "1"   > /dev/gpio/SX1301_RESET/value
sleep 0.1
# Low, pause
echo "0"   > /dev/gpio/SX1301_RESET/value
