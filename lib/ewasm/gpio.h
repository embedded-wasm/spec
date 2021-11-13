#ifndef WASME_GPIO_H
#define WASME_GPIO_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// Init, takes device index and config and returns handle (or error)
typedef int32_t gpio_init_f(void *ctx, uint32_t port, uint32_t pin, uint32_t mode);

/// Deinit, takes device handle and deinitialises
typedef int32_t gpio_deinit_f(void *ctx, int32_t handle);

/// Write, takes device handle, writes data_out to address
typedef int32_t gpio_set_f(void *ctx, int32_t handle, uint32_t value);

/// Read, takes device handle, reads data_in from address
typedef int32_t gpio_get_f(void *ctx, int32_t handle, uint32_t* value);


/// GPIO driver object
typedef struct {
    const gpio_init_f *init;
    const gpio_deinit_f *deinit;
    const gpio_set_f *set;
    const gpio_get_f *get;
} gpio_drv_t;

#endif
