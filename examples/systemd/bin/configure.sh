#!/bin/sh

set -e;

DEVID=$1
ETC_DIR_ROOT=$2
ETC_DIR="${ETC_DIR_ROOT}/${DEVID}"
CONFIG_DIR="/sys/kernel/${DEVID}"

for config_file in $(find "${ETC_DIR}" -type f | sort); do
    if [ -f "${CONFIG_DIR}/$(basename $config_file)" ]; then
        cat $config_file > "${CONFIG_DIR}/$(basename $config_file)"
        echo "Configured $(basename $config_file)"
    else
        echo "Config does not exists: $(basename $config_file)" 1>&2
    fi
done
