#include<linux/module.h>
#include<linux/init.h>
#include <linux/delay.h>
#include<linux/spi/spi.h>
#include <iotd.h>
#include <iotd/interface.h>
#include <iotd/logging.h>
#include <linux/ktime.h>
#include <iotd/rs/provided.h>


static int prepare_spi_trx(struct spi_transfer transfer[2], const void * write, void * read, unsigned int write_size, unsigned int read_size, int concurrent);


/* CALLBACKS */
void __used set_gpio_rst(void * ref, unsigned int value){
    gpiod_set_value(((iotd_device_state_t *)(ref))->gpio_rst, value);
}

unsigned int __used get_irq_status(void * ref){
    return gpiod_get_value(((iotd_device_state_t *)(ref))->gpio_irq);
}


static int prepare_spi_trx(struct spi_transfer transfer[2], const void * write, void * read, unsigned int write_size, unsigned int read_size, int concurrent){
    // prepare the first transfer (tx only or tx+rx if concurrent)
    transfer[0].tx_buf = write;
    transfer[0].rx_buf = (concurrent || write == NULL) ? read : NULL;
    transfer[0].len = concurrent ? MIN(write_size, read_size) : (write == NULL ? read_size : write_size);
    transfer[0].cs_change = 0;
    transfer[0].tx_nbits = SPI_NBITS_SINGLE;
    transfer[0].rx_nbits = SPI_NBITS_SINGLE;
    transfer[0].bits_per_word = 8;
    transfer[0].delay.value = 200;
    transfer[0].delay.unit = SPI_DELAY_UNIT_NSECS;

    if(!concurrent && read_size > 0 && write != NULL){
        // prepare the second transfer transfer (tx only or tx+rx if concurrent)
        transfer[1].tx_buf = NULL;
        transfer[1].rx_buf = read;
        transfer[1].len = read_size;
        transfer[1].cs_change = 0;
        transfer[1].tx_nbits = SPI_NBITS_SINGLE;
        transfer[1].rx_nbits = SPI_NBITS_SINGLE;
        transfer[1].bits_per_word = 8;
        transfer[1].delay.value = 200;
        transfer[1].delay.unit = SPI_DELAY_UNIT_NSECS;
    }

    return (!concurrent && read_size > 0 && write != NULL) ? 2 : 1;
}

//todo improve with dma async trx. not really needed due to limited payload sizes
int __used spi_transaction(void * ref, const void * write, void * read, unsigned int write_size, unsigned int read_size, int concurrent){
    struct spi_transfer transfer[2];
    const int len = prepare_spi_trx(transfer, write, read, write_size, read_size, concurrent);
    return spi_sync_transfer(((iotd_device_state_t *) ref)->spi, transfer, len);
}


/* Interface */

void * load_driver(iotd_device_state_t * ref) {
    return r_driver_init((void *) ref, param_no_irq, param_aps_proto, &ref->config);
}

/* utilities */

void __used * __shim_kzalloc(unsigned int size, unsigned int flags){
    return kzalloc(size, flags);
}
void __used __shim_kfree(void * ptr){
    return kfree(ptr);
}

void __used __shim_usleep_range(unsigned int min, unsigned int max) {
    usleep_range(min, max);
}

int __used __shim_mutex_lock(void * ref) {
    struct mutex * mut = &((iotd_device_state_t *) ref)->context_mutex;
    return mutex_lock_interruptible(mut);
}

void __used __shim_mutex_unlock(void * ref) {
    struct mutex * mut = &((iotd_device_state_t *) ref)->context_mutex;
    if (mutex_is_locked(mut))
        mutex_unlock(mut);
}

unsigned long long int __used __shim_get_timestamp(void) {
    struct timespec64 ts;
    ktime_get_real_ts64(&ts);
    return ts.tv_sec;
}