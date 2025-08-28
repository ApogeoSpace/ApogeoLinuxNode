#include <iotd/logging.h>
#include <iotd/sysfs_tunables.h>

static ssize_t __used write_config_vaue(struct kobject *kobj, struct kobj_attribute *attr, const char *buf, size_t count){
    iotd_device_state_t * module_state = container_of(kobj, struct iotd_device_state, sysfs_options);

    //handle key special case if not tpm
    if (!memcmp(attr->attr.name, "node_key", 8)){
        if(count < 32){
            LOG_ERROR("Expected key length of 32, got %lu\n", count);
            return 0;
        }
        memcpy(module_state->config.node_key, buf, 32);
        r_config_notify(module_state->driver, &module_state->config);
        return count;
    }

    /* Everyhing else is an int */
    u32 r;
    if(count == 0) return 0;
    if(count > 2 && *buf == '0' && *(buf + 1) == 'x')
        sscanf(buf, "0x%x", &r);
    else
        sscanf(buf, "%d", &r);

#define SET_IF_ATTR(attr_name, attr_var, l, op)  \
                                    if (!memcmp(attr->attr.name, attr_name, l)){    \
                                        module_state->config.attr_var = (op);       \
                                    }

    SET_IF_ATTR("spreading_factor", spreading_factor, 16, r)
    else SET_IF_ATTR("bandwidth", bandwidth, 9, (u16) r)
    else SET_IF_ATTR("use_header", use_header, 10, r != 0)
    else SET_IF_ATTR("use_payload_crc", use_payload_crc, 15, r != 0)
    else SET_IF_ATTR("use_cad", use_cad, 7, r != 0)
    else SET_IF_ATTR("carrier_frequency", carrier_frequency, 17, r)
    else SET_IF_ATTR("nation_id", nation_id, 9, (u16) r)
    else SET_IF_ATTR("node_id", node_id, 7, r)
    else SET_IF_ATTR("output_power", output_power, 12, (u16) r)
    else SET_IF_ATTR("coding_rate", coding_rate, 11, (u8) r)
    else SET_IF_ATTR("use_ldro", use_ldro, 8, r != 0)
    else SET_IF_ATTR("preamble_length", preamble_length, 15, (u16) r)
    else {
        LOG_ERROR("Cannot set property %s. Unnkown.", attr->attr.name);
        return 0;
    }

#undef SET_IF_ATTR

    r_config_notify(module_state->driver, &module_state->config);
    return count;
}

static ssize_t __used read_config_vaue(struct kobject *kobj, struct kobj_attribute *attr, char *buf){ 
    iotd_device_state_t * module_state = container_of(kobj, struct iotd_device_state, sysfs_options);
    
#define RETURN_IF_ATTR(attr_name, attr_var, l, fmt, ...)  \
                                        if (!memcmp(attr->attr.name, attr_name, l)){    \
                                            return sysfs_emit(buf, fmt, (u32) module_state->config.attr_var __VA_ARGS__);   \
                                        }
                
    RETURN_IF_ATTR("spreading_factor", spreading_factor, 16, "%d\n")
    else RETURN_IF_ATTR("bandwidth", bandwidth, 9, "%d\n")
    else RETURN_IF_ATTR("use_header", use_header, 10, "%d\n")
    else RETURN_IF_ATTR("use_payload_crc", use_payload_crc, 15, "%d\n")
    else RETURN_IF_ATTR("use_cad", use_cad, 7, "%d\n")
    else RETURN_IF_ATTR("carrier_frequency", carrier_frequency, 17, "%d Hz\n")
    else RETURN_IF_ATTR("nation_id", nation_id, 9, "%d\n")
    else RETURN_IF_ATTR("node_id", node_id, 7, "%x\n")
    else RETURN_IF_ATTR("output_power", output_power, 12, "%d dBm\n")
    else RETURN_IF_ATTR("coding_rate", coding_rate, 11, "4/%d\n", +4)
    else RETURN_IF_ATTR("use_ldro", use_ldro, 8, "%d\n")
    else RETURN_IF_ATTR("preamble_length", preamble_length, 15, "%d\n")

#undef RETURN_IF_ATTR

    LOG_ERROR("Cannot get property %s. Unnkown.", attr->attr.name);
    return 0;
}

static struct kobj_attribute spreading_factor = __ATTR(spreading_factor, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute bandwidth = __ATTR(bandwidth, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute use_header = __ATTR(use_header, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute use_payload_crc = __ATTR(use_payload_crc, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute use_cad = __ATTR(use_cad, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute carrier_frequency = __ATTR(carrier_frequency, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute output_power = __ATTR(output_power, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute nation_id = __ATTR(nation_id, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute node_id = __ATTR(node_id, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute node_key = __ATTR(node_key, 0600, NULL, write_config_vaue);
static struct kobj_attribute coding_rate = __ATTR(coding_rate, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute use_ldro = __ATTR(use_ldro, 0664, read_config_vaue, write_config_vaue);
static struct kobj_attribute preamble_length = __ATTR(preamble_length, 0664, read_config_vaue, write_config_vaue);

static struct attribute * conf_attrs[] = {
    &spreading_factor.attr,
    &bandwidth.attr,
    &use_header.attr,
    &use_payload_crc.attr,
    &use_cad.attr,
    &carrier_frequency.attr,
    &output_power.attr,
    &nation_id.attr,
    &node_id.attr,
    &node_key.attr,
    &coding_rate.attr,
    &use_ldro.attr,
    &preamble_length.attr,
    NULL,
};

ATTRIBUTE_GROUPS(conf);
static const struct kobj_type conf_ktype = {
    .sysfs_ops = &kobj_sysfs_ops,
    .default_groups = conf_groups,
};
   

int register_config_sysfs(iotd_device_state_t * device, int index) {
    char devname[8];
    int l = snprintf(devname, 8, "iotd%d", index);
    if (l < 0 || l >= sizeof(devname)){
        return -EINVAL;
    }

    int retval = kobject_init_and_add(&device->sysfs_options, &conf_ktype, kernel_kobj, "iotd%d", index);
    if (retval){
        return -ENOMEM;
    }

    return 0;
}

void clear_config_sysfs(iotd_device_state_t * device) {
    kobject_put(&device->sysfs_options);
}