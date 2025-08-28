#!/bin/sh

set -e;

DEVID=$1
FILE=$2

cat "$FILE" > "/dev/${DEVID}"