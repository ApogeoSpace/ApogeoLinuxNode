#ifndef __IOTD__LOGGING_H__
#define __IOTD__LOGGING_H__

#include <iotd.h>
#include <iotd/rs/provided.h>

#define _LOG(LEVEL, fmt, ...) printk(LEVEL "%s: " fmt, THIS_MODULE->name, ##__VA_ARGS__)
#define _TRACE(LEVEL, fmt, ...) printk(LEVEL "%s: [%s][%s:%d] " fmt, THIS_MODULE->name, __FUNCTION__, __FILE__, __LINE__,  ##__VA_ARGS__)

#define LOG_DEBUG(fmt, ...) _LOG(KERN_DEBUG, fmt, ##__VA_ARGS__)
#define LOG_INFO(fmt, ...) _LOG(KERN_INFO, fmt, ##__VA_ARGS__)
#define LOG_NOTICE(fmt, ...) _LOG(KERN_NOTICE, fmt, ##__VA_ARGS__)
#define LOG_WARNING(fmt, ...) _LOG(KERN_WARNING, fmt, ##__VA_ARGS__)
#define LOG_ERROR(fmt, ...) _LOG(KERN_ERR, fmt, ##__VA_ARGS__)
#define LOG_CRITICAL(fmt, ...) _LOG(KERN_CRIT, fmt, ##__VA_ARGS__)
#define LOG_ALERT(fmt, ...) _LOG(KERN_ALERT, fmt, ##__VA_ARGS__)
#define LOG_EMERG(fmt, ...) _TRACE(KERN_EMERG, fmt, ##__VA_ARGS__)
#define TRACE(fmt, ...) _TRACE(KERN_DEBUG, fmt, ##__VA_ARGS__)

void __internal_log_shim(ShimLogLevel level, const char * msg);

#endif