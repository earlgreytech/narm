#!/bin/bash

mkdir -p bin/temp

for f in *.s
do
  echo "Assembling $f file..."
  # take action on each file. $f store current file name
  OBJECTFILE="bin/temp/$(basename "$f" .s).o"
  FINALFILE="bin/$(basename "$f" .s)"
  arm-none-eabi-as -march=armv6 -o $OBJECTFILE $f
  arm-none-eabi-ld -T link.ld -o $FINALFILE $OBJECTFILE
done

rm -rf bin/temp