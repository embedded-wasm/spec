/// Embedded WASM abstract C SPI platform interface

#ifndef EWASM_SPI_H
#define EWASM_SPI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/// SPI Init, takes device index and config and returns handle (or error)
typedef int32_t spi_init_f(const void *ctx, uint32_t dev, uint32_t baud, int32_t mosi, int32_t miso, int32_t sck, int32_t cs);

/// SPI Deinit, takes device handle and deinitialises
typedef int32_t spi_deinit_f(const void *ctx, int32_t handle);

/// SPI Write, takes device handle, writes data_out
typedef int32_t spi_write_f(const void *ctx, int32_t handle, uint8_t *data_out, uint32_t length_out);

/// SPI Transfer, takes device handle, transfers (reads and writes) data
typedef int32_t spi_transfer_f(const void *ctx, int32_t handle, uint8_t *data, uint32_t length);

/// SPI Exec, takes device handle, writes data_out then reads data_in from address
// TODO: THIS IS NOT YET IMPLEMENTED / AVAILABLE
typedef int32_t spi_exec_f(const void *ctx, int32_t handle);

/// C platform SPI driver object
typedef struct {
    spi_init_f      *init;
    spi_deinit_f    *deinit;
    spi_write_f     *write;
    spi_transfer_f  *transfer;
    spi_exec_f      *exec;
} spi_drv_t;


#ifdef __cplusplus
}
#endif

#endif
