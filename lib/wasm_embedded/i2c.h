/// Embedded WASM abstract C I2C platform interface

#ifndef EWASM_I2C_H
#define EWASM_I2C_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// Init, takes device index and config and returns handle (or error)
typedef int32_t i2c_init_f(const void *ctx, uint32_t dev, uint32_t baud, int32_t sda, int32_t scl);

/// Deinit, takes device handle and deinitialises
typedef int32_t i2c_deinit_f(const void *ctx, int32_t handle);

/// Write, takes device handle, writes data_out to address
typedef int32_t i2c_write_f(const void *ctx, int32_t handle, uint16_t address, uint8_t *data_out, uint32_t length_out);

/// Read, takes device handle, reads data_in from address
typedef int32_t i2c_read_f(const void *ctx, int32_t handle, uint16_t address, uint8_t *data_in, uint32_t length_in);

/// WriteRead, takes device handle, writes data_out then reads data_in from address
typedef int32_t i2c_write_read_f(const void *ctx, int32_t handle, uint16_t address,
                                uint8_t *data_out, uint32_t length_out,
                                uint8_t *data_in, uint32_t length_in);

/// C platform I2C driver object
typedef struct {
    i2c_init_f *init;
    i2c_deinit_f *deinit;
    i2c_write_f *write;
    i2c_read_f *read;
    i2c_write_read_f *write_read;
} i2c_drv_t;

#ifdef __cplusplus
}
#endif

#endif
