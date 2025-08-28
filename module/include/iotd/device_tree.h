#ifndef __IOTD__DEVICE_TREE_H__
#define __IOTD__DEVICE_TREE_H__

#include <linux/of.h>
#include <linux/of_device.h>

#include <iotd.h>

extern const struct of_device_id iotd_of_match[];

#define DEVICE_TREE_DEFAULT_CONF_SF_PROP            "sf"
#define DEVICE_TREE_DEFAULT_CONF_CR_PROP            "cr"
#define DEVICE_TREE_DEFAULT_CONF_LDRO_PROP          "ldro"

#define DEVICE_TREE_DEFAULT_CONF_BW_PROP            "bw"
#define DEVICE_TREE_DEFAULT_CONF_HEADER_PROP        "header"
#define DEVICE_TREE_DEFAULT_CONF_CRC_PROP           "crc"
#define DEVICE_TREE_DEFAULT_CONF_CAD_PROP           "cad"
#define DEVICE_TREE_DEFAULT_CONF_PREAMBLE_LEN_PROP  "preamble"
#define DEVICE_TREE_DEFAULT_CONF_PA_BOOST_PROP      "pa_boost"

#define DEVICE_TREE_DEFAULT_CONF_XTAL_PROP          "xtal"
#define DEVICE_TREE_DEFAULT_CONF_XTAL_TCXO_PROP     "tcxo"

#define DEVICE_TREE_DEFAULT_CONF_FREQ_PROP          "freq"
#define DEVICE_TREE_DEFAULT_CONF_PWR_PROP           "pwr"

#define DEVICE_TREE_DEFAULT_CONF_COUNTRY_PROP       "nation_id"

#define DEVICE_TREE_DEFAULT_CONF_NODE_ID_PROP       "node_id"


int get_dt_initial_config(const struct device_node * match, IotdConfig * ptr);
#endif