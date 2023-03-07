// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

//! This file contains enums and consts for use within the Rust codebase.
//!
//! These definitions are for information that depends on the top-specific chip
//! configuration, which includes:
//! - Device Memory Information (for Peripherals and Memory)
//! - PLIC Interrupt ID Names and Source Mappings
//! - Alert ID Names and Source Mappings
//! - Pinmux Pin/Select Names
//! - Power Manager Wakeups

/// Peripheral base address for uart0 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_UART0_BASE_ADDR: usize = 0x40000000;

/// Peripheral size for uart0 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_UART0_BASE_ADDR and
/// `TOP_EARLGREY_UART0_BASE_ADDR + TOP_EARLGREY_UART0_SIZE_BYTES`.
pub const TOP_EARLGREY_UART0_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for uart1 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_UART1_BASE_ADDR: usize = 0x40010000;

/// Peripheral size for uart1 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_UART1_BASE_ADDR and
/// `TOP_EARLGREY_UART1_BASE_ADDR + TOP_EARLGREY_UART1_SIZE_BYTES`.
pub const TOP_EARLGREY_UART1_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for uart2 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_UART2_BASE_ADDR: usize = 0x40020000;

/// Peripheral size for uart2 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_UART2_BASE_ADDR and
/// `TOP_EARLGREY_UART2_BASE_ADDR + TOP_EARLGREY_UART2_SIZE_BYTES`.
pub const TOP_EARLGREY_UART2_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for uart3 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_UART3_BASE_ADDR: usize = 0x40030000;

/// Peripheral size for uart3 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_UART3_BASE_ADDR and
/// `TOP_EARLGREY_UART3_BASE_ADDR + TOP_EARLGREY_UART3_SIZE_BYTES`.
pub const TOP_EARLGREY_UART3_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for gpio in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_GPIO_BASE_ADDR: usize = 0x40040000;

/// Peripheral size for gpio in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_GPIO_BASE_ADDR and
/// `TOP_EARLGREY_GPIO_BASE_ADDR + TOP_EARLGREY_GPIO_SIZE_BYTES`.
pub const TOP_EARLGREY_GPIO_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for spi_device in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SPI_DEVICE_BASE_ADDR: usize = 0x40050000;

/// Peripheral size for spi_device in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SPI_DEVICE_BASE_ADDR and
/// `TOP_EARLGREY_SPI_DEVICE_BASE_ADDR + TOP_EARLGREY_SPI_DEVICE_SIZE_BYTES`.
pub const TOP_EARLGREY_SPI_DEVICE_SIZE_BYTES: usize = 0x2000;
/// Peripheral base address for i2c0 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_I2C0_BASE_ADDR: usize = 0x40080000;

/// Peripheral size for i2c0 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_I2C0_BASE_ADDR and
/// `TOP_EARLGREY_I2C0_BASE_ADDR + TOP_EARLGREY_I2C0_SIZE_BYTES`.
pub const TOP_EARLGREY_I2C0_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for i2c1 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_I2C1_BASE_ADDR: usize = 0x40090000;

/// Peripheral size for i2c1 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_I2C1_BASE_ADDR and
/// `TOP_EARLGREY_I2C1_BASE_ADDR + TOP_EARLGREY_I2C1_SIZE_BYTES`.
pub const TOP_EARLGREY_I2C1_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for i2c2 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_I2C2_BASE_ADDR: usize = 0x400A0000;

/// Peripheral size for i2c2 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_I2C2_BASE_ADDR and
/// `TOP_EARLGREY_I2C2_BASE_ADDR + TOP_EARLGREY_I2C2_SIZE_BYTES`.
pub const TOP_EARLGREY_I2C2_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for pattgen in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_PATTGEN_BASE_ADDR: usize = 0x400E0000;

/// Peripheral size for pattgen in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_PATTGEN_BASE_ADDR and
/// `TOP_EARLGREY_PATTGEN_BASE_ADDR + TOP_EARLGREY_PATTGEN_SIZE_BYTES`.
pub const TOP_EARLGREY_PATTGEN_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for rv_timer in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RV_TIMER_BASE_ADDR: usize = 0x40100000;

/// Peripheral size for rv_timer in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RV_TIMER_BASE_ADDR and
/// `TOP_EARLGREY_RV_TIMER_BASE_ADDR + TOP_EARLGREY_RV_TIMER_SIZE_BYTES`.
pub const TOP_EARLGREY_RV_TIMER_SIZE_BYTES: usize = 0x200;
/// Peripheral base address for core device on otp_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_OTP_CTRL_CORE_BASE_ADDR: usize = 0x40130000;

/// Peripheral size for core device on otp_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_OTP_CTRL_CORE_BASE_ADDR and
/// `TOP_EARLGREY_OTP_CTRL_CORE_BASE_ADDR + TOP_EARLGREY_OTP_CTRL_CORE_SIZE_BYTES`.
pub const TOP_EARLGREY_OTP_CTRL_CORE_SIZE_BYTES: usize = 0x2000;
/// Peripheral base address for prim device on otp_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_OTP_CTRL_PRIM_BASE_ADDR: usize = 0x40132000;

/// Peripheral size for prim device on otp_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_OTP_CTRL_PRIM_BASE_ADDR and
/// `TOP_EARLGREY_OTP_CTRL_PRIM_BASE_ADDR + TOP_EARLGREY_OTP_CTRL_PRIM_SIZE_BYTES`.
pub const TOP_EARLGREY_OTP_CTRL_PRIM_SIZE_BYTES: usize = 0x20;
/// Peripheral base address for lc_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_LC_CTRL_BASE_ADDR: usize = 0x40140000;

/// Peripheral size for lc_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_LC_CTRL_BASE_ADDR and
/// `TOP_EARLGREY_LC_CTRL_BASE_ADDR + TOP_EARLGREY_LC_CTRL_SIZE_BYTES`.
pub const TOP_EARLGREY_LC_CTRL_SIZE_BYTES: usize = 0x100;
/// Peripheral base address for alert_handler in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_ALERT_HANDLER_BASE_ADDR: usize = 0x40150000;

/// Peripheral size for alert_handler in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_ALERT_HANDLER_BASE_ADDR and
/// `TOP_EARLGREY_ALERT_HANDLER_BASE_ADDR + TOP_EARLGREY_ALERT_HANDLER_SIZE_BYTES`.
pub const TOP_EARLGREY_ALERT_HANDLER_SIZE_BYTES: usize = 0x800;
/// Peripheral base address for spi_host0 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SPI_HOST0_BASE_ADDR: usize = 0x40300000;

/// Peripheral size for spi_host0 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SPI_HOST0_BASE_ADDR and
/// `TOP_EARLGREY_SPI_HOST0_BASE_ADDR + TOP_EARLGREY_SPI_HOST0_SIZE_BYTES`.
pub const TOP_EARLGREY_SPI_HOST0_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for spi_host1 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SPI_HOST1_BASE_ADDR: usize = 0x40310000;

/// Peripheral size for spi_host1 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SPI_HOST1_BASE_ADDR and
/// `TOP_EARLGREY_SPI_HOST1_BASE_ADDR + TOP_EARLGREY_SPI_HOST1_SIZE_BYTES`.
pub const TOP_EARLGREY_SPI_HOST1_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for usbdev in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_USBDEV_BASE_ADDR: usize = 0x40320000;

/// Peripheral size for usbdev in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_USBDEV_BASE_ADDR and
/// `TOP_EARLGREY_USBDEV_BASE_ADDR + TOP_EARLGREY_USBDEV_SIZE_BYTES`.
pub const TOP_EARLGREY_USBDEV_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for pwrmgr_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_PWRMGR_AON_BASE_ADDR: usize = 0x40400000;

/// Peripheral size for pwrmgr_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_PWRMGR_AON_BASE_ADDR and
/// `TOP_EARLGREY_PWRMGR_AON_BASE_ADDR + TOP_EARLGREY_PWRMGR_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_PWRMGR_AON_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for rstmgr_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RSTMGR_AON_BASE_ADDR: usize = 0x40410000;

/// Peripheral size for rstmgr_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RSTMGR_AON_BASE_ADDR and
/// `TOP_EARLGREY_RSTMGR_AON_BASE_ADDR + TOP_EARLGREY_RSTMGR_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_RSTMGR_AON_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for clkmgr_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_CLKMGR_AON_BASE_ADDR: usize = 0x40420000;

/// Peripheral size for clkmgr_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_CLKMGR_AON_BASE_ADDR and
/// `TOP_EARLGREY_CLKMGR_AON_BASE_ADDR + TOP_EARLGREY_CLKMGR_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_CLKMGR_AON_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for sysrst_ctrl_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SYSRST_CTRL_AON_BASE_ADDR: usize = 0x40430000;

/// Peripheral size for sysrst_ctrl_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SYSRST_CTRL_AON_BASE_ADDR and
/// `TOP_EARLGREY_SYSRST_CTRL_AON_BASE_ADDR + TOP_EARLGREY_SYSRST_CTRL_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_SYSRST_CTRL_AON_SIZE_BYTES: usize = 0x100;
/// Peripheral base address for adc_ctrl_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_ADC_CTRL_AON_BASE_ADDR: usize = 0x40440000;

/// Peripheral size for adc_ctrl_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_ADC_CTRL_AON_BASE_ADDR and
/// `TOP_EARLGREY_ADC_CTRL_AON_BASE_ADDR + TOP_EARLGREY_ADC_CTRL_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_ADC_CTRL_AON_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for pwm_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_PWM_AON_BASE_ADDR: usize = 0x40450000;

/// Peripheral size for pwm_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_PWM_AON_BASE_ADDR and
/// `TOP_EARLGREY_PWM_AON_BASE_ADDR + TOP_EARLGREY_PWM_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_PWM_AON_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for pinmux_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_PINMUX_AON_BASE_ADDR: usize = 0x40460000;

/// Peripheral size for pinmux_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_PINMUX_AON_BASE_ADDR and
/// `TOP_EARLGREY_PINMUX_AON_BASE_ADDR + TOP_EARLGREY_PINMUX_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_PINMUX_AON_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for aon_timer_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_AON_TIMER_AON_BASE_ADDR: usize = 0x40470000;

/// Peripheral size for aon_timer_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_AON_TIMER_AON_BASE_ADDR and
/// `TOP_EARLGREY_AON_TIMER_AON_BASE_ADDR + TOP_EARLGREY_AON_TIMER_AON_SIZE_BYTES`.
pub const TOP_EARLGREY_AON_TIMER_AON_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for ast in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_AST_BASE_ADDR: usize = 0x40480000;

/// Peripheral size for ast in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_AST_BASE_ADDR and
/// `TOP_EARLGREY_AST_BASE_ADDR + TOP_EARLGREY_AST_SIZE_BYTES`.
pub const TOP_EARLGREY_AST_SIZE_BYTES: usize = 0x400;
/// Peripheral base address for sensor_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SENSOR_CTRL_BASE_ADDR: usize = 0x40490000;

/// Peripheral size for sensor_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SENSOR_CTRL_BASE_ADDR and
/// `TOP_EARLGREY_SENSOR_CTRL_BASE_ADDR + TOP_EARLGREY_SENSOR_CTRL_SIZE_BYTES`.
pub const TOP_EARLGREY_SENSOR_CTRL_SIZE_BYTES: usize = 0x40;
/// Peripheral base address for regs device on sram_ctrl_ret_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SRAM_CTRL_RET_AON_REGS_BASE_ADDR: usize = 0x40500000;

/// Peripheral size for regs device on sram_ctrl_ret_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SRAM_CTRL_RET_AON_REGS_BASE_ADDR and
/// `TOP_EARLGREY_SRAM_CTRL_RET_AON_REGS_BASE_ADDR + TOP_EARLGREY_SRAM_CTRL_RET_AON_REGS_SIZE_BYTES`.
pub const TOP_EARLGREY_SRAM_CTRL_RET_AON_REGS_SIZE_BYTES: usize = 0x20;
/// Peripheral base address for ram device on sram_ctrl_ret_aon in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SRAM_CTRL_RET_AON_RAM_BASE_ADDR: usize = 0x40600000;

/// Peripheral size for ram device on sram_ctrl_ret_aon in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SRAM_CTRL_RET_AON_RAM_BASE_ADDR and
/// `TOP_EARLGREY_SRAM_CTRL_RET_AON_RAM_BASE_ADDR + TOP_EARLGREY_SRAM_CTRL_RET_AON_RAM_SIZE_BYTES`.
pub const TOP_EARLGREY_SRAM_CTRL_RET_AON_RAM_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for core device on flash_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_FLASH_CTRL_CORE_BASE_ADDR: usize = 0x41000000;

/// Peripheral size for core device on flash_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_FLASH_CTRL_CORE_BASE_ADDR and
/// `TOP_EARLGREY_FLASH_CTRL_CORE_BASE_ADDR + TOP_EARLGREY_FLASH_CTRL_CORE_SIZE_BYTES`.
pub const TOP_EARLGREY_FLASH_CTRL_CORE_SIZE_BYTES: usize = 0x200;
/// Peripheral base address for prim device on flash_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_FLASH_CTRL_PRIM_BASE_ADDR: usize = 0x41008000;

/// Peripheral size for prim device on flash_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_FLASH_CTRL_PRIM_BASE_ADDR and
/// `TOP_EARLGREY_FLASH_CTRL_PRIM_BASE_ADDR + TOP_EARLGREY_FLASH_CTRL_PRIM_SIZE_BYTES`.
pub const TOP_EARLGREY_FLASH_CTRL_PRIM_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for mem device on flash_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_FLASH_CTRL_MEM_BASE_ADDR: usize = 0x20000000;

/// Peripheral size for mem device on flash_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_FLASH_CTRL_MEM_BASE_ADDR and
/// `TOP_EARLGREY_FLASH_CTRL_MEM_BASE_ADDR + TOP_EARLGREY_FLASH_CTRL_MEM_SIZE_BYTES`.
pub const TOP_EARLGREY_FLASH_CTRL_MEM_SIZE_BYTES: usize = 0x100000;
/// Peripheral base address for regs device on rv_dm in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RV_DM_REGS_BASE_ADDR: usize = 0x41200000;

/// Peripheral size for regs device on rv_dm in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RV_DM_REGS_BASE_ADDR and
/// `TOP_EARLGREY_RV_DM_REGS_BASE_ADDR + TOP_EARLGREY_RV_DM_REGS_SIZE_BYTES`.
pub const TOP_EARLGREY_RV_DM_REGS_SIZE_BYTES: usize = 0x4;
/// Peripheral base address for mem device on rv_dm in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RV_DM_MEM_BASE_ADDR: usize = 0x10000;

/// Peripheral size for mem device on rv_dm in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RV_DM_MEM_BASE_ADDR and
/// `TOP_EARLGREY_RV_DM_MEM_BASE_ADDR + TOP_EARLGREY_RV_DM_MEM_SIZE_BYTES`.
pub const TOP_EARLGREY_RV_DM_MEM_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for rv_plic in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RV_PLIC_BASE_ADDR: usize = 0x48000000;

/// Peripheral size for rv_plic in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RV_PLIC_BASE_ADDR and
/// `TOP_EARLGREY_RV_PLIC_BASE_ADDR + TOP_EARLGREY_RV_PLIC_SIZE_BYTES`.
pub const TOP_EARLGREY_RV_PLIC_SIZE_BYTES: usize = 0x8000000;
/// Peripheral base address for aes in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_AES_BASE_ADDR: usize = 0x41100000;

/// Peripheral size for aes in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_AES_BASE_ADDR and
/// `TOP_EARLGREY_AES_BASE_ADDR + TOP_EARLGREY_AES_SIZE_BYTES`.
pub const TOP_EARLGREY_AES_SIZE_BYTES: usize = 0x100;
/// Peripheral base address for hmac in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_HMAC_BASE_ADDR: usize = 0x41110000;

/// Peripheral size for hmac in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_HMAC_BASE_ADDR and
/// `TOP_EARLGREY_HMAC_BASE_ADDR + TOP_EARLGREY_HMAC_SIZE_BYTES`.
pub const TOP_EARLGREY_HMAC_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for kmac in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_KMAC_BASE_ADDR: usize = 0x41120000;

/// Peripheral size for kmac in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_KMAC_BASE_ADDR and
/// `TOP_EARLGREY_KMAC_BASE_ADDR + TOP_EARLGREY_KMAC_SIZE_BYTES`.
pub const TOP_EARLGREY_KMAC_SIZE_BYTES: usize = 0x1000;
/// Peripheral base address for otbn in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_OTBN_BASE_ADDR: usize = 0x41130000;

/// Peripheral size for otbn in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_OTBN_BASE_ADDR and
/// `TOP_EARLGREY_OTBN_BASE_ADDR + TOP_EARLGREY_OTBN_SIZE_BYTES`.
pub const TOP_EARLGREY_OTBN_SIZE_BYTES: usize = 0x10000;
/// Peripheral base address for keymgr in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_KEYMGR_BASE_ADDR: usize = 0x41140000;

/// Peripheral size for keymgr in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_KEYMGR_BASE_ADDR and
/// `TOP_EARLGREY_KEYMGR_BASE_ADDR + TOP_EARLGREY_KEYMGR_SIZE_BYTES`.
pub const TOP_EARLGREY_KEYMGR_SIZE_BYTES: usize = 0x100;
/// Peripheral base address for csrng in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_CSRNG_BASE_ADDR: usize = 0x41150000;

/// Peripheral size for csrng in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_CSRNG_BASE_ADDR and
/// `TOP_EARLGREY_CSRNG_BASE_ADDR + TOP_EARLGREY_CSRNG_SIZE_BYTES`.
pub const TOP_EARLGREY_CSRNG_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for entropy_src in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_ENTROPY_SRC_BASE_ADDR: usize = 0x41160000;

/// Peripheral size for entropy_src in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_ENTROPY_SRC_BASE_ADDR and
/// `TOP_EARLGREY_ENTROPY_SRC_BASE_ADDR + TOP_EARLGREY_ENTROPY_SRC_SIZE_BYTES`.
pub const TOP_EARLGREY_ENTROPY_SRC_SIZE_BYTES: usize = 0x100;
/// Peripheral base address for edn0 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_EDN0_BASE_ADDR: usize = 0x41170000;

/// Peripheral size for edn0 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_EDN0_BASE_ADDR and
/// `TOP_EARLGREY_EDN0_BASE_ADDR + TOP_EARLGREY_EDN0_SIZE_BYTES`.
pub const TOP_EARLGREY_EDN0_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for edn1 in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_EDN1_BASE_ADDR: usize = 0x41180000;

/// Peripheral size for edn1 in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_EDN1_BASE_ADDR and
/// `TOP_EARLGREY_EDN1_BASE_ADDR + TOP_EARLGREY_EDN1_SIZE_BYTES`.
pub const TOP_EARLGREY_EDN1_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for regs device on sram_ctrl_main in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SRAM_CTRL_MAIN_REGS_BASE_ADDR: usize = 0x411C0000;

/// Peripheral size for regs device on sram_ctrl_main in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SRAM_CTRL_MAIN_REGS_BASE_ADDR and
/// `TOP_EARLGREY_SRAM_CTRL_MAIN_REGS_BASE_ADDR + TOP_EARLGREY_SRAM_CTRL_MAIN_REGS_SIZE_BYTES`.
pub const TOP_EARLGREY_SRAM_CTRL_MAIN_REGS_SIZE_BYTES: usize = 0x20;
/// Peripheral base address for ram device on sram_ctrl_main in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_SRAM_CTRL_MAIN_RAM_BASE_ADDR: usize = 0x10000000;

/// Peripheral size for ram device on sram_ctrl_main in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_SRAM_CTRL_MAIN_RAM_BASE_ADDR and
/// `TOP_EARLGREY_SRAM_CTRL_MAIN_RAM_BASE_ADDR + TOP_EARLGREY_SRAM_CTRL_MAIN_RAM_SIZE_BYTES`.
pub const TOP_EARLGREY_SRAM_CTRL_MAIN_RAM_SIZE_BYTES: usize = 0x20000;
/// Peripheral base address for regs device on rom_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_ROM_CTRL_REGS_BASE_ADDR: usize = 0x411E0000;

/// Peripheral size for regs device on rom_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_ROM_CTRL_REGS_BASE_ADDR and
/// `TOP_EARLGREY_ROM_CTRL_REGS_BASE_ADDR + TOP_EARLGREY_ROM_CTRL_REGS_SIZE_BYTES`.
pub const TOP_EARLGREY_ROM_CTRL_REGS_SIZE_BYTES: usize = 0x80;
/// Peripheral base address for rom device on rom_ctrl in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_ROM_CTRL_ROM_BASE_ADDR: usize = 0x8000;

/// Peripheral size for rom device on rom_ctrl in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_ROM_CTRL_ROM_BASE_ADDR and
/// `TOP_EARLGREY_ROM_CTRL_ROM_BASE_ADDR + TOP_EARLGREY_ROM_CTRL_ROM_SIZE_BYTES`.
pub const TOP_EARLGREY_ROM_CTRL_ROM_SIZE_BYTES: usize = 0x8000;
/// Peripheral base address for cfg device on rv_core_ibex in top earlgrey.
///
/// This should be used with #mmio_region_from_addr to access the memory-mapped
/// registers associated with the peripheral (usually via a DIF).
pub const TOP_EARLGREY_RV_CORE_IBEX_CFG_BASE_ADDR: usize = 0x411F0000;

/// Peripheral size for cfg device on rv_core_ibex in top earlgrey.
///
/// This is the size (in bytes) of the peripheral's reserved memory area. All
/// memory-mapped registers associated with this peripheral should have an
/// address between #TOP_EARLGREY_RV_CORE_IBEX_CFG_BASE_ADDR and
/// `TOP_EARLGREY_RV_CORE_IBEX_CFG_BASE_ADDR + TOP_EARLGREY_RV_CORE_IBEX_CFG_SIZE_BYTES`.
pub const TOP_EARLGREY_RV_CORE_IBEX_CFG_SIZE_BYTES: usize = 0x100;

/// Memory base address for ram_ret_aon in top earlgrey.
pub const RAM_RET_AON_BASE_ADDR: usize = 0x40600000;

/// Memory size for ram_ret_aon in top earlgrey.
pub const RAM_RET_AON_SIZE_BYTES: usize = 0x1000;
/// Memory base address for eflash in top earlgrey.
pub const EFLASH_BASE_ADDR: usize = 0x20000000;

/// Memory size for eflash in top earlgrey.
pub const EFLASH_SIZE_BYTES: usize = 0x100000;
/// Memory base address for ram_main in top earlgrey.
pub const RAM_MAIN_BASE_ADDR: usize = 0x10000000;

/// Memory size for ram_main in top earlgrey.
pub const RAM_MAIN_SIZE_BYTES: usize = 0x20000;
/// Memory base address for rom in top earlgrey.
pub const ROM_BASE_ADDR: usize = 0x8000;

/// Memory size for rom in top earlgrey.
pub const ROM_SIZE_BYTES: usize = 0x8000;

/// PLIC Interrupt Source Peripheral.
///
/// Enumeration used to determine which peripheral asserted the corresponding
/// interrupt.
#[repr(u32)]
pub enum TopEarlgreyPlicPeripheral {
    /// Unknown Peripheral
    Unknown = 0,
    /// uart0
    Uart0 = 1,
    /// uart1
    Uart1 = 2,
    /// uart2
    Uart2 = 3,
    /// uart3
    Uart3 = 4,
    /// gpio
    Gpio = 5,
    /// spi_device
    SpiDevice = 6,
    /// i2c0
    I2c0 = 7,
    /// i2c1
    I2c1 = 8,
    /// i2c2
    I2c2 = 9,
    /// pattgen
    Pattgen = 10,
    /// rv_timer
    RvTimer = 11,
    /// otp_ctrl
    OtpCtrl = 12,
    /// alert_handler
    AlertHandler = 13,
    /// spi_host0
    SpiHost0 = 14,
    /// spi_host1
    SpiHost1 = 15,
    /// usbdev
    Usbdev = 16,
    /// pwrmgr_aon
    PwrmgrAon = 17,
    /// sysrst_ctrl_aon
    SysrstCtrlAon = 18,
    /// adc_ctrl_aon
    AdcCtrlAon = 19,
    /// aon_timer_aon
    AonTimerAon = 20,
    /// sensor_ctrl
    SensorCtrl = 21,
    /// flash_ctrl
    FlashCtrl = 22,
    /// hmac
    Hmac = 23,
    /// kmac
    Kmac = 24,
    /// otbn
    Otbn = 25,
    /// keymgr
    Keymgr = 26,
    /// csrng
    Csrng = 27,
    /// entropy_src
    EntropySrc = 28,
    /// edn0
    Edn0 = 29,
    /// edn1
    Edn1 = 30,
    /// \internal Number of PLIC peripheral
    End = 31,
}

/// PLIC Interrupt Source.
///
/// Enumeration of all PLIC interrupt sources. The interrupt sources belonging to
/// the same peripheral are guaranteed to be consecutive.
#[repr(u32)]
pub enum TopEarlgreyPlicIrqId {
    /// No Interrupt
    None = 0,
    /// uart0_tx_watermark
    Uart0TxWatermark = 1,
    /// uart0_rx_watermark
    Uart0RxWatermark = 2,
    /// uart0_tx_empty
    Uart0TxEmpty = 3,
    /// uart0_rx_overflow
    Uart0RxOverflow = 4,
    /// uart0_rx_frame_err
    Uart0RxFrameErr = 5,
    /// uart0_rx_break_err
    Uart0RxBreakErr = 6,
    /// uart0_rx_timeout
    Uart0RxTimeout = 7,
    /// uart0_rx_parity_err
    Uart0RxParityErr = 8,
    /// uart1_tx_watermark
    Uart1TxWatermark = 9,
    /// uart1_rx_watermark
    Uart1RxWatermark = 10,
    /// uart1_tx_empty
    Uart1TxEmpty = 11,
    /// uart1_rx_overflow
    Uart1RxOverflow = 12,
    /// uart1_rx_frame_err
    Uart1RxFrameErr = 13,
    /// uart1_rx_break_err
    Uart1RxBreakErr = 14,
    /// uart1_rx_timeout
    Uart1RxTimeout = 15,
    /// uart1_rx_parity_err
    Uart1RxParityErr = 16,
    /// uart2_tx_watermark
    Uart2TxWatermark = 17,
    /// uart2_rx_watermark
    Uart2RxWatermark = 18,
    /// uart2_tx_empty
    Uart2TxEmpty = 19,
    /// uart2_rx_overflow
    Uart2RxOverflow = 20,
    /// uart2_rx_frame_err
    Uart2RxFrameErr = 21,
    /// uart2_rx_break_err
    Uart2RxBreakErr = 22,
    /// uart2_rx_timeout
    Uart2RxTimeout = 23,
    /// uart2_rx_parity_err
    Uart2RxParityErr = 24,
    /// uart3_tx_watermark
    Uart3TxWatermark = 25,
    /// uart3_rx_watermark
    Uart3RxWatermark = 26,
    /// uart3_tx_empty
    Uart3TxEmpty = 27,
    /// uart3_rx_overflow
    Uart3RxOverflow = 28,
    /// uart3_rx_frame_err
    Uart3RxFrameErr = 29,
    /// uart3_rx_break_err
    Uart3RxBreakErr = 30,
    /// uart3_rx_timeout
    Uart3RxTimeout = 31,
    /// uart3_rx_parity_err
    Uart3RxParityErr = 32,
    /// gpio_gpio 0
    GpioGpio0 = 33,
    /// gpio_gpio 1
    GpioGpio1 = 34,
    /// gpio_gpio 2
    GpioGpio2 = 35,
    /// gpio_gpio 3
    GpioGpio3 = 36,
    /// gpio_gpio 4
    GpioGpio4 = 37,
    /// gpio_gpio 5
    GpioGpio5 = 38,
    /// gpio_gpio 6
    GpioGpio6 = 39,
    /// gpio_gpio 7
    GpioGpio7 = 40,
    /// gpio_gpio 8
    GpioGpio8 = 41,
    /// gpio_gpio 9
    GpioGpio9 = 42,
    /// gpio_gpio 10
    GpioGpio10 = 43,
    /// gpio_gpio 11
    GpioGpio11 = 44,
    /// gpio_gpio 12
    GpioGpio12 = 45,
    /// gpio_gpio 13
    GpioGpio13 = 46,
    /// gpio_gpio 14
    GpioGpio14 = 47,
    /// gpio_gpio 15
    GpioGpio15 = 48,
    /// gpio_gpio 16
    GpioGpio16 = 49,
    /// gpio_gpio 17
    GpioGpio17 = 50,
    /// gpio_gpio 18
    GpioGpio18 = 51,
    /// gpio_gpio 19
    GpioGpio19 = 52,
    /// gpio_gpio 20
    GpioGpio20 = 53,
    /// gpio_gpio 21
    GpioGpio21 = 54,
    /// gpio_gpio 22
    GpioGpio22 = 55,
    /// gpio_gpio 23
    GpioGpio23 = 56,
    /// gpio_gpio 24
    GpioGpio24 = 57,
    /// gpio_gpio 25
    GpioGpio25 = 58,
    /// gpio_gpio 26
    GpioGpio26 = 59,
    /// gpio_gpio 27
    GpioGpio27 = 60,
    /// gpio_gpio 28
    GpioGpio28 = 61,
    /// gpio_gpio 29
    GpioGpio29 = 62,
    /// gpio_gpio 30
    GpioGpio30 = 63,
    /// gpio_gpio 31
    GpioGpio31 = 64,
    /// spi_device_generic_rx_full
    SpiDeviceGenericRxFull = 65,
    /// spi_device_generic_rx_watermark
    SpiDeviceGenericRxWatermark = 66,
    /// spi_device_generic_tx_watermark
    SpiDeviceGenericTxWatermark = 67,
    /// spi_device_generic_rx_error
    SpiDeviceGenericRxError = 68,
    /// spi_device_generic_rx_overflow
    SpiDeviceGenericRxOverflow = 69,
    /// spi_device_generic_tx_underflow
    SpiDeviceGenericTxUnderflow = 70,
    /// spi_device_upload_cmdfifo_not_empty
    SpiDeviceUploadCmdfifoNotEmpty = 71,
    /// spi_device_upload_payload_not_empty
    SpiDeviceUploadPayloadNotEmpty = 72,
    /// spi_device_upload_payload_overflow
    SpiDeviceUploadPayloadOverflow = 73,
    /// spi_device_readbuf_watermark
    SpiDeviceReadbufWatermark = 74,
    /// spi_device_readbuf_flip
    SpiDeviceReadbufFlip = 75,
    /// spi_device_tpm_header_not_empty
    SpiDeviceTpmHeaderNotEmpty = 76,
    /// i2c0_fmt_threshold
    I2c0FmtThreshold = 77,
    /// i2c0_rx_threshold
    I2c0RxThreshold = 78,
    /// i2c0_fmt_overflow
    I2c0FmtOverflow = 79,
    /// i2c0_rx_overflow
    I2c0RxOverflow = 80,
    /// i2c0_nak
    I2c0Nak = 81,
    /// i2c0_scl_interference
    I2c0SclInterference = 82,
    /// i2c0_sda_interference
    I2c0SdaInterference = 83,
    /// i2c0_stretch_timeout
    I2c0StretchTimeout = 84,
    /// i2c0_sda_unstable
    I2c0SdaUnstable = 85,
    /// i2c0_cmd_complete
    I2c0CmdComplete = 86,
    /// i2c0_tx_stretch
    I2c0TxStretch = 87,
    /// i2c0_tx_overflow
    I2c0TxOverflow = 88,
    /// i2c0_acq_full
    I2c0AcqFull = 89,
    /// i2c0_unexp_stop
    I2c0UnexpStop = 90,
    /// i2c0_host_timeout
    I2c0HostTimeout = 91,
    /// i2c1_fmt_threshold
    I2c1FmtThreshold = 92,
    /// i2c1_rx_threshold
    I2c1RxThreshold = 93,
    /// i2c1_fmt_overflow
    I2c1FmtOverflow = 94,
    /// i2c1_rx_overflow
    I2c1RxOverflow = 95,
    /// i2c1_nak
    I2c1Nak = 96,
    /// i2c1_scl_interference
    I2c1SclInterference = 97,
    /// i2c1_sda_interference
    I2c1SdaInterference = 98,
    /// i2c1_stretch_timeout
    I2c1StretchTimeout = 99,
    /// i2c1_sda_unstable
    I2c1SdaUnstable = 100,
    /// i2c1_cmd_complete
    I2c1CmdComplete = 101,
    /// i2c1_tx_stretch
    I2c1TxStretch = 102,
    /// i2c1_tx_overflow
    I2c1TxOverflow = 103,
    /// i2c1_acq_full
    I2c1AcqFull = 104,
    /// i2c1_unexp_stop
    I2c1UnexpStop = 105,
    /// i2c1_host_timeout
    I2c1HostTimeout = 106,
    /// i2c2_fmt_threshold
    I2c2FmtThreshold = 107,
    /// i2c2_rx_threshold
    I2c2RxThreshold = 108,
    /// i2c2_fmt_overflow
    I2c2FmtOverflow = 109,
    /// i2c2_rx_overflow
    I2c2RxOverflow = 110,
    /// i2c2_nak
    I2c2Nak = 111,
    /// i2c2_scl_interference
    I2c2SclInterference = 112,
    /// i2c2_sda_interference
    I2c2SdaInterference = 113,
    /// i2c2_stretch_timeout
    I2c2StretchTimeout = 114,
    /// i2c2_sda_unstable
    I2c2SdaUnstable = 115,
    /// i2c2_cmd_complete
    I2c2CmdComplete = 116,
    /// i2c2_tx_stretch
    I2c2TxStretch = 117,
    /// i2c2_tx_overflow
    I2c2TxOverflow = 118,
    /// i2c2_acq_full
    I2c2AcqFull = 119,
    /// i2c2_unexp_stop
    I2c2UnexpStop = 120,
    /// i2c2_host_timeout
    I2c2HostTimeout = 121,
    /// pattgen_done_ch0
    PattgenDoneCh0 = 122,
    /// pattgen_done_ch1
    PattgenDoneCh1 = 123,
    /// rv_timer_timer_expired_hart0_timer0
    RvTimerTimerExpiredHart0Timer0 = 124,
    /// otp_ctrl_otp_operation_done
    OtpCtrlOtpOperationDone = 125,
    /// otp_ctrl_otp_error
    OtpCtrlOtpError = 126,
    /// alert_handler_classa
    AlertHandlerClassa = 127,
    /// alert_handler_classb
    AlertHandlerClassb = 128,
    /// alert_handler_classc
    AlertHandlerClassc = 129,
    /// alert_handler_classd
    AlertHandlerClassd = 130,
    /// spi_host0_error
    SpiHost0Error = 131,
    /// spi_host0_spi_event
    SpiHost0SpiEvent = 132,
    /// spi_host1_error
    SpiHost1Error = 133,
    /// spi_host1_spi_event
    SpiHost1SpiEvent = 134,
    /// usbdev_pkt_received
    UsbdevPktReceived = 135,
    /// usbdev_pkt_sent
    UsbdevPktSent = 136,
    /// usbdev_disconnected
    UsbdevDisconnected = 137,
    /// usbdev_host_lost
    UsbdevHostLost = 138,
    /// usbdev_link_reset
    UsbdevLinkReset = 139,
    /// usbdev_link_suspend
    UsbdevLinkSuspend = 140,
    /// usbdev_link_resume
    UsbdevLinkResume = 141,
    /// usbdev_av_empty
    UsbdevAvEmpty = 142,
    /// usbdev_rx_full
    UsbdevRxFull = 143,
    /// usbdev_av_overflow
    UsbdevAvOverflow = 144,
    /// usbdev_link_in_err
    UsbdevLinkInErr = 145,
    /// usbdev_rx_crc_err
    UsbdevRxCrcErr = 146,
    /// usbdev_rx_pid_err
    UsbdevRxPidErr = 147,
    /// usbdev_rx_bitstuff_err
    UsbdevRxBitstuffErr = 148,
    /// usbdev_frame
    UsbdevFrame = 149,
    /// usbdev_powered
    UsbdevPowered = 150,
    /// usbdev_link_out_err
    UsbdevLinkOutErr = 151,
    /// pwrmgr_aon_wakeup
    PwrmgrAonWakeup = 152,
    /// sysrst_ctrl_aon_event_detected
    SysrstCtrlAonEventDetected = 153,
    /// adc_ctrl_aon_match_done
    AdcCtrlAonMatchDone = 154,
    /// aon_timer_aon_wkup_timer_expired
    AonTimerAonWkupTimerExpired = 155,
    /// aon_timer_aon_wdog_timer_bark
    AonTimerAonWdogTimerBark = 156,
    /// sensor_ctrl_io_status_change
    SensorCtrlIoStatusChange = 157,
    /// sensor_ctrl_init_status_change
    SensorCtrlInitStatusChange = 158,
    /// flash_ctrl_prog_empty
    FlashCtrlProgEmpty = 159,
    /// flash_ctrl_prog_lvl
    FlashCtrlProgLvl = 160,
    /// flash_ctrl_rd_full
    FlashCtrlRdFull = 161,
    /// flash_ctrl_rd_lvl
    FlashCtrlRdLvl = 162,
    /// flash_ctrl_op_done
    FlashCtrlOpDone = 163,
    /// flash_ctrl_corr_err
    FlashCtrlCorrErr = 164,
    /// hmac_hmac_done
    HmacHmacDone = 165,
    /// hmac_fifo_empty
    HmacFifoEmpty = 166,
    /// hmac_hmac_err
    HmacHmacErr = 167,
    /// kmac_kmac_done
    KmacKmacDone = 168,
    /// kmac_fifo_empty
    KmacFifoEmpty = 169,
    /// kmac_kmac_err
    KmacKmacErr = 170,
    /// otbn_done
    OtbnDone = 171,
    /// keymgr_op_done
    KeymgrOpDone = 172,
    /// csrng_cs_cmd_req_done
    CsrngCsCmdReqDone = 173,
    /// csrng_cs_entropy_req
    CsrngCsEntropyReq = 174,
    /// csrng_cs_hw_inst_exc
    CsrngCsHwInstExc = 175,
    /// csrng_cs_fatal_err
    CsrngCsFatalErr = 176,
    /// entropy_src_es_entropy_valid
    EntropySrcEsEntropyValid = 177,
    /// entropy_src_es_health_test_failed
    EntropySrcEsHealthTestFailed = 178,
    /// entropy_src_es_observe_fifo_ready
    EntropySrcEsObserveFifoReady = 179,
    /// entropy_src_es_fatal_err
    EntropySrcEsFatalErr = 180,
    /// edn0_edn_cmd_req_done
    Edn0EdnCmdReqDone = 181,
    /// edn0_edn_fatal_err
    Edn0EdnFatalErr = 182,
    /// edn1_edn_cmd_req_done
    Edn1EdnCmdReqDone = 183,
    /// edn1_edn_fatal_err
    Edn1EdnFatalErr = 184,
    /// \internal Number of Interrupt ID.
    End = 185,
}

/// PLIC Interrupt Target.
///
/// Enumeration used to determine which set of IE, CC, threshold registers to
/// access for a given interrupt target.
pub enum TopEarlgreyPlicTarget {
    /// Ibex Core 0
    Ibex0 = 0,
    /// \internal Final number of PLIC target
    End = 1,
}

/// Alert Handler Source Peripheral.
///
/// Enumeration used to determine which peripheral asserted the corresponding
/// alert.
#[repr(u32)]
pub enum TopEarlgreyAlertPeripheral {
    /// uart0
    Uart0 = 0,
    /// uart1
    Uart1 = 1,
    /// uart2
    Uart2 = 2,
    /// uart3
    Uart3 = 3,
    /// gpio
    Gpio = 4,
    /// spi_device
    SpiDevice = 5,
    /// i2c0
    I2c0 = 6,
    /// i2c1
    I2c1 = 7,
    /// i2c2
    I2c2 = 8,
    /// pattgen
    Pattgen = 9,
    /// rv_timer
    RvTimer = 10,
    /// otp_ctrl
    OtpCtrl = 11,
    /// lc_ctrl
    LcCtrl = 12,
    /// spi_host0
    SpiHost0 = 13,
    /// spi_host1
    SpiHost1 = 14,
    /// usbdev
    Usbdev = 15,
    /// pwrmgr_aon
    PwrmgrAon = 16,
    /// rstmgr_aon
    RstmgrAon = 17,
    /// clkmgr_aon
    ClkmgrAon = 18,
    /// sysrst_ctrl_aon
    SysrstCtrlAon = 19,
    /// adc_ctrl_aon
    AdcCtrlAon = 20,
    /// pwm_aon
    PwmAon = 21,
    /// pinmux_aon
    PinmuxAon = 22,
    /// aon_timer_aon
    AonTimerAon = 23,
    /// sensor_ctrl
    SensorCtrl = 24,
    /// sram_ctrl_ret_aon
    SramCtrlRetAon = 25,
    /// flash_ctrl
    FlashCtrl = 26,
    /// rv_dm
    RvDm = 27,
    /// rv_plic
    RvPlic = 28,
    /// aes
    Aes = 29,
    /// hmac
    Hmac = 30,
    /// kmac
    Kmac = 31,
    /// otbn
    Otbn = 32,
    /// keymgr
    Keymgr = 33,
    /// csrng
    Csrng = 34,
    /// entropy_src
    EntropySrc = 35,
    /// edn0
    Edn0 = 36,
    /// edn1
    Edn1 = 37,
    /// sram_ctrl_main
    SramCtrlMain = 38,
    /// rom_ctrl
    RomCtrl = 39,
    /// rv_core_ibex
    RvCoreIbex = 40,
    /// \internal Final number of Alert peripheral
    End = 41,
}

/// Alert Handler Alert Source.
///
/// Enumeration of all Alert Handler Alert Sources. The alert sources belonging to
/// the same peripheral are guaranteed to be consecutive.
#[repr(u32)]
pub enum TopEarlgreyAlertId {
    /// uart0_fatal_fault
    Uart0FatalFault = 0,
    /// uart1_fatal_fault
    Uart1FatalFault = 1,
    /// uart2_fatal_fault
    Uart2FatalFault = 2,
    /// uart3_fatal_fault
    Uart3FatalFault = 3,
    /// gpio_fatal_fault
    GpioFatalFault = 4,
    /// spi_device_fatal_fault
    SpiDeviceFatalFault = 5,
    /// i2c0_fatal_fault
    I2c0FatalFault = 6,
    /// i2c1_fatal_fault
    I2c1FatalFault = 7,
    /// i2c2_fatal_fault
    I2c2FatalFault = 8,
    /// pattgen_fatal_fault
    PattgenFatalFault = 9,
    /// rv_timer_fatal_fault
    RvTimerFatalFault = 10,
    /// otp_ctrl_fatal_macro_error
    OtpCtrlFatalMacroError = 11,
    /// otp_ctrl_fatal_check_error
    OtpCtrlFatalCheckError = 12,
    /// otp_ctrl_fatal_bus_integ_error
    OtpCtrlFatalBusIntegError = 13,
    /// otp_ctrl_fatal_prim_otp_alert
    OtpCtrlFatalPrimOtpAlert = 14,
    /// otp_ctrl_recov_prim_otp_alert
    OtpCtrlRecovPrimOtpAlert = 15,
    /// lc_ctrl_fatal_prog_error
    LcCtrlFatalProgError = 16,
    /// lc_ctrl_fatal_state_error
    LcCtrlFatalStateError = 17,
    /// lc_ctrl_fatal_bus_integ_error
    LcCtrlFatalBusIntegError = 18,
    /// spi_host0_fatal_fault
    SpiHost0FatalFault = 19,
    /// spi_host1_fatal_fault
    SpiHost1FatalFault = 20,
    /// usbdev_fatal_fault
    UsbdevFatalFault = 21,
    /// pwrmgr_aon_fatal_fault
    PwrmgrAonFatalFault = 22,
    /// rstmgr_aon_fatal_fault
    RstmgrAonFatalFault = 23,
    /// rstmgr_aon_fatal_cnsty_fault
    RstmgrAonFatalCnstyFault = 24,
    /// clkmgr_aon_recov_fault
    ClkmgrAonRecovFault = 25,
    /// clkmgr_aon_fatal_fault
    ClkmgrAonFatalFault = 26,
    /// sysrst_ctrl_aon_fatal_fault
    SysrstCtrlAonFatalFault = 27,
    /// adc_ctrl_aon_fatal_fault
    AdcCtrlAonFatalFault = 28,
    /// pwm_aon_fatal_fault
    PwmAonFatalFault = 29,
    /// pinmux_aon_fatal_fault
    PinmuxAonFatalFault = 30,
    /// aon_timer_aon_fatal_fault
    AonTimerAonFatalFault = 31,
    /// sensor_ctrl_recov_alert
    SensorCtrlRecovAlert = 32,
    /// sensor_ctrl_fatal_alert
    SensorCtrlFatalAlert = 33,
    /// sram_ctrl_ret_aon_fatal_error
    SramCtrlRetAonFatalError = 34,
    /// flash_ctrl_recov_err
    FlashCtrlRecovErr = 35,
    /// flash_ctrl_fatal_std_err
    FlashCtrlFatalStdErr = 36,
    /// flash_ctrl_fatal_err
    FlashCtrlFatalErr = 37,
    /// flash_ctrl_fatal_prim_flash_alert
    FlashCtrlFatalPrimFlashAlert = 38,
    /// flash_ctrl_recov_prim_flash_alert
    FlashCtrlRecovPrimFlashAlert = 39,
    /// rv_dm_fatal_fault
    RvDmFatalFault = 40,
    /// rv_plic_fatal_fault
    RvPlicFatalFault = 41,
    /// aes_recov_ctrl_update_err
    AesRecovCtrlUpdateErr = 42,
    /// aes_fatal_fault
    AesFatalFault = 43,
    /// hmac_fatal_fault
    HmacFatalFault = 44,
    /// kmac_recov_operation_err
    KmacRecovOperationErr = 45,
    /// kmac_fatal_fault_err
    KmacFatalFaultErr = 46,
    /// otbn_fatal
    OtbnFatal = 47,
    /// otbn_recov
    OtbnRecov = 48,
    /// keymgr_recov_operation_err
    KeymgrRecovOperationErr = 49,
    /// keymgr_fatal_fault_err
    KeymgrFatalFaultErr = 50,
    /// csrng_recov_alert
    CsrngRecovAlert = 51,
    /// csrng_fatal_alert
    CsrngFatalAlert = 52,
    /// entropy_src_recov_alert
    EntropySrcRecovAlert = 53,
    /// entropy_src_fatal_alert
    EntropySrcFatalAlert = 54,
    /// edn0_recov_alert
    Edn0RecovAlert = 55,
    /// edn0_fatal_alert
    Edn0FatalAlert = 56,
    /// edn1_recov_alert
    Edn1RecovAlert = 57,
    /// edn1_fatal_alert
    Edn1FatalAlert = 58,
    /// sram_ctrl_main_fatal_error
    SramCtrlMainFatalError = 59,
    /// rom_ctrl_fatal
    RomCtrlFatal = 60,
    /// rv_core_ibex_fatal_sw_err
    RvCoreIbexFatalSwErr = 61,
    /// rv_core_ibex_recov_sw_err
    RvCoreIbexRecovSwErr = 62,
    /// rv_core_ibex_fatal_hw_err
    RvCoreIbexFatalHwErr = 63,
    /// rv_core_ibex_recov_hw_err
    RvCoreIbexRecovHwErr = 64,
    /// \internal The number of Alert ID.
    End = 65,
}

/// PLIC Interrupt Source to Peripheral Map
///
/// This array is a mapping from `TopEarlgreyPlicIrqId` to
/// `TopEarlgreyPlicPeripheral`.
pub const TOP_EARLGREY_PLIC_INTERRUPT_FOR_PERIPHERAL: [TopEarlgreyPlicPeripheral; 185] = [
    // None -> TopEarlgreyPlicPeripheral::Unknown
    TopEarlgreyPlicPeripheral::Unknown,
    // Uart0TxWatermark -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxWatermark -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0TxEmpty -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxOverflow -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxFrameErr -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxBreakErr -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxTimeout -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart0RxParityErr -> TopEarlgreyPlicPeripheral::Uart0
    TopEarlgreyPlicPeripheral::Uart0,
    // Uart1TxWatermark -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxWatermark -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1TxEmpty -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxOverflow -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxFrameErr -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxBreakErr -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxTimeout -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart1RxParityErr -> TopEarlgreyPlicPeripheral::Uart1
    TopEarlgreyPlicPeripheral::Uart1,
    // Uart2TxWatermark -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxWatermark -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2TxEmpty -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxOverflow -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxFrameErr -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxBreakErr -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxTimeout -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart2RxParityErr -> TopEarlgreyPlicPeripheral::Uart2
    TopEarlgreyPlicPeripheral::Uart2,
    // Uart3TxWatermark -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxWatermark -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3TxEmpty -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxOverflow -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxFrameErr -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxBreakErr -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxTimeout -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // Uart3RxParityErr -> TopEarlgreyPlicPeripheral::Uart3
    TopEarlgreyPlicPeripheral::Uart3,
    // GpioGpio0 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio1 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio2 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio3 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio4 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio5 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio6 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio7 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio8 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio9 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio10 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio11 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio12 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio13 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio14 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio15 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio16 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio17 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio18 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio19 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio20 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio21 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio22 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio23 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio24 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio25 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio26 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio27 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio28 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio29 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio30 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // GpioGpio31 -> TopEarlgreyPlicPeripheral::Gpio
    TopEarlgreyPlicPeripheral::Gpio,
    // SpiDeviceGenericRxFull -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceGenericRxWatermark -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceGenericTxWatermark -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceGenericRxError -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceGenericRxOverflow -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceGenericTxUnderflow -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceUploadCmdfifoNotEmpty -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceUploadPayloadNotEmpty -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceUploadPayloadOverflow -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceReadbufWatermark -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceReadbufFlip -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // SpiDeviceTpmHeaderNotEmpty -> TopEarlgreyPlicPeripheral::SpiDevice
    TopEarlgreyPlicPeripheral::SpiDevice,
    // I2c0FmtThreshold -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0RxThreshold -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0FmtOverflow -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0RxOverflow -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0Nak -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0SclInterference -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0SdaInterference -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0StretchTimeout -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0SdaUnstable -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0CmdComplete -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0TxStretch -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0TxOverflow -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0AcqFull -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0UnexpStop -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c0HostTimeout -> TopEarlgreyPlicPeripheral::I2c0
    TopEarlgreyPlicPeripheral::I2c0,
    // I2c1FmtThreshold -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1RxThreshold -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1FmtOverflow -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1RxOverflow -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1Nak -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1SclInterference -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1SdaInterference -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1StretchTimeout -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1SdaUnstable -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1CmdComplete -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1TxStretch -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1TxOverflow -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1AcqFull -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1UnexpStop -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c1HostTimeout -> TopEarlgreyPlicPeripheral::I2c1
    TopEarlgreyPlicPeripheral::I2c1,
    // I2c2FmtThreshold -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2RxThreshold -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2FmtOverflow -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2RxOverflow -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2Nak -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2SclInterference -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2SdaInterference -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2StretchTimeout -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2SdaUnstable -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2CmdComplete -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2TxStretch -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2TxOverflow -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2AcqFull -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2UnexpStop -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // I2c2HostTimeout -> TopEarlgreyPlicPeripheral::I2c2
    TopEarlgreyPlicPeripheral::I2c2,
    // PattgenDoneCh0 -> TopEarlgreyPlicPeripheral::Pattgen
    TopEarlgreyPlicPeripheral::Pattgen,
    // PattgenDoneCh1 -> TopEarlgreyPlicPeripheral::Pattgen
    TopEarlgreyPlicPeripheral::Pattgen,
    // RvTimerTimerExpiredHart0Timer0 -> TopEarlgreyPlicPeripheral::RvTimer
    TopEarlgreyPlicPeripheral::RvTimer,
    // OtpCtrlOtpOperationDone -> TopEarlgreyPlicPeripheral::OtpCtrl
    TopEarlgreyPlicPeripheral::OtpCtrl,
    // OtpCtrlOtpError -> TopEarlgreyPlicPeripheral::OtpCtrl
    TopEarlgreyPlicPeripheral::OtpCtrl,
    // AlertHandlerClassa -> TopEarlgreyPlicPeripheral::AlertHandler
    TopEarlgreyPlicPeripheral::AlertHandler,
    // AlertHandlerClassb -> TopEarlgreyPlicPeripheral::AlertHandler
    TopEarlgreyPlicPeripheral::AlertHandler,
    // AlertHandlerClassc -> TopEarlgreyPlicPeripheral::AlertHandler
    TopEarlgreyPlicPeripheral::AlertHandler,
    // AlertHandlerClassd -> TopEarlgreyPlicPeripheral::AlertHandler
    TopEarlgreyPlicPeripheral::AlertHandler,
    // SpiHost0Error -> TopEarlgreyPlicPeripheral::SpiHost0
    TopEarlgreyPlicPeripheral::SpiHost0,
    // SpiHost0SpiEvent -> TopEarlgreyPlicPeripheral::SpiHost0
    TopEarlgreyPlicPeripheral::SpiHost0,
    // SpiHost1Error -> TopEarlgreyPlicPeripheral::SpiHost1
    TopEarlgreyPlicPeripheral::SpiHost1,
    // SpiHost1SpiEvent -> TopEarlgreyPlicPeripheral::SpiHost1
    TopEarlgreyPlicPeripheral::SpiHost1,
    // UsbdevPktReceived -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevPktSent -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevDisconnected -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevHostLost -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevLinkReset -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevLinkSuspend -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevLinkResume -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevAvEmpty -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevRxFull -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevAvOverflow -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevLinkInErr -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevRxCrcErr -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevRxPidErr -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevRxBitstuffErr -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevFrame -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevPowered -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // UsbdevLinkOutErr -> TopEarlgreyPlicPeripheral::Usbdev
    TopEarlgreyPlicPeripheral::Usbdev,
    // PwrmgrAonWakeup -> TopEarlgreyPlicPeripheral::PwrmgrAon
    TopEarlgreyPlicPeripheral::PwrmgrAon,
    // SysrstCtrlAonEventDetected -> TopEarlgreyPlicPeripheral::SysrstCtrlAon
    TopEarlgreyPlicPeripheral::SysrstCtrlAon,
    // AdcCtrlAonMatchDone -> TopEarlgreyPlicPeripheral::AdcCtrlAon
    TopEarlgreyPlicPeripheral::AdcCtrlAon,
    // AonTimerAonWkupTimerExpired -> TopEarlgreyPlicPeripheral::AonTimerAon
    TopEarlgreyPlicPeripheral::AonTimerAon,
    // AonTimerAonWdogTimerBark -> TopEarlgreyPlicPeripheral::AonTimerAon
    TopEarlgreyPlicPeripheral::AonTimerAon,
    // SensorCtrlIoStatusChange -> TopEarlgreyPlicPeripheral::SensorCtrl
    TopEarlgreyPlicPeripheral::SensorCtrl,
    // SensorCtrlInitStatusChange -> TopEarlgreyPlicPeripheral::SensorCtrl
    TopEarlgreyPlicPeripheral::SensorCtrl,
    // FlashCtrlProgEmpty -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // FlashCtrlProgLvl -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // FlashCtrlRdFull -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // FlashCtrlRdLvl -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // FlashCtrlOpDone -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // FlashCtrlCorrErr -> TopEarlgreyPlicPeripheral::FlashCtrl
    TopEarlgreyPlicPeripheral::FlashCtrl,
    // HmacHmacDone -> TopEarlgreyPlicPeripheral::Hmac
    TopEarlgreyPlicPeripheral::Hmac,
    // HmacFifoEmpty -> TopEarlgreyPlicPeripheral::Hmac
    TopEarlgreyPlicPeripheral::Hmac,
    // HmacHmacErr -> TopEarlgreyPlicPeripheral::Hmac
    TopEarlgreyPlicPeripheral::Hmac,
    // KmacKmacDone -> TopEarlgreyPlicPeripheral::Kmac
    TopEarlgreyPlicPeripheral::Kmac,
    // KmacFifoEmpty -> TopEarlgreyPlicPeripheral::Kmac
    TopEarlgreyPlicPeripheral::Kmac,
    // KmacKmacErr -> TopEarlgreyPlicPeripheral::Kmac
    TopEarlgreyPlicPeripheral::Kmac,
    // OtbnDone -> TopEarlgreyPlicPeripheral::Otbn
    TopEarlgreyPlicPeripheral::Otbn,
    // KeymgrOpDone -> TopEarlgreyPlicPeripheral::Keymgr
    TopEarlgreyPlicPeripheral::Keymgr,
    // CsrngCsCmdReqDone -> TopEarlgreyPlicPeripheral::Csrng
    TopEarlgreyPlicPeripheral::Csrng,
    // CsrngCsEntropyReq -> TopEarlgreyPlicPeripheral::Csrng
    TopEarlgreyPlicPeripheral::Csrng,
    // CsrngCsHwInstExc -> TopEarlgreyPlicPeripheral::Csrng
    TopEarlgreyPlicPeripheral::Csrng,
    // CsrngCsFatalErr -> TopEarlgreyPlicPeripheral::Csrng
    TopEarlgreyPlicPeripheral::Csrng,
    // EntropySrcEsEntropyValid -> TopEarlgreyPlicPeripheral::EntropySrc
    TopEarlgreyPlicPeripheral::EntropySrc,
    // EntropySrcEsHealthTestFailed -> TopEarlgreyPlicPeripheral::EntropySrc
    TopEarlgreyPlicPeripheral::EntropySrc,
    // EntropySrcEsObserveFifoReady -> TopEarlgreyPlicPeripheral::EntropySrc
    TopEarlgreyPlicPeripheral::EntropySrc,
    // EntropySrcEsFatalErr -> TopEarlgreyPlicPeripheral::EntropySrc
    TopEarlgreyPlicPeripheral::EntropySrc,
    // Edn0EdnCmdReqDone -> TopEarlgreyPlicPeripheral::Edn0
    TopEarlgreyPlicPeripheral::Edn0,
    // Edn0EdnFatalErr -> TopEarlgreyPlicPeripheral::Edn0
    TopEarlgreyPlicPeripheral::Edn0,
    // Edn1EdnCmdReqDone -> TopEarlgreyPlicPeripheral::Edn1
    TopEarlgreyPlicPeripheral::Edn1,
    // Edn1EdnFatalErr -> TopEarlgreyPlicPeripheral::Edn1
    TopEarlgreyPlicPeripheral::Edn1,
];

/// Alert Handler Alert Source to Peripheral Map
///
/// This array is a mapping from `TopEarlgreyAlertId` to
/// `TopEarlgreyAlertPeripheral`.
pub const TOP_EARLGREY_ALERT_FOR_PERIPHERAL: [TopEarlgreyAlertPeripheral; 65] = [
    // Uart0FatalFault -> TopEarlgreyAlertPeripheral::Uart0
    TopEarlgreyAlertPeripheral::Uart0,
    // Uart1FatalFault -> TopEarlgreyAlertPeripheral::Uart1
    TopEarlgreyAlertPeripheral::Uart1,
    // Uart2FatalFault -> TopEarlgreyAlertPeripheral::Uart2
    TopEarlgreyAlertPeripheral::Uart2,
    // Uart3FatalFault -> TopEarlgreyAlertPeripheral::Uart3
    TopEarlgreyAlertPeripheral::Uart3,
    // GpioFatalFault -> TopEarlgreyAlertPeripheral::Gpio
    TopEarlgreyAlertPeripheral::Gpio,
    // SpiDeviceFatalFault -> TopEarlgreyAlertPeripheral::SpiDevice
    TopEarlgreyAlertPeripheral::SpiDevice,
    // I2c0FatalFault -> TopEarlgreyAlertPeripheral::I2c0
    TopEarlgreyAlertPeripheral::I2c0,
    // I2c1FatalFault -> TopEarlgreyAlertPeripheral::I2c1
    TopEarlgreyAlertPeripheral::I2c1,
    // I2c2FatalFault -> TopEarlgreyAlertPeripheral::I2c2
    TopEarlgreyAlertPeripheral::I2c2,
    // PattgenFatalFault -> TopEarlgreyAlertPeripheral::Pattgen
    TopEarlgreyAlertPeripheral::Pattgen,
    // RvTimerFatalFault -> TopEarlgreyAlertPeripheral::RvTimer
    TopEarlgreyAlertPeripheral::RvTimer,
    // OtpCtrlFatalMacroError -> TopEarlgreyAlertPeripheral::OtpCtrl
    TopEarlgreyAlertPeripheral::OtpCtrl,
    // OtpCtrlFatalCheckError -> TopEarlgreyAlertPeripheral::OtpCtrl
    TopEarlgreyAlertPeripheral::OtpCtrl,
    // OtpCtrlFatalBusIntegError -> TopEarlgreyAlertPeripheral::OtpCtrl
    TopEarlgreyAlertPeripheral::OtpCtrl,
    // OtpCtrlFatalPrimOtpAlert -> TopEarlgreyAlertPeripheral::OtpCtrl
    TopEarlgreyAlertPeripheral::OtpCtrl,
    // OtpCtrlRecovPrimOtpAlert -> TopEarlgreyAlertPeripheral::OtpCtrl
    TopEarlgreyAlertPeripheral::OtpCtrl,
    // LcCtrlFatalProgError -> TopEarlgreyAlertPeripheral::LcCtrl
    TopEarlgreyAlertPeripheral::LcCtrl,
    // LcCtrlFatalStateError -> TopEarlgreyAlertPeripheral::LcCtrl
    TopEarlgreyAlertPeripheral::LcCtrl,
    // LcCtrlFatalBusIntegError -> TopEarlgreyAlertPeripheral::LcCtrl
    TopEarlgreyAlertPeripheral::LcCtrl,
    // SpiHost0FatalFault -> TopEarlgreyAlertPeripheral::SpiHost0
    TopEarlgreyAlertPeripheral::SpiHost0,
    // SpiHost1FatalFault -> TopEarlgreyAlertPeripheral::SpiHost1
    TopEarlgreyAlertPeripheral::SpiHost1,
    // UsbdevFatalFault -> TopEarlgreyAlertPeripheral::Usbdev
    TopEarlgreyAlertPeripheral::Usbdev,
    // PwrmgrAonFatalFault -> TopEarlgreyAlertPeripheral::PwrmgrAon
    TopEarlgreyAlertPeripheral::PwrmgrAon,
    // RstmgrAonFatalFault -> TopEarlgreyAlertPeripheral::RstmgrAon
    TopEarlgreyAlertPeripheral::RstmgrAon,
    // RstmgrAonFatalCnstyFault -> TopEarlgreyAlertPeripheral::RstmgrAon
    TopEarlgreyAlertPeripheral::RstmgrAon,
    // ClkmgrAonRecovFault -> TopEarlgreyAlertPeripheral::ClkmgrAon
    TopEarlgreyAlertPeripheral::ClkmgrAon,
    // ClkmgrAonFatalFault -> TopEarlgreyAlertPeripheral::ClkmgrAon
    TopEarlgreyAlertPeripheral::ClkmgrAon,
    // SysrstCtrlAonFatalFault -> TopEarlgreyAlertPeripheral::SysrstCtrlAon
    TopEarlgreyAlertPeripheral::SysrstCtrlAon,
    // AdcCtrlAonFatalFault -> TopEarlgreyAlertPeripheral::AdcCtrlAon
    TopEarlgreyAlertPeripheral::AdcCtrlAon,
    // PwmAonFatalFault -> TopEarlgreyAlertPeripheral::PwmAon
    TopEarlgreyAlertPeripheral::PwmAon,
    // PinmuxAonFatalFault -> TopEarlgreyAlertPeripheral::PinmuxAon
    TopEarlgreyAlertPeripheral::PinmuxAon,
    // AonTimerAonFatalFault -> TopEarlgreyAlertPeripheral::AonTimerAon
    TopEarlgreyAlertPeripheral::AonTimerAon,
    // SensorCtrlRecovAlert -> TopEarlgreyAlertPeripheral::SensorCtrl
    TopEarlgreyAlertPeripheral::SensorCtrl,
    // SensorCtrlFatalAlert -> TopEarlgreyAlertPeripheral::SensorCtrl
    TopEarlgreyAlertPeripheral::SensorCtrl,
    // SramCtrlRetAonFatalError -> TopEarlgreyAlertPeripheral::SramCtrlRetAon
    TopEarlgreyAlertPeripheral::SramCtrlRetAon,
    // FlashCtrlRecovErr -> TopEarlgreyAlertPeripheral::FlashCtrl
    TopEarlgreyAlertPeripheral::FlashCtrl,
    // FlashCtrlFatalStdErr -> TopEarlgreyAlertPeripheral::FlashCtrl
    TopEarlgreyAlertPeripheral::FlashCtrl,
    // FlashCtrlFatalErr -> TopEarlgreyAlertPeripheral::FlashCtrl
    TopEarlgreyAlertPeripheral::FlashCtrl,
    // FlashCtrlFatalPrimFlashAlert -> TopEarlgreyAlertPeripheral::FlashCtrl
    TopEarlgreyAlertPeripheral::FlashCtrl,
    // FlashCtrlRecovPrimFlashAlert -> TopEarlgreyAlertPeripheral::FlashCtrl
    TopEarlgreyAlertPeripheral::FlashCtrl,
    // RvDmFatalFault -> TopEarlgreyAlertPeripheral::RvDm
    TopEarlgreyAlertPeripheral::RvDm,
    // RvPlicFatalFault -> TopEarlgreyAlertPeripheral::RvPlic
    TopEarlgreyAlertPeripheral::RvPlic,
    // AesRecovCtrlUpdateErr -> TopEarlgreyAlertPeripheral::Aes
    TopEarlgreyAlertPeripheral::Aes,
    // AesFatalFault -> TopEarlgreyAlertPeripheral::Aes
    TopEarlgreyAlertPeripheral::Aes,
    // HmacFatalFault -> TopEarlgreyAlertPeripheral::Hmac
    TopEarlgreyAlertPeripheral::Hmac,
    // KmacRecovOperationErr -> TopEarlgreyAlertPeripheral::Kmac
    TopEarlgreyAlertPeripheral::Kmac,
    // KmacFatalFaultErr -> TopEarlgreyAlertPeripheral::Kmac
    TopEarlgreyAlertPeripheral::Kmac,
    // OtbnFatal -> TopEarlgreyAlertPeripheral::Otbn
    TopEarlgreyAlertPeripheral::Otbn,
    // OtbnRecov -> TopEarlgreyAlertPeripheral::Otbn
    TopEarlgreyAlertPeripheral::Otbn,
    // KeymgrRecovOperationErr -> TopEarlgreyAlertPeripheral::Keymgr
    TopEarlgreyAlertPeripheral::Keymgr,
    // KeymgrFatalFaultErr -> TopEarlgreyAlertPeripheral::Keymgr
    TopEarlgreyAlertPeripheral::Keymgr,
    // CsrngRecovAlert -> TopEarlgreyAlertPeripheral::Csrng
    TopEarlgreyAlertPeripheral::Csrng,
    // CsrngFatalAlert -> TopEarlgreyAlertPeripheral::Csrng
    TopEarlgreyAlertPeripheral::Csrng,
    // EntropySrcRecovAlert -> TopEarlgreyAlertPeripheral::EntropySrc
    TopEarlgreyAlertPeripheral::EntropySrc,
    // EntropySrcFatalAlert -> TopEarlgreyAlertPeripheral::EntropySrc
    TopEarlgreyAlertPeripheral::EntropySrc,
    // Edn0RecovAlert -> TopEarlgreyAlertPeripheral::Edn0
    TopEarlgreyAlertPeripheral::Edn0,
    // Edn0FatalAlert -> TopEarlgreyAlertPeripheral::Edn0
    TopEarlgreyAlertPeripheral::Edn0,
    // Edn1RecovAlert -> TopEarlgreyAlertPeripheral::Edn1
    TopEarlgreyAlertPeripheral::Edn1,
    // Edn1FatalAlert -> TopEarlgreyAlertPeripheral::Edn1
    TopEarlgreyAlertPeripheral::Edn1,
    // SramCtrlMainFatalError -> TopEarlgreyAlertPeripheral::SramCtrlMain
    TopEarlgreyAlertPeripheral::SramCtrlMain,
    // RomCtrlFatal -> TopEarlgreyAlertPeripheral::RomCtrl
    TopEarlgreyAlertPeripheral::RomCtrl,
    // RvCoreIbexFatalSwErr -> TopEarlgreyAlertPeripheral::RvCoreIbex
    TopEarlgreyAlertPeripheral::RvCoreIbex,
    // RvCoreIbexRecovSwErr -> TopEarlgreyAlertPeripheral::RvCoreIbex
    TopEarlgreyAlertPeripheral::RvCoreIbex,
    // RvCoreIbexFatalHwErr -> TopEarlgreyAlertPeripheral::RvCoreIbex
    TopEarlgreyAlertPeripheral::RvCoreIbex,
    // RvCoreIbexRecovHwErr -> TopEarlgreyAlertPeripheral::RvCoreIbex
    TopEarlgreyAlertPeripheral::RvCoreIbex,
];

// PERIPH_INSEL ranges from 0 to NUM_MIO_PADS + 2 -1}
//  0 and 1 are tied to value 0 and 1
pub const NUM_MIO_PADS: usize = 47;
pub const NUM_DIO_PADS: usize = 16;

pub const PINMUX_MIO_PERIPH_INSEL_IDX_OFFSET: usize = 2;
pub const PINMUX_PERIPH_OUTSEL_IDX_OFFSET: usize = 3;

/// Pinmux Peripheral Input.
#[repr(u32)]
pub enum TopEarlgreyPinmuxPeripheralIn {
    /// Peripheral Input 0
    GpioGpio0 = 0,
    /// Peripheral Input 1
    GpioGpio1 = 1,
    /// Peripheral Input 2
    GpioGpio2 = 2,
    /// Peripheral Input 3
    GpioGpio3 = 3,
    /// Peripheral Input 4
    GpioGpio4 = 4,
    /// Peripheral Input 5
    GpioGpio5 = 5,
    /// Peripheral Input 6
    GpioGpio6 = 6,
    /// Peripheral Input 7
    GpioGpio7 = 7,
    /// Peripheral Input 8
    GpioGpio8 = 8,
    /// Peripheral Input 9
    GpioGpio9 = 9,
    /// Peripheral Input 10
    GpioGpio10 = 10,
    /// Peripheral Input 11
    GpioGpio11 = 11,
    /// Peripheral Input 12
    GpioGpio12 = 12,
    /// Peripheral Input 13
    GpioGpio13 = 13,
    /// Peripheral Input 14
    GpioGpio14 = 14,
    /// Peripheral Input 15
    GpioGpio15 = 15,
    /// Peripheral Input 16
    GpioGpio16 = 16,
    /// Peripheral Input 17
    GpioGpio17 = 17,
    /// Peripheral Input 18
    GpioGpio18 = 18,
    /// Peripheral Input 19
    GpioGpio19 = 19,
    /// Peripheral Input 20
    GpioGpio20 = 20,
    /// Peripheral Input 21
    GpioGpio21 = 21,
    /// Peripheral Input 22
    GpioGpio22 = 22,
    /// Peripheral Input 23
    GpioGpio23 = 23,
    /// Peripheral Input 24
    GpioGpio24 = 24,
    /// Peripheral Input 25
    GpioGpio25 = 25,
    /// Peripheral Input 26
    GpioGpio26 = 26,
    /// Peripheral Input 27
    GpioGpio27 = 27,
    /// Peripheral Input 28
    GpioGpio28 = 28,
    /// Peripheral Input 29
    GpioGpio29 = 29,
    /// Peripheral Input 30
    GpioGpio30 = 30,
    /// Peripheral Input 31
    GpioGpio31 = 31,
    /// Peripheral Input 32
    I2c0Sda = 32,
    /// Peripheral Input 33
    I2c0Scl = 33,
    /// Peripheral Input 34
    I2c1Sda = 34,
    /// Peripheral Input 35
    I2c1Scl = 35,
    /// Peripheral Input 36
    I2c2Sda = 36,
    /// Peripheral Input 37
    I2c2Scl = 37,
    /// Peripheral Input 38
    SpiHost1Sd0 = 38,
    /// Peripheral Input 39
    SpiHost1Sd1 = 39,
    /// Peripheral Input 40
    SpiHost1Sd2 = 40,
    /// Peripheral Input 41
    SpiHost1Sd3 = 41,
    /// Peripheral Input 42
    Uart0Rx = 42,
    /// Peripheral Input 43
    Uart1Rx = 43,
    /// Peripheral Input 44
    Uart2Rx = 44,
    /// Peripheral Input 45
    Uart3Rx = 45,
    /// Peripheral Input 46
    SpiDeviceTpmCsb = 46,
    /// Peripheral Input 47
    FlashCtrlTck = 47,
    /// Peripheral Input 48
    FlashCtrlTms = 48,
    /// Peripheral Input 49
    FlashCtrlTdi = 49,
    /// Peripheral Input 50
    SysrstCtrlAonAcPresent = 50,
    /// Peripheral Input 51
    SysrstCtrlAonKey0In = 51,
    /// Peripheral Input 52
    SysrstCtrlAonKey1In = 52,
    /// Peripheral Input 53
    SysrstCtrlAonKey2In = 53,
    /// Peripheral Input 54
    SysrstCtrlAonPwrbIn = 54,
    /// Peripheral Input 55
    SysrstCtrlAonLidOpen = 55,
    /// Peripheral Input 56
    UsbdevSense = 56,
    /// \internal Number of peripheral input
    End = 57,
}

/// Pinmux MIO Input Selector.
#[repr(u32)]
pub enum TopEarlgreyPinmuxInsel {
    /// Tie constantly to zero
    ConstantZero = 0,
    /// Tie constantly to one
    ConstantOne = 1,
    /// MIO Pad 0
    Ioa0 = 2,
    /// MIO Pad 1
    Ioa1 = 3,
    /// MIO Pad 2
    Ioa2 = 4,
    /// MIO Pad 3
    Ioa3 = 5,
    /// MIO Pad 4
    Ioa4 = 6,
    /// MIO Pad 5
    Ioa5 = 7,
    /// MIO Pad 6
    Ioa6 = 8,
    /// MIO Pad 7
    Ioa7 = 9,
    /// MIO Pad 8
    Ioa8 = 10,
    /// MIO Pad 9
    Iob0 = 11,
    /// MIO Pad 10
    Iob1 = 12,
    /// MIO Pad 11
    Iob2 = 13,
    /// MIO Pad 12
    Iob3 = 14,
    /// MIO Pad 13
    Iob4 = 15,
    /// MIO Pad 14
    Iob5 = 16,
    /// MIO Pad 15
    Iob6 = 17,
    /// MIO Pad 16
    Iob7 = 18,
    /// MIO Pad 17
    Iob8 = 19,
    /// MIO Pad 18
    Iob9 = 20,
    /// MIO Pad 19
    Iob10 = 21,
    /// MIO Pad 20
    Iob11 = 22,
    /// MIO Pad 21
    Iob12 = 23,
    /// MIO Pad 22
    Ioc0 = 24,
    /// MIO Pad 23
    Ioc1 = 25,
    /// MIO Pad 24
    Ioc2 = 26,
    /// MIO Pad 25
    Ioc3 = 27,
    /// MIO Pad 26
    Ioc4 = 28,
    /// MIO Pad 27
    Ioc5 = 29,
    /// MIO Pad 28
    Ioc6 = 30,
    /// MIO Pad 29
    Ioc7 = 31,
    /// MIO Pad 30
    Ioc8 = 32,
    /// MIO Pad 31
    Ioc9 = 33,
    /// MIO Pad 32
    Ioc10 = 34,
    /// MIO Pad 33
    Ioc11 = 35,
    /// MIO Pad 34
    Ioc12 = 36,
    /// MIO Pad 35
    Ior0 = 37,
    /// MIO Pad 36
    Ior1 = 38,
    /// MIO Pad 37
    Ior2 = 39,
    /// MIO Pad 38
    Ior3 = 40,
    /// MIO Pad 39
    Ior4 = 41,
    /// MIO Pad 40
    Ior5 = 42,
    /// MIO Pad 41
    Ior6 = 43,
    /// MIO Pad 42
    Ior7 = 44,
    /// MIO Pad 43
    Ior10 = 45,
    /// MIO Pad 44
    Ior11 = 46,
    /// MIO Pad 45
    Ior12 = 47,
    /// MIO Pad 46
    Ior13 = 48,
    /// \internal Number of valid insel value
    End = 49,
}

/// Pinmux MIO Output.
pub enum TopEarlgreyPinmuxMioOut {
    /// MIO Pad 0
    Ioa0 = 0,
    /// MIO Pad 1
    Ioa1 = 1,
    /// MIO Pad 2
    Ioa2 = 2,
    /// MIO Pad 3
    Ioa3 = 3,
    /// MIO Pad 4
    Ioa4 = 4,
    /// MIO Pad 5
    Ioa5 = 5,
    /// MIO Pad 6
    Ioa6 = 6,
    /// MIO Pad 7
    Ioa7 = 7,
    /// MIO Pad 8
    Ioa8 = 8,
    /// MIO Pad 9
    Iob0 = 9,
    /// MIO Pad 10
    Iob1 = 10,
    /// MIO Pad 11
    Iob2 = 11,
    /// MIO Pad 12
    Iob3 = 12,
    /// MIO Pad 13
    Iob4 = 13,
    /// MIO Pad 14
    Iob5 = 14,
    /// MIO Pad 15
    Iob6 = 15,
    /// MIO Pad 16
    Iob7 = 16,
    /// MIO Pad 17
    Iob8 = 17,
    /// MIO Pad 18
    Iob9 = 18,
    /// MIO Pad 19
    Iob10 = 19,
    /// MIO Pad 20
    Iob11 = 20,
    /// MIO Pad 21
    Iob12 = 21,
    /// MIO Pad 22
    Ioc0 = 22,
    /// MIO Pad 23
    Ioc1 = 23,
    /// MIO Pad 24
    Ioc2 = 24,
    /// MIO Pad 25
    Ioc3 = 25,
    /// MIO Pad 26
    Ioc4 = 26,
    /// MIO Pad 27
    Ioc5 = 27,
    /// MIO Pad 28
    Ioc6 = 28,
    /// MIO Pad 29
    Ioc7 = 29,
    /// MIO Pad 30
    Ioc8 = 30,
    /// MIO Pad 31
    Ioc9 = 31,
    /// MIO Pad 32
    Ioc10 = 32,
    /// MIO Pad 33
    Ioc11 = 33,
    /// MIO Pad 34
    Ioc12 = 34,
    /// MIO Pad 35
    Ior0 = 35,
    /// MIO Pad 36
    Ior1 = 36,
    /// MIO Pad 37
    Ior2 = 37,
    /// MIO Pad 38
    Ior3 = 38,
    /// MIO Pad 39
    Ior4 = 39,
    /// MIO Pad 40
    Ior5 = 40,
    /// MIO Pad 41
    Ior6 = 41,
    /// MIO Pad 42
    Ior7 = 42,
    /// MIO Pad 43
    Ior10 = 43,
    /// MIO Pad 44
    Ior11 = 44,
    /// MIO Pad 45
    Ior12 = 45,
    /// MIO Pad 46
    Ior13 = 46,
    /// \internal Number of valid mio output
    End = 47,
}

/// Pinmux Peripheral Output Selector.
#[repr(u32)]
pub enum TopEarlgreyPinmuxOutsel {
    /// Tie constantly to zero
    ConstantZero = 0,
    /// Tie constantly to one
    ConstantOne = 1,
    /// Tie constantly to high-Z
    ConstantHighZ = 2,
    /// Peripheral Output 0
    GpioGpio0 = 3,
    /// Peripheral Output 1
    GpioGpio1 = 4,
    /// Peripheral Output 2
    GpioGpio2 = 5,
    /// Peripheral Output 3
    GpioGpio3 = 6,
    /// Peripheral Output 4
    GpioGpio4 = 7,
    /// Peripheral Output 5
    GpioGpio5 = 8,
    /// Peripheral Output 6
    GpioGpio6 = 9,
    /// Peripheral Output 7
    GpioGpio7 = 10,
    /// Peripheral Output 8
    GpioGpio8 = 11,
    /// Peripheral Output 9
    GpioGpio9 = 12,
    /// Peripheral Output 10
    GpioGpio10 = 13,
    /// Peripheral Output 11
    GpioGpio11 = 14,
    /// Peripheral Output 12
    GpioGpio12 = 15,
    /// Peripheral Output 13
    GpioGpio13 = 16,
    /// Peripheral Output 14
    GpioGpio14 = 17,
    /// Peripheral Output 15
    GpioGpio15 = 18,
    /// Peripheral Output 16
    GpioGpio16 = 19,
    /// Peripheral Output 17
    GpioGpio17 = 20,
    /// Peripheral Output 18
    GpioGpio18 = 21,
    /// Peripheral Output 19
    GpioGpio19 = 22,
    /// Peripheral Output 20
    GpioGpio20 = 23,
    /// Peripheral Output 21
    GpioGpio21 = 24,
    /// Peripheral Output 22
    GpioGpio22 = 25,
    /// Peripheral Output 23
    GpioGpio23 = 26,
    /// Peripheral Output 24
    GpioGpio24 = 27,
    /// Peripheral Output 25
    GpioGpio25 = 28,
    /// Peripheral Output 26
    GpioGpio26 = 29,
    /// Peripheral Output 27
    GpioGpio27 = 30,
    /// Peripheral Output 28
    GpioGpio28 = 31,
    /// Peripheral Output 29
    GpioGpio29 = 32,
    /// Peripheral Output 30
    GpioGpio30 = 33,
    /// Peripheral Output 31
    GpioGpio31 = 34,
    /// Peripheral Output 32
    I2c0Sda = 35,
    /// Peripheral Output 33
    I2c0Scl = 36,
    /// Peripheral Output 34
    I2c1Sda = 37,
    /// Peripheral Output 35
    I2c1Scl = 38,
    /// Peripheral Output 36
    I2c2Sda = 39,
    /// Peripheral Output 37
    I2c2Scl = 40,
    /// Peripheral Output 38
    SpiHost1Sd0 = 41,
    /// Peripheral Output 39
    SpiHost1Sd1 = 42,
    /// Peripheral Output 40
    SpiHost1Sd2 = 43,
    /// Peripheral Output 41
    SpiHost1Sd3 = 44,
    /// Peripheral Output 42
    Uart0Tx = 45,
    /// Peripheral Output 43
    Uart1Tx = 46,
    /// Peripheral Output 44
    Uart2Tx = 47,
    /// Peripheral Output 45
    Uart3Tx = 48,
    /// Peripheral Output 46
    PattgenPda0Tx = 49,
    /// Peripheral Output 47
    PattgenPcl0Tx = 50,
    /// Peripheral Output 48
    PattgenPda1Tx = 51,
    /// Peripheral Output 49
    PattgenPcl1Tx = 52,
    /// Peripheral Output 50
    SpiHost1Sck = 53,
    /// Peripheral Output 51
    SpiHost1Csb = 54,
    /// Peripheral Output 52
    FlashCtrlTdo = 55,
    /// Peripheral Output 53
    SensorCtrlAstDebugOut0 = 56,
    /// Peripheral Output 54
    SensorCtrlAstDebugOut1 = 57,
    /// Peripheral Output 55
    SensorCtrlAstDebugOut2 = 58,
    /// Peripheral Output 56
    SensorCtrlAstDebugOut3 = 59,
    /// Peripheral Output 57
    SensorCtrlAstDebugOut4 = 60,
    /// Peripheral Output 58
    SensorCtrlAstDebugOut5 = 61,
    /// Peripheral Output 59
    SensorCtrlAstDebugOut6 = 62,
    /// Peripheral Output 60
    SensorCtrlAstDebugOut7 = 63,
    /// Peripheral Output 61
    SensorCtrlAstDebugOut8 = 64,
    /// Peripheral Output 62
    PwmAonPwm0 = 65,
    /// Peripheral Output 63
    PwmAonPwm1 = 66,
    /// Peripheral Output 64
    PwmAonPwm2 = 67,
    /// Peripheral Output 65
    PwmAonPwm3 = 68,
    /// Peripheral Output 66
    PwmAonPwm4 = 69,
    /// Peripheral Output 67
    PwmAonPwm5 = 70,
    /// Peripheral Output 68
    OtpCtrlTest0 = 71,
    /// Peripheral Output 69
    SysrstCtrlAonBatDisable = 72,
    /// Peripheral Output 70
    SysrstCtrlAonKey0Out = 73,
    /// Peripheral Output 71
    SysrstCtrlAonKey1Out = 74,
    /// Peripheral Output 72
    SysrstCtrlAonKey2Out = 75,
    /// Peripheral Output 73
    SysrstCtrlAonPwrbOut = 76,
    /// Peripheral Output 74
    SysrstCtrlAonZ3Wakeup = 77,
    /// \internal Number of valid outsel value
    End = 78,
}

/// Dedicated Pad Selects
pub enum TopEarlgreyDirectPads {
    UsbdevUsbDp = 0,
    UsbdevUsbDn = 1,
    SpiHost0Sd0 = 2,
    SpiHost0Sd1 = 3,
    SpiHost0Sd2 = 4,
    SpiHost0Sd3 = 5,
    SpiDeviceSd0 = 6,
    SpiDeviceSd1 = 7,
    SpiDeviceSd2 = 8,
    SpiDeviceSd3 = 9,
    SysrstCtrlAonEcRstL = 10,
    SysrstCtrlAonFlashWpL = 11,
    SpiDeviceSck = 12,
    SpiDeviceCsb = 13,
    SpiHost0Sck = 14,
    SpiHost0Csb = 15,
    /// \internal Number of valid direct pad
    End = 16,
}

/// Muxed Pad Selects
pub enum TopEarlgreyMuxedPads {
    Ioa0 = 0,
    Ioa1 = 1,
    Ioa2 = 2,
    Ioa3 = 3,
    Ioa4 = 4,
    Ioa5 = 5,
    Ioa6 = 6,
    Ioa7 = 7,
    Ioa8 = 8,
    Iob0 = 9,
    Iob1 = 10,
    Iob2 = 11,
    Iob3 = 12,
    Iob4 = 13,
    Iob5 = 14,
    Iob6 = 15,
    Iob7 = 16,
    Iob8 = 17,
    Iob9 = 18,
    Iob10 = 19,
    Iob11 = 20,
    Iob12 = 21,
    Ioc0 = 22,
    Ioc1 = 23,
    Ioc2 = 24,
    Ioc3 = 25,
    Ioc4 = 26,
    Ioc5 = 27,
    Ioc6 = 28,
    Ioc7 = 29,
    Ioc8 = 30,
    Ioc9 = 31,
    Ioc10 = 32,
    Ioc11 = 33,
    Ioc12 = 34,
    Ior0 = 35,
    Ior1 = 36,
    Ior2 = 37,
    Ior3 = 38,
    Ior4 = 39,
    Ior5 = 40,
    Ior6 = 41,
    Ior7 = 42,
    Ior10 = 43,
    Ior11 = 44,
    Ior12 = 45,
    Ior13 = 46,
    /// \internal Number of valid muxed pad
    End = 47,
}

/// Power Manager Wakeup Signals
pub enum TopEarlgreyPowerManagerWakeUps {
    SysrstCtrlAonWkupReq = 0,
    AdcCtrlAonWkupReq = 1,
    PinmuxAonPinWkupReq = 2,
    PinmuxAonUsbWkupReq = 3,
    AonTimerAonWkupReq = 4,
    SensorCtrlWkupReq = 5,
    /// \internal Number of valid pwrmgr wakeup signal
    End = 6,
}

/// Reset Manager Software Controlled Resets
pub enum TopEarlgreyResetManagerSwResets {
    SpiDevice = 0,
    SpiHost0 = 1,
    SpiHost1 = 2,
    Usb = 3,
    UsbAon = 4,
    I2c0 = 5,
    I2c1 = 6,
    I2c2 = 7,
    /// \internal Number of valid rstmgr software reset request
    End = 8,
}

/// Power Manager Reset Request Signals
pub enum TopEarlgreyPowerManagerResetRequests {
    SysrstCtrlAonRstReq = 0,
    AonTimerAonAonTimerRstReq = 1,
    /// \internal Number of valid pwrmgr reset_request signal
    End = 2,
}

/// Clock Manager Software-Controlled ("Gated") Clocks.
///
/// The Software has full control over these clocks.
pub enum TopEarlgreyGateableClocks {
    /// Clock clk_io_div4_peri in group peri
    IoDiv4Peri = 0,
    /// Clock clk_io_div2_peri in group peri
    IoDiv2Peri = 1,
    /// Clock clk_io_peri in group peri
    IoPeri = 2,
    /// Clock clk_usb_peri in group peri
    UsbPeri = 3,
    /// \internal Number of Valid Gateable Clock
    End = 4,
}

/// Clock Manager Software-Hinted Clocks.
///
/// The Software has partial control over these clocks. It can ask them to stop,
/// but the clock manager is in control of whether the clock actually is stopped.
pub enum TopEarlgreyHintableClocks {
    /// Clock clk_main_aes in group trans
    MainAes = 0,
    /// Clock clk_main_hmac in group trans
    MainHmac = 1,
    /// Clock clk_main_kmac in group trans
    MainKmac = 2,
    /// Clock clk_main_otbn in group trans
    MainOtbn = 3,
    /// \internal Number of Valid Hintable Clock
    End = 4,
}

/// MMIO Region
///
/// MMIO region excludes any memory that is separate from the module
/// configuration space, i.e. ROM, main SRAM, and flash are excluded but
/// retention SRAM, spi_device memory, or usbdev memory are included.
pub const TOP_EARLGREY_MMIO_BASE_ADDR: usize = 0x40000000;
pub const TOP_EARLGREY_MMIO_SIZE_BYTES: usize = 0x10000000;