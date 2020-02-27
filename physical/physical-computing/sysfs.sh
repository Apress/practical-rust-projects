echo "2" > /sys/class/gpio/export
echo "out" > /sys/class/gpio/gpio2/direction

echo "0" > /sys/class/gpio/gpio2/value
