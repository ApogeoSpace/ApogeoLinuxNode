#include <linux/init.h>
#include <linux/module.h>
#include <linux/device.h>
#include <linux/spi/spi.h>
#include <linux/of.h>
#include <linux/of_device.h>
#include <linux/gpio/consumer.h>

#include <iotd/logging.h>
#include <iotd/device_tree.h>
#include <iotd/spi.h>
#include <iotd/interrupt.h>
#include <iotd/interface.h>
#include <iotd/chardev.h>
#include <iotd/sysfs_tunables.h>

static int iotd_spi_probe(struct spi_device *spi);
static void iotd_spi_remove(struct spi_device *spi);
static void driver_uninit(struct spi_device *spi, iotd_device_state_t * dev);

const struct spi_device_id iotd_id[] = {
    { "aps_iotd", 0 },
    { }
};

struct spi_driver iotd_spi_driver = {
    .driver = {
        .name = "iotd",
        .of_match_table = iotd_of_match,
    },
    .id_table = iotd_id,
    .probe = iotd_spi_probe,
    .remove = iotd_spi_remove
};

MODULE_DEVICE_TABLE(spi, iotd_id);

static int iotd_spi_probe(struct spi_device *spi) {
    LOG_DEBUG("iotd device probing...\n");

    const struct of_device_id * match = of_match_device(iotd_of_match, &spi->dev);
    if(!match) {
        LOG_ERROR("Could not find devicetree entry for iotd device.\n");
        goto cleanup;
    }

    iotd_device_state_t * module_state = NULL;
    const int dev_index = module_device_alloc(&module_state);
    LOG_DEBUG("Initializing device with index %d\n", dev_index);

    if(dev_index < 0 || module_state == NULL){
        LOG_ERROR("Device allocation failed during init\n");
        goto cleanup;
    }
    module_state->spi = spi;
        
    // initialize GPIOs
    module_state->gpio_rst = devm_gpiod_get(&spi->dev, "rst", GPIOD_OUT_HIGH);
    module_state->gpio_irq = devm_gpiod_get(&spi->dev, "irq", GPIOD_IN);
    if (IS_ERR(module_state->gpio_rst) || IS_ERR(module_state->gpio_irq)) {
        LOG_ERROR("Gpio initialization failed. Status is rst=%lld irq=%lld", (u64) module_state->gpio_rst, (u64) module_state->gpio_irq);
        goto cleanup;
    }    
    gpiod_set_value(module_state->gpio_rst, 0);

    // Initialize IRQ
    if (!param_no_irq) {
        mutex_init(&module_state->context_mutex);
        LOG_DEBUG("Requesting IRQ\n");
        u32 irqno = gpiod_to_irq(module_state->gpio_irq);
        if (irqno < 0) {
            LOG_ERROR("Cannot retreive gpio irq number\n");
            goto cleanup;
        }

        if (request_irq(irqno, (irq_handler_t) iotd_irq_handler, IRQF_TRIGGER_RISING , "iotd", (void *)(module_state)) < 0) {
            LOG_ERROR("Cannot attach irq\n");
            goto cleanup;
        }
        module_state->irq_num = irqno;
    } else {
        LOG_INFO("Module loaded without IRQ. Polling mode will be used.");
    }

    // Initialize defaults from dtb
    if (get_dt_initial_config(spi->dev.of_node, &module_state->config) != 0){
        goto cleanup;
    }

    if (chardev_init(module_state, dev_index) < 0){
        LOG_ERROR("CANNOT initialize chardev");
        goto cleanup;
    }

    if(register_config_sysfs(module_state, dev_index)){
        LOG_ERROR("Cannot set kernel fs data.");
        goto cleanup;
    }

    // Load driver
    module_state->driver = load_driver(module_state);
    if(IS_ERR(module_state->driver)) {
        LOG_ERROR("Cannot initialize driver core.\n");
        goto cleanup;
    }

    LOG_INFO("Initialized device as /dev/iotd%d", dev_index);
    TRACE("Config read:\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n\t%d\n", 
        module_state->config.spreading_factor,
        module_state->config.bandwidth,
        module_state->config.use_header,
        module_state->config.use_payload_crc,
        module_state->config.use_cad,
        module_state->config.xtal_freq,
        module_state->config.carrier_frequency,
        module_state->config.output_power,
        module_state->config.nation_id,
        module_state->config.node_id
    );
    return 0;

cleanup:
    driver_uninit(spi, module_state);
    return -EINVAL;
}

static void driver_uninit(struct spi_device *spi, iotd_device_state_t * module_state){
    LOG_DEBUG("Freeing device at %llX", (unsigned long long) module_state);
    if (module_state == NULL || IS_ERR(module_state))
        return;

    const int index = module_device_index(module_state);

    if(!IS_ERR(module_state->driver) && module_state->driver){
        r_device_drop(module_state->driver);
    }

    chardev_free(module_state, index);
    clear_config_sysfs(module_state);

    if (!param_no_irq && module_state->irq_num >= 0 ) {
        free_irq(module_state->irq_num, (void *)(module_state));
        module_state->irq_num = -1;
    }

    if(!IS_ERR(module_state->gpio_rst)){
        gpiod_set_value(module_state->gpio_rst, 0);         //place the device in reset
        devm_gpiod_put(&spi->dev, module_state->gpio_rst);
    }
    if(!IS_ERR(module_state->gpio_irq)){
        devm_gpiod_put(&spi->dev, module_state->gpio_irq);
    }
    module_device_free(index);
}

static void iotd_spi_remove(struct spi_device *spi)
{
    LOG_DEBUG("iotd remove\n");

    for(int i = 0; i < MAX_DEVICES; i++){
        iotd_device_state_t * module_state = module_device_get(i);
        if(module_state == NULL || IS_ERR(module_state))
            continue;

        if(module_state->spi == spi)
            driver_uninit(spi, module_state);
    }

}

