#include <linux/init.h>
#include <linux/module.h>
#include <linux/spi/spi.h>
#include <linux/moduleparam.h>

#include <iotd.h>
#include <iotd/device_tree.h>
#include <iotd/spi.h>
#include <iotd/logging.h>

/* Globals */
static iotd_device_state_t * module_devices[MAX_DEVICES] = {0};
struct class * iotd_dev_class = NULL;
dev_t iotd_major_number;

iotd_device_state_t * module_device_get(u32 index){
    if (index >= MAX_DEVICES)
        return ERR_PTR(-EINVAL);
    if (module_devices[index] == NULL)
        return ERR_PTR(-EINVAL);

    return module_devices[index];
}

int module_device_index(iotd_device_state_t * dev){
    if (dev == NULL || IS_ERR(dev))
        return -EINVAL;

    for (int i = 0; i < MAX_DEVICES; i++) {
        if (module_devices[i] == dev)
            return i;
    }
    return -EINVAL;
}

int module_device_alloc(iotd_device_state_t ** ret){
    *ret = NULL;
    
    for(u32 i = 0; i < MAX_DEVICES; i++){
        if(module_devices[i] == NULL){
            module_devices[i] =  kzalloc(sizeof(iotd_device_state_t), GFP_KERNEL);
            if (module_devices[i] == NULL || IS_ERR(module_devices[i])) {
                LOG_ERROR("Failed to instantiate the device at index %d\n", i);
                return -EINVAL;
            }
            *ret = module_devices[i];
            return i;
        }
    }
    return -EINVAL;
}

int module_device_free(u32 index){
    if (module_devices[index] == NULL)
        return -EINVAL;

    memset(module_devices[index], 0, sizeof(iotd_device_state_t));
    kfree(module_devices[index]);
    LOG_INFO("Device %d destroyed\n", index);
    module_devices[index] = NULL;
    return 0;
}


/* Parameters & config*/
/** Behavioural */
u32 param_no_irq = 0;
module_param(param_no_irq, uint, 0);

u32 param_aps_proto = 1;
module_param(param_aps_proto, uint, 0);

/** Driver settings */

/* Descriptions */
MODULE_PARM_DESC(param_no_irq, "Determines how to handle the device interrupt requests. Set 0 to use interrupts, otherwise polling will be used.");
MODULE_PARM_DESC(param_aps_proto, "When true transmission and reception are handled with the APS communication protocol.");


MODULE_AUTHOR("Stefano Fontana");
MODULE_DESCRIPTION("Apogeo Space IoT Device Driver (SX127x)");
MODULE_LICENSE("GPL");

static int __init iotd_init(void) {
    LOG_INFO("Loading iotd driver.\n");
    
    if (alloc_chrdev_region(&iotd_major_number, 0, MAX_DEVICES, "iotd") < 0) {
        LOG_ERROR(KERN_ALERT "Failed to register device major number\n");
        return -EINVAL;
    }

    // Register the device class
    iotd_dev_class = class_create("iot_device");
    if (IS_ERR(iotd_dev_class)) {
        unregister_chrdev_region(iotd_major_number, MAX_DEVICES);
        LOG_ERROR("Failed to register device class\n");
        return PTR_ERR(iotd_dev_class);
    }

    return spi_register_driver(&iotd_spi_driver);
}

static void __exit iotd_exit(void) {
    LOG_INFO("Unloading iotd driver\n");
    spi_unregister_driver(&iotd_spi_driver);

    LOG_DEBUG("Destroying classes\n");
    class_destroy(iotd_dev_class);
    unregister_chrdev_region(iotd_major_number, MAX_DEVICES);
}


module_init(iotd_init);
module_exit(iotd_exit);
