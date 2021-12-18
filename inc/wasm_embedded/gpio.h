/// Embedded WASM abstract C GPIO platform interface

#ifndef EWASM_GPIO_H
#define EWASM_GPIO_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// Init, takes device index and config and returns handle (or error)
typedef int32_t gpio_init_f(const void *ctx, uint32_t port, uint32_t pin, uint32_t mode);

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

#endif
