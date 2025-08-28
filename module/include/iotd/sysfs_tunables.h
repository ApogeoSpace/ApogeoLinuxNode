#ifndef __IOTD__SYSFS_TUNABLES_H__
#define __IOTD__SYSFS_TUNABLES_H__

#include <iotd.h>

int register_config_sysfs(iotd_device_state_t * device, int index);
void clear_config_sysfs(iotd_device_state_t * device);

#endif