#ifndef __IOTD__CHARDEV_H_
#define __IOTD__CHARDEV_H_

#include <linux/module.h>
#include <linux/init.h>
#include <linux/cdev.h>

int chardev_init(struct iotd_device_state * state, int index);
void chardev_free(struct iotd_device_state * state, int index);

#endif