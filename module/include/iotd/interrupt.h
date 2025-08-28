#ifndef __IOTD__INTERRUPT_H__
#define __IOTD__INTERRUPT_H__

#include <linux/interrupt.h>
irqreturn_t iotd_irq_handler(int irq, void *dev_id);

#endif