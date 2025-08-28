#include <iotd.h>
#include <iotd/interrupt.h>
#include <linux/module.h>
#include <linux/init.h>

#include <iotd/logging.h>

irqreturn_t iotd_irq_handler(int irq, void *dev) {
    iotd_device_state_t * ref = (iotd_device_state_t *) dev;
    if (ref->driver == NULL) {
        return -ENOENT;
    }
    return r_irq_notify(ref->driver);
}