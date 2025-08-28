#ifndef __IOTD__RS__BINDINGS_H__
#define __IOTD__RS__BINDINGS_H__

void set_gpio_rst(void * ref, unsigned int value);
unsigned int get_irq_status(void * ref);
int spi_transaction(void * ref, const void * write, void * read, unsigned int write_size, unsigned int read_size, int concurrent);


void __shim_usleep_range(unsigned int min, unsigned int max);
void * __shim_kzalloc(unsigned int size, unsigned int flags);
void __shim_kfree(void * ptr);

int __shim_mutex_lock(void * ref);
void __shim_mutex_unlock(void * ref);
unsigned long long int __shim_get_timestamp(void);


#endif