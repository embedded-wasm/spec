/// Embedded WASM abstract C GPIO platform interface

#ifndef WASME_GPIO_DRIVER_H
#define WASME_GPIO_DRIVER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// Init, takes device index and config and returns handle (or error)
typedef int32_t gpio_init_f(const void *ctx, int32_t port, int32_t pin, uint32_t mode);

/// Deinit, takes device handle and deinitialises
typedef int32_t gpio_deinit_f(const void *ctx, int32_t handle);

/// Write, takes device handle, writes data_out to address
typedef int32_t gpio_set_f(const void *ctx, int32_t handle, uint32_t value);

/// Read, takes device handle, reads data_in from address
typedef int32_t gpio_get_f(const void *ctx, int32_t handle, uint32_t* value);


/// C platform GPIO driver object
typedef struct {
    gpio_init_f *init;
    gpio_deinit_f *deinit;
    gpio_set_f *set;
    gpio_get_f *get;
} gpio_drv_t;

/// GPIO initialisation command
typedef struct gpio_init_s {
    int32_t port;
    int32_t pin;
    int32_t mode;
} gpio_init_t __attribute__((packed));

#endif
