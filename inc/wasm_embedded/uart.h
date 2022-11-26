/// Embedded WASM abstract C UART platform interface

#ifndef WASME_UART_DRIVER_H
#define WASME_UART_DRIVER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// Init, takes device index and config and returns handle (or error)
typedef int32_t uart_init_f(const void *ctx, uint32_t dev, uint32_t baud, int32_t sda, int32_t scl);

/// Deinit, takes device handle and deinitialises
typedef int32_t uart_deinit_f(const void *ctx, int32_t handle);

/// Write, takes device handle, writes data_out to address
typedef int32_t uart_write_f(const void *ctx, int32_t handle, uint32_t flags, uint8_t *data_out, uint32_t length_out);

/// Read, takes device handle, reads data_in from address
typedef int32_t uart_read_f(const void *ctx, int32_t handle, uint32_t flags, uint8_t *data_in, uint32_t length_in);

/// C platform UART driver object
typedef struct {
    uart_init_f *init;
    uart_deinit_f *deinit;
    uart_write_f *write;
    uart_read_f *read;
} uart_drv_t;

#ifdef __cplusplus
}
#endif

#endif
