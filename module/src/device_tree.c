#include <iotd/device_tree.h>

const struct of_device_id iotd_of_match[] = {
    { .compatible = "aps,aps_iotd" },
    { .compatible = "aps_iotd" },
    { /*sentinel*/ }
};

MODULE_DEVICE_TABLE(of, iotd_of_match);

static IotdConfig __module_config_default = {
    .spreading_factor = 7,
    .bandwidth = 5,
    .coding_rate = 2,
    .use_header = TRUE,
    .use_payload_crc = TRUE,
    .use_cad = TRUE,
    .use_ldro = FALSE,
    .preamble_length = 8,
    .xtal_freq = 32000000,
    .xtal_is_tcxo = FALSE,
    .carrier_frequency = 169000000,
    .output_power = 100,
    .nation_id = 1,
    .pa_boost = 0
};


int get_dt_initial_config(const struct device_node * match, IotdConfig * ptr){
    memcpy(ptr, &__module_config_default, sizeof(IotdConfig));

#define READ_DT_VAL(PROP_NAME, match, field, op) \
        do {                                                                        \
            u32 rval, var;                                                          \
            if ((rval = of_property_read_u32_index(match, PROP_NAME, 0, &var))) {   \
                if (rval != -EINVAL) { return -EINVAL; }                            \
            }                                                                       \
            else ptr->field = (op);                                                 \
        } while(0)

    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_SF_PROP, match, spreading_factor, (u8) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_CR_PROP, match, coding_rate, (u8) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_BW_PROP, match, bandwidth, (u16) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_HEADER_PROP, match, use_header, var != 0);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_CRC_PROP, match, use_payload_crc, var != 0);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_CAD_PROP, match, use_cad, var != 0);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_LDRO_PROP, match, use_ldro, var != 0);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_PREAMBLE_LEN_PROP, match, preamble_length, (u16) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_XTAL_PROP, match, xtal_freq, var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_XTAL_TCXO_PROP, match, xtal_is_tcxo, var != 0);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_FREQ_PROP, match, carrier_frequency, var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_PWR_PROP, match, output_power, (u16) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_COUNTRY_PROP, match, nation_id, (u16) var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_NODE_ID_PROP, match, node_id, var);
    READ_DT_VAL(DEVICE_TREE_DEFAULT_CONF_PA_BOOST_PROP, match, node_id, (u8) var);

#undef READ_DT_VAL

    return 0;
}
