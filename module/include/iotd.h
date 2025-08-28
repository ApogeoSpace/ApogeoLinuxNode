#ifndef __IOTD_H__
#define __IOTD_H__

#include <linux/types.h>
#include <linux/mutex.h>
#include <linux/spi/spi.h>
#include <linux/cdev.h>
#include <linux/gpio/consumer.h>
#include <iotd/rs/provided.h>

#define MAX_DEVICES 5

struct iotd_device_state {
    struct spi_device * spi;
    struct gpio_desc * gpio_rst;
    struct gpio_desc * gpio_irq;
    struct cdev iodev;
    struct kobject sysfs_options;
    struct mutex context_mutex;
    // chardev
    int irq_num;
    void * driver;

    IotdConfig config;
};

typedef struct iotd_device_state iotd_device_state_t;

extern u32 param_no_irq;                        // behavioural config not to be modified runtime
extern u32 param_aps_proto;                     // behavioural config not to be modified runtime
extern u32 param_pa_boost;                     // behavioural config not to be modified runtime
extern dev_t iotd_major_number;
extern struct class * iotd_dev_class;

iotd_device_state_t * module_device_get(u32 index);
int module_device_index(iotd_device_state_t * dev);
int module_device_alloc(iotd_device_state_t ** ret);
int module_device_free(u32 index);

#endif