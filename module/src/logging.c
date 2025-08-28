#include <linux/module.h>
#include <linux/init.h>

#include <iotd/logging.h>

void __internal_log_shim(ShimLogLevel level, const char * msg){
    switch (level) {
        case InternalLogLevelTrace:
            TRACE("*%s", msg); break;
        case InternalLogLevelDebug:
            LOG_DEBUG("*%s", msg); break;
        case InternalLogLevelInfo:
            LOG_INFO("*%s", msg); break;
        case InternalLogLevelNotice:
            LOG_NOTICE("*%s", msg); break;
        case InternalLogLevelWarning:
            LOG_WARNING("*%s", msg); break;
        case InternalLogLevelError:
            LOG_ERROR("*%s", msg); break;
        case InternalLogLevelAlert:
            LOG_ALERT("*%s", msg); break;
        default:
            LOG_EMERG("*%s", msg); break;
    }
}