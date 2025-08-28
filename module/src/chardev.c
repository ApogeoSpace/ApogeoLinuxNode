#include <iotd.h>
#include <iotd/logging.h>
#include <iotd/chardev.h>
#include <iotd/rs/provided.h>

static int dev_open(struct inode* inodep, struct file* filep);
static int dev_release(struct inode* inodep, struct file* filep);
static ssize_t dev_read(struct file* filep, char* buffer, size_t len, loff_t* offset);
static ssize_t dev_write(struct file* filep, const char* buffer, size_t len, loff_t* offset);

static struct file_operations fops = {
    .open = dev_open,
    .read = dev_read,
    .write = dev_write,
    .release = dev_release,
};


static int dev_open(struct inode * inode, struct file * filep) {
    iotd_device_state_t * module_state = container_of(inode->i_cdev, iotd_device_state_t, iodev);
    filep->private_data = (void *) (module_state->driver);
    LOG_DEBUG("Device has been opened in %d %d mode %llX\n", filep->f_mode & FMODE_READ, filep->f_mode & FMODE_WRITE, (unsigned long long) module_state);
    return r_dev_opened(filep->private_data, (filep->f_mode & FMODE_WRITE) != 0);
}

static int dev_release(struct inode * inode, struct file * filep) {
    LOG_DEBUG("Device has been closed. privdata = %llX\n", (unsigned long long) filep->private_data);
    return r_dev_close(filep->private_data);
}

static ssize_t dev_read(struct file * file, char * buffer, size_t len, loff_t * offset) {
    LOG_DEBUG("Device read. privdata = %llX, BUF = %llX, OFF=%lld\n", (unsigned long long) file->private_data, (unsigned long long) buffer, *offset);

    if (*offset > 0){
        return 0;
    }

    char buf[256+7] = {0xAA};

    int ret = r_receive(file->private_data, buf, len);
    LOG_DEBUG("Receive result %d\n", ret);
    if (ret <= 0) {
        return ret;
    }

    [[maybe_unused]] int _foo = copy_to_user(buffer, buf, ret);
    *offset = ret;
    return ret;
}

static ssize_t dev_write(struct file * file, const char * buffer, size_t len, loff_t * offset) {
    LOG_DEBUG("Device write. privdata = %llX, BUF = %llX\n", (unsigned long long) file->private_data, (unsigned long long) buffer);
    if (len > 255) return 0;

    char buf[256] = {0x0};
    char * b = buf;
    for(int _i = 0; _i < len; _i++)
        get_user(*(b++), buffer++);
    
    return r_transmit(file->private_data, &buf[0], len);
}


int chardev_init(struct iotd_device_state * state, int index){
    struct device * rr;
    char devname[8];
    int l = snprintf(devname, 8, "iotd%d", index);

    if (l < 0 || l >= sizeof(devname)){
        return -EINVAL;
    }

    if ((rr = device_create(iotd_dev_class, NULL, MKDEV(MAJOR(iotd_major_number), index), state, devname)), IS_ERR(rr)){
        return PTR_ERR(rr);
    }

    int r;
    cdev_init(&state->iodev, &fops);
    if ((r = cdev_add(&state->iodev, MKDEV(MAJOR(iotd_major_number), index), 1)) < 0) {
        device_destroy(iotd_dev_class,  MKDEV(MAJOR(iotd_major_number), index));
        return r;
    }

    return 0;
}

void chardev_free(iotd_device_state_t * state, int index){
    cdev_del(&state->iodev);
    device_destroy(iotd_dev_class,  MKDEV(MAJOR(iotd_major_number), index));
}