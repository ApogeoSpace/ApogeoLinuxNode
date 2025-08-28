#ifndef __IOTD__SPI_H__
#define __IOTD__SPI_H__

#include <linux/init.h>
#include <linux/module.h>
#include <linux/device.h>
#include <linux/spi/spi.h>
#include <linux/of.h>
#include <linux/of_device.h>

extern const struct spi_device_id iotd_id[];
extern struct spi_driver iotd_spi_driver;

#endif