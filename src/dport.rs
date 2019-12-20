#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DPORT_PRO_BOOT_REMAP_CTRL_REG"]
    pub dport_pro_boot_remap_ctrl_reg: DPORT_PRO_BOOT_REMAP_CTRL_REG,
    #[doc = "0x04 - DPORT_APP_BOOT_REMAP_CTRL_REG"]
    pub dport_app_boot_remap_ctrl_reg: DPORT_APP_BOOT_REMAP_CTRL_REG,
    #[doc = "0x08 - DPORT_ACCESS_CHECK_REG"]
    pub dport_access_check_reg: DPORT_ACCESS_CHECK_REG,
    #[doc = "0x0c - DPORT_PRO_DPORT_APB_MASK0_REG"]
    pub dport_pro_dport_apb_mask0_reg: DPORT_PRO_DPORT_APB_MASK0_REG,
    #[doc = "0x10 - DPORT_PRO_DPORT_APB_MASK1_REG"]
    pub dport_pro_dport_apb_mask1_reg: DPORT_PRO_DPORT_APB_MASK1_REG,
    #[doc = "0x14 - DPORT_APP_DPORT_APB_MASK0_REG"]
    pub dport_app_dport_apb_mask0_reg: DPORT_APP_DPORT_APB_MASK0_REG,
    #[doc = "0x18 - DPORT_APP_DPORT_APB_MASK1_REG"]
    pub dport_app_dport_apb_mask1_reg: DPORT_APP_DPORT_APB_MASK1_REG,
    #[doc = "0x1c - DPORT_PERI_CLK_EN_REG"]
    pub dport_peri_clk_en_reg: DPORT_PERI_CLK_EN_REG,
    #[doc = "0x20 - DPORT_PERI_RST_EN_REG"]
    pub dport_peri_rst_en_reg: DPORT_PERI_RST_EN_REG,
    #[doc = "0x24 - DPORT_WIFI_BB_CFG_REG"]
    pub dport_wifi_bb_cfg_reg: DPORT_WIFI_BB_CFG_REG,
    #[doc = "0x28 - DPORT_WIFI_BB_CFG_2_REG"]
    pub dport_wifi_bb_cfg_2_reg: DPORT_WIFI_BB_CFG_2_REG,
    #[doc = "0x2c - DPORT_APPCPU_CTRL_A_REG"]
    pub dport_appcpu_ctrl_a_reg: DPORT_APPCPU_CTRL_A_REG,
    #[doc = "0x30 - DPORT_APPCPU_CTRL_B_REG"]
    pub dport_appcpu_ctrl_b_reg: DPORT_APPCPU_CTRL_B_REG,
    #[doc = "0x34 - DPORT_APPCPU_CTRL_C_REG"]
    pub dport_appcpu_ctrl_c_reg: DPORT_APPCPU_CTRL_C_REG,
    #[doc = "0x38 - DPORT_APPCPU_CTRL_D_REG"]
    pub dport_appcpu_ctrl_d_reg: DPORT_APPCPU_CTRL_D_REG,
    #[doc = "0x3c - DPORT_CPU_PER_CONF_REG"]
    pub dport_cpu_per_conf_reg: DPORT_CPU_PER_CONF_REG,
    #[doc = "0x40 - DPORT_PRO_CACHE_CTRL_REG"]
    pub dport_pro_cache_ctrl_reg: DPORT_PRO_CACHE_CTRL_REG,
    #[doc = "0x44 - DPORT_PRO_CACHE_CTRL1_REG"]
    pub dport_pro_cache_ctrl1_reg: DPORT_PRO_CACHE_CTRL1_REG,
    #[doc = "0x48 - DPORT_PRO_CACHE_LOCK_0_ADDR_REG"]
    pub dport_pro_cache_lock_0_addr_reg: DPORT_PRO_CACHE_LOCK_0_ADDR_REG,
    #[doc = "0x4c - DPORT_PRO_CACHE_LOCK_1_ADDR_REG"]
    pub dport_pro_cache_lock_1_addr_reg: DPORT_PRO_CACHE_LOCK_1_ADDR_REG,
    #[doc = "0x50 - DPORT_PRO_CACHE_LOCK_2_ADDR_REG"]
    pub dport_pro_cache_lock_2_addr_reg: DPORT_PRO_CACHE_LOCK_2_ADDR_REG,
    #[doc = "0x54 - DPORT_PRO_CACHE_LOCK_3_ADDR_REG"]
    pub dport_pro_cache_lock_3_addr_reg: DPORT_PRO_CACHE_LOCK_3_ADDR_REG,
    #[doc = "0x58 - DPORT_APP_CACHE_CTRL_REG"]
    pub dport_app_cache_ctrl_reg: DPORT_APP_CACHE_CTRL_REG,
    #[doc = "0x5c - DPORT_APP_CACHE_CTRL1_REG"]
    pub dport_app_cache_ctrl1_reg: DPORT_APP_CACHE_CTRL1_REG,
    #[doc = "0x60 - DPORT_APP_CACHE_LOCK_0_ADDR_REG"]
    pub dport_app_cache_lock_0_addr_reg: DPORT_APP_CACHE_LOCK_0_ADDR_REG,
    #[doc = "0x64 - DPORT_APP_CACHE_LOCK_1_ADDR_REG"]
    pub dport_app_cache_lock_1_addr_reg: DPORT_APP_CACHE_LOCK_1_ADDR_REG,
    #[doc = "0x68 - DPORT_APP_CACHE_LOCK_2_ADDR_REG"]
    pub dport_app_cache_lock_2_addr_reg: DPORT_APP_CACHE_LOCK_2_ADDR_REG,
    #[doc = "0x6c - DPORT_APP_CACHE_LOCK_3_ADDR_REG"]
    pub dport_app_cache_lock_3_addr_reg: DPORT_APP_CACHE_LOCK_3_ADDR_REG,
    #[doc = "0x70 - DPORT_TRACEMEM_MUX_MODE_REG"]
    pub dport_tracemem_mux_mode_reg: DPORT_TRACEMEM_MUX_MODE_REG,
    #[doc = "0x74 - DPORT_PRO_TRACEMEM_ENA_REG"]
    pub dport_pro_tracemem_ena_reg: DPORT_PRO_TRACEMEM_ENA_REG,
    #[doc = "0x78 - DPORT_APP_TRACEMEM_ENA_REG"]
    pub dport_app_tracemem_ena_reg: DPORT_APP_TRACEMEM_ENA_REG,
    #[doc = "0x7c - DPORT_CACHE_MUX_MODE_REG"]
    pub dport_cache_mux_mode_reg: DPORT_CACHE_MUX_MODE_REG,
    #[doc = "0x80 - DPORT_IMMU_PAGE_MODE_REG"]
    pub dport_immu_page_mode_reg: DPORT_IMMU_PAGE_MODE_REG,
    #[doc = "0x84 - DPORT_DMMU_PAGE_MODE_REG"]
    pub dport_dmmu_page_mode_reg: DPORT_DMMU_PAGE_MODE_REG,
    #[doc = "0x88 - DPORT_ROM_MPU_ENA_REG"]
    pub dport_rom_mpu_ena_reg: DPORT_ROM_MPU_ENA_REG,
    #[doc = "0x8c - DPORT_MEM_PD_MASK_REG"]
    pub dport_mem_pd_mask_reg: DPORT_MEM_PD_MASK_REG,
    #[doc = "0x90 - DPORT_ROM_PD_CTRL_REG"]
    pub dport_rom_pd_ctrl_reg: DPORT_ROM_PD_CTRL_REG,
    #[doc = "0x94 - DPORT_ROM_FO_CTRL_REG"]
    pub dport_rom_fo_ctrl_reg: DPORT_ROM_FO_CTRL_REG,
    #[doc = "0x98 - DPORT_SRAM_PD_CTRL_0_REG"]
    pub dport_sram_pd_ctrl_0_reg: DPORT_SRAM_PD_CTRL_0_REG,
    #[doc = "0x9c - DPORT_SRAM_PD_CTRL_1_REG"]
    pub dport_sram_pd_ctrl_1_reg: DPORT_SRAM_PD_CTRL_1_REG,
    #[doc = "0xa0 - DPORT_SRAM_FO_CTRL_0_REG"]
    pub dport_sram_fo_ctrl_0_reg: DPORT_SRAM_FO_CTRL_0_REG,
    #[doc = "0xa4 - DPORT_SRAM_FO_CTRL_1_REG"]
    pub dport_sram_fo_ctrl_1_reg: DPORT_SRAM_FO_CTRL_1_REG,
    #[doc = "0xa8 - DPORT_IRAM_DRAM_AHB_SEL_REG"]
    pub dport_iram_dram_ahb_sel_reg: DPORT_IRAM_DRAM_AHB_SEL_REG,
    #[doc = "0xac - DPORT_TAG_FO_CTRL_REG"]
    pub dport_tag_fo_ctrl_reg: DPORT_TAG_FO_CTRL_REG,
    #[doc = "0xb0 - DPORT_AHB_LITE_MASK_REG"]
    pub dport_ahb_lite_mask_reg: DPORT_AHB_LITE_MASK_REG,
    #[doc = "0xb4 - DPORT_AHB_MPU_TABLE_0_REG"]
    pub dport_ahb_mpu_table_0_reg: DPORT_AHB_MPU_TABLE_0_REG,
    #[doc = "0xb8 - DPORT_AHB_MPU_TABLE_1_REG"]
    pub dport_ahb_mpu_table_1_reg: DPORT_AHB_MPU_TABLE_1_REG,
    #[doc = "0xbc - DPORT_HOST_INF_SEL_REG"]
    pub dport_host_inf_sel_reg: DPORT_HOST_INF_SEL_REG,
    #[doc = "0xc0 - DPORT_PERIP_CLK_EN_REG"]
    pub dport_perip_clk_en_reg: DPORT_PERIP_CLK_EN_REG,
    #[doc = "0xc4 - DPORT_PERIP_RST_EN_REG"]
    pub dport_perip_rst_en_reg: DPORT_PERIP_RST_EN_REG,
    _reserved50: [u8; 4usize],
    #[doc = "0xcc - DPORT_WIFI_CLK_EN_REG"]
    pub dport_wifi_clk_en_reg: DPORT_WIFI_CLK_EN_REG,
    #[doc = "0xd0 - DPORT_CORE_RST_EN_REG"]
    pub dport_core_rst_en_reg: DPORT_CORE_RST_EN_REG,
    #[doc = "0xd4 - DPORT_BT_LPCK_DIV_INT_REG"]
    pub dport_bt_lpck_div_int_reg: DPORT_BT_LPCK_DIV_INT_REG,
    #[doc = "0xd8 - DPORT_BT_LPCK_DIV_FRAC_REG"]
    pub dport_bt_lpck_div_frac_reg: DPORT_BT_LPCK_DIV_FRAC_REG,
    #[doc = "0xdc - DPORT_CPU_INTR_FROM_CPU_0_REG"]
    pub dport_cpu_intr_from_cpu_0_reg: DPORT_CPU_INTR_FROM_CPU_0_REG,
    #[doc = "0xe0 - DPORT_CPU_INTR_FROM_CPU_1_REG"]
    pub dport_cpu_intr_from_cpu_1_reg: DPORT_CPU_INTR_FROM_CPU_1_REG,
    #[doc = "0xe4 - DPORT_CPU_INTR_FROM_CPU_2_REG"]
    pub dport_cpu_intr_from_cpu_2_reg: DPORT_CPU_INTR_FROM_CPU_2_REG,
    #[doc = "0xe8 - DPORT_CPU_INTR_FROM_CPU_3_REG"]
    pub dport_cpu_intr_from_cpu_3_reg: DPORT_CPU_INTR_FROM_CPU_3_REG,
    #[doc = "0xec - DPORT_PRO_INTR_STATUS_0_REG"]
    pub dport_pro_intr_status_0_reg: DPORT_PRO_INTR_STATUS_0_REG,
    #[doc = "0xf0 - DPORT_PRO_INTR_STATUS_1_REG"]
    pub dport_pro_intr_status_1_reg: DPORT_PRO_INTR_STATUS_1_REG,
    #[doc = "0xf4 - DPORT_PRO_INTR_STATUS_2_REG"]
    pub dport_pro_intr_status_2_reg: DPORT_PRO_INTR_STATUS_2_REG,
    #[doc = "0xf8 - DPORT_APP_INTR_STATUS_0_REG"]
    pub dport_app_intr_status_0_reg: DPORT_APP_INTR_STATUS_0_REG,
    #[doc = "0xfc - DPORT_APP_INTR_STATUS_1_REG"]
    pub dport_app_intr_status_1_reg: DPORT_APP_INTR_STATUS_1_REG,
    #[doc = "0x100 - DPORT_APP_INTR_STATUS_2_REG"]
    pub dport_app_intr_status_2_reg: DPORT_APP_INTR_STATUS_2_REG,
    #[doc = "0x104 - DPORT_PRO_MAC_INTR_MAP_REG"]
    pub dport_pro_mac_intr_map_reg: DPORT_PRO_MAC_INTR_MAP_REG,
    #[doc = "0x108 - DPORT_PRO_MAC_NMI_MAP_REG"]
    pub dport_pro_mac_nmi_map_reg: DPORT_PRO_MAC_NMI_MAP_REG,
    #[doc = "0x10c - DPORT_PRO_BB_INT_MAP_REG"]
    pub dport_pro_bb_int_map_reg: DPORT_PRO_BB_INT_MAP_REG,
    #[doc = "0x110 - DPORT_PRO_BT_MAC_INT_MAP_REG"]
    pub dport_pro_bt_mac_int_map_reg: DPORT_PRO_BT_MAC_INT_MAP_REG,
    #[doc = "0x114 - DPORT_PRO_BT_BB_INT_MAP_REG"]
    pub dport_pro_bt_bb_int_map_reg: DPORT_PRO_BT_BB_INT_MAP_REG,
    #[doc = "0x118 - DPORT_PRO_BT_BB_NMI_MAP_REG"]
    pub dport_pro_bt_bb_nmi_map_reg: DPORT_PRO_BT_BB_NMI_MAP_REG,
    #[doc = "0x11c - DPORT_PRO_RWBT_IRQ_MAP_REG"]
    pub dport_pro_rwbt_irq_map_reg: DPORT_PRO_RWBT_IRQ_MAP_REG,
    #[doc = "0x120 - DPORT_PRO_RWBLE_IRQ_MAP_REG"]
    pub dport_pro_rwble_irq_map_reg: DPORT_PRO_RWBLE_IRQ_MAP_REG,
    #[doc = "0x124 - DPORT_PRO_RWBT_NMI_MAP_REG"]
    pub dport_pro_rwbt_nmi_map_reg: DPORT_PRO_RWBT_NMI_MAP_REG,
    #[doc = "0x128 - DPORT_PRO_RWBLE_NMI_MAP_REG"]
    pub dport_pro_rwble_nmi_map_reg: DPORT_PRO_RWBLE_NMI_MAP_REG,
    #[doc = "0x12c - DPORT_PRO_SLC0_INTR_MAP_REG"]
    pub dport_pro_slc0_intr_map_reg: DPORT_PRO_SLC0_INTR_MAP_REG,
    #[doc = "0x130 - DPORT_PRO_SLC1_INTR_MAP_REG"]
    pub dport_pro_slc1_intr_map_reg: DPORT_PRO_SLC1_INTR_MAP_REG,
    #[doc = "0x134 - DPORT_PRO_UHCI0_INTR_MAP_REG"]
    pub dport_pro_uhci0_intr_map_reg: DPORT_PRO_UHCI0_INTR_MAP_REG,
    #[doc = "0x138 - DPORT_PRO_UHCI1_INTR_MAP_REG"]
    pub dport_pro_uhci1_intr_map_reg: DPORT_PRO_UHCI1_INTR_MAP_REG,
    #[doc = "0x13c - DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg_t0_level_int_map_reg: DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG,
    #[doc = "0x140 - DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg_t1_level_int_map_reg: DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG,
    #[doc = "0x144 - DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg_wdt_level_int_map_reg: DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG,
    #[doc = "0x148 - DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg_lact_level_int_map_reg: DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG,
    #[doc = "0x14c - DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg1_t0_level_int_map_reg: DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG,
    #[doc = "0x150 - DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg1_t1_level_int_map_reg: DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG,
    #[doc = "0x154 - DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg1_wdt_level_int_map_reg: DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG,
    #[doc = "0x158 - DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG"]
    pub dport_pro_tg1_lact_level_int_map_reg: DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG,
    #[doc = "0x15c - DPORT_PRO_GPIO_INTERRUPT_MAP_REG"]
    pub dport_pro_gpio_interrupt_map_reg: DPORT_PRO_GPIO_INTERRUPT_MAP_REG,
    #[doc = "0x160 - DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG"]
    pub dport_pro_gpio_interrupt_nmi_map_reg: DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG,
    #[doc = "0x164 - DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG"]
    pub dport_pro_cpu_intr_from_cpu_0_map_reg: DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG,
    #[doc = "0x168 - DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG"]
    pub dport_pro_cpu_intr_from_cpu_1_map_reg: DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG,
    #[doc = "0x16c - DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG"]
    pub dport_pro_cpu_intr_from_cpu_2_map_reg: DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG,
    #[doc = "0x170 - DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG"]
    pub dport_pro_cpu_intr_from_cpu_3_map_reg: DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG,
    #[doc = "0x174 - DPORT_PRO_SPI_INTR_0_MAP_REG"]
    pub dport_pro_spi_intr_0_map_reg: DPORT_PRO_SPI_INTR_0_MAP_REG,
    #[doc = "0x178 - DPORT_PRO_SPI_INTR_1_MAP_REG"]
    pub dport_pro_spi_intr_1_map_reg: DPORT_PRO_SPI_INTR_1_MAP_REG,
    #[doc = "0x17c - DPORT_PRO_SPI_INTR_2_MAP_REG"]
    pub dport_pro_spi_intr_2_map_reg: DPORT_PRO_SPI_INTR_2_MAP_REG,
    #[doc = "0x180 - DPORT_PRO_SPI_INTR_3_MAP_REG"]
    pub dport_pro_spi_intr_3_map_reg: DPORT_PRO_SPI_INTR_3_MAP_REG,
    #[doc = "0x184 - DPORT_PRO_I2S0_INT_MAP_REG"]
    pub dport_pro_i2s0_int_map_reg: DPORT_PRO_I2S0_INT_MAP_REG,
    #[doc = "0x188 - DPORT_PRO_I2S1_INT_MAP_REG"]
    pub dport_pro_i2s1_int_map_reg: DPORT_PRO_I2S1_INT_MAP_REG,
    #[doc = "0x18c - DPORT_PRO_UART_INTR_MAP_REG"]
    pub dport_pro_uart_intr_map_reg: DPORT_PRO_UART_INTR_MAP_REG,
    #[doc = "0x190 - DPORT_PRO_UART1_INTR_MAP_REG"]
    pub dport_pro_uart1_intr_map_reg: DPORT_PRO_UART1_INTR_MAP_REG,
    #[doc = "0x194 - DPORT_PRO_UART2_INTR_MAP_REG"]
    pub dport_pro_uart2_intr_map_reg: DPORT_PRO_UART2_INTR_MAP_REG,
    #[doc = "0x198 - DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG"]
    pub dport_pro_sdio_host_interrupt_map_reg: DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG,
    #[doc = "0x19c - DPORT_PRO_EMAC_INT_MAP_REG"]
    pub dport_pro_emac_int_map_reg: DPORT_PRO_EMAC_INT_MAP_REG,
    #[doc = "0x1a0 - DPORT_PRO_PWM0_INTR_MAP_REG"]
    pub dport_pro_pwm0_intr_map_reg: DPORT_PRO_PWM0_INTR_MAP_REG,
    #[doc = "0x1a4 - DPORT_PRO_PWM1_INTR_MAP_REG"]
    pub dport_pro_pwm1_intr_map_reg: DPORT_PRO_PWM1_INTR_MAP_REG,
    #[doc = "0x1a8 - DPORT_PRO_PWM2_INTR_MAP_REG"]
    pub dport_pro_pwm2_intr_map_reg: DPORT_PRO_PWM2_INTR_MAP_REG,
    #[doc = "0x1ac - DPORT_PRO_PWM3_INTR_MAP_REG"]
    pub dport_pro_pwm3_intr_map_reg: DPORT_PRO_PWM3_INTR_MAP_REG,
    #[doc = "0x1b0 - DPORT_PRO_LEDC_INT_MAP_REG"]
    pub dport_pro_ledc_int_map_reg: DPORT_PRO_LEDC_INT_MAP_REG,
    #[doc = "0x1b4 - DPORT_PRO_EFUSE_INT_MAP_REG"]
    pub dport_pro_efuse_int_map_reg: DPORT_PRO_EFUSE_INT_MAP_REG,
    #[doc = "0x1b8 - DPORT_PRO_CAN_INT_MAP_REG"]
    pub dport_pro_can_int_map_reg: DPORT_PRO_CAN_INT_MAP_REG,
    #[doc = "0x1bc - DPORT_PRO_RTC_CORE_INTR_MAP_REG"]
    pub dport_pro_rtc_core_intr_map_reg: DPORT_PRO_RTC_CORE_INTR_MAP_REG,
    #[doc = "0x1c0 - DPORT_PRO_RMT_INTR_MAP_REG"]
    pub dport_pro_rmt_intr_map_reg: DPORT_PRO_RMT_INTR_MAP_REG,
    #[doc = "0x1c4 - DPORT_PRO_PCNT_INTR_MAP_REG"]
    pub dport_pro_pcnt_intr_map_reg: DPORT_PRO_PCNT_INTR_MAP_REG,
    #[doc = "0x1c8 - DPORT_PRO_I2C_EXT0_INTR_MAP_REG"]
    pub dport_pro_i2c_ext0_intr_map_reg: DPORT_PRO_I2C_EXT0_INTR_MAP_REG,
    #[doc = "0x1cc - DPORT_PRO_I2C_EXT1_INTR_MAP_REG"]
    pub dport_pro_i2c_ext1_intr_map_reg: DPORT_PRO_I2C_EXT1_INTR_MAP_REG,
    #[doc = "0x1d0 - DPORT_PRO_RSA_INTR_MAP_REG"]
    pub dport_pro_rsa_intr_map_reg: DPORT_PRO_RSA_INTR_MAP_REG,
    #[doc = "0x1d4 - DPORT_PRO_SPI1_DMA_INT_MAP_REG"]
    pub dport_pro_spi1_dma_int_map_reg: DPORT_PRO_SPI1_DMA_INT_MAP_REG,
    #[doc = "0x1d8 - DPORT_PRO_SPI2_DMA_INT_MAP_REG"]
    pub dport_pro_spi2_dma_int_map_reg: DPORT_PRO_SPI2_DMA_INT_MAP_REG,
    #[doc = "0x1dc - DPORT_PRO_SPI3_DMA_INT_MAP_REG"]
    pub dport_pro_spi3_dma_int_map_reg: DPORT_PRO_SPI3_DMA_INT_MAP_REG,
    #[doc = "0x1e0 - DPORT_PRO_WDG_INT_MAP_REG"]
    pub dport_pro_wdg_int_map_reg: DPORT_PRO_WDG_INT_MAP_REG,
    #[doc = "0x1e4 - DPORT_PRO_TIMER_INT1_MAP_REG"]
    pub dport_pro_timer_int1_map_reg: DPORT_PRO_TIMER_INT1_MAP_REG,
    #[doc = "0x1e8 - DPORT_PRO_TIMER_INT2_MAP_REG"]
    pub dport_pro_timer_int2_map_reg: DPORT_PRO_TIMER_INT2_MAP_REG,
    #[doc = "0x1ec - DPORT_PRO_TG_T0_EDGE_INT_MAP_REG"]
    pub dport_pro_tg_t0_edge_int_map_reg: DPORT_PRO_TG_T0_EDGE_INT_MAP_REG,
    #[doc = "0x1f0 - DPORT_PRO_TG_T1_EDGE_INT_MAP_REG"]
    pub dport_pro_tg_t1_edge_int_map_reg: DPORT_PRO_TG_T1_EDGE_INT_MAP_REG,
    #[doc = "0x1f4 - DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG"]
    pub dport_pro_tg_wdt_edge_int_map_reg: DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG,
    #[doc = "0x1f8 - DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG"]
    pub dport_pro_tg_lact_edge_int_map_reg: DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG,
    #[doc = "0x1fc - DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG"]
    pub dport_pro_tg1_t0_edge_int_map_reg: DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG,
    #[doc = "0x200 - DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG"]
    pub dport_pro_tg1_t1_edge_int_map_reg: DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG,
    #[doc = "0x204 - DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG"]
    pub dport_pro_tg1_wdt_edge_int_map_reg: DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG,
    #[doc = "0x208 - DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG"]
    pub dport_pro_tg1_lact_edge_int_map_reg: DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG,
    #[doc = "0x20c - DPORT_PRO_MMU_IA_INT_MAP_REG"]
    pub dport_pro_mmu_ia_int_map_reg: DPORT_PRO_MMU_IA_INT_MAP_REG,
    #[doc = "0x210 - DPORT_PRO_MPU_IA_INT_MAP_REG"]
    pub dport_pro_mpu_ia_int_map_reg: DPORT_PRO_MPU_IA_INT_MAP_REG,
    #[doc = "0x214 - DPORT_PRO_CACHE_IA_INT_MAP_REG"]
    pub dport_pro_cache_ia_int_map_reg: DPORT_PRO_CACHE_IA_INT_MAP_REG,
    #[doc = "0x218 - DPORT_APP_MAC_INTR_MAP_REG"]
    pub dport_app_mac_intr_map_reg: DPORT_APP_MAC_INTR_MAP_REG,
    #[doc = "0x21c - DPORT_APP_MAC_NMI_MAP_REG"]
    pub dport_app_mac_nmi_map_reg: DPORT_APP_MAC_NMI_MAP_REG,
    #[doc = "0x220 - DPORT_APP_BB_INT_MAP_REG"]
    pub dport_app_bb_int_map_reg: DPORT_APP_BB_INT_MAP_REG,
    #[doc = "0x224 - DPORT_APP_BT_MAC_INT_MAP_REG"]
    pub dport_app_bt_mac_int_map_reg: DPORT_APP_BT_MAC_INT_MAP_REG,
    #[doc = "0x228 - DPORT_APP_BT_BB_INT_MAP_REG"]
    pub dport_app_bt_bb_int_map_reg: DPORT_APP_BT_BB_INT_MAP_REG,
    #[doc = "0x22c - DPORT_APP_BT_BB_NMI_MAP_REG"]
    pub dport_app_bt_bb_nmi_map_reg: DPORT_APP_BT_BB_NMI_MAP_REG,
    #[doc = "0x230 - DPORT_APP_RWBT_IRQ_MAP_REG"]
    pub dport_app_rwbt_irq_map_reg: DPORT_APP_RWBT_IRQ_MAP_REG,
    #[doc = "0x234 - DPORT_APP_RWBLE_IRQ_MAP_REG"]
    pub dport_app_rwble_irq_map_reg: DPORT_APP_RWBLE_IRQ_MAP_REG,
    #[doc = "0x238 - DPORT_APP_RWBT_NMI_MAP_REG"]
    pub dport_app_rwbt_nmi_map_reg: DPORT_APP_RWBT_NMI_MAP_REG,
    #[doc = "0x23c - DPORT_APP_RWBLE_NMI_MAP_REG"]
    pub dport_app_rwble_nmi_map_reg: DPORT_APP_RWBLE_NMI_MAP_REG,
    #[doc = "0x240 - DPORT_APP_SLC0_INTR_MAP_REG"]
    pub dport_app_slc0_intr_map_reg: DPORT_APP_SLC0_INTR_MAP_REG,
    #[doc = "0x244 - DPORT_APP_SLC1_INTR_MAP_REG"]
    pub dport_app_slc1_intr_map_reg: DPORT_APP_SLC1_INTR_MAP_REG,
    #[doc = "0x248 - DPORT_APP_UHCI0_INTR_MAP_REG"]
    pub dport_app_uhci0_intr_map_reg: DPORT_APP_UHCI0_INTR_MAP_REG,
    #[doc = "0x24c - DPORT_APP_UHCI1_INTR_MAP_REG"]
    pub dport_app_uhci1_intr_map_reg: DPORT_APP_UHCI1_INTR_MAP_REG,
    #[doc = "0x250 - DPORT_APP_TG_T0_LEVEL_INT_MAP_REG"]
    pub dport_app_tg_t0_level_int_map_reg: DPORT_APP_TG_T0_LEVEL_INT_MAP_REG,
    #[doc = "0x254 - DPORT_APP_TG_T1_LEVEL_INT_MAP_REG"]
    pub dport_app_tg_t1_level_int_map_reg: DPORT_APP_TG_T1_LEVEL_INT_MAP_REG,
    #[doc = "0x258 - DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG"]
    pub dport_app_tg_wdt_level_int_map_reg: DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG,
    #[doc = "0x25c - DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG"]
    pub dport_app_tg_lact_level_int_map_reg: DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG,
    #[doc = "0x260 - DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG"]
    pub dport_app_tg1_t0_level_int_map_reg: DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG,
    #[doc = "0x264 - DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG"]
    pub dport_app_tg1_t1_level_int_map_reg: DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG,
    #[doc = "0x268 - DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG"]
    pub dport_app_tg1_wdt_level_int_map_reg: DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG,
    #[doc = "0x26c - DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG"]
    pub dport_app_tg1_lact_level_int_map_reg: DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG,
    #[doc = "0x270 - DPORT_APP_GPIO_INTERRUPT_MAP_REG"]
    pub dport_app_gpio_interrupt_map_reg: DPORT_APP_GPIO_INTERRUPT_MAP_REG,
    #[doc = "0x274 - DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG"]
    pub dport_app_gpio_interrupt_nmi_map_reg: DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG,
    #[doc = "0x278 - DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG"]
    pub dport_app_cpu_intr_from_cpu_0_map_reg: DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG,
    #[doc = "0x27c - DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG"]
    pub dport_app_cpu_intr_from_cpu_1_map_reg: DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG,
    #[doc = "0x280 - DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG"]
    pub dport_app_cpu_intr_from_cpu_2_map_reg: DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG,
    #[doc = "0x284 - DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG"]
    pub dport_app_cpu_intr_from_cpu_3_map_reg: DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG,
    #[doc = "0x288 - DPORT_APP_SPI_INTR_0_MAP_REG"]
    pub dport_app_spi_intr_0_map_reg: DPORT_APP_SPI_INTR_0_MAP_REG,
    #[doc = "0x28c - DPORT_APP_SPI_INTR_1_MAP_REG"]
    pub dport_app_spi_intr_1_map_reg: DPORT_APP_SPI_INTR_1_MAP_REG,
    #[doc = "0x290 - DPORT_APP_SPI_INTR_2_MAP_REG"]
    pub dport_app_spi_intr_2_map_reg: DPORT_APP_SPI_INTR_2_MAP_REG,
    #[doc = "0x294 - DPORT_APP_SPI_INTR_3_MAP_REG"]
    pub dport_app_spi_intr_3_map_reg: DPORT_APP_SPI_INTR_3_MAP_REG,
    #[doc = "0x298 - DPORT_APP_I2S0_INT_MAP_REG"]
    pub dport_app_i2s0_int_map_reg: DPORT_APP_I2S0_INT_MAP_REG,
    #[doc = "0x29c - DPORT_APP_I2S1_INT_MAP_REG"]
    pub dport_app_i2s1_int_map_reg: DPORT_APP_I2S1_INT_MAP_REG,
    #[doc = "0x2a0 - DPORT_APP_UART_INTR_MAP_REG"]
    pub dport_app_uart_intr_map_reg: DPORT_APP_UART_INTR_MAP_REG,
    #[doc = "0x2a4 - DPORT_APP_UART1_INTR_MAP_REG"]
    pub dport_app_uart1_intr_map_reg: DPORT_APP_UART1_INTR_MAP_REG,
    #[doc = "0x2a8 - DPORT_APP_UART2_INTR_MAP_REG"]
    pub dport_app_uart2_intr_map_reg: DPORT_APP_UART2_INTR_MAP_REG,
    #[doc = "0x2ac - DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG"]
    pub dport_app_sdio_host_interrupt_map_reg: DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG,
    #[doc = "0x2b0 - DPORT_APP_EMAC_INT_MAP_REG"]
    pub dport_app_emac_int_map_reg: DPORT_APP_EMAC_INT_MAP_REG,
    #[doc = "0x2b4 - DPORT_APP_PWM0_INTR_MAP_REG"]
    pub dport_app_pwm0_intr_map_reg: DPORT_APP_PWM0_INTR_MAP_REG,
    #[doc = "0x2b8 - DPORT_APP_PWM1_INTR_MAP_REG"]
    pub dport_app_pwm1_intr_map_reg: DPORT_APP_PWM1_INTR_MAP_REG,
    #[doc = "0x2bc - DPORT_APP_PWM2_INTR_MAP_REG"]
    pub dport_app_pwm2_intr_map_reg: DPORT_APP_PWM2_INTR_MAP_REG,
    #[doc = "0x2c0 - DPORT_APP_PWM3_INTR_MAP_REG"]
    pub dport_app_pwm3_intr_map_reg: DPORT_APP_PWM3_INTR_MAP_REG,
    #[doc = "0x2c4 - DPORT_APP_LEDC_INT_MAP_REG"]
    pub dport_app_ledc_int_map_reg: DPORT_APP_LEDC_INT_MAP_REG,
    #[doc = "0x2c8 - DPORT_APP_EFUSE_INT_MAP_REG"]
    pub dport_app_efuse_int_map_reg: DPORT_APP_EFUSE_INT_MAP_REG,
    #[doc = "0x2cc - DPORT_APP_CAN_INT_MAP_REG"]
    pub dport_app_can_int_map_reg: DPORT_APP_CAN_INT_MAP_REG,
    #[doc = "0x2d0 - DPORT_APP_RTC_CORE_INTR_MAP_REG"]
    pub dport_app_rtc_core_intr_map_reg: DPORT_APP_RTC_CORE_INTR_MAP_REG,
    #[doc = "0x2d4 - DPORT_APP_RMT_INTR_MAP_REG"]
    pub dport_app_rmt_intr_map_reg: DPORT_APP_RMT_INTR_MAP_REG,
    #[doc = "0x2d8 - DPORT_APP_PCNT_INTR_MAP_REG"]
    pub dport_app_pcnt_intr_map_reg: DPORT_APP_PCNT_INTR_MAP_REG,
    #[doc = "0x2dc - DPORT_APP_I2C_EXT0_INTR_MAP_REG"]
    pub dport_app_i2c_ext0_intr_map_reg: DPORT_APP_I2C_EXT0_INTR_MAP_REG,
    #[doc = "0x2e0 - DPORT_APP_I2C_EXT1_INTR_MAP_REG"]
    pub dport_app_i2c_ext1_intr_map_reg: DPORT_APP_I2C_EXT1_INTR_MAP_REG,
    #[doc = "0x2e4 - DPORT_APP_RSA_INTR_MAP_REG"]
    pub dport_app_rsa_intr_map_reg: DPORT_APP_RSA_INTR_MAP_REG,
    #[doc = "0x2e8 - DPORT_APP_SPI1_DMA_INT_MAP_REG"]
    pub dport_app_spi1_dma_int_map_reg: DPORT_APP_SPI1_DMA_INT_MAP_REG,
    #[doc = "0x2ec - DPORT_APP_SPI2_DMA_INT_MAP_REG"]
    pub dport_app_spi2_dma_int_map_reg: DPORT_APP_SPI2_DMA_INT_MAP_REG,
    #[doc = "0x2f0 - DPORT_APP_SPI3_DMA_INT_MAP_REG"]
    pub dport_app_spi3_dma_int_map_reg: DPORT_APP_SPI3_DMA_INT_MAP_REG,
    #[doc = "0x2f4 - DPORT_APP_WDG_INT_MAP_REG"]
    pub dport_app_wdg_int_map_reg: DPORT_APP_WDG_INT_MAP_REG,
    #[doc = "0x2f8 - DPORT_APP_TIMER_INT1_MAP_REG"]
    pub dport_app_timer_int1_map_reg: DPORT_APP_TIMER_INT1_MAP_REG,
    #[doc = "0x2fc - DPORT_APP_TIMER_INT2_MAP_REG"]
    pub dport_app_timer_int2_map_reg: DPORT_APP_TIMER_INT2_MAP_REG,
    #[doc = "0x300 - DPORT_APP_TG_T0_EDGE_INT_MAP_REG"]
    pub dport_app_tg_t0_edge_int_map_reg: DPORT_APP_TG_T0_EDGE_INT_MAP_REG,
    #[doc = "0x304 - DPORT_APP_TG_T1_EDGE_INT_MAP_REG"]
    pub dport_app_tg_t1_edge_int_map_reg: DPORT_APP_TG_T1_EDGE_INT_MAP_REG,
    #[doc = "0x308 - DPORT_APP_TG_WDT_EDGE_INT_MAP_REG"]
    pub dport_app_tg_wdt_edge_int_map_reg: DPORT_APP_TG_WDT_EDGE_INT_MAP_REG,
    #[doc = "0x30c - DPORT_APP_TG_LACT_EDGE_INT_MAP_REG"]
    pub dport_app_tg_lact_edge_int_map_reg: DPORT_APP_TG_LACT_EDGE_INT_MAP_REG,
    #[doc = "0x310 - DPORT_APP_TG1_T0_EDGE_INT_MAP_REG"]
    pub dport_app_tg1_t0_edge_int_map_reg: DPORT_APP_TG1_T0_EDGE_INT_MAP_REG,
    #[doc = "0x314 - DPORT_APP_TG1_T1_EDGE_INT_MAP_REG"]
    pub dport_app_tg1_t1_edge_int_map_reg: DPORT_APP_TG1_T1_EDGE_INT_MAP_REG,
    #[doc = "0x318 - DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG"]
    pub dport_app_tg1_wdt_edge_int_map_reg: DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG,
    #[doc = "0x31c - DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG"]
    pub dport_app_tg1_lact_edge_int_map_reg: DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG,
    #[doc = "0x320 - DPORT_APP_MMU_IA_INT_MAP_REG"]
    pub dport_app_mmu_ia_int_map_reg: DPORT_APP_MMU_IA_INT_MAP_REG,
    #[doc = "0x324 - DPORT_APP_MPU_IA_INT_MAP_REG"]
    pub dport_app_mpu_ia_int_map_reg: DPORT_APP_MPU_IA_INT_MAP_REG,
    #[doc = "0x328 - DPORT_APP_CACHE_IA_INT_MAP_REG"]
    pub dport_app_cache_ia_int_map_reg: DPORT_APP_CACHE_IA_INT_MAP_REG,
    #[doc = "0x32c - DPORT_AHBLITE_MPU_TABLE_UART_REG"]
    pub dport_ahblite_mpu_table_uart_reg: DPORT_AHBLITE_MPU_TABLE_UART_REG,
    #[doc = "0x330 - DPORT_AHBLITE_MPU_TABLE_SPI1_REG"]
    pub dport_ahblite_mpu_table_spi1_reg: DPORT_AHBLITE_MPU_TABLE_SPI1_REG,
    #[doc = "0x334 - DPORT_AHBLITE_MPU_TABLE_SPI0_REG"]
    pub dport_ahblite_mpu_table_spi0_reg: DPORT_AHBLITE_MPU_TABLE_SPI0_REG,
    #[doc = "0x338 - DPORT_AHBLITE_MPU_TABLE_GPIO_REG"]
    pub dport_ahblite_mpu_table_gpio_reg: DPORT_AHBLITE_MPU_TABLE_GPIO_REG,
    #[doc = "0x33c - DPORT_AHBLITE_MPU_TABLE_FE2_REG"]
    pub dport_ahblite_mpu_table_fe2_reg: DPORT_AHBLITE_MPU_TABLE_FE2_REG,
    #[doc = "0x340 - DPORT_AHBLITE_MPU_TABLE_FE_REG"]
    pub dport_ahblite_mpu_table_fe_reg: DPORT_AHBLITE_MPU_TABLE_FE_REG,
    #[doc = "0x344 - DPORT_AHBLITE_MPU_TABLE_TIMER_REG"]
    pub dport_ahblite_mpu_table_timer_reg: DPORT_AHBLITE_MPU_TABLE_TIMER_REG,
    #[doc = "0x348 - DPORT_AHBLITE_MPU_TABLE_RTC_REG"]
    pub dport_ahblite_mpu_table_rtc_reg: DPORT_AHBLITE_MPU_TABLE_RTC_REG,
    #[doc = "0x34c - DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG"]
    pub dport_ahblite_mpu_table_io_mux_reg: DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG,
    #[doc = "0x350 - DPORT_AHBLITE_MPU_TABLE_WDG_REG"]
    pub dport_ahblite_mpu_table_wdg_reg: DPORT_AHBLITE_MPU_TABLE_WDG_REG,
    #[doc = "0x354 - DPORT_AHBLITE_MPU_TABLE_HINF_REG"]
    pub dport_ahblite_mpu_table_hinf_reg: DPORT_AHBLITE_MPU_TABLE_HINF_REG,
    #[doc = "0x358 - DPORT_AHBLITE_MPU_TABLE_UHCI1_REG"]
    pub dport_ahblite_mpu_table_uhci1_reg: DPORT_AHBLITE_MPU_TABLE_UHCI1_REG,
    #[doc = "0x35c - DPORT_AHBLITE_MPU_TABLE_MISC_REG"]
    pub dport_ahblite_mpu_table_misc_reg: DPORT_AHBLITE_MPU_TABLE_MISC_REG,
    #[doc = "0x360 - DPORT_AHBLITE_MPU_TABLE_I2C_REG"]
    pub dport_ahblite_mpu_table_i2c_reg: DPORT_AHBLITE_MPU_TABLE_I2C_REG,
    #[doc = "0x364 - DPORT_AHBLITE_MPU_TABLE_I2S0_REG"]
    pub dport_ahblite_mpu_table_i2s0_reg: DPORT_AHBLITE_MPU_TABLE_I2S0_REG,
    #[doc = "0x368 - DPORT_AHBLITE_MPU_TABLE_UART1_REG"]
    pub dport_ahblite_mpu_table_uart1_reg: DPORT_AHBLITE_MPU_TABLE_UART1_REG,
    #[doc = "0x36c - DPORT_AHBLITE_MPU_TABLE_BT_REG"]
    pub dport_ahblite_mpu_table_bt_reg: DPORT_AHBLITE_MPU_TABLE_BT_REG,
    #[doc = "0x370 - DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG"]
    pub dport_ahblite_mpu_table_bt_buffer_reg: DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG,
    #[doc = "0x374 - DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG"]
    pub dport_ahblite_mpu_table_i2c_ext0_reg: DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG,
    #[doc = "0x378 - DPORT_AHBLITE_MPU_TABLE_UHCI0_REG"]
    pub dport_ahblite_mpu_table_uhci0_reg: DPORT_AHBLITE_MPU_TABLE_UHCI0_REG,
    #[doc = "0x37c - DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG"]
    pub dport_ahblite_mpu_table_slchost_reg: DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG,
    #[doc = "0x380 - DPORT_AHBLITE_MPU_TABLE_RMT_REG"]
    pub dport_ahblite_mpu_table_rmt_reg: DPORT_AHBLITE_MPU_TABLE_RMT_REG,
    #[doc = "0x384 - DPORT_AHBLITE_MPU_TABLE_PCNT_REG"]
    pub dport_ahblite_mpu_table_pcnt_reg: DPORT_AHBLITE_MPU_TABLE_PCNT_REG,
    #[doc = "0x388 - DPORT_AHBLITE_MPU_TABLE_SLC_REG"]
    pub dport_ahblite_mpu_table_slc_reg: DPORT_AHBLITE_MPU_TABLE_SLC_REG,
    #[doc = "0x38c - DPORT_AHBLITE_MPU_TABLE_LEDC_REG"]
    pub dport_ahblite_mpu_table_ledc_reg: DPORT_AHBLITE_MPU_TABLE_LEDC_REG,
    #[doc = "0x390 - DPORT_AHBLITE_MPU_TABLE_EFUSE_REG"]
    pub dport_ahblite_mpu_table_efuse_reg: DPORT_AHBLITE_MPU_TABLE_EFUSE_REG,
    #[doc = "0x394 - DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
    pub dport_ahblite_mpu_table_spi_encrypt_reg: DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG,
    #[doc = "0x398 - DPORT_AHBLITE_MPU_TABLE_BB_REG"]
    pub dport_ahblite_mpu_table_bb_reg: DPORT_AHBLITE_MPU_TABLE_BB_REG,
    #[doc = "0x39c - DPORT_AHBLITE_MPU_TABLE_PWM0_REG"]
    pub dport_ahblite_mpu_table_pwm0_reg: DPORT_AHBLITE_MPU_TABLE_PWM0_REG,
    #[doc = "0x3a0 - DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG"]
    pub dport_ahblite_mpu_table_timergroup_reg: DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG,
    #[doc = "0x3a4 - DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG"]
    pub dport_ahblite_mpu_table_timergroup1_reg: DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG,
    #[doc = "0x3a8 - DPORT_AHBLITE_MPU_TABLE_SPI2_REG"]
    pub dport_ahblite_mpu_table_spi2_reg: DPORT_AHBLITE_MPU_TABLE_SPI2_REG,
    #[doc = "0x3ac - DPORT_AHBLITE_MPU_TABLE_SPI3_REG"]
    pub dport_ahblite_mpu_table_spi3_reg: DPORT_AHBLITE_MPU_TABLE_SPI3_REG,
    #[doc = "0x3b0 - DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG"]
    pub dport_ahblite_mpu_table_apb_ctrl_reg: DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG,
    #[doc = "0x3b4 - DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG"]
    pub dport_ahblite_mpu_table_i2c_ext1_reg: DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG,
    #[doc = "0x3b8 - DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG"]
    pub dport_ahblite_mpu_table_sdio_host_reg: DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG,
    #[doc = "0x3bc - DPORT_AHBLITE_MPU_TABLE_EMAC_REG"]
    pub dport_ahblite_mpu_table_emac_reg: DPORT_AHBLITE_MPU_TABLE_EMAC_REG,
    #[doc = "0x3c0 - DPORT_AHBLITE_MPU_TABLE_CAN_REG"]
    pub dport_ahblite_mpu_table_can_reg: DPORT_AHBLITE_MPU_TABLE_CAN_REG,
    #[doc = "0x3c4 - DPORT_AHBLITE_MPU_TABLE_PWM1_REG"]
    pub dport_ahblite_mpu_table_pwm1_reg: DPORT_AHBLITE_MPU_TABLE_PWM1_REG,
    #[doc = "0x3c8 - DPORT_AHBLITE_MPU_TABLE_I2S1_REG"]
    pub dport_ahblite_mpu_table_i2s1_reg: DPORT_AHBLITE_MPU_TABLE_I2S1_REG,
    #[doc = "0x3cc - DPORT_AHBLITE_MPU_TABLE_UART2_REG"]
    pub dport_ahblite_mpu_table_uart2_reg: DPORT_AHBLITE_MPU_TABLE_UART2_REG,
    #[doc = "0x3d0 - DPORT_AHBLITE_MPU_TABLE_PWM2_REG"]
    pub dport_ahblite_mpu_table_pwm2_reg: DPORT_AHBLITE_MPU_TABLE_PWM2_REG,
    #[doc = "0x3d4 - DPORT_AHBLITE_MPU_TABLE_PWM3_REG"]
    pub dport_ahblite_mpu_table_pwm3_reg: DPORT_AHBLITE_MPU_TABLE_PWM3_REG,
    #[doc = "0x3d8 - DPORT_AHBLITE_MPU_TABLE_RWBT_REG"]
    pub dport_ahblite_mpu_table_rwbt_reg: DPORT_AHBLITE_MPU_TABLE_RWBT_REG,
    #[doc = "0x3dc - DPORT_AHBLITE_MPU_TABLE_BTMAC_REG"]
    pub dport_ahblite_mpu_table_btmac_reg: DPORT_AHBLITE_MPU_TABLE_BTMAC_REG,
    #[doc = "0x3e0 - DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG"]
    pub dport_ahblite_mpu_table_wifimac_reg: DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG,
    #[doc = "0x3e4 - DPORT_AHBLITE_MPU_TABLE_PWR_REG"]
    pub dport_ahblite_mpu_table_pwr_reg: DPORT_AHBLITE_MPU_TABLE_PWR_REG,
    #[doc = "0x3e8 - DPORT_MEM_ACCESS_DBUG0_REG"]
    pub dport_mem_access_dbug0_reg: DPORT_MEM_ACCESS_DBUG0_REG,
    #[doc = "0x3ec - DPORT_MEM_ACCESS_DBUG1_REG"]
    pub dport_mem_access_dbug1_reg: DPORT_MEM_ACCESS_DBUG1_REG,
    #[doc = "0x3f0 - DPORT_PRO_DCACHE_DBUG0_REG"]
    pub dport_pro_dcache_dbug0_reg: DPORT_PRO_DCACHE_DBUG0_REG,
    #[doc = "0x3f4 - DPORT_PRO_DCACHE_DBUG1_REG"]
    pub dport_pro_dcache_dbug1_reg: DPORT_PRO_DCACHE_DBUG1_REG,
    #[doc = "0x3f8 - DPORT_PRO_DCACHE_DBUG2_REG"]
    pub dport_pro_dcache_dbug2_reg: DPORT_PRO_DCACHE_DBUG2_REG,
    #[doc = "0x3fc - DPORT_PRO_DCACHE_DBUG3_REG"]
    pub dport_pro_dcache_dbug3_reg: DPORT_PRO_DCACHE_DBUG3_REG,
    #[doc = "0x400 - DPORT_PRO_DCACHE_DBUG4_REG"]
    pub dport_pro_dcache_dbug4_reg: DPORT_PRO_DCACHE_DBUG4_REG,
    #[doc = "0x404 - DPORT_PRO_DCACHE_DBUG5_REG"]
    pub dport_pro_dcache_dbug5_reg: DPORT_PRO_DCACHE_DBUG5_REG,
    #[doc = "0x408 - DPORT_PRO_DCACHE_DBUG6_REG"]
    pub dport_pro_dcache_dbug6_reg: DPORT_PRO_DCACHE_DBUG6_REG,
    #[doc = "0x40c - DPORT_PRO_DCACHE_DBUG7_REG"]
    pub dport_pro_dcache_dbug7_reg: DPORT_PRO_DCACHE_DBUG7_REG,
    #[doc = "0x410 - DPORT_PRO_DCACHE_DBUG8_REG"]
    pub dport_pro_dcache_dbug8_reg: DPORT_PRO_DCACHE_DBUG8_REG,
    #[doc = "0x414 - DPORT_PRO_DCACHE_DBUG9_REG"]
    pub dport_pro_dcache_dbug9_reg: DPORT_PRO_DCACHE_DBUG9_REG,
    #[doc = "0x418 - DPORT_APP_DCACHE_DBUG0_REG"]
    pub dport_app_dcache_dbug0_reg: DPORT_APP_DCACHE_DBUG0_REG,
    #[doc = "0x41c - DPORT_APP_DCACHE_DBUG1_REG"]
    pub dport_app_dcache_dbug1_reg: DPORT_APP_DCACHE_DBUG1_REG,
    #[doc = "0x420 - DPORT_APP_DCACHE_DBUG2_REG"]
    pub dport_app_dcache_dbug2_reg: DPORT_APP_DCACHE_DBUG2_REG,
    #[doc = "0x424 - DPORT_APP_DCACHE_DBUG3_REG"]
    pub dport_app_dcache_dbug3_reg: DPORT_APP_DCACHE_DBUG3_REG,
    #[doc = "0x428 - DPORT_APP_DCACHE_DBUG4_REG"]
    pub dport_app_dcache_dbug4_reg: DPORT_APP_DCACHE_DBUG4_REG,
    #[doc = "0x42c - DPORT_APP_DCACHE_DBUG5_REG"]
    pub dport_app_dcache_dbug5_reg: DPORT_APP_DCACHE_DBUG5_REG,
    #[doc = "0x430 - DPORT_APP_DCACHE_DBUG6_REG"]
    pub dport_app_dcache_dbug6_reg: DPORT_APP_DCACHE_DBUG6_REG,
    #[doc = "0x434 - DPORT_APP_DCACHE_DBUG7_REG"]
    pub dport_app_dcache_dbug7_reg: DPORT_APP_DCACHE_DBUG7_REG,
    #[doc = "0x438 - DPORT_APP_DCACHE_DBUG8_REG"]
    pub dport_app_dcache_dbug8_reg: DPORT_APP_DCACHE_DBUG8_REG,
    #[doc = "0x43c - DPORT_APP_DCACHE_DBUG9_REG"]
    pub dport_app_dcache_dbug9_reg: DPORT_APP_DCACHE_DBUG9_REG,
    #[doc = "0x440 - DPORT_PRO_CPU_RECORD_CTRL_REG"]
    pub dport_pro_cpu_record_ctrl_reg: DPORT_PRO_CPU_RECORD_CTRL_REG,
    #[doc = "0x444 - DPORT_PRO_CPU_RECORD_STATUS_REG"]
    pub dport_pro_cpu_record_status_reg: DPORT_PRO_CPU_RECORD_STATUS_REG,
    #[doc = "0x448 - DPORT_PRO_CPU_RECORD_PID_REG"]
    pub dport_pro_cpu_record_pid_reg: DPORT_PRO_CPU_RECORD_PID_REG,
    #[doc = "0x44c - DPORT_PRO_CPU_RECORD_PDEBUGINST_REG"]
    pub dport_pro_cpu_record_pdebuginst_reg: DPORT_PRO_CPU_RECORD_PDEBUGINST_REG,
    #[doc = "0x450 - DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG"]
    pub dport_pro_cpu_record_pdebugstatus_reg: DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG,
    #[doc = "0x454 - DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
    pub dport_pro_cpu_record_pdebugdata_reg: DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG,
    #[doc = "0x458 - DPORT_PRO_CPU_RECORD_PDEBUGPC_REG"]
    pub dport_pro_cpu_record_pdebugpc_reg: DPORT_PRO_CPU_RECORD_PDEBUGPC_REG,
    #[doc = "0x45c - DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG"]
    pub dport_pro_cpu_record_pdebugls0stat_reg: DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG,
    #[doc = "0x460 - DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG"]
    pub dport_pro_cpu_record_pdebugls0addr_reg: DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG,
    #[doc = "0x464 - DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG"]
    pub dport_pro_cpu_record_pdebugls0data_reg: DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG,
    #[doc = "0x468 - DPORT_APP_CPU_RECORD_CTRL_REG"]
    pub dport_app_cpu_record_ctrl_reg: DPORT_APP_CPU_RECORD_CTRL_REG,
    #[doc = "0x46c - DPORT_APP_CPU_RECORD_STATUS_REG"]
    pub dport_app_cpu_record_status_reg: DPORT_APP_CPU_RECORD_STATUS_REG,
    #[doc = "0x470 - DPORT_APP_CPU_RECORD_PID_REG"]
    pub dport_app_cpu_record_pid_reg: DPORT_APP_CPU_RECORD_PID_REG,
    #[doc = "0x474 - DPORT_APP_CPU_RECORD_PDEBUGINST_REG"]
    pub dport_app_cpu_record_pdebuginst_reg: DPORT_APP_CPU_RECORD_PDEBUGINST_REG,
    #[doc = "0x478 - DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
    pub dport_app_cpu_record_pdebugstatus_reg: DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG,
    #[doc = "0x47c - DPORT_APP_CPU_RECORD_PDEBUGDATA_REG"]
    pub dport_app_cpu_record_pdebugdata_reg: DPORT_APP_CPU_RECORD_PDEBUGDATA_REG,
    #[doc = "0x480 - DPORT_APP_CPU_RECORD_PDEBUGPC_REG"]
    pub dport_app_cpu_record_pdebugpc_reg: DPORT_APP_CPU_RECORD_PDEBUGPC_REG,
    #[doc = "0x484 - DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG"]
    pub dport_app_cpu_record_pdebugls0stat_reg: DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG,
    #[doc = "0x488 - DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG"]
    pub dport_app_cpu_record_pdebugls0addr_reg: DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG,
    #[doc = "0x48c - DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG"]
    pub dport_app_cpu_record_pdebugls0data_reg: DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG,
    #[doc = "0x490 - DPORT_RSA_PD_CTRL_REG"]
    pub dport_rsa_pd_ctrl_reg: DPORT_RSA_PD_CTRL_REG,
    #[doc = "0x494 - DPORT_ROM_MPU_TABLE0_REG"]
    pub dport_rom_mpu_table0_reg: DPORT_ROM_MPU_TABLE0_REG,
    #[doc = "0x498 - DPORT_ROM_MPU_TABLE1_REG"]
    pub dport_rom_mpu_table1_reg: DPORT_ROM_MPU_TABLE1_REG,
    #[doc = "0x49c - DPORT_ROM_MPU_TABLE2_REG"]
    pub dport_rom_mpu_table2_reg: DPORT_ROM_MPU_TABLE2_REG,
    #[doc = "0x4a0 - DPORT_ROM_MPU_TABLE3_REG"]
    pub dport_rom_mpu_table3_reg: DPORT_ROM_MPU_TABLE3_REG,
    #[doc = "0x4a4 - DPORT_SHROM_MPU_TABLE0_REG"]
    pub dport_shrom_mpu_table0_reg: DPORT_SHROM_MPU_TABLE0_REG,
    #[doc = "0x4a8 - DPORT_SHROM_MPU_TABLE1_REG"]
    pub dport_shrom_mpu_table1_reg: DPORT_SHROM_MPU_TABLE1_REG,
    #[doc = "0x4ac - DPORT_SHROM_MPU_TABLE2_REG"]
    pub dport_shrom_mpu_table2_reg: DPORT_SHROM_MPU_TABLE2_REG,
    #[doc = "0x4b0 - DPORT_SHROM_MPU_TABLE3_REG"]
    pub dport_shrom_mpu_table3_reg: DPORT_SHROM_MPU_TABLE3_REG,
    #[doc = "0x4b4 - DPORT_SHROM_MPU_TABLE4_REG"]
    pub dport_shrom_mpu_table4_reg: DPORT_SHROM_MPU_TABLE4_REG,
    #[doc = "0x4b8 - DPORT_SHROM_MPU_TABLE5_REG"]
    pub dport_shrom_mpu_table5_reg: DPORT_SHROM_MPU_TABLE5_REG,
    #[doc = "0x4bc - DPORT_SHROM_MPU_TABLE6_REG"]
    pub dport_shrom_mpu_table6_reg: DPORT_SHROM_MPU_TABLE6_REG,
    #[doc = "0x4c0 - DPORT_SHROM_MPU_TABLE7_REG"]
    pub dport_shrom_mpu_table7_reg: DPORT_SHROM_MPU_TABLE7_REG,
    #[doc = "0x4c4 - DPORT_SHROM_MPU_TABLE8_REG"]
    pub dport_shrom_mpu_table8_reg: DPORT_SHROM_MPU_TABLE8_REG,
    #[doc = "0x4c8 - DPORT_SHROM_MPU_TABLE9_REG"]
    pub dport_shrom_mpu_table9_reg: DPORT_SHROM_MPU_TABLE9_REG,
    #[doc = "0x4cc - DPORT_SHROM_MPU_TABLE10_REG"]
    pub dport_shrom_mpu_table10_reg: DPORT_SHROM_MPU_TABLE10_REG,
    #[doc = "0x4d0 - DPORT_SHROM_MPU_TABLE11_REG"]
    pub dport_shrom_mpu_table11_reg: DPORT_SHROM_MPU_TABLE11_REG,
    #[doc = "0x4d4 - DPORT_SHROM_MPU_TABLE12_REG"]
    pub dport_shrom_mpu_table12_reg: DPORT_SHROM_MPU_TABLE12_REG,
    #[doc = "0x4d8 - DPORT_SHROM_MPU_TABLE13_REG"]
    pub dport_shrom_mpu_table13_reg: DPORT_SHROM_MPU_TABLE13_REG,
    #[doc = "0x4dc - DPORT_SHROM_MPU_TABLE14_REG"]
    pub dport_shrom_mpu_table14_reg: DPORT_SHROM_MPU_TABLE14_REG,
    #[doc = "0x4e0 - DPORT_SHROM_MPU_TABLE15_REG"]
    pub dport_shrom_mpu_table15_reg: DPORT_SHROM_MPU_TABLE15_REG,
    #[doc = "0x4e4 - DPORT_SHROM_MPU_TABLE16_REG"]
    pub dport_shrom_mpu_table16_reg: DPORT_SHROM_MPU_TABLE16_REG,
    #[doc = "0x4e8 - DPORT_SHROM_MPU_TABLE17_REG"]
    pub dport_shrom_mpu_table17_reg: DPORT_SHROM_MPU_TABLE17_REG,
    #[doc = "0x4ec - DPORT_SHROM_MPU_TABLE18_REG"]
    pub dport_shrom_mpu_table18_reg: DPORT_SHROM_MPU_TABLE18_REG,
    #[doc = "0x4f0 - DPORT_SHROM_MPU_TABLE19_REG"]
    pub dport_shrom_mpu_table19_reg: DPORT_SHROM_MPU_TABLE19_REG,
    #[doc = "0x4f4 - DPORT_SHROM_MPU_TABLE20_REG"]
    pub dport_shrom_mpu_table20_reg: DPORT_SHROM_MPU_TABLE20_REG,
    #[doc = "0x4f8 - DPORT_SHROM_MPU_TABLE21_REG"]
    pub dport_shrom_mpu_table21_reg: DPORT_SHROM_MPU_TABLE21_REG,
    #[doc = "0x4fc - DPORT_SHROM_MPU_TABLE22_REG"]
    pub dport_shrom_mpu_table22_reg: DPORT_SHROM_MPU_TABLE22_REG,
    #[doc = "0x500 - DPORT_SHROM_MPU_TABLE23_REG"]
    pub dport_shrom_mpu_table23_reg: DPORT_SHROM_MPU_TABLE23_REG,
    #[doc = "0x504 - DPORT_IMMU_TABLE0_REG"]
    pub dport_immu_table0_reg: DPORT_IMMU_TABLE0_REG,
    #[doc = "0x508 - DPORT_IMMU_TABLE1_REG"]
    pub dport_immu_table1_reg: DPORT_IMMU_TABLE1_REG,
    #[doc = "0x50c - DPORT_IMMU_TABLE2_REG"]
    pub dport_immu_table2_reg: DPORT_IMMU_TABLE2_REG,
    #[doc = "0x510 - DPORT_IMMU_TABLE3_REG"]
    pub dport_immu_table3_reg: DPORT_IMMU_TABLE3_REG,
    #[doc = "0x514 - DPORT_IMMU_TABLE4_REG"]
    pub dport_immu_table4_reg: DPORT_IMMU_TABLE4_REG,
    #[doc = "0x518 - DPORT_IMMU_TABLE5_REG"]
    pub dport_immu_table5_reg: DPORT_IMMU_TABLE5_REG,
    #[doc = "0x51c - DPORT_IMMU_TABLE6_REG"]
    pub dport_immu_table6_reg: DPORT_IMMU_TABLE6_REG,
    #[doc = "0x520 - DPORT_IMMU_TABLE7_REG"]
    pub dport_immu_table7_reg: DPORT_IMMU_TABLE7_REG,
    #[doc = "0x524 - DPORT_IMMU_TABLE8_REG"]
    pub dport_immu_table8_reg: DPORT_IMMU_TABLE8_REG,
    #[doc = "0x528 - DPORT_IMMU_TABLE9_REG"]
    pub dport_immu_table9_reg: DPORT_IMMU_TABLE9_REG,
    #[doc = "0x52c - DPORT_IMMU_TABLE10_REG"]
    pub dport_immu_table10_reg: DPORT_IMMU_TABLE10_REG,
    #[doc = "0x530 - DPORT_IMMU_TABLE11_REG"]
    pub dport_immu_table11_reg: DPORT_IMMU_TABLE11_REG,
    #[doc = "0x534 - DPORT_IMMU_TABLE12_REG"]
    pub dport_immu_table12_reg: DPORT_IMMU_TABLE12_REG,
    #[doc = "0x538 - DPORT_IMMU_TABLE13_REG"]
    pub dport_immu_table13_reg: DPORT_IMMU_TABLE13_REG,
    #[doc = "0x53c - DPORT_IMMU_TABLE14_REG"]
    pub dport_immu_table14_reg: DPORT_IMMU_TABLE14_REG,
    #[doc = "0x540 - DPORT_IMMU_TABLE15_REG"]
    pub dport_immu_table15_reg: DPORT_IMMU_TABLE15_REG,
    #[doc = "0x544 - DPORT_DMMU_TABLE0_REG"]
    pub dport_dmmu_table0_reg: DPORT_DMMU_TABLE0_REG,
    #[doc = "0x548 - DPORT_DMMU_TABLE1_REG"]
    pub dport_dmmu_table1_reg: DPORT_DMMU_TABLE1_REG,
    #[doc = "0x54c - DPORT_DMMU_TABLE2_REG"]
    pub dport_dmmu_table2_reg: DPORT_DMMU_TABLE2_REG,
    #[doc = "0x550 - DPORT_DMMU_TABLE3_REG"]
    pub dport_dmmu_table3_reg: DPORT_DMMU_TABLE3_REG,
    #[doc = "0x554 - DPORT_DMMU_TABLE4_REG"]
    pub dport_dmmu_table4_reg: DPORT_DMMU_TABLE4_REG,
    #[doc = "0x558 - DPORT_DMMU_TABLE5_REG"]
    pub dport_dmmu_table5_reg: DPORT_DMMU_TABLE5_REG,
    #[doc = "0x55c - DPORT_DMMU_TABLE6_REG"]
    pub dport_dmmu_table6_reg: DPORT_DMMU_TABLE6_REG,
    #[doc = "0x560 - DPORT_DMMU_TABLE7_REG"]
    pub dport_dmmu_table7_reg: DPORT_DMMU_TABLE7_REG,
    #[doc = "0x564 - DPORT_DMMU_TABLE8_REG"]
    pub dport_dmmu_table8_reg: DPORT_DMMU_TABLE8_REG,
    #[doc = "0x568 - DPORT_DMMU_TABLE9_REG"]
    pub dport_dmmu_table9_reg: DPORT_DMMU_TABLE9_REG,
    #[doc = "0x56c - DPORT_DMMU_TABLE10_REG"]
    pub dport_dmmu_table10_reg: DPORT_DMMU_TABLE10_REG,
    #[doc = "0x570 - DPORT_DMMU_TABLE11_REG"]
    pub dport_dmmu_table11_reg: DPORT_DMMU_TABLE11_REG,
    #[doc = "0x574 - DPORT_DMMU_TABLE12_REG"]
    pub dport_dmmu_table12_reg: DPORT_DMMU_TABLE12_REG,
    #[doc = "0x578 - DPORT_DMMU_TABLE13_REG"]
    pub dport_dmmu_table13_reg: DPORT_DMMU_TABLE13_REG,
    #[doc = "0x57c - DPORT_DMMU_TABLE14_REG"]
    pub dport_dmmu_table14_reg: DPORT_DMMU_TABLE14_REG,
    #[doc = "0x580 - DPORT_DMMU_TABLE15_REG"]
    pub dport_dmmu_table15_reg: DPORT_DMMU_TABLE15_REG,
    #[doc = "0x584 - DPORT_PRO_INTRUSION_CTRL_REG"]
    pub dport_pro_intrusion_ctrl_reg: DPORT_PRO_INTRUSION_CTRL_REG,
    #[doc = "0x588 - DPORT_PRO_INTRUSION_STATUS_REG"]
    pub dport_pro_intrusion_status_reg: DPORT_PRO_INTRUSION_STATUS_REG,
    #[doc = "0x58c - DPORT_APP_INTRUSION_CTRL_REG"]
    pub dport_app_intrusion_ctrl_reg: DPORT_APP_INTRUSION_CTRL_REG,
    #[doc = "0x590 - DPORT_APP_INTRUSION_STATUS_REG"]
    pub dport_app_intrusion_status_reg: DPORT_APP_INTRUSION_STATUS_REG,
    #[doc = "0x594 - DPORT_FRONT_END_MEM_PD_REG"]
    pub dport_front_end_mem_pd_reg: DPORT_FRONT_END_MEM_PD_REG,
    #[doc = "0x598 - DPORT_MMU_IA_INT_EN_REG"]
    pub dport_mmu_ia_int_en_reg: DPORT_MMU_IA_INT_EN_REG,
    #[doc = "0x59c - DPORT_MPU_IA_INT_EN_REG"]
    pub dport_mpu_ia_int_en_reg: DPORT_MPU_IA_INT_EN_REG,
    #[doc = "0x5a0 - DPORT_CACHE_IA_INT_EN_REG"]
    pub dport_cache_ia_int_en_reg: DPORT_CACHE_IA_INT_EN_REG,
    #[doc = "0x5a4 - DPORT_SECURE_BOOT_CTRL_REG"]
    pub dport_secure_boot_ctrl_reg: DPORT_SECURE_BOOT_CTRL_REG,
    #[doc = "0x5a8 - DPORT_SPI_DMA_CHAN_SEL_REG"]
    pub dport_spi_dma_chan_sel_reg: DPORT_SPI_DMA_CHAN_SEL_REG,
    #[doc = "0x5ac - DPORT_PRO_VECBASE_CTRL_REG"]
    pub dport_pro_vecbase_ctrl_reg: DPORT_PRO_VECBASE_CTRL_REG,
    #[doc = "0x5b0 - DPORT_PRO_VECBASE_SET_REG"]
    pub dport_pro_vecbase_set_reg: DPORT_PRO_VECBASE_SET_REG,
    #[doc = "0x5b4 - DPORT_APP_VECBASE_CTRL_REG"]
    pub dport_app_vecbase_ctrl_reg: DPORT_APP_VECBASE_CTRL_REG,
    #[doc = "0x5b8 - DPORT_APP_VECBASE_SET_REG"]
    pub dport_app_vecbase_set_reg: DPORT_APP_VECBASE_SET_REG,
    _reserved366: [u8; 2624usize],
    #[doc = "0xffc - DPORT_DATE_REG"]
    pub dport_date_reg: DPORT_DATE_REG,
}
#[doc = "DPORT_PRO_BOOT_REMAP_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_boot_remap_ctrl_reg](dport_pro_boot_remap_ctrl_reg) module"]
pub type DPORT_PRO_BOOT_REMAP_CTRL_REG = crate::Reg<u32, _DPORT_PRO_BOOT_REMAP_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_BOOT_REMAP_CTRL_REG;
#[doc = "`read()` method returns [dport_pro_boot_remap_ctrl_reg::R](dport_pro_boot_remap_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_BOOT_REMAP_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_boot_remap_ctrl_reg::W](dport_pro_boot_remap_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_BOOT_REMAP_CTRL_REG {}
#[doc = "DPORT_PRO_BOOT_REMAP_CTRL_REG"]
pub mod dport_pro_boot_remap_ctrl_reg;
#[doc = "DPORT_APP_BOOT_REMAP_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_boot_remap_ctrl_reg](dport_app_boot_remap_ctrl_reg) module"]
pub type DPORT_APP_BOOT_REMAP_CTRL_REG = crate::Reg<u32, _DPORT_APP_BOOT_REMAP_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_BOOT_REMAP_CTRL_REG;
#[doc = "`read()` method returns [dport_app_boot_remap_ctrl_reg::R](dport_app_boot_remap_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_BOOT_REMAP_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_boot_remap_ctrl_reg::W](dport_app_boot_remap_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_BOOT_REMAP_CTRL_REG {}
#[doc = "DPORT_APP_BOOT_REMAP_CTRL_REG"]
pub mod dport_app_boot_remap_ctrl_reg;
#[doc = "DPORT_ACCESS_CHECK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_access_check_reg](dport_access_check_reg) module"]
pub type DPORT_ACCESS_CHECK_REG = crate::Reg<u32, _DPORT_ACCESS_CHECK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ACCESS_CHECK_REG;
#[doc = "`read()` method returns [dport_access_check_reg::R](dport_access_check_reg::R) reader structure"]
impl crate::Readable for DPORT_ACCESS_CHECK_REG {}
#[doc = "`write(|w| ..)` method takes [dport_access_check_reg::W](dport_access_check_reg::W) writer structure"]
impl crate::Writable for DPORT_ACCESS_CHECK_REG {}
#[doc = "DPORT_ACCESS_CHECK_REG"]
pub mod dport_access_check_reg;
#[doc = "DPORT_PRO_DPORT_APB_MASK0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dport_apb_mask0_reg](dport_pro_dport_apb_mask0_reg) module"]
pub type DPORT_PRO_DPORT_APB_MASK0_REG = crate::Reg<u32, _DPORT_PRO_DPORT_APB_MASK0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DPORT_APB_MASK0_REG;
#[doc = "`read()` method returns [dport_pro_dport_apb_mask0_reg::R](dport_pro_dport_apb_mask0_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DPORT_APB_MASK0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dport_apb_mask0_reg::W](dport_pro_dport_apb_mask0_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DPORT_APB_MASK0_REG {}
#[doc = "DPORT_PRO_DPORT_APB_MASK0_REG"]
pub mod dport_pro_dport_apb_mask0_reg;
#[doc = "DPORT_PRO_DPORT_APB_MASK1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dport_apb_mask1_reg](dport_pro_dport_apb_mask1_reg) module"]
pub type DPORT_PRO_DPORT_APB_MASK1_REG = crate::Reg<u32, _DPORT_PRO_DPORT_APB_MASK1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DPORT_APB_MASK1_REG;
#[doc = "`read()` method returns [dport_pro_dport_apb_mask1_reg::R](dport_pro_dport_apb_mask1_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DPORT_APB_MASK1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dport_apb_mask1_reg::W](dport_pro_dport_apb_mask1_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DPORT_APB_MASK1_REG {}
#[doc = "DPORT_PRO_DPORT_APB_MASK1_REG"]
pub mod dport_pro_dport_apb_mask1_reg;
#[doc = "DPORT_APP_DPORT_APB_MASK0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dport_apb_mask0_reg](dport_app_dport_apb_mask0_reg) module"]
pub type DPORT_APP_DPORT_APB_MASK0_REG = crate::Reg<u32, _DPORT_APP_DPORT_APB_MASK0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DPORT_APB_MASK0_REG;
#[doc = "`read()` method returns [dport_app_dport_apb_mask0_reg::R](dport_app_dport_apb_mask0_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DPORT_APB_MASK0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dport_apb_mask0_reg::W](dport_app_dport_apb_mask0_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DPORT_APB_MASK0_REG {}
#[doc = "DPORT_APP_DPORT_APB_MASK0_REG"]
pub mod dport_app_dport_apb_mask0_reg;
#[doc = "DPORT_APP_DPORT_APB_MASK1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dport_apb_mask1_reg](dport_app_dport_apb_mask1_reg) module"]
pub type DPORT_APP_DPORT_APB_MASK1_REG = crate::Reg<u32, _DPORT_APP_DPORT_APB_MASK1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DPORT_APB_MASK1_REG;
#[doc = "`read()` method returns [dport_app_dport_apb_mask1_reg::R](dport_app_dport_apb_mask1_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DPORT_APB_MASK1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dport_apb_mask1_reg::W](dport_app_dport_apb_mask1_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DPORT_APB_MASK1_REG {}
#[doc = "DPORT_APP_DPORT_APB_MASK1_REG"]
pub mod dport_app_dport_apb_mask1_reg;
#[doc = "DPORT_PERI_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_peri_clk_en_reg](dport_peri_clk_en_reg) module"]
pub type DPORT_PERI_CLK_EN_REG = crate::Reg<u32, _DPORT_PERI_CLK_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PERI_CLK_EN_REG;
#[doc = "`read()` method returns [dport_peri_clk_en_reg::R](dport_peri_clk_en_reg::R) reader structure"]
impl crate::Readable for DPORT_PERI_CLK_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_peri_clk_en_reg::W](dport_peri_clk_en_reg::W) writer structure"]
impl crate::Writable for DPORT_PERI_CLK_EN_REG {}
#[doc = "DPORT_PERI_CLK_EN_REG"]
pub mod dport_peri_clk_en_reg;
#[doc = "DPORT_PERI_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_peri_rst_en_reg](dport_peri_rst_en_reg) module"]
pub type DPORT_PERI_RST_EN_REG = crate::Reg<u32, _DPORT_PERI_RST_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PERI_RST_EN_REG;
#[doc = "`read()` method returns [dport_peri_rst_en_reg::R](dport_peri_rst_en_reg::R) reader structure"]
impl crate::Readable for DPORT_PERI_RST_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_peri_rst_en_reg::W](dport_peri_rst_en_reg::W) writer structure"]
impl crate::Writable for DPORT_PERI_RST_EN_REG {}
#[doc = "DPORT_PERI_RST_EN_REG"]
pub mod dport_peri_rst_en_reg;
#[doc = "DPORT_WIFI_BB_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_wifi_bb_cfg_reg](dport_wifi_bb_cfg_reg) module"]
pub type DPORT_WIFI_BB_CFG_REG = crate::Reg<u32, _DPORT_WIFI_BB_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_WIFI_BB_CFG_REG;
#[doc = "`read()` method returns [dport_wifi_bb_cfg_reg::R](dport_wifi_bb_cfg_reg::R) reader structure"]
impl crate::Readable for DPORT_WIFI_BB_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [dport_wifi_bb_cfg_reg::W](dport_wifi_bb_cfg_reg::W) writer structure"]
impl crate::Writable for DPORT_WIFI_BB_CFG_REG {}
#[doc = "DPORT_WIFI_BB_CFG_REG"]
pub mod dport_wifi_bb_cfg_reg;
#[doc = "DPORT_WIFI_BB_CFG_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_wifi_bb_cfg_2_reg](dport_wifi_bb_cfg_2_reg) module"]
pub type DPORT_WIFI_BB_CFG_2_REG = crate::Reg<u32, _DPORT_WIFI_BB_CFG_2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_WIFI_BB_CFG_2_REG;
#[doc = "`read()` method returns [dport_wifi_bb_cfg_2_reg::R](dport_wifi_bb_cfg_2_reg::R) reader structure"]
impl crate::Readable for DPORT_WIFI_BB_CFG_2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_wifi_bb_cfg_2_reg::W](dport_wifi_bb_cfg_2_reg::W) writer structure"]
impl crate::Writable for DPORT_WIFI_BB_CFG_2_REG {}
#[doc = "DPORT_WIFI_BB_CFG_2_REG"]
pub mod dport_wifi_bb_cfg_2_reg;
#[doc = "DPORT_APPCPU_CTRL_A_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_appcpu_ctrl_a_reg](dport_appcpu_ctrl_a_reg) module"]
pub type DPORT_APPCPU_CTRL_A_REG = crate::Reg<u32, _DPORT_APPCPU_CTRL_A_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APPCPU_CTRL_A_REG;
#[doc = "`read()` method returns [dport_appcpu_ctrl_a_reg::R](dport_appcpu_ctrl_a_reg::R) reader structure"]
impl crate::Readable for DPORT_APPCPU_CTRL_A_REG {}
#[doc = "`write(|w| ..)` method takes [dport_appcpu_ctrl_a_reg::W](dport_appcpu_ctrl_a_reg::W) writer structure"]
impl crate::Writable for DPORT_APPCPU_CTRL_A_REG {}
#[doc = "DPORT_APPCPU_CTRL_A_REG"]
pub mod dport_appcpu_ctrl_a_reg;
#[doc = "DPORT_APPCPU_CTRL_B_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_appcpu_ctrl_b_reg](dport_appcpu_ctrl_b_reg) module"]
pub type DPORT_APPCPU_CTRL_B_REG = crate::Reg<u32, _DPORT_APPCPU_CTRL_B_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APPCPU_CTRL_B_REG;
#[doc = "`read()` method returns [dport_appcpu_ctrl_b_reg::R](dport_appcpu_ctrl_b_reg::R) reader structure"]
impl crate::Readable for DPORT_APPCPU_CTRL_B_REG {}
#[doc = "`write(|w| ..)` method takes [dport_appcpu_ctrl_b_reg::W](dport_appcpu_ctrl_b_reg::W) writer structure"]
impl crate::Writable for DPORT_APPCPU_CTRL_B_REG {}
#[doc = "DPORT_APPCPU_CTRL_B_REG"]
pub mod dport_appcpu_ctrl_b_reg;
#[doc = "DPORT_APPCPU_CTRL_C_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_appcpu_ctrl_c_reg](dport_appcpu_ctrl_c_reg) module"]
pub type DPORT_APPCPU_CTRL_C_REG = crate::Reg<u32, _DPORT_APPCPU_CTRL_C_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APPCPU_CTRL_C_REG;
#[doc = "`read()` method returns [dport_appcpu_ctrl_c_reg::R](dport_appcpu_ctrl_c_reg::R) reader structure"]
impl crate::Readable for DPORT_APPCPU_CTRL_C_REG {}
#[doc = "`write(|w| ..)` method takes [dport_appcpu_ctrl_c_reg::W](dport_appcpu_ctrl_c_reg::W) writer structure"]
impl crate::Writable for DPORT_APPCPU_CTRL_C_REG {}
#[doc = "DPORT_APPCPU_CTRL_C_REG"]
pub mod dport_appcpu_ctrl_c_reg;
#[doc = "DPORT_APPCPU_CTRL_D_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_appcpu_ctrl_d_reg](dport_appcpu_ctrl_d_reg) module"]
pub type DPORT_APPCPU_CTRL_D_REG = crate::Reg<u32, _DPORT_APPCPU_CTRL_D_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APPCPU_CTRL_D_REG;
#[doc = "`read()` method returns [dport_appcpu_ctrl_d_reg::R](dport_appcpu_ctrl_d_reg::R) reader structure"]
impl crate::Readable for DPORT_APPCPU_CTRL_D_REG {}
#[doc = "`write(|w| ..)` method takes [dport_appcpu_ctrl_d_reg::W](dport_appcpu_ctrl_d_reg::W) writer structure"]
impl crate::Writable for DPORT_APPCPU_CTRL_D_REG {}
#[doc = "DPORT_APPCPU_CTRL_D_REG"]
pub mod dport_appcpu_ctrl_d_reg;
#[doc = "DPORT_CPU_PER_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cpu_per_conf_reg](dport_cpu_per_conf_reg) module"]
pub type DPORT_CPU_PER_CONF_REG = crate::Reg<u32, _DPORT_CPU_PER_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CPU_PER_CONF_REG;
#[doc = "`read()` method returns [dport_cpu_per_conf_reg::R](dport_cpu_per_conf_reg::R) reader structure"]
impl crate::Readable for DPORT_CPU_PER_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cpu_per_conf_reg::W](dport_cpu_per_conf_reg::W) writer structure"]
impl crate::Writable for DPORT_CPU_PER_CONF_REG {}
#[doc = "DPORT_CPU_PER_CONF_REG"]
pub mod dport_cpu_per_conf_reg;
#[doc = "DPORT_PRO_CACHE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_ctrl_reg](dport_pro_cache_ctrl_reg) module"]
pub type DPORT_PRO_CACHE_CTRL_REG = crate::Reg<u32, _DPORT_PRO_CACHE_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_CTRL_REG;
#[doc = "`read()` method returns [dport_pro_cache_ctrl_reg::R](dport_pro_cache_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_ctrl_reg::W](dport_pro_cache_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_CTRL_REG {}
#[doc = "DPORT_PRO_CACHE_CTRL_REG"]
pub mod dport_pro_cache_ctrl_reg;
#[doc = "DPORT_PRO_CACHE_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_ctrl1_reg](dport_pro_cache_ctrl1_reg) module"]
pub type DPORT_PRO_CACHE_CTRL1_REG = crate::Reg<u32, _DPORT_PRO_CACHE_CTRL1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_CTRL1_REG;
#[doc = "`read()` method returns [dport_pro_cache_ctrl1_reg::R](dport_pro_cache_ctrl1_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_CTRL1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_ctrl1_reg::W](dport_pro_cache_ctrl1_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_CTRL1_REG {}
#[doc = "DPORT_PRO_CACHE_CTRL1_REG"]
pub mod dport_pro_cache_ctrl1_reg;
#[doc = "DPORT_PRO_CACHE_LOCK_0_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_lock_0_addr_reg](dport_pro_cache_lock_0_addr_reg) module"]
pub type DPORT_PRO_CACHE_LOCK_0_ADDR_REG = crate::Reg<u32, _DPORT_PRO_CACHE_LOCK_0_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_LOCK_0_ADDR_REG;
#[doc = "`read()` method returns [dport_pro_cache_lock_0_addr_reg::R](dport_pro_cache_lock_0_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_LOCK_0_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_lock_0_addr_reg::W](dport_pro_cache_lock_0_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_LOCK_0_ADDR_REG {}
#[doc = "DPORT_PRO_CACHE_LOCK_0_ADDR_REG"]
pub mod dport_pro_cache_lock_0_addr_reg;
#[doc = "DPORT_PRO_CACHE_LOCK_1_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_lock_1_addr_reg](dport_pro_cache_lock_1_addr_reg) module"]
pub type DPORT_PRO_CACHE_LOCK_1_ADDR_REG = crate::Reg<u32, _DPORT_PRO_CACHE_LOCK_1_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_LOCK_1_ADDR_REG;
#[doc = "`read()` method returns [dport_pro_cache_lock_1_addr_reg::R](dport_pro_cache_lock_1_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_LOCK_1_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_lock_1_addr_reg::W](dport_pro_cache_lock_1_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_LOCK_1_ADDR_REG {}
#[doc = "DPORT_PRO_CACHE_LOCK_1_ADDR_REG"]
pub mod dport_pro_cache_lock_1_addr_reg;
#[doc = "DPORT_PRO_CACHE_LOCK_2_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_lock_2_addr_reg](dport_pro_cache_lock_2_addr_reg) module"]
pub type DPORT_PRO_CACHE_LOCK_2_ADDR_REG = crate::Reg<u32, _DPORT_PRO_CACHE_LOCK_2_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_LOCK_2_ADDR_REG;
#[doc = "`read()` method returns [dport_pro_cache_lock_2_addr_reg::R](dport_pro_cache_lock_2_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_LOCK_2_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_lock_2_addr_reg::W](dport_pro_cache_lock_2_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_LOCK_2_ADDR_REG {}
#[doc = "DPORT_PRO_CACHE_LOCK_2_ADDR_REG"]
pub mod dport_pro_cache_lock_2_addr_reg;
#[doc = "DPORT_PRO_CACHE_LOCK_3_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_lock_3_addr_reg](dport_pro_cache_lock_3_addr_reg) module"]
pub type DPORT_PRO_CACHE_LOCK_3_ADDR_REG = crate::Reg<u32, _DPORT_PRO_CACHE_LOCK_3_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_LOCK_3_ADDR_REG;
#[doc = "`read()` method returns [dport_pro_cache_lock_3_addr_reg::R](dport_pro_cache_lock_3_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_LOCK_3_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_lock_3_addr_reg::W](dport_pro_cache_lock_3_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_LOCK_3_ADDR_REG {}
#[doc = "DPORT_PRO_CACHE_LOCK_3_ADDR_REG"]
pub mod dport_pro_cache_lock_3_addr_reg;
#[doc = "DPORT_APP_CACHE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_ctrl_reg](dport_app_cache_ctrl_reg) module"]
pub type DPORT_APP_CACHE_CTRL_REG = crate::Reg<u32, _DPORT_APP_CACHE_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_CTRL_REG;
#[doc = "`read()` method returns [dport_app_cache_ctrl_reg::R](dport_app_cache_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_ctrl_reg::W](dport_app_cache_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_CTRL_REG {}
#[doc = "DPORT_APP_CACHE_CTRL_REG"]
pub mod dport_app_cache_ctrl_reg;
#[doc = "DPORT_APP_CACHE_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_ctrl1_reg](dport_app_cache_ctrl1_reg) module"]
pub type DPORT_APP_CACHE_CTRL1_REG = crate::Reg<u32, _DPORT_APP_CACHE_CTRL1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_CTRL1_REG;
#[doc = "`read()` method returns [dport_app_cache_ctrl1_reg::R](dport_app_cache_ctrl1_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_CTRL1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_ctrl1_reg::W](dport_app_cache_ctrl1_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_CTRL1_REG {}
#[doc = "DPORT_APP_CACHE_CTRL1_REG"]
pub mod dport_app_cache_ctrl1_reg;
#[doc = "DPORT_APP_CACHE_LOCK_0_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_lock_0_addr_reg](dport_app_cache_lock_0_addr_reg) module"]
pub type DPORT_APP_CACHE_LOCK_0_ADDR_REG = crate::Reg<u32, _DPORT_APP_CACHE_LOCK_0_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_LOCK_0_ADDR_REG;
#[doc = "`read()` method returns [dport_app_cache_lock_0_addr_reg::R](dport_app_cache_lock_0_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_LOCK_0_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_lock_0_addr_reg::W](dport_app_cache_lock_0_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_LOCK_0_ADDR_REG {}
#[doc = "DPORT_APP_CACHE_LOCK_0_ADDR_REG"]
pub mod dport_app_cache_lock_0_addr_reg;
#[doc = "DPORT_APP_CACHE_LOCK_1_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_lock_1_addr_reg](dport_app_cache_lock_1_addr_reg) module"]
pub type DPORT_APP_CACHE_LOCK_1_ADDR_REG = crate::Reg<u32, _DPORT_APP_CACHE_LOCK_1_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_LOCK_1_ADDR_REG;
#[doc = "`read()` method returns [dport_app_cache_lock_1_addr_reg::R](dport_app_cache_lock_1_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_LOCK_1_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_lock_1_addr_reg::W](dport_app_cache_lock_1_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_LOCK_1_ADDR_REG {}
#[doc = "DPORT_APP_CACHE_LOCK_1_ADDR_REG"]
pub mod dport_app_cache_lock_1_addr_reg;
#[doc = "DPORT_APP_CACHE_LOCK_2_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_lock_2_addr_reg](dport_app_cache_lock_2_addr_reg) module"]
pub type DPORT_APP_CACHE_LOCK_2_ADDR_REG = crate::Reg<u32, _DPORT_APP_CACHE_LOCK_2_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_LOCK_2_ADDR_REG;
#[doc = "`read()` method returns [dport_app_cache_lock_2_addr_reg::R](dport_app_cache_lock_2_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_LOCK_2_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_lock_2_addr_reg::W](dport_app_cache_lock_2_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_LOCK_2_ADDR_REG {}
#[doc = "DPORT_APP_CACHE_LOCK_2_ADDR_REG"]
pub mod dport_app_cache_lock_2_addr_reg;
#[doc = "DPORT_APP_CACHE_LOCK_3_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_lock_3_addr_reg](dport_app_cache_lock_3_addr_reg) module"]
pub type DPORT_APP_CACHE_LOCK_3_ADDR_REG = crate::Reg<u32, _DPORT_APP_CACHE_LOCK_3_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_LOCK_3_ADDR_REG;
#[doc = "`read()` method returns [dport_app_cache_lock_3_addr_reg::R](dport_app_cache_lock_3_addr_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_LOCK_3_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_lock_3_addr_reg::W](dport_app_cache_lock_3_addr_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_LOCK_3_ADDR_REG {}
#[doc = "DPORT_APP_CACHE_LOCK_3_ADDR_REG"]
pub mod dport_app_cache_lock_3_addr_reg;
#[doc = "DPORT_TRACEMEM_MUX_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_tracemem_mux_mode_reg](dport_tracemem_mux_mode_reg) module"]
pub type DPORT_TRACEMEM_MUX_MODE_REG = crate::Reg<u32, _DPORT_TRACEMEM_MUX_MODE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_TRACEMEM_MUX_MODE_REG;
#[doc = "`read()` method returns [dport_tracemem_mux_mode_reg::R](dport_tracemem_mux_mode_reg::R) reader structure"]
impl crate::Readable for DPORT_TRACEMEM_MUX_MODE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_tracemem_mux_mode_reg::W](dport_tracemem_mux_mode_reg::W) writer structure"]
impl crate::Writable for DPORT_TRACEMEM_MUX_MODE_REG {}
#[doc = "DPORT_TRACEMEM_MUX_MODE_REG"]
pub mod dport_tracemem_mux_mode_reg;
#[doc = "DPORT_PRO_TRACEMEM_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tracemem_ena_reg](dport_pro_tracemem_ena_reg) module"]
pub type DPORT_PRO_TRACEMEM_ENA_REG = crate::Reg<u32, _DPORT_PRO_TRACEMEM_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TRACEMEM_ENA_REG;
#[doc = "`read()` method returns [dport_pro_tracemem_ena_reg::R](dport_pro_tracemem_ena_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TRACEMEM_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tracemem_ena_reg::W](dport_pro_tracemem_ena_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TRACEMEM_ENA_REG {}
#[doc = "DPORT_PRO_TRACEMEM_ENA_REG"]
pub mod dport_pro_tracemem_ena_reg;
#[doc = "DPORT_APP_TRACEMEM_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tracemem_ena_reg](dport_app_tracemem_ena_reg) module"]
pub type DPORT_APP_TRACEMEM_ENA_REG = crate::Reg<u32, _DPORT_APP_TRACEMEM_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TRACEMEM_ENA_REG;
#[doc = "`read()` method returns [dport_app_tracemem_ena_reg::R](dport_app_tracemem_ena_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TRACEMEM_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tracemem_ena_reg::W](dport_app_tracemem_ena_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TRACEMEM_ENA_REG {}
#[doc = "DPORT_APP_TRACEMEM_ENA_REG"]
pub mod dport_app_tracemem_ena_reg;
#[doc = "DPORT_CACHE_MUX_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cache_mux_mode_reg](dport_cache_mux_mode_reg) module"]
pub type DPORT_CACHE_MUX_MODE_REG = crate::Reg<u32, _DPORT_CACHE_MUX_MODE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CACHE_MUX_MODE_REG;
#[doc = "`read()` method returns [dport_cache_mux_mode_reg::R](dport_cache_mux_mode_reg::R) reader structure"]
impl crate::Readable for DPORT_CACHE_MUX_MODE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cache_mux_mode_reg::W](dport_cache_mux_mode_reg::W) writer structure"]
impl crate::Writable for DPORT_CACHE_MUX_MODE_REG {}
#[doc = "DPORT_CACHE_MUX_MODE_REG"]
pub mod dport_cache_mux_mode_reg;
#[doc = "DPORT_IMMU_PAGE_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_page_mode_reg](dport_immu_page_mode_reg) module"]
pub type DPORT_IMMU_PAGE_MODE_REG = crate::Reg<u32, _DPORT_IMMU_PAGE_MODE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_PAGE_MODE_REG;
#[doc = "`read()` method returns [dport_immu_page_mode_reg::R](dport_immu_page_mode_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_PAGE_MODE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_page_mode_reg::W](dport_immu_page_mode_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_PAGE_MODE_REG {}
#[doc = "DPORT_IMMU_PAGE_MODE_REG"]
pub mod dport_immu_page_mode_reg;
#[doc = "DPORT_DMMU_PAGE_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_page_mode_reg](dport_dmmu_page_mode_reg) module"]
pub type DPORT_DMMU_PAGE_MODE_REG = crate::Reg<u32, _DPORT_DMMU_PAGE_MODE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_PAGE_MODE_REG;
#[doc = "`read()` method returns [dport_dmmu_page_mode_reg::R](dport_dmmu_page_mode_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_PAGE_MODE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_page_mode_reg::W](dport_dmmu_page_mode_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_PAGE_MODE_REG {}
#[doc = "DPORT_DMMU_PAGE_MODE_REG"]
pub mod dport_dmmu_page_mode_reg;
#[doc = "DPORT_ROM_MPU_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_mpu_ena_reg](dport_rom_mpu_ena_reg) module"]
pub type DPORT_ROM_MPU_ENA_REG = crate::Reg<u32, _DPORT_ROM_MPU_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_MPU_ENA_REG;
#[doc = "`read()` method returns [dport_rom_mpu_ena_reg::R](dport_rom_mpu_ena_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_MPU_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_mpu_ena_reg::W](dport_rom_mpu_ena_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_MPU_ENA_REG {}
#[doc = "DPORT_ROM_MPU_ENA_REG"]
pub mod dport_rom_mpu_ena_reg;
#[doc = "DPORT_MEM_PD_MASK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_mem_pd_mask_reg](dport_mem_pd_mask_reg) module"]
pub type DPORT_MEM_PD_MASK_REG = crate::Reg<u32, _DPORT_MEM_PD_MASK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_MEM_PD_MASK_REG;
#[doc = "`read()` method returns [dport_mem_pd_mask_reg::R](dport_mem_pd_mask_reg::R) reader structure"]
impl crate::Readable for DPORT_MEM_PD_MASK_REG {}
#[doc = "`write(|w| ..)` method takes [dport_mem_pd_mask_reg::W](dport_mem_pd_mask_reg::W) writer structure"]
impl crate::Writable for DPORT_MEM_PD_MASK_REG {}
#[doc = "DPORT_MEM_PD_MASK_REG"]
pub mod dport_mem_pd_mask_reg;
#[doc = "DPORT_ROM_PD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_pd_ctrl_reg](dport_rom_pd_ctrl_reg) module"]
pub type DPORT_ROM_PD_CTRL_REG = crate::Reg<u32, _DPORT_ROM_PD_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_PD_CTRL_REG;
#[doc = "`read()` method returns [dport_rom_pd_ctrl_reg::R](dport_rom_pd_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_PD_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_pd_ctrl_reg::W](dport_rom_pd_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_PD_CTRL_REG {}
#[doc = "DPORT_ROM_PD_CTRL_REG"]
pub mod dport_rom_pd_ctrl_reg;
#[doc = "DPORT_ROM_FO_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_fo_ctrl_reg](dport_rom_fo_ctrl_reg) module"]
pub type DPORT_ROM_FO_CTRL_REG = crate::Reg<u32, _DPORT_ROM_FO_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_FO_CTRL_REG;
#[doc = "`read()` method returns [dport_rom_fo_ctrl_reg::R](dport_rom_fo_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_FO_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_fo_ctrl_reg::W](dport_rom_fo_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_FO_CTRL_REG {}
#[doc = "DPORT_ROM_FO_CTRL_REG"]
pub mod dport_rom_fo_ctrl_reg;
#[doc = "DPORT_SRAM_PD_CTRL_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_sram_pd_ctrl_0_reg](dport_sram_pd_ctrl_0_reg) module"]
pub type DPORT_SRAM_PD_CTRL_0_REG = crate::Reg<u32, _DPORT_SRAM_PD_CTRL_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SRAM_PD_CTRL_0_REG;
#[doc = "`read()` method returns [dport_sram_pd_ctrl_0_reg::R](dport_sram_pd_ctrl_0_reg::R) reader structure"]
impl crate::Readable for DPORT_SRAM_PD_CTRL_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_sram_pd_ctrl_0_reg::W](dport_sram_pd_ctrl_0_reg::W) writer structure"]
impl crate::Writable for DPORT_SRAM_PD_CTRL_0_REG {}
#[doc = "DPORT_SRAM_PD_CTRL_0_REG"]
pub mod dport_sram_pd_ctrl_0_reg;
#[doc = "DPORT_SRAM_PD_CTRL_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_sram_pd_ctrl_1_reg](dport_sram_pd_ctrl_1_reg) module"]
pub type DPORT_SRAM_PD_CTRL_1_REG = crate::Reg<u32, _DPORT_SRAM_PD_CTRL_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SRAM_PD_CTRL_1_REG;
#[doc = "`read()` method returns [dport_sram_pd_ctrl_1_reg::R](dport_sram_pd_ctrl_1_reg::R) reader structure"]
impl crate::Readable for DPORT_SRAM_PD_CTRL_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_sram_pd_ctrl_1_reg::W](dport_sram_pd_ctrl_1_reg::W) writer structure"]
impl crate::Writable for DPORT_SRAM_PD_CTRL_1_REG {}
#[doc = "DPORT_SRAM_PD_CTRL_1_REG"]
pub mod dport_sram_pd_ctrl_1_reg;
#[doc = "DPORT_SRAM_FO_CTRL_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_sram_fo_ctrl_0_reg](dport_sram_fo_ctrl_0_reg) module"]
pub type DPORT_SRAM_FO_CTRL_0_REG = crate::Reg<u32, _DPORT_SRAM_FO_CTRL_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SRAM_FO_CTRL_0_REG;
#[doc = "`read()` method returns [dport_sram_fo_ctrl_0_reg::R](dport_sram_fo_ctrl_0_reg::R) reader structure"]
impl crate::Readable for DPORT_SRAM_FO_CTRL_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_sram_fo_ctrl_0_reg::W](dport_sram_fo_ctrl_0_reg::W) writer structure"]
impl crate::Writable for DPORT_SRAM_FO_CTRL_0_REG {}
#[doc = "DPORT_SRAM_FO_CTRL_0_REG"]
pub mod dport_sram_fo_ctrl_0_reg;
#[doc = "DPORT_SRAM_FO_CTRL_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_sram_fo_ctrl_1_reg](dport_sram_fo_ctrl_1_reg) module"]
pub type DPORT_SRAM_FO_CTRL_1_REG = crate::Reg<u32, _DPORT_SRAM_FO_CTRL_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SRAM_FO_CTRL_1_REG;
#[doc = "`read()` method returns [dport_sram_fo_ctrl_1_reg::R](dport_sram_fo_ctrl_1_reg::R) reader structure"]
impl crate::Readable for DPORT_SRAM_FO_CTRL_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_sram_fo_ctrl_1_reg::W](dport_sram_fo_ctrl_1_reg::W) writer structure"]
impl crate::Writable for DPORT_SRAM_FO_CTRL_1_REG {}
#[doc = "DPORT_SRAM_FO_CTRL_1_REG"]
pub mod dport_sram_fo_ctrl_1_reg;
#[doc = "DPORT_IRAM_DRAM_AHB_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_iram_dram_ahb_sel_reg](dport_iram_dram_ahb_sel_reg) module"]
pub type DPORT_IRAM_DRAM_AHB_SEL_REG = crate::Reg<u32, _DPORT_IRAM_DRAM_AHB_SEL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IRAM_DRAM_AHB_SEL_REG;
#[doc = "`read()` method returns [dport_iram_dram_ahb_sel_reg::R](dport_iram_dram_ahb_sel_reg::R) reader structure"]
impl crate::Readable for DPORT_IRAM_DRAM_AHB_SEL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_iram_dram_ahb_sel_reg::W](dport_iram_dram_ahb_sel_reg::W) writer structure"]
impl crate::Writable for DPORT_IRAM_DRAM_AHB_SEL_REG {}
#[doc = "DPORT_IRAM_DRAM_AHB_SEL_REG"]
pub mod dport_iram_dram_ahb_sel_reg;
#[doc = "DPORT_TAG_FO_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_tag_fo_ctrl_reg](dport_tag_fo_ctrl_reg) module"]
pub type DPORT_TAG_FO_CTRL_REG = crate::Reg<u32, _DPORT_TAG_FO_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_TAG_FO_CTRL_REG;
#[doc = "`read()` method returns [dport_tag_fo_ctrl_reg::R](dport_tag_fo_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_TAG_FO_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_tag_fo_ctrl_reg::W](dport_tag_fo_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_TAG_FO_CTRL_REG {}
#[doc = "DPORT_TAG_FO_CTRL_REG"]
pub mod dport_tag_fo_ctrl_reg;
#[doc = "DPORT_AHB_LITE_MASK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahb_lite_mask_reg](dport_ahb_lite_mask_reg) module"]
pub type DPORT_AHB_LITE_MASK_REG = crate::Reg<u32, _DPORT_AHB_LITE_MASK_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHB_LITE_MASK_REG;
#[doc = "`read()` method returns [dport_ahb_lite_mask_reg::R](dport_ahb_lite_mask_reg::R) reader structure"]
impl crate::Readable for DPORT_AHB_LITE_MASK_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahb_lite_mask_reg::W](dport_ahb_lite_mask_reg::W) writer structure"]
impl crate::Writable for DPORT_AHB_LITE_MASK_REG {}
#[doc = "DPORT_AHB_LITE_MASK_REG"]
pub mod dport_ahb_lite_mask_reg;
#[doc = "DPORT_AHB_MPU_TABLE_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahb_mpu_table_0_reg](dport_ahb_mpu_table_0_reg) module"]
pub type DPORT_AHB_MPU_TABLE_0_REG = crate::Reg<u32, _DPORT_AHB_MPU_TABLE_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHB_MPU_TABLE_0_REG;
#[doc = "`read()` method returns [dport_ahb_mpu_table_0_reg::R](dport_ahb_mpu_table_0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHB_MPU_TABLE_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahb_mpu_table_0_reg::W](dport_ahb_mpu_table_0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHB_MPU_TABLE_0_REG {}
#[doc = "DPORT_AHB_MPU_TABLE_0_REG"]
pub mod dport_ahb_mpu_table_0_reg;
#[doc = "DPORT_AHB_MPU_TABLE_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahb_mpu_table_1_reg](dport_ahb_mpu_table_1_reg) module"]
pub type DPORT_AHB_MPU_TABLE_1_REG = crate::Reg<u32, _DPORT_AHB_MPU_TABLE_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHB_MPU_TABLE_1_REG;
#[doc = "`read()` method returns [dport_ahb_mpu_table_1_reg::R](dport_ahb_mpu_table_1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHB_MPU_TABLE_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahb_mpu_table_1_reg::W](dport_ahb_mpu_table_1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHB_MPU_TABLE_1_REG {}
#[doc = "DPORT_AHB_MPU_TABLE_1_REG"]
pub mod dport_ahb_mpu_table_1_reg;
#[doc = "DPORT_HOST_INF_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_host_inf_sel_reg](dport_host_inf_sel_reg) module"]
pub type DPORT_HOST_INF_SEL_REG = crate::Reg<u32, _DPORT_HOST_INF_SEL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_HOST_INF_SEL_REG;
#[doc = "`read()` method returns [dport_host_inf_sel_reg::R](dport_host_inf_sel_reg::R) reader structure"]
impl crate::Readable for DPORT_HOST_INF_SEL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_host_inf_sel_reg::W](dport_host_inf_sel_reg::W) writer structure"]
impl crate::Writable for DPORT_HOST_INF_SEL_REG {}
#[doc = "DPORT_HOST_INF_SEL_REG"]
pub mod dport_host_inf_sel_reg;
#[doc = "DPORT_PERIP_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_perip_clk_en_reg](dport_perip_clk_en_reg) module"]
pub type DPORT_PERIP_CLK_EN_REG = crate::Reg<u32, _DPORT_PERIP_CLK_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PERIP_CLK_EN_REG;
#[doc = "`read()` method returns [dport_perip_clk_en_reg::R](dport_perip_clk_en_reg::R) reader structure"]
impl crate::Readable for DPORT_PERIP_CLK_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_perip_clk_en_reg::W](dport_perip_clk_en_reg::W) writer structure"]
impl crate::Writable for DPORT_PERIP_CLK_EN_REG {}
#[doc = "DPORT_PERIP_CLK_EN_REG"]
pub mod dport_perip_clk_en_reg;
#[doc = "DPORT_PERIP_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_perip_rst_en_reg](dport_perip_rst_en_reg) module"]
pub type DPORT_PERIP_RST_EN_REG = crate::Reg<u32, _DPORT_PERIP_RST_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PERIP_RST_EN_REG;
#[doc = "`read()` method returns [dport_perip_rst_en_reg::R](dport_perip_rst_en_reg::R) reader structure"]
impl crate::Readable for DPORT_PERIP_RST_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_perip_rst_en_reg::W](dport_perip_rst_en_reg::W) writer structure"]
impl crate::Writable for DPORT_PERIP_RST_EN_REG {}
#[doc = "DPORT_PERIP_RST_EN_REG"]
pub mod dport_perip_rst_en_reg;
#[doc = "DPORT_WIFI_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_wifi_clk_en_reg](dport_wifi_clk_en_reg) module"]
pub type DPORT_WIFI_CLK_EN_REG = crate::Reg<u32, _DPORT_WIFI_CLK_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_WIFI_CLK_EN_REG;
#[doc = "`read()` method returns [dport_wifi_clk_en_reg::R](dport_wifi_clk_en_reg::R) reader structure"]
impl crate::Readable for DPORT_WIFI_CLK_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_wifi_clk_en_reg::W](dport_wifi_clk_en_reg::W) writer structure"]
impl crate::Writable for DPORT_WIFI_CLK_EN_REG {}
#[doc = "DPORT_WIFI_CLK_EN_REG"]
pub mod dport_wifi_clk_en_reg;
#[doc = "DPORT_CORE_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_core_rst_en_reg](dport_core_rst_en_reg) module"]
pub type DPORT_CORE_RST_EN_REG = crate::Reg<u32, _DPORT_CORE_RST_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CORE_RST_EN_REG;
#[doc = "`read()` method returns [dport_core_rst_en_reg::R](dport_core_rst_en_reg::R) reader structure"]
impl crate::Readable for DPORT_CORE_RST_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_core_rst_en_reg::W](dport_core_rst_en_reg::W) writer structure"]
impl crate::Writable for DPORT_CORE_RST_EN_REG {}
#[doc = "DPORT_CORE_RST_EN_REG"]
pub mod dport_core_rst_en_reg;
#[doc = "DPORT_BT_LPCK_DIV_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_bt_lpck_div_int_reg](dport_bt_lpck_div_int_reg) module"]
pub type DPORT_BT_LPCK_DIV_INT_REG = crate::Reg<u32, _DPORT_BT_LPCK_DIV_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_BT_LPCK_DIV_INT_REG;
#[doc = "`read()` method returns [dport_bt_lpck_div_int_reg::R](dport_bt_lpck_div_int_reg::R) reader structure"]
impl crate::Readable for DPORT_BT_LPCK_DIV_INT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_bt_lpck_div_int_reg::W](dport_bt_lpck_div_int_reg::W) writer structure"]
impl crate::Writable for DPORT_BT_LPCK_DIV_INT_REG {}
#[doc = "DPORT_BT_LPCK_DIV_INT_REG"]
pub mod dport_bt_lpck_div_int_reg;
#[doc = "DPORT_BT_LPCK_DIV_FRAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_bt_lpck_div_frac_reg](dport_bt_lpck_div_frac_reg) module"]
pub type DPORT_BT_LPCK_DIV_FRAC_REG = crate::Reg<u32, _DPORT_BT_LPCK_DIV_FRAC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_BT_LPCK_DIV_FRAC_REG;
#[doc = "`read()` method returns [dport_bt_lpck_div_frac_reg::R](dport_bt_lpck_div_frac_reg::R) reader structure"]
impl crate::Readable for DPORT_BT_LPCK_DIV_FRAC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_bt_lpck_div_frac_reg::W](dport_bt_lpck_div_frac_reg::W) writer structure"]
impl crate::Writable for DPORT_BT_LPCK_DIV_FRAC_REG {}
#[doc = "DPORT_BT_LPCK_DIV_FRAC_REG"]
pub mod dport_bt_lpck_div_frac_reg;
#[doc = "DPORT_CPU_INTR_FROM_CPU_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cpu_intr_from_cpu_0_reg](dport_cpu_intr_from_cpu_0_reg) module"]
pub type DPORT_CPU_INTR_FROM_CPU_0_REG = crate::Reg<u32, _DPORT_CPU_INTR_FROM_CPU_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CPU_INTR_FROM_CPU_0_REG;
#[doc = "`read()` method returns [dport_cpu_intr_from_cpu_0_reg::R](dport_cpu_intr_from_cpu_0_reg::R) reader structure"]
impl crate::Readable for DPORT_CPU_INTR_FROM_CPU_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cpu_intr_from_cpu_0_reg::W](dport_cpu_intr_from_cpu_0_reg::W) writer structure"]
impl crate::Writable for DPORT_CPU_INTR_FROM_CPU_0_REG {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_0_REG"]
pub mod dport_cpu_intr_from_cpu_0_reg;
#[doc = "DPORT_CPU_INTR_FROM_CPU_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cpu_intr_from_cpu_1_reg](dport_cpu_intr_from_cpu_1_reg) module"]
pub type DPORT_CPU_INTR_FROM_CPU_1_REG = crate::Reg<u32, _DPORT_CPU_INTR_FROM_CPU_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CPU_INTR_FROM_CPU_1_REG;
#[doc = "`read()` method returns [dport_cpu_intr_from_cpu_1_reg::R](dport_cpu_intr_from_cpu_1_reg::R) reader structure"]
impl crate::Readable for DPORT_CPU_INTR_FROM_CPU_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cpu_intr_from_cpu_1_reg::W](dport_cpu_intr_from_cpu_1_reg::W) writer structure"]
impl crate::Writable for DPORT_CPU_INTR_FROM_CPU_1_REG {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_1_REG"]
pub mod dport_cpu_intr_from_cpu_1_reg;
#[doc = "DPORT_CPU_INTR_FROM_CPU_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cpu_intr_from_cpu_2_reg](dport_cpu_intr_from_cpu_2_reg) module"]
pub type DPORT_CPU_INTR_FROM_CPU_2_REG = crate::Reg<u32, _DPORT_CPU_INTR_FROM_CPU_2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CPU_INTR_FROM_CPU_2_REG;
#[doc = "`read()` method returns [dport_cpu_intr_from_cpu_2_reg::R](dport_cpu_intr_from_cpu_2_reg::R) reader structure"]
impl crate::Readable for DPORT_CPU_INTR_FROM_CPU_2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cpu_intr_from_cpu_2_reg::W](dport_cpu_intr_from_cpu_2_reg::W) writer structure"]
impl crate::Writable for DPORT_CPU_INTR_FROM_CPU_2_REG {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_2_REG"]
pub mod dport_cpu_intr_from_cpu_2_reg;
#[doc = "DPORT_CPU_INTR_FROM_CPU_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cpu_intr_from_cpu_3_reg](dport_cpu_intr_from_cpu_3_reg) module"]
pub type DPORT_CPU_INTR_FROM_CPU_3_REG = crate::Reg<u32, _DPORT_CPU_INTR_FROM_CPU_3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CPU_INTR_FROM_CPU_3_REG;
#[doc = "`read()` method returns [dport_cpu_intr_from_cpu_3_reg::R](dport_cpu_intr_from_cpu_3_reg::R) reader structure"]
impl crate::Readable for DPORT_CPU_INTR_FROM_CPU_3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cpu_intr_from_cpu_3_reg::W](dport_cpu_intr_from_cpu_3_reg::W) writer structure"]
impl crate::Writable for DPORT_CPU_INTR_FROM_CPU_3_REG {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_3_REG"]
pub mod dport_cpu_intr_from_cpu_3_reg;
#[doc = "DPORT_PRO_INTR_STATUS_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_intr_status_0_reg](dport_pro_intr_status_0_reg) module"]
pub type DPORT_PRO_INTR_STATUS_0_REG = crate::Reg<u32, _DPORT_PRO_INTR_STATUS_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_INTR_STATUS_0_REG;
#[doc = "`read()` method returns [dport_pro_intr_status_0_reg::R](dport_pro_intr_status_0_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_INTR_STATUS_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_intr_status_0_reg::W](dport_pro_intr_status_0_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_INTR_STATUS_0_REG {}
#[doc = "DPORT_PRO_INTR_STATUS_0_REG"]
pub mod dport_pro_intr_status_0_reg;
#[doc = "DPORT_PRO_INTR_STATUS_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_intr_status_1_reg](dport_pro_intr_status_1_reg) module"]
pub type DPORT_PRO_INTR_STATUS_1_REG = crate::Reg<u32, _DPORT_PRO_INTR_STATUS_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_INTR_STATUS_1_REG;
#[doc = "`read()` method returns [dport_pro_intr_status_1_reg::R](dport_pro_intr_status_1_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_INTR_STATUS_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_intr_status_1_reg::W](dport_pro_intr_status_1_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_INTR_STATUS_1_REG {}
#[doc = "DPORT_PRO_INTR_STATUS_1_REG"]
pub mod dport_pro_intr_status_1_reg;
#[doc = "DPORT_PRO_INTR_STATUS_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_intr_status_2_reg](dport_pro_intr_status_2_reg) module"]
pub type DPORT_PRO_INTR_STATUS_2_REG = crate::Reg<u32, _DPORT_PRO_INTR_STATUS_2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_INTR_STATUS_2_REG;
#[doc = "`read()` method returns [dport_pro_intr_status_2_reg::R](dport_pro_intr_status_2_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_INTR_STATUS_2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_intr_status_2_reg::W](dport_pro_intr_status_2_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_INTR_STATUS_2_REG {}
#[doc = "DPORT_PRO_INTR_STATUS_2_REG"]
pub mod dport_pro_intr_status_2_reg;
#[doc = "DPORT_APP_INTR_STATUS_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_intr_status_0_reg](dport_app_intr_status_0_reg) module"]
pub type DPORT_APP_INTR_STATUS_0_REG = crate::Reg<u32, _DPORT_APP_INTR_STATUS_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_INTR_STATUS_0_REG;
#[doc = "`read()` method returns [dport_app_intr_status_0_reg::R](dport_app_intr_status_0_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_INTR_STATUS_0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_intr_status_0_reg::W](dport_app_intr_status_0_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_INTR_STATUS_0_REG {}
#[doc = "DPORT_APP_INTR_STATUS_0_REG"]
pub mod dport_app_intr_status_0_reg;
#[doc = "DPORT_APP_INTR_STATUS_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_intr_status_1_reg](dport_app_intr_status_1_reg) module"]
pub type DPORT_APP_INTR_STATUS_1_REG = crate::Reg<u32, _DPORT_APP_INTR_STATUS_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_INTR_STATUS_1_REG;
#[doc = "`read()` method returns [dport_app_intr_status_1_reg::R](dport_app_intr_status_1_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_INTR_STATUS_1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_intr_status_1_reg::W](dport_app_intr_status_1_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_INTR_STATUS_1_REG {}
#[doc = "DPORT_APP_INTR_STATUS_1_REG"]
pub mod dport_app_intr_status_1_reg;
#[doc = "DPORT_APP_INTR_STATUS_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_intr_status_2_reg](dport_app_intr_status_2_reg) module"]
pub type DPORT_APP_INTR_STATUS_2_REG = crate::Reg<u32, _DPORT_APP_INTR_STATUS_2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_INTR_STATUS_2_REG;
#[doc = "`read()` method returns [dport_app_intr_status_2_reg::R](dport_app_intr_status_2_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_INTR_STATUS_2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_intr_status_2_reg::W](dport_app_intr_status_2_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_INTR_STATUS_2_REG {}
#[doc = "DPORT_APP_INTR_STATUS_2_REG"]
pub mod dport_app_intr_status_2_reg;
#[doc = "DPORT_PRO_MAC_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_mac_intr_map_reg](dport_pro_mac_intr_map_reg) module"]
pub type DPORT_PRO_MAC_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_MAC_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_MAC_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_mac_intr_map_reg::R](dport_pro_mac_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_MAC_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_mac_intr_map_reg::W](dport_pro_mac_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_MAC_INTR_MAP_REG {}
#[doc = "DPORT_PRO_MAC_INTR_MAP_REG"]
pub mod dport_pro_mac_intr_map_reg;
#[doc = "DPORT_PRO_MAC_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_mac_nmi_map_reg](dport_pro_mac_nmi_map_reg) module"]
pub type DPORT_PRO_MAC_NMI_MAP_REG = crate::Reg<u32, _DPORT_PRO_MAC_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_MAC_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_pro_mac_nmi_map_reg::R](dport_pro_mac_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_MAC_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_mac_nmi_map_reg::W](dport_pro_mac_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_MAC_NMI_MAP_REG {}
#[doc = "DPORT_PRO_MAC_NMI_MAP_REG"]
pub mod dport_pro_mac_nmi_map_reg;
#[doc = "DPORT_PRO_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_bb_int_map_reg](dport_pro_bb_int_map_reg) module"]
pub type DPORT_PRO_BB_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_BB_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_BB_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_bb_int_map_reg::R](dport_pro_bb_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_BB_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_bb_int_map_reg::W](dport_pro_bb_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_BB_INT_MAP_REG {}
#[doc = "DPORT_PRO_BB_INT_MAP_REG"]
pub mod dport_pro_bb_int_map_reg;
#[doc = "DPORT_PRO_BT_MAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_bt_mac_int_map_reg](dport_pro_bt_mac_int_map_reg) module"]
pub type DPORT_PRO_BT_MAC_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_BT_MAC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_BT_MAC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_bt_mac_int_map_reg::R](dport_pro_bt_mac_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_BT_MAC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_bt_mac_int_map_reg::W](dport_pro_bt_mac_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_BT_MAC_INT_MAP_REG {}
#[doc = "DPORT_PRO_BT_MAC_INT_MAP_REG"]
pub mod dport_pro_bt_mac_int_map_reg;
#[doc = "DPORT_PRO_BT_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_bt_bb_int_map_reg](dport_pro_bt_bb_int_map_reg) module"]
pub type DPORT_PRO_BT_BB_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_BT_BB_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_BT_BB_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_bt_bb_int_map_reg::R](dport_pro_bt_bb_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_BT_BB_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_bt_bb_int_map_reg::W](dport_pro_bt_bb_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_BT_BB_INT_MAP_REG {}
#[doc = "DPORT_PRO_BT_BB_INT_MAP_REG"]
pub mod dport_pro_bt_bb_int_map_reg;
#[doc = "DPORT_PRO_BT_BB_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_bt_bb_nmi_map_reg](dport_pro_bt_bb_nmi_map_reg) module"]
pub type DPORT_PRO_BT_BB_NMI_MAP_REG = crate::Reg<u32, _DPORT_PRO_BT_BB_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_BT_BB_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_pro_bt_bb_nmi_map_reg::R](dport_pro_bt_bb_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_BT_BB_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_bt_bb_nmi_map_reg::W](dport_pro_bt_bb_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_BT_BB_NMI_MAP_REG {}
#[doc = "DPORT_PRO_BT_BB_NMI_MAP_REG"]
pub mod dport_pro_bt_bb_nmi_map_reg;
#[doc = "DPORT_PRO_RWBT_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rwbt_irq_map_reg](dport_pro_rwbt_irq_map_reg) module"]
pub type DPORT_PRO_RWBT_IRQ_MAP_REG = crate::Reg<u32, _DPORT_PRO_RWBT_IRQ_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RWBT_IRQ_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rwbt_irq_map_reg::R](dport_pro_rwbt_irq_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RWBT_IRQ_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rwbt_irq_map_reg::W](dport_pro_rwbt_irq_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RWBT_IRQ_MAP_REG {}
#[doc = "DPORT_PRO_RWBT_IRQ_MAP_REG"]
pub mod dport_pro_rwbt_irq_map_reg;
#[doc = "DPORT_PRO_RWBLE_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rwble_irq_map_reg](dport_pro_rwble_irq_map_reg) module"]
pub type DPORT_PRO_RWBLE_IRQ_MAP_REG = crate::Reg<u32, _DPORT_PRO_RWBLE_IRQ_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RWBLE_IRQ_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rwble_irq_map_reg::R](dport_pro_rwble_irq_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RWBLE_IRQ_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rwble_irq_map_reg::W](dport_pro_rwble_irq_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RWBLE_IRQ_MAP_REG {}
#[doc = "DPORT_PRO_RWBLE_IRQ_MAP_REG"]
pub mod dport_pro_rwble_irq_map_reg;
#[doc = "DPORT_PRO_RWBT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rwbt_nmi_map_reg](dport_pro_rwbt_nmi_map_reg) module"]
pub type DPORT_PRO_RWBT_NMI_MAP_REG = crate::Reg<u32, _DPORT_PRO_RWBT_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RWBT_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rwbt_nmi_map_reg::R](dport_pro_rwbt_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RWBT_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rwbt_nmi_map_reg::W](dport_pro_rwbt_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RWBT_NMI_MAP_REG {}
#[doc = "DPORT_PRO_RWBT_NMI_MAP_REG"]
pub mod dport_pro_rwbt_nmi_map_reg;
#[doc = "DPORT_PRO_RWBLE_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rwble_nmi_map_reg](dport_pro_rwble_nmi_map_reg) module"]
pub type DPORT_PRO_RWBLE_NMI_MAP_REG = crate::Reg<u32, _DPORT_PRO_RWBLE_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RWBLE_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rwble_nmi_map_reg::R](dport_pro_rwble_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RWBLE_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rwble_nmi_map_reg::W](dport_pro_rwble_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RWBLE_NMI_MAP_REG {}
#[doc = "DPORT_PRO_RWBLE_NMI_MAP_REG"]
pub mod dport_pro_rwble_nmi_map_reg;
#[doc = "DPORT_PRO_SLC0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_slc0_intr_map_reg](dport_pro_slc0_intr_map_reg) module"]
pub type DPORT_PRO_SLC0_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_SLC0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SLC0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_slc0_intr_map_reg::R](dport_pro_slc0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SLC0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_slc0_intr_map_reg::W](dport_pro_slc0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SLC0_INTR_MAP_REG {}
#[doc = "DPORT_PRO_SLC0_INTR_MAP_REG"]
pub mod dport_pro_slc0_intr_map_reg;
#[doc = "DPORT_PRO_SLC1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_slc1_intr_map_reg](dport_pro_slc1_intr_map_reg) module"]
pub type DPORT_PRO_SLC1_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_SLC1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SLC1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_slc1_intr_map_reg::R](dport_pro_slc1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SLC1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_slc1_intr_map_reg::W](dport_pro_slc1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SLC1_INTR_MAP_REG {}
#[doc = "DPORT_PRO_SLC1_INTR_MAP_REG"]
pub mod dport_pro_slc1_intr_map_reg;
#[doc = "DPORT_PRO_UHCI0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_uhci0_intr_map_reg](dport_pro_uhci0_intr_map_reg) module"]
pub type DPORT_PRO_UHCI0_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_UHCI0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_UHCI0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_uhci0_intr_map_reg::R](dport_pro_uhci0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_UHCI0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_uhci0_intr_map_reg::W](dport_pro_uhci0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_UHCI0_INTR_MAP_REG {}
#[doc = "DPORT_PRO_UHCI0_INTR_MAP_REG"]
pub mod dport_pro_uhci0_intr_map_reg;
#[doc = "DPORT_PRO_UHCI1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_uhci1_intr_map_reg](dport_pro_uhci1_intr_map_reg) module"]
pub type DPORT_PRO_UHCI1_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_UHCI1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_UHCI1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_uhci1_intr_map_reg::R](dport_pro_uhci1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_UHCI1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_uhci1_intr_map_reg::W](dport_pro_uhci1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_UHCI1_INTR_MAP_REG {}
#[doc = "DPORT_PRO_UHCI1_INTR_MAP_REG"]
pub mod dport_pro_uhci1_intr_map_reg;
#[doc = "DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_t0_level_int_map_reg](dport_pro_tg_t0_level_int_map_reg) module"]
pub type DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_t0_level_int_map_reg::R](dport_pro_tg_t0_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_t0_level_int_map_reg::W](dport_pro_tg_t0_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg_t0_level_int_map_reg;
#[doc = "DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_t1_level_int_map_reg](dport_pro_tg_t1_level_int_map_reg) module"]
pub type DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_t1_level_int_map_reg::R](dport_pro_tg_t1_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_t1_level_int_map_reg::W](dport_pro_tg_t1_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg_t1_level_int_map_reg;
#[doc = "DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_wdt_level_int_map_reg](dport_pro_tg_wdt_level_int_map_reg) module"]
pub type DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_wdt_level_int_map_reg::R](dport_pro_tg_wdt_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_wdt_level_int_map_reg::W](dport_pro_tg_wdt_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg_wdt_level_int_map_reg;
#[doc = "DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_lact_level_int_map_reg](dport_pro_tg_lact_level_int_map_reg) module"]
pub type DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_lact_level_int_map_reg::R](dport_pro_tg_lact_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_lact_level_int_map_reg::W](dport_pro_tg_lact_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg_lact_level_int_map_reg;
#[doc = "DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_t0_level_int_map_reg](dport_pro_tg1_t0_level_int_map_reg) module"]
pub type DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_t0_level_int_map_reg::R](dport_pro_tg1_t0_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_t0_level_int_map_reg::W](dport_pro_tg1_t0_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg1_t0_level_int_map_reg;
#[doc = "DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_t1_level_int_map_reg](dport_pro_tg1_t1_level_int_map_reg) module"]
pub type DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_t1_level_int_map_reg::R](dport_pro_tg1_t1_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_t1_level_int_map_reg::W](dport_pro_tg1_t1_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg1_t1_level_int_map_reg;
#[doc = "DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_wdt_level_int_map_reg](dport_pro_tg1_wdt_level_int_map_reg) module"]
pub type DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_wdt_level_int_map_reg::R](dport_pro_tg1_wdt_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_wdt_level_int_map_reg::W](dport_pro_tg1_wdt_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg1_wdt_level_int_map_reg;
#[doc = "DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_lact_level_int_map_reg](dport_pro_tg1_lact_level_int_map_reg) module"]
pub type DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_lact_level_int_map_reg::R](dport_pro_tg1_lact_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_lact_level_int_map_reg::W](dport_pro_tg1_lact_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG"]
pub mod dport_pro_tg1_lact_level_int_map_reg;
#[doc = "DPORT_PRO_GPIO_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_gpio_interrupt_map_reg](dport_pro_gpio_interrupt_map_reg) module"]
pub type DPORT_PRO_GPIO_INTERRUPT_MAP_REG = crate::Reg<u32, _DPORT_PRO_GPIO_INTERRUPT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_GPIO_INTERRUPT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_gpio_interrupt_map_reg::R](dport_pro_gpio_interrupt_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_GPIO_INTERRUPT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_gpio_interrupt_map_reg::W](dport_pro_gpio_interrupt_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_GPIO_INTERRUPT_MAP_REG {}
#[doc = "DPORT_PRO_GPIO_INTERRUPT_MAP_REG"]
pub mod dport_pro_gpio_interrupt_map_reg;
#[doc = "DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_gpio_interrupt_nmi_map_reg](dport_pro_gpio_interrupt_nmi_map_reg) module"]
pub type DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_pro_gpio_interrupt_nmi_map_reg::R](dport_pro_gpio_interrupt_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_gpio_interrupt_nmi_map_reg::W](dport_pro_gpio_interrupt_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG {}
#[doc = "DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG"]
pub mod dport_pro_gpio_interrupt_nmi_map_reg;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_intr_from_cpu_0_map_reg](dport_pro_cpu_intr_from_cpu_0_map_reg) module"]
pub type DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG;
#[doc = "`read()` method returns [dport_pro_cpu_intr_from_cpu_0_map_reg::R](dport_pro_cpu_intr_from_cpu_0_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_intr_from_cpu_0_map_reg::W](dport_pro_cpu_intr_from_cpu_0_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG"]
pub mod dport_pro_cpu_intr_from_cpu_0_map_reg;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_intr_from_cpu_1_map_reg](dport_pro_cpu_intr_from_cpu_1_map_reg) module"]
pub type DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG;
#[doc = "`read()` method returns [dport_pro_cpu_intr_from_cpu_1_map_reg::R](dport_pro_cpu_intr_from_cpu_1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_intr_from_cpu_1_map_reg::W](dport_pro_cpu_intr_from_cpu_1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG"]
pub mod dport_pro_cpu_intr_from_cpu_1_map_reg;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_intr_from_cpu_2_map_reg](dport_pro_cpu_intr_from_cpu_2_map_reg) module"]
pub type DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG;
#[doc = "`read()` method returns [dport_pro_cpu_intr_from_cpu_2_map_reg::R](dport_pro_cpu_intr_from_cpu_2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_intr_from_cpu_2_map_reg::W](dport_pro_cpu_intr_from_cpu_2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG"]
pub mod dport_pro_cpu_intr_from_cpu_2_map_reg;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_intr_from_cpu_3_map_reg](dport_pro_cpu_intr_from_cpu_3_map_reg) module"]
pub type DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG;
#[doc = "`read()` method returns [dport_pro_cpu_intr_from_cpu_3_map_reg::R](dport_pro_cpu_intr_from_cpu_3_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_intr_from_cpu_3_map_reg::W](dport_pro_cpu_intr_from_cpu_3_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG"]
pub mod dport_pro_cpu_intr_from_cpu_3_map_reg;
#[doc = "DPORT_PRO_SPI_INTR_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi_intr_0_map_reg](dport_pro_spi_intr_0_map_reg) module"]
pub type DPORT_PRO_SPI_INTR_0_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI_INTR_0_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI_INTR_0_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi_intr_0_map_reg::R](dport_pro_spi_intr_0_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI_INTR_0_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi_intr_0_map_reg::W](dport_pro_spi_intr_0_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI_INTR_0_MAP_REG {}
#[doc = "DPORT_PRO_SPI_INTR_0_MAP_REG"]
pub mod dport_pro_spi_intr_0_map_reg;
#[doc = "DPORT_PRO_SPI_INTR_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi_intr_1_map_reg](dport_pro_spi_intr_1_map_reg) module"]
pub type DPORT_PRO_SPI_INTR_1_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI_INTR_1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI_INTR_1_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi_intr_1_map_reg::R](dport_pro_spi_intr_1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI_INTR_1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi_intr_1_map_reg::W](dport_pro_spi_intr_1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI_INTR_1_MAP_REG {}
#[doc = "DPORT_PRO_SPI_INTR_1_MAP_REG"]
pub mod dport_pro_spi_intr_1_map_reg;
#[doc = "DPORT_PRO_SPI_INTR_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi_intr_2_map_reg](dport_pro_spi_intr_2_map_reg) module"]
pub type DPORT_PRO_SPI_INTR_2_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI_INTR_2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI_INTR_2_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi_intr_2_map_reg::R](dport_pro_spi_intr_2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI_INTR_2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi_intr_2_map_reg::W](dport_pro_spi_intr_2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI_INTR_2_MAP_REG {}
#[doc = "DPORT_PRO_SPI_INTR_2_MAP_REG"]
pub mod dport_pro_spi_intr_2_map_reg;
#[doc = "DPORT_PRO_SPI_INTR_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi_intr_3_map_reg](dport_pro_spi_intr_3_map_reg) module"]
pub type DPORT_PRO_SPI_INTR_3_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI_INTR_3_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI_INTR_3_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi_intr_3_map_reg::R](dport_pro_spi_intr_3_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI_INTR_3_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi_intr_3_map_reg::W](dport_pro_spi_intr_3_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI_INTR_3_MAP_REG {}
#[doc = "DPORT_PRO_SPI_INTR_3_MAP_REG"]
pub mod dport_pro_spi_intr_3_map_reg;
#[doc = "DPORT_PRO_I2S0_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_i2s0_int_map_reg](dport_pro_i2s0_int_map_reg) module"]
pub type DPORT_PRO_I2S0_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_I2S0_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_I2S0_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_i2s0_int_map_reg::R](dport_pro_i2s0_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_I2S0_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_i2s0_int_map_reg::W](dport_pro_i2s0_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_I2S0_INT_MAP_REG {}
#[doc = "DPORT_PRO_I2S0_INT_MAP_REG"]
pub mod dport_pro_i2s0_int_map_reg;
#[doc = "DPORT_PRO_I2S1_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_i2s1_int_map_reg](dport_pro_i2s1_int_map_reg) module"]
pub type DPORT_PRO_I2S1_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_I2S1_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_I2S1_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_i2s1_int_map_reg::R](dport_pro_i2s1_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_I2S1_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_i2s1_int_map_reg::W](dport_pro_i2s1_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_I2S1_INT_MAP_REG {}
#[doc = "DPORT_PRO_I2S1_INT_MAP_REG"]
pub mod dport_pro_i2s1_int_map_reg;
#[doc = "DPORT_PRO_UART_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_uart_intr_map_reg](dport_pro_uart_intr_map_reg) module"]
pub type DPORT_PRO_UART_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_UART_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_UART_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_uart_intr_map_reg::R](dport_pro_uart_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_UART_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_uart_intr_map_reg::W](dport_pro_uart_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_UART_INTR_MAP_REG {}
#[doc = "DPORT_PRO_UART_INTR_MAP_REG"]
pub mod dport_pro_uart_intr_map_reg;
#[doc = "DPORT_PRO_UART1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_uart1_intr_map_reg](dport_pro_uart1_intr_map_reg) module"]
pub type DPORT_PRO_UART1_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_UART1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_UART1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_uart1_intr_map_reg::R](dport_pro_uart1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_UART1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_uart1_intr_map_reg::W](dport_pro_uart1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_UART1_INTR_MAP_REG {}
#[doc = "DPORT_PRO_UART1_INTR_MAP_REG"]
pub mod dport_pro_uart1_intr_map_reg;
#[doc = "DPORT_PRO_UART2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_uart2_intr_map_reg](dport_pro_uart2_intr_map_reg) module"]
pub type DPORT_PRO_UART2_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_UART2_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_UART2_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_uart2_intr_map_reg::R](dport_pro_uart2_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_UART2_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_uart2_intr_map_reg::W](dport_pro_uart2_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_UART2_INTR_MAP_REG {}
#[doc = "DPORT_PRO_UART2_INTR_MAP_REG"]
pub mod dport_pro_uart2_intr_map_reg;
#[doc = "DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_sdio_host_interrupt_map_reg](dport_pro_sdio_host_interrupt_map_reg) module"]
pub type DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_sdio_host_interrupt_map_reg::R](dport_pro_sdio_host_interrupt_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_sdio_host_interrupt_map_reg::W](dport_pro_sdio_host_interrupt_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG {}
#[doc = "DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG"]
pub mod dport_pro_sdio_host_interrupt_map_reg;
#[doc = "DPORT_PRO_EMAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_emac_int_map_reg](dport_pro_emac_int_map_reg) module"]
pub type DPORT_PRO_EMAC_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_EMAC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_EMAC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_emac_int_map_reg::R](dport_pro_emac_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_EMAC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_emac_int_map_reg::W](dport_pro_emac_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_EMAC_INT_MAP_REG {}
#[doc = "DPORT_PRO_EMAC_INT_MAP_REG"]
pub mod dport_pro_emac_int_map_reg;
#[doc = "DPORT_PRO_PWM0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_pwm0_intr_map_reg](dport_pro_pwm0_intr_map_reg) module"]
pub type DPORT_PRO_PWM0_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_PWM0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_PWM0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_pwm0_intr_map_reg::R](dport_pro_pwm0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_PWM0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_pwm0_intr_map_reg::W](dport_pro_pwm0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_PWM0_INTR_MAP_REG {}
#[doc = "DPORT_PRO_PWM0_INTR_MAP_REG"]
pub mod dport_pro_pwm0_intr_map_reg;
#[doc = "DPORT_PRO_PWM1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_pwm1_intr_map_reg](dport_pro_pwm1_intr_map_reg) module"]
pub type DPORT_PRO_PWM1_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_PWM1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_PWM1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_pwm1_intr_map_reg::R](dport_pro_pwm1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_PWM1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_pwm1_intr_map_reg::W](dport_pro_pwm1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_PWM1_INTR_MAP_REG {}
#[doc = "DPORT_PRO_PWM1_INTR_MAP_REG"]
pub mod dport_pro_pwm1_intr_map_reg;
#[doc = "DPORT_PRO_PWM2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_pwm2_intr_map_reg](dport_pro_pwm2_intr_map_reg) module"]
pub type DPORT_PRO_PWM2_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_PWM2_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_PWM2_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_pwm2_intr_map_reg::R](dport_pro_pwm2_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_PWM2_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_pwm2_intr_map_reg::W](dport_pro_pwm2_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_PWM2_INTR_MAP_REG {}
#[doc = "DPORT_PRO_PWM2_INTR_MAP_REG"]
pub mod dport_pro_pwm2_intr_map_reg;
#[doc = "DPORT_PRO_PWM3_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_pwm3_intr_map_reg](dport_pro_pwm3_intr_map_reg) module"]
pub type DPORT_PRO_PWM3_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_PWM3_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_PWM3_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_pwm3_intr_map_reg::R](dport_pro_pwm3_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_PWM3_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_pwm3_intr_map_reg::W](dport_pro_pwm3_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_PWM3_INTR_MAP_REG {}
#[doc = "DPORT_PRO_PWM3_INTR_MAP_REG"]
pub mod dport_pro_pwm3_intr_map_reg;
#[doc = "DPORT_PRO_LEDC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_ledc_int_map_reg](dport_pro_ledc_int_map_reg) module"]
pub type DPORT_PRO_LEDC_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_LEDC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_LEDC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_ledc_int_map_reg::R](dport_pro_ledc_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_LEDC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_ledc_int_map_reg::W](dport_pro_ledc_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_LEDC_INT_MAP_REG {}
#[doc = "DPORT_PRO_LEDC_INT_MAP_REG"]
pub mod dport_pro_ledc_int_map_reg;
#[doc = "DPORT_PRO_EFUSE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_efuse_int_map_reg](dport_pro_efuse_int_map_reg) module"]
pub type DPORT_PRO_EFUSE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_EFUSE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_EFUSE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_efuse_int_map_reg::R](dport_pro_efuse_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_EFUSE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_efuse_int_map_reg::W](dport_pro_efuse_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_EFUSE_INT_MAP_REG {}
#[doc = "DPORT_PRO_EFUSE_INT_MAP_REG"]
pub mod dport_pro_efuse_int_map_reg;
#[doc = "DPORT_PRO_CAN_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_can_int_map_reg](dport_pro_can_int_map_reg) module"]
pub type DPORT_PRO_CAN_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_CAN_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CAN_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_can_int_map_reg::R](dport_pro_can_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CAN_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_can_int_map_reg::W](dport_pro_can_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CAN_INT_MAP_REG {}
#[doc = "DPORT_PRO_CAN_INT_MAP_REG"]
pub mod dport_pro_can_int_map_reg;
#[doc = "DPORT_PRO_RTC_CORE_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rtc_core_intr_map_reg](dport_pro_rtc_core_intr_map_reg) module"]
pub type DPORT_PRO_RTC_CORE_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_RTC_CORE_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RTC_CORE_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rtc_core_intr_map_reg::R](dport_pro_rtc_core_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RTC_CORE_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rtc_core_intr_map_reg::W](dport_pro_rtc_core_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RTC_CORE_INTR_MAP_REG {}
#[doc = "DPORT_PRO_RTC_CORE_INTR_MAP_REG"]
pub mod dport_pro_rtc_core_intr_map_reg;
#[doc = "DPORT_PRO_RMT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rmt_intr_map_reg](dport_pro_rmt_intr_map_reg) module"]
pub type DPORT_PRO_RMT_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_RMT_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RMT_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rmt_intr_map_reg::R](dport_pro_rmt_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RMT_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rmt_intr_map_reg::W](dport_pro_rmt_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RMT_INTR_MAP_REG {}
#[doc = "DPORT_PRO_RMT_INTR_MAP_REG"]
pub mod dport_pro_rmt_intr_map_reg;
#[doc = "DPORT_PRO_PCNT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_pcnt_intr_map_reg](dport_pro_pcnt_intr_map_reg) module"]
pub type DPORT_PRO_PCNT_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_PCNT_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_PCNT_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_pcnt_intr_map_reg::R](dport_pro_pcnt_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_PCNT_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_pcnt_intr_map_reg::W](dport_pro_pcnt_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_PCNT_INTR_MAP_REG {}
#[doc = "DPORT_PRO_PCNT_INTR_MAP_REG"]
pub mod dport_pro_pcnt_intr_map_reg;
#[doc = "DPORT_PRO_I2C_EXT0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_i2c_ext0_intr_map_reg](dport_pro_i2c_ext0_intr_map_reg) module"]
pub type DPORT_PRO_I2C_EXT0_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_I2C_EXT0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_I2C_EXT0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_i2c_ext0_intr_map_reg::R](dport_pro_i2c_ext0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_I2C_EXT0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_i2c_ext0_intr_map_reg::W](dport_pro_i2c_ext0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_I2C_EXT0_INTR_MAP_REG {}
#[doc = "DPORT_PRO_I2C_EXT0_INTR_MAP_REG"]
pub mod dport_pro_i2c_ext0_intr_map_reg;
#[doc = "DPORT_PRO_I2C_EXT1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_i2c_ext1_intr_map_reg](dport_pro_i2c_ext1_intr_map_reg) module"]
pub type DPORT_PRO_I2C_EXT1_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_I2C_EXT1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_I2C_EXT1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_i2c_ext1_intr_map_reg::R](dport_pro_i2c_ext1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_I2C_EXT1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_i2c_ext1_intr_map_reg::W](dport_pro_i2c_ext1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_I2C_EXT1_INTR_MAP_REG {}
#[doc = "DPORT_PRO_I2C_EXT1_INTR_MAP_REG"]
pub mod dport_pro_i2c_ext1_intr_map_reg;
#[doc = "DPORT_PRO_RSA_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_rsa_intr_map_reg](dport_pro_rsa_intr_map_reg) module"]
pub type DPORT_PRO_RSA_INTR_MAP_REG = crate::Reg<u32, _DPORT_PRO_RSA_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_RSA_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_pro_rsa_intr_map_reg::R](dport_pro_rsa_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_RSA_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_rsa_intr_map_reg::W](dport_pro_rsa_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_RSA_INTR_MAP_REG {}
#[doc = "DPORT_PRO_RSA_INTR_MAP_REG"]
pub mod dport_pro_rsa_intr_map_reg;
#[doc = "DPORT_PRO_SPI1_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi1_dma_int_map_reg](dport_pro_spi1_dma_int_map_reg) module"]
pub type DPORT_PRO_SPI1_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI1_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI1_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi1_dma_int_map_reg::R](dport_pro_spi1_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI1_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi1_dma_int_map_reg::W](dport_pro_spi1_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI1_DMA_INT_MAP_REG {}
#[doc = "DPORT_PRO_SPI1_DMA_INT_MAP_REG"]
pub mod dport_pro_spi1_dma_int_map_reg;
#[doc = "DPORT_PRO_SPI2_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi2_dma_int_map_reg](dport_pro_spi2_dma_int_map_reg) module"]
pub type DPORT_PRO_SPI2_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI2_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI2_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi2_dma_int_map_reg::R](dport_pro_spi2_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI2_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi2_dma_int_map_reg::W](dport_pro_spi2_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI2_DMA_INT_MAP_REG {}
#[doc = "DPORT_PRO_SPI2_DMA_INT_MAP_REG"]
pub mod dport_pro_spi2_dma_int_map_reg;
#[doc = "DPORT_PRO_SPI3_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_spi3_dma_int_map_reg](dport_pro_spi3_dma_int_map_reg) module"]
pub type DPORT_PRO_SPI3_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_SPI3_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_SPI3_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_spi3_dma_int_map_reg::R](dport_pro_spi3_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_SPI3_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_spi3_dma_int_map_reg::W](dport_pro_spi3_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_SPI3_DMA_INT_MAP_REG {}
#[doc = "DPORT_PRO_SPI3_DMA_INT_MAP_REG"]
pub mod dport_pro_spi3_dma_int_map_reg;
#[doc = "DPORT_PRO_WDG_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_wdg_int_map_reg](dport_pro_wdg_int_map_reg) module"]
pub type DPORT_PRO_WDG_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_WDG_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_WDG_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_wdg_int_map_reg::R](dport_pro_wdg_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_WDG_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_wdg_int_map_reg::W](dport_pro_wdg_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_WDG_INT_MAP_REG {}
#[doc = "DPORT_PRO_WDG_INT_MAP_REG"]
pub mod dport_pro_wdg_int_map_reg;
#[doc = "DPORT_PRO_TIMER_INT1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_timer_int1_map_reg](dport_pro_timer_int1_map_reg) module"]
pub type DPORT_PRO_TIMER_INT1_MAP_REG = crate::Reg<u32, _DPORT_PRO_TIMER_INT1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TIMER_INT1_MAP_REG;
#[doc = "`read()` method returns [dport_pro_timer_int1_map_reg::R](dport_pro_timer_int1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TIMER_INT1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_timer_int1_map_reg::W](dport_pro_timer_int1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TIMER_INT1_MAP_REG {}
#[doc = "DPORT_PRO_TIMER_INT1_MAP_REG"]
pub mod dport_pro_timer_int1_map_reg;
#[doc = "DPORT_PRO_TIMER_INT2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_timer_int2_map_reg](dport_pro_timer_int2_map_reg) module"]
pub type DPORT_PRO_TIMER_INT2_MAP_REG = crate::Reg<u32, _DPORT_PRO_TIMER_INT2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TIMER_INT2_MAP_REG;
#[doc = "`read()` method returns [dport_pro_timer_int2_map_reg::R](dport_pro_timer_int2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TIMER_INT2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_timer_int2_map_reg::W](dport_pro_timer_int2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TIMER_INT2_MAP_REG {}
#[doc = "DPORT_PRO_TIMER_INT2_MAP_REG"]
pub mod dport_pro_timer_int2_map_reg;
#[doc = "DPORT_PRO_TG_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_t0_edge_int_map_reg](dport_pro_tg_t0_edge_int_map_reg) module"]
pub type DPORT_PRO_TG_T0_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_T0_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_T0_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_t0_edge_int_map_reg::R](dport_pro_tg_t0_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_T0_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_t0_edge_int_map_reg::W](dport_pro_tg_t0_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_T0_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_T0_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg_t0_edge_int_map_reg;
#[doc = "DPORT_PRO_TG_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_t1_edge_int_map_reg](dport_pro_tg_t1_edge_int_map_reg) module"]
pub type DPORT_PRO_TG_T1_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_T1_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_T1_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_t1_edge_int_map_reg::R](dport_pro_tg_t1_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_T1_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_t1_edge_int_map_reg::W](dport_pro_tg_t1_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_T1_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_T1_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg_t1_edge_int_map_reg;
#[doc = "DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_wdt_edge_int_map_reg](dport_pro_tg_wdt_edge_int_map_reg) module"]
pub type DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_wdt_edge_int_map_reg::R](dport_pro_tg_wdt_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_wdt_edge_int_map_reg::W](dport_pro_tg_wdt_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg_wdt_edge_int_map_reg;
#[doc = "DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg_lact_edge_int_map_reg](dport_pro_tg_lact_edge_int_map_reg) module"]
pub type DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg_lact_edge_int_map_reg::R](dport_pro_tg_lact_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg_lact_edge_int_map_reg::W](dport_pro_tg_lact_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg_lact_edge_int_map_reg;
#[doc = "DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_t0_edge_int_map_reg](dport_pro_tg1_t0_edge_int_map_reg) module"]
pub type DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_t0_edge_int_map_reg::R](dport_pro_tg1_t0_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_t0_edge_int_map_reg::W](dport_pro_tg1_t0_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg1_t0_edge_int_map_reg;
#[doc = "DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_t1_edge_int_map_reg](dport_pro_tg1_t1_edge_int_map_reg) module"]
pub type DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_t1_edge_int_map_reg::R](dport_pro_tg1_t1_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_t1_edge_int_map_reg::W](dport_pro_tg1_t1_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg1_t1_edge_int_map_reg;
#[doc = "DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_wdt_edge_int_map_reg](dport_pro_tg1_wdt_edge_int_map_reg) module"]
pub type DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_wdt_edge_int_map_reg::R](dport_pro_tg1_wdt_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_wdt_edge_int_map_reg::W](dport_pro_tg1_wdt_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg1_wdt_edge_int_map_reg;
#[doc = "DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_tg1_lact_edge_int_map_reg](dport_pro_tg1_lact_edge_int_map_reg) module"]
pub type DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG =
    crate::Reg<u32, _DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_tg1_lact_edge_int_map_reg::R](dport_pro_tg1_lact_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_tg1_lact_edge_int_map_reg::W](dport_pro_tg1_lact_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG"]
pub mod dport_pro_tg1_lact_edge_int_map_reg;
#[doc = "DPORT_PRO_MMU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_mmu_ia_int_map_reg](dport_pro_mmu_ia_int_map_reg) module"]
pub type DPORT_PRO_MMU_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_MMU_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_MMU_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_mmu_ia_int_map_reg::R](dport_pro_mmu_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_MMU_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_mmu_ia_int_map_reg::W](dport_pro_mmu_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_MMU_IA_INT_MAP_REG {}
#[doc = "DPORT_PRO_MMU_IA_INT_MAP_REG"]
pub mod dport_pro_mmu_ia_int_map_reg;
#[doc = "DPORT_PRO_MPU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_mpu_ia_int_map_reg](dport_pro_mpu_ia_int_map_reg) module"]
pub type DPORT_PRO_MPU_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_MPU_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_MPU_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_mpu_ia_int_map_reg::R](dport_pro_mpu_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_MPU_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_mpu_ia_int_map_reg::W](dport_pro_mpu_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_MPU_IA_INT_MAP_REG {}
#[doc = "DPORT_PRO_MPU_IA_INT_MAP_REG"]
pub mod dport_pro_mpu_ia_int_map_reg;
#[doc = "DPORT_PRO_CACHE_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cache_ia_int_map_reg](dport_pro_cache_ia_int_map_reg) module"]
pub type DPORT_PRO_CACHE_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_PRO_CACHE_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CACHE_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_pro_cache_ia_int_map_reg::R](dport_pro_cache_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CACHE_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cache_ia_int_map_reg::W](dport_pro_cache_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CACHE_IA_INT_MAP_REG {}
#[doc = "DPORT_PRO_CACHE_IA_INT_MAP_REG"]
pub mod dport_pro_cache_ia_int_map_reg;
#[doc = "DPORT_APP_MAC_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_mac_intr_map_reg](dport_app_mac_intr_map_reg) module"]
pub type DPORT_APP_MAC_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_MAC_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_MAC_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_mac_intr_map_reg::R](dport_app_mac_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_MAC_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_mac_intr_map_reg::W](dport_app_mac_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_MAC_INTR_MAP_REG {}
#[doc = "DPORT_APP_MAC_INTR_MAP_REG"]
pub mod dport_app_mac_intr_map_reg;
#[doc = "DPORT_APP_MAC_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_mac_nmi_map_reg](dport_app_mac_nmi_map_reg) module"]
pub type DPORT_APP_MAC_NMI_MAP_REG = crate::Reg<u32, _DPORT_APP_MAC_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_MAC_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_app_mac_nmi_map_reg::R](dport_app_mac_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_MAC_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_mac_nmi_map_reg::W](dport_app_mac_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_MAC_NMI_MAP_REG {}
#[doc = "DPORT_APP_MAC_NMI_MAP_REG"]
pub mod dport_app_mac_nmi_map_reg;
#[doc = "DPORT_APP_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_bb_int_map_reg](dport_app_bb_int_map_reg) module"]
pub type DPORT_APP_BB_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_BB_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_BB_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_bb_int_map_reg::R](dport_app_bb_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_BB_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_bb_int_map_reg::W](dport_app_bb_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_BB_INT_MAP_REG {}
#[doc = "DPORT_APP_BB_INT_MAP_REG"]
pub mod dport_app_bb_int_map_reg;
#[doc = "DPORT_APP_BT_MAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_bt_mac_int_map_reg](dport_app_bt_mac_int_map_reg) module"]
pub type DPORT_APP_BT_MAC_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_BT_MAC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_BT_MAC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_bt_mac_int_map_reg::R](dport_app_bt_mac_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_BT_MAC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_bt_mac_int_map_reg::W](dport_app_bt_mac_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_BT_MAC_INT_MAP_REG {}
#[doc = "DPORT_APP_BT_MAC_INT_MAP_REG"]
pub mod dport_app_bt_mac_int_map_reg;
#[doc = "DPORT_APP_BT_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_bt_bb_int_map_reg](dport_app_bt_bb_int_map_reg) module"]
pub type DPORT_APP_BT_BB_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_BT_BB_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_BT_BB_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_bt_bb_int_map_reg::R](dport_app_bt_bb_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_BT_BB_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_bt_bb_int_map_reg::W](dport_app_bt_bb_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_BT_BB_INT_MAP_REG {}
#[doc = "DPORT_APP_BT_BB_INT_MAP_REG"]
pub mod dport_app_bt_bb_int_map_reg;
#[doc = "DPORT_APP_BT_BB_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_bt_bb_nmi_map_reg](dport_app_bt_bb_nmi_map_reg) module"]
pub type DPORT_APP_BT_BB_NMI_MAP_REG = crate::Reg<u32, _DPORT_APP_BT_BB_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_BT_BB_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_app_bt_bb_nmi_map_reg::R](dport_app_bt_bb_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_BT_BB_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_bt_bb_nmi_map_reg::W](dport_app_bt_bb_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_BT_BB_NMI_MAP_REG {}
#[doc = "DPORT_APP_BT_BB_NMI_MAP_REG"]
pub mod dport_app_bt_bb_nmi_map_reg;
#[doc = "DPORT_APP_RWBT_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rwbt_irq_map_reg](dport_app_rwbt_irq_map_reg) module"]
pub type DPORT_APP_RWBT_IRQ_MAP_REG = crate::Reg<u32, _DPORT_APP_RWBT_IRQ_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RWBT_IRQ_MAP_REG;
#[doc = "`read()` method returns [dport_app_rwbt_irq_map_reg::R](dport_app_rwbt_irq_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RWBT_IRQ_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rwbt_irq_map_reg::W](dport_app_rwbt_irq_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RWBT_IRQ_MAP_REG {}
#[doc = "DPORT_APP_RWBT_IRQ_MAP_REG"]
pub mod dport_app_rwbt_irq_map_reg;
#[doc = "DPORT_APP_RWBLE_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rwble_irq_map_reg](dport_app_rwble_irq_map_reg) module"]
pub type DPORT_APP_RWBLE_IRQ_MAP_REG = crate::Reg<u32, _DPORT_APP_RWBLE_IRQ_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RWBLE_IRQ_MAP_REG;
#[doc = "`read()` method returns [dport_app_rwble_irq_map_reg::R](dport_app_rwble_irq_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RWBLE_IRQ_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rwble_irq_map_reg::W](dport_app_rwble_irq_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RWBLE_IRQ_MAP_REG {}
#[doc = "DPORT_APP_RWBLE_IRQ_MAP_REG"]
pub mod dport_app_rwble_irq_map_reg;
#[doc = "DPORT_APP_RWBT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rwbt_nmi_map_reg](dport_app_rwbt_nmi_map_reg) module"]
pub type DPORT_APP_RWBT_NMI_MAP_REG = crate::Reg<u32, _DPORT_APP_RWBT_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RWBT_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_app_rwbt_nmi_map_reg::R](dport_app_rwbt_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RWBT_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rwbt_nmi_map_reg::W](dport_app_rwbt_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RWBT_NMI_MAP_REG {}
#[doc = "DPORT_APP_RWBT_NMI_MAP_REG"]
pub mod dport_app_rwbt_nmi_map_reg;
#[doc = "DPORT_APP_RWBLE_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rwble_nmi_map_reg](dport_app_rwble_nmi_map_reg) module"]
pub type DPORT_APP_RWBLE_NMI_MAP_REG = crate::Reg<u32, _DPORT_APP_RWBLE_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RWBLE_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_app_rwble_nmi_map_reg::R](dport_app_rwble_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RWBLE_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rwble_nmi_map_reg::W](dport_app_rwble_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RWBLE_NMI_MAP_REG {}
#[doc = "DPORT_APP_RWBLE_NMI_MAP_REG"]
pub mod dport_app_rwble_nmi_map_reg;
#[doc = "DPORT_APP_SLC0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_slc0_intr_map_reg](dport_app_slc0_intr_map_reg) module"]
pub type DPORT_APP_SLC0_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_SLC0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SLC0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_slc0_intr_map_reg::R](dport_app_slc0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SLC0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_slc0_intr_map_reg::W](dport_app_slc0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SLC0_INTR_MAP_REG {}
#[doc = "DPORT_APP_SLC0_INTR_MAP_REG"]
pub mod dport_app_slc0_intr_map_reg;
#[doc = "DPORT_APP_SLC1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_slc1_intr_map_reg](dport_app_slc1_intr_map_reg) module"]
pub type DPORT_APP_SLC1_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_SLC1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SLC1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_slc1_intr_map_reg::R](dport_app_slc1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SLC1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_slc1_intr_map_reg::W](dport_app_slc1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SLC1_INTR_MAP_REG {}
#[doc = "DPORT_APP_SLC1_INTR_MAP_REG"]
pub mod dport_app_slc1_intr_map_reg;
#[doc = "DPORT_APP_UHCI0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_uhci0_intr_map_reg](dport_app_uhci0_intr_map_reg) module"]
pub type DPORT_APP_UHCI0_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_UHCI0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_UHCI0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_uhci0_intr_map_reg::R](dport_app_uhci0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_UHCI0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_uhci0_intr_map_reg::W](dport_app_uhci0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_UHCI0_INTR_MAP_REG {}
#[doc = "DPORT_APP_UHCI0_INTR_MAP_REG"]
pub mod dport_app_uhci0_intr_map_reg;
#[doc = "DPORT_APP_UHCI1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_uhci1_intr_map_reg](dport_app_uhci1_intr_map_reg) module"]
pub type DPORT_APP_UHCI1_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_UHCI1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_UHCI1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_uhci1_intr_map_reg::R](dport_app_uhci1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_UHCI1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_uhci1_intr_map_reg::W](dport_app_uhci1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_UHCI1_INTR_MAP_REG {}
#[doc = "DPORT_APP_UHCI1_INTR_MAP_REG"]
pub mod dport_app_uhci1_intr_map_reg;
#[doc = "DPORT_APP_TG_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_t0_level_int_map_reg](dport_app_tg_t0_level_int_map_reg) module"]
pub type DPORT_APP_TG_T0_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_T0_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_T0_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_t0_level_int_map_reg::R](dport_app_tg_t0_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_T0_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_t0_level_int_map_reg::W](dport_app_tg_t0_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_T0_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_T0_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg_t0_level_int_map_reg;
#[doc = "DPORT_APP_TG_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_t1_level_int_map_reg](dport_app_tg_t1_level_int_map_reg) module"]
pub type DPORT_APP_TG_T1_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_T1_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_T1_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_t1_level_int_map_reg::R](dport_app_tg_t1_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_T1_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_t1_level_int_map_reg::W](dport_app_tg_t1_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_T1_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_T1_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg_t1_level_int_map_reg;
#[doc = "DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_wdt_level_int_map_reg](dport_app_tg_wdt_level_int_map_reg) module"]
pub type DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_wdt_level_int_map_reg::R](dport_app_tg_wdt_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_wdt_level_int_map_reg::W](dport_app_tg_wdt_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg_wdt_level_int_map_reg;
#[doc = "DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_lact_level_int_map_reg](dport_app_tg_lact_level_int_map_reg) module"]
pub type DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_lact_level_int_map_reg::R](dport_app_tg_lact_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_lact_level_int_map_reg::W](dport_app_tg_lact_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg_lact_level_int_map_reg;
#[doc = "DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_t0_level_int_map_reg](dport_app_tg1_t0_level_int_map_reg) module"]
pub type DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_t0_level_int_map_reg::R](dport_app_tg1_t0_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_t0_level_int_map_reg::W](dport_app_tg1_t0_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg1_t0_level_int_map_reg;
#[doc = "DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_t1_level_int_map_reg](dport_app_tg1_t1_level_int_map_reg) module"]
pub type DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_t1_level_int_map_reg::R](dport_app_tg1_t1_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_t1_level_int_map_reg::W](dport_app_tg1_t1_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg1_t1_level_int_map_reg;
#[doc = "DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_wdt_level_int_map_reg](dport_app_tg1_wdt_level_int_map_reg) module"]
pub type DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_wdt_level_int_map_reg::R](dport_app_tg1_wdt_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_wdt_level_int_map_reg::W](dport_app_tg1_wdt_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg1_wdt_level_int_map_reg;
#[doc = "DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_lact_level_int_map_reg](dport_app_tg1_lact_level_int_map_reg) module"]
pub type DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG =
    crate::Reg<u32, _DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_lact_level_int_map_reg::R](dport_app_tg1_lact_level_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_lact_level_int_map_reg::W](dport_app_tg1_lact_level_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG"]
pub mod dport_app_tg1_lact_level_int_map_reg;
#[doc = "DPORT_APP_GPIO_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_gpio_interrupt_map_reg](dport_app_gpio_interrupt_map_reg) module"]
pub type DPORT_APP_GPIO_INTERRUPT_MAP_REG = crate::Reg<u32, _DPORT_APP_GPIO_INTERRUPT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_GPIO_INTERRUPT_MAP_REG;
#[doc = "`read()` method returns [dport_app_gpio_interrupt_map_reg::R](dport_app_gpio_interrupt_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_GPIO_INTERRUPT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_gpio_interrupt_map_reg::W](dport_app_gpio_interrupt_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_GPIO_INTERRUPT_MAP_REG {}
#[doc = "DPORT_APP_GPIO_INTERRUPT_MAP_REG"]
pub mod dport_app_gpio_interrupt_map_reg;
#[doc = "DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_gpio_interrupt_nmi_map_reg](dport_app_gpio_interrupt_nmi_map_reg) module"]
pub type DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG =
    crate::Reg<u32, _DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG;
#[doc = "`read()` method returns [dport_app_gpio_interrupt_nmi_map_reg::R](dport_app_gpio_interrupt_nmi_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_gpio_interrupt_nmi_map_reg::W](dport_app_gpio_interrupt_nmi_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG {}
#[doc = "DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG"]
pub mod dport_app_gpio_interrupt_nmi_map_reg;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_intr_from_cpu_0_map_reg](dport_app_cpu_intr_from_cpu_0_map_reg) module"]
pub type DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG =
    crate::Reg<u32, _DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG;
#[doc = "`read()` method returns [dport_app_cpu_intr_from_cpu_0_map_reg::R](dport_app_cpu_intr_from_cpu_0_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_intr_from_cpu_0_map_reg::W](dport_app_cpu_intr_from_cpu_0_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG"]
pub mod dport_app_cpu_intr_from_cpu_0_map_reg;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_intr_from_cpu_1_map_reg](dport_app_cpu_intr_from_cpu_1_map_reg) module"]
pub type DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG =
    crate::Reg<u32, _DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG;
#[doc = "`read()` method returns [dport_app_cpu_intr_from_cpu_1_map_reg::R](dport_app_cpu_intr_from_cpu_1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_intr_from_cpu_1_map_reg::W](dport_app_cpu_intr_from_cpu_1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG"]
pub mod dport_app_cpu_intr_from_cpu_1_map_reg;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_intr_from_cpu_2_map_reg](dport_app_cpu_intr_from_cpu_2_map_reg) module"]
pub type DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG =
    crate::Reg<u32, _DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG;
#[doc = "`read()` method returns [dport_app_cpu_intr_from_cpu_2_map_reg::R](dport_app_cpu_intr_from_cpu_2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_intr_from_cpu_2_map_reg::W](dport_app_cpu_intr_from_cpu_2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG"]
pub mod dport_app_cpu_intr_from_cpu_2_map_reg;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_intr_from_cpu_3_map_reg](dport_app_cpu_intr_from_cpu_3_map_reg) module"]
pub type DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG =
    crate::Reg<u32, _DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG;
#[doc = "`read()` method returns [dport_app_cpu_intr_from_cpu_3_map_reg::R](dport_app_cpu_intr_from_cpu_3_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_intr_from_cpu_3_map_reg::W](dport_app_cpu_intr_from_cpu_3_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG"]
pub mod dport_app_cpu_intr_from_cpu_3_map_reg;
#[doc = "DPORT_APP_SPI_INTR_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi_intr_0_map_reg](dport_app_spi_intr_0_map_reg) module"]
pub type DPORT_APP_SPI_INTR_0_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI_INTR_0_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI_INTR_0_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi_intr_0_map_reg::R](dport_app_spi_intr_0_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI_INTR_0_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi_intr_0_map_reg::W](dport_app_spi_intr_0_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI_INTR_0_MAP_REG {}
#[doc = "DPORT_APP_SPI_INTR_0_MAP_REG"]
pub mod dport_app_spi_intr_0_map_reg;
#[doc = "DPORT_APP_SPI_INTR_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi_intr_1_map_reg](dport_app_spi_intr_1_map_reg) module"]
pub type DPORT_APP_SPI_INTR_1_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI_INTR_1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI_INTR_1_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi_intr_1_map_reg::R](dport_app_spi_intr_1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI_INTR_1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi_intr_1_map_reg::W](dport_app_spi_intr_1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI_INTR_1_MAP_REG {}
#[doc = "DPORT_APP_SPI_INTR_1_MAP_REG"]
pub mod dport_app_spi_intr_1_map_reg;
#[doc = "DPORT_APP_SPI_INTR_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi_intr_2_map_reg](dport_app_spi_intr_2_map_reg) module"]
pub type DPORT_APP_SPI_INTR_2_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI_INTR_2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI_INTR_2_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi_intr_2_map_reg::R](dport_app_spi_intr_2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI_INTR_2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi_intr_2_map_reg::W](dport_app_spi_intr_2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI_INTR_2_MAP_REG {}
#[doc = "DPORT_APP_SPI_INTR_2_MAP_REG"]
pub mod dport_app_spi_intr_2_map_reg;
#[doc = "DPORT_APP_SPI_INTR_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi_intr_3_map_reg](dport_app_spi_intr_3_map_reg) module"]
pub type DPORT_APP_SPI_INTR_3_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI_INTR_3_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI_INTR_3_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi_intr_3_map_reg::R](dport_app_spi_intr_3_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI_INTR_3_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi_intr_3_map_reg::W](dport_app_spi_intr_3_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI_INTR_3_MAP_REG {}
#[doc = "DPORT_APP_SPI_INTR_3_MAP_REG"]
pub mod dport_app_spi_intr_3_map_reg;
#[doc = "DPORT_APP_I2S0_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_i2s0_int_map_reg](dport_app_i2s0_int_map_reg) module"]
pub type DPORT_APP_I2S0_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_I2S0_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_I2S0_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_i2s0_int_map_reg::R](dport_app_i2s0_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_I2S0_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_i2s0_int_map_reg::W](dport_app_i2s0_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_I2S0_INT_MAP_REG {}
#[doc = "DPORT_APP_I2S0_INT_MAP_REG"]
pub mod dport_app_i2s0_int_map_reg;
#[doc = "DPORT_APP_I2S1_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_i2s1_int_map_reg](dport_app_i2s1_int_map_reg) module"]
pub type DPORT_APP_I2S1_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_I2S1_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_I2S1_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_i2s1_int_map_reg::R](dport_app_i2s1_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_I2S1_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_i2s1_int_map_reg::W](dport_app_i2s1_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_I2S1_INT_MAP_REG {}
#[doc = "DPORT_APP_I2S1_INT_MAP_REG"]
pub mod dport_app_i2s1_int_map_reg;
#[doc = "DPORT_APP_UART_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_uart_intr_map_reg](dport_app_uart_intr_map_reg) module"]
pub type DPORT_APP_UART_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_UART_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_UART_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_uart_intr_map_reg::R](dport_app_uart_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_UART_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_uart_intr_map_reg::W](dport_app_uart_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_UART_INTR_MAP_REG {}
#[doc = "DPORT_APP_UART_INTR_MAP_REG"]
pub mod dport_app_uart_intr_map_reg;
#[doc = "DPORT_APP_UART1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_uart1_intr_map_reg](dport_app_uart1_intr_map_reg) module"]
pub type DPORT_APP_UART1_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_UART1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_UART1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_uart1_intr_map_reg::R](dport_app_uart1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_UART1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_uart1_intr_map_reg::W](dport_app_uart1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_UART1_INTR_MAP_REG {}
#[doc = "DPORT_APP_UART1_INTR_MAP_REG"]
pub mod dport_app_uart1_intr_map_reg;
#[doc = "DPORT_APP_UART2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_uart2_intr_map_reg](dport_app_uart2_intr_map_reg) module"]
pub type DPORT_APP_UART2_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_UART2_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_UART2_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_uart2_intr_map_reg::R](dport_app_uart2_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_UART2_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_uart2_intr_map_reg::W](dport_app_uart2_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_UART2_INTR_MAP_REG {}
#[doc = "DPORT_APP_UART2_INTR_MAP_REG"]
pub mod dport_app_uart2_intr_map_reg;
#[doc = "DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_sdio_host_interrupt_map_reg](dport_app_sdio_host_interrupt_map_reg) module"]
pub type DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG =
    crate::Reg<u32, _DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG;
#[doc = "`read()` method returns [dport_app_sdio_host_interrupt_map_reg::R](dport_app_sdio_host_interrupt_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_sdio_host_interrupt_map_reg::W](dport_app_sdio_host_interrupt_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG {}
#[doc = "DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG"]
pub mod dport_app_sdio_host_interrupt_map_reg;
#[doc = "DPORT_APP_EMAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_emac_int_map_reg](dport_app_emac_int_map_reg) module"]
pub type DPORT_APP_EMAC_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_EMAC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_EMAC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_emac_int_map_reg::R](dport_app_emac_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_EMAC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_emac_int_map_reg::W](dport_app_emac_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_EMAC_INT_MAP_REG {}
#[doc = "DPORT_APP_EMAC_INT_MAP_REG"]
pub mod dport_app_emac_int_map_reg;
#[doc = "DPORT_APP_PWM0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_pwm0_intr_map_reg](dport_app_pwm0_intr_map_reg) module"]
pub type DPORT_APP_PWM0_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_PWM0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_PWM0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_pwm0_intr_map_reg::R](dport_app_pwm0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_PWM0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_pwm0_intr_map_reg::W](dport_app_pwm0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_PWM0_INTR_MAP_REG {}
#[doc = "DPORT_APP_PWM0_INTR_MAP_REG"]
pub mod dport_app_pwm0_intr_map_reg;
#[doc = "DPORT_APP_PWM1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_pwm1_intr_map_reg](dport_app_pwm1_intr_map_reg) module"]
pub type DPORT_APP_PWM1_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_PWM1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_PWM1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_pwm1_intr_map_reg::R](dport_app_pwm1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_PWM1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_pwm1_intr_map_reg::W](dport_app_pwm1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_PWM1_INTR_MAP_REG {}
#[doc = "DPORT_APP_PWM1_INTR_MAP_REG"]
pub mod dport_app_pwm1_intr_map_reg;
#[doc = "DPORT_APP_PWM2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_pwm2_intr_map_reg](dport_app_pwm2_intr_map_reg) module"]
pub type DPORT_APP_PWM2_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_PWM2_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_PWM2_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_pwm2_intr_map_reg::R](dport_app_pwm2_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_PWM2_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_pwm2_intr_map_reg::W](dport_app_pwm2_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_PWM2_INTR_MAP_REG {}
#[doc = "DPORT_APP_PWM2_INTR_MAP_REG"]
pub mod dport_app_pwm2_intr_map_reg;
#[doc = "DPORT_APP_PWM3_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_pwm3_intr_map_reg](dport_app_pwm3_intr_map_reg) module"]
pub type DPORT_APP_PWM3_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_PWM3_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_PWM3_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_pwm3_intr_map_reg::R](dport_app_pwm3_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_PWM3_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_pwm3_intr_map_reg::W](dport_app_pwm3_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_PWM3_INTR_MAP_REG {}
#[doc = "DPORT_APP_PWM3_INTR_MAP_REG"]
pub mod dport_app_pwm3_intr_map_reg;
#[doc = "DPORT_APP_LEDC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_ledc_int_map_reg](dport_app_ledc_int_map_reg) module"]
pub type DPORT_APP_LEDC_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_LEDC_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_LEDC_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_ledc_int_map_reg::R](dport_app_ledc_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_LEDC_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_ledc_int_map_reg::W](dport_app_ledc_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_LEDC_INT_MAP_REG {}
#[doc = "DPORT_APP_LEDC_INT_MAP_REG"]
pub mod dport_app_ledc_int_map_reg;
#[doc = "DPORT_APP_EFUSE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_efuse_int_map_reg](dport_app_efuse_int_map_reg) module"]
pub type DPORT_APP_EFUSE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_EFUSE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_EFUSE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_efuse_int_map_reg::R](dport_app_efuse_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_EFUSE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_efuse_int_map_reg::W](dport_app_efuse_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_EFUSE_INT_MAP_REG {}
#[doc = "DPORT_APP_EFUSE_INT_MAP_REG"]
pub mod dport_app_efuse_int_map_reg;
#[doc = "DPORT_APP_CAN_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_can_int_map_reg](dport_app_can_int_map_reg) module"]
pub type DPORT_APP_CAN_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_CAN_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CAN_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_can_int_map_reg::R](dport_app_can_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CAN_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_can_int_map_reg::W](dport_app_can_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CAN_INT_MAP_REG {}
#[doc = "DPORT_APP_CAN_INT_MAP_REG"]
pub mod dport_app_can_int_map_reg;
#[doc = "DPORT_APP_RTC_CORE_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rtc_core_intr_map_reg](dport_app_rtc_core_intr_map_reg) module"]
pub type DPORT_APP_RTC_CORE_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_RTC_CORE_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RTC_CORE_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_rtc_core_intr_map_reg::R](dport_app_rtc_core_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RTC_CORE_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rtc_core_intr_map_reg::W](dport_app_rtc_core_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RTC_CORE_INTR_MAP_REG {}
#[doc = "DPORT_APP_RTC_CORE_INTR_MAP_REG"]
pub mod dport_app_rtc_core_intr_map_reg;
#[doc = "DPORT_APP_RMT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rmt_intr_map_reg](dport_app_rmt_intr_map_reg) module"]
pub type DPORT_APP_RMT_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_RMT_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RMT_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_rmt_intr_map_reg::R](dport_app_rmt_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RMT_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rmt_intr_map_reg::W](dport_app_rmt_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RMT_INTR_MAP_REG {}
#[doc = "DPORT_APP_RMT_INTR_MAP_REG"]
pub mod dport_app_rmt_intr_map_reg;
#[doc = "DPORT_APP_PCNT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_pcnt_intr_map_reg](dport_app_pcnt_intr_map_reg) module"]
pub type DPORT_APP_PCNT_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_PCNT_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_PCNT_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_pcnt_intr_map_reg::R](dport_app_pcnt_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_PCNT_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_pcnt_intr_map_reg::W](dport_app_pcnt_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_PCNT_INTR_MAP_REG {}
#[doc = "DPORT_APP_PCNT_INTR_MAP_REG"]
pub mod dport_app_pcnt_intr_map_reg;
#[doc = "DPORT_APP_I2C_EXT0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_i2c_ext0_intr_map_reg](dport_app_i2c_ext0_intr_map_reg) module"]
pub type DPORT_APP_I2C_EXT0_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_I2C_EXT0_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_I2C_EXT0_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_i2c_ext0_intr_map_reg::R](dport_app_i2c_ext0_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_I2C_EXT0_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_i2c_ext0_intr_map_reg::W](dport_app_i2c_ext0_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_I2C_EXT0_INTR_MAP_REG {}
#[doc = "DPORT_APP_I2C_EXT0_INTR_MAP_REG"]
pub mod dport_app_i2c_ext0_intr_map_reg;
#[doc = "DPORT_APP_I2C_EXT1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_i2c_ext1_intr_map_reg](dport_app_i2c_ext1_intr_map_reg) module"]
pub type DPORT_APP_I2C_EXT1_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_I2C_EXT1_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_I2C_EXT1_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_i2c_ext1_intr_map_reg::R](dport_app_i2c_ext1_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_I2C_EXT1_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_i2c_ext1_intr_map_reg::W](dport_app_i2c_ext1_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_I2C_EXT1_INTR_MAP_REG {}
#[doc = "DPORT_APP_I2C_EXT1_INTR_MAP_REG"]
pub mod dport_app_i2c_ext1_intr_map_reg;
#[doc = "DPORT_APP_RSA_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_rsa_intr_map_reg](dport_app_rsa_intr_map_reg) module"]
pub type DPORT_APP_RSA_INTR_MAP_REG = crate::Reg<u32, _DPORT_APP_RSA_INTR_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_RSA_INTR_MAP_REG;
#[doc = "`read()` method returns [dport_app_rsa_intr_map_reg::R](dport_app_rsa_intr_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_RSA_INTR_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_rsa_intr_map_reg::W](dport_app_rsa_intr_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_RSA_INTR_MAP_REG {}
#[doc = "DPORT_APP_RSA_INTR_MAP_REG"]
pub mod dport_app_rsa_intr_map_reg;
#[doc = "DPORT_APP_SPI1_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi1_dma_int_map_reg](dport_app_spi1_dma_int_map_reg) module"]
pub type DPORT_APP_SPI1_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI1_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI1_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi1_dma_int_map_reg::R](dport_app_spi1_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI1_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi1_dma_int_map_reg::W](dport_app_spi1_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI1_DMA_INT_MAP_REG {}
#[doc = "DPORT_APP_SPI1_DMA_INT_MAP_REG"]
pub mod dport_app_spi1_dma_int_map_reg;
#[doc = "DPORT_APP_SPI2_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi2_dma_int_map_reg](dport_app_spi2_dma_int_map_reg) module"]
pub type DPORT_APP_SPI2_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI2_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI2_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi2_dma_int_map_reg::R](dport_app_spi2_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI2_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi2_dma_int_map_reg::W](dport_app_spi2_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI2_DMA_INT_MAP_REG {}
#[doc = "DPORT_APP_SPI2_DMA_INT_MAP_REG"]
pub mod dport_app_spi2_dma_int_map_reg;
#[doc = "DPORT_APP_SPI3_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_spi3_dma_int_map_reg](dport_app_spi3_dma_int_map_reg) module"]
pub type DPORT_APP_SPI3_DMA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_SPI3_DMA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_SPI3_DMA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_spi3_dma_int_map_reg::R](dport_app_spi3_dma_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_SPI3_DMA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_spi3_dma_int_map_reg::W](dport_app_spi3_dma_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_SPI3_DMA_INT_MAP_REG {}
#[doc = "DPORT_APP_SPI3_DMA_INT_MAP_REG"]
pub mod dport_app_spi3_dma_int_map_reg;
#[doc = "DPORT_APP_WDG_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_wdg_int_map_reg](dport_app_wdg_int_map_reg) module"]
pub type DPORT_APP_WDG_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_WDG_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_WDG_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_wdg_int_map_reg::R](dport_app_wdg_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_WDG_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_wdg_int_map_reg::W](dport_app_wdg_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_WDG_INT_MAP_REG {}
#[doc = "DPORT_APP_WDG_INT_MAP_REG"]
pub mod dport_app_wdg_int_map_reg;
#[doc = "DPORT_APP_TIMER_INT1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_timer_int1_map_reg](dport_app_timer_int1_map_reg) module"]
pub type DPORT_APP_TIMER_INT1_MAP_REG = crate::Reg<u32, _DPORT_APP_TIMER_INT1_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TIMER_INT1_MAP_REG;
#[doc = "`read()` method returns [dport_app_timer_int1_map_reg::R](dport_app_timer_int1_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TIMER_INT1_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_timer_int1_map_reg::W](dport_app_timer_int1_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TIMER_INT1_MAP_REG {}
#[doc = "DPORT_APP_TIMER_INT1_MAP_REG"]
pub mod dport_app_timer_int1_map_reg;
#[doc = "DPORT_APP_TIMER_INT2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_timer_int2_map_reg](dport_app_timer_int2_map_reg) module"]
pub type DPORT_APP_TIMER_INT2_MAP_REG = crate::Reg<u32, _DPORT_APP_TIMER_INT2_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TIMER_INT2_MAP_REG;
#[doc = "`read()` method returns [dport_app_timer_int2_map_reg::R](dport_app_timer_int2_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TIMER_INT2_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_timer_int2_map_reg::W](dport_app_timer_int2_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TIMER_INT2_MAP_REG {}
#[doc = "DPORT_APP_TIMER_INT2_MAP_REG"]
pub mod dport_app_timer_int2_map_reg;
#[doc = "DPORT_APP_TG_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_t0_edge_int_map_reg](dport_app_tg_t0_edge_int_map_reg) module"]
pub type DPORT_APP_TG_T0_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_T0_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_T0_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_t0_edge_int_map_reg::R](dport_app_tg_t0_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_T0_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_t0_edge_int_map_reg::W](dport_app_tg_t0_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_T0_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_T0_EDGE_INT_MAP_REG"]
pub mod dport_app_tg_t0_edge_int_map_reg;
#[doc = "DPORT_APP_TG_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_t1_edge_int_map_reg](dport_app_tg_t1_edge_int_map_reg) module"]
pub type DPORT_APP_TG_T1_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_T1_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_T1_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_t1_edge_int_map_reg::R](dport_app_tg_t1_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_T1_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_t1_edge_int_map_reg::W](dport_app_tg_t1_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_T1_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_T1_EDGE_INT_MAP_REG"]
pub mod dport_app_tg_t1_edge_int_map_reg;
#[doc = "DPORT_APP_TG_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_wdt_edge_int_map_reg](dport_app_tg_wdt_edge_int_map_reg) module"]
pub type DPORT_APP_TG_WDT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_WDT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_WDT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_wdt_edge_int_map_reg::R](dport_app_tg_wdt_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_WDT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_wdt_edge_int_map_reg::W](dport_app_tg_wdt_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_WDT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_WDT_EDGE_INT_MAP_REG"]
pub mod dport_app_tg_wdt_edge_int_map_reg;
#[doc = "DPORT_APP_TG_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg_lact_edge_int_map_reg](dport_app_tg_lact_edge_int_map_reg) module"]
pub type DPORT_APP_TG_LACT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG_LACT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG_LACT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg_lact_edge_int_map_reg::R](dport_app_tg_lact_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG_LACT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg_lact_edge_int_map_reg::W](dport_app_tg_lact_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG_LACT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG_LACT_EDGE_INT_MAP_REG"]
pub mod dport_app_tg_lact_edge_int_map_reg;
#[doc = "DPORT_APP_TG1_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_t0_edge_int_map_reg](dport_app_tg1_t0_edge_int_map_reg) module"]
pub type DPORT_APP_TG1_T0_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG1_T0_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_T0_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_t0_edge_int_map_reg::R](dport_app_tg1_t0_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_T0_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_t0_edge_int_map_reg::W](dport_app_tg1_t0_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_T0_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_T0_EDGE_INT_MAP_REG"]
pub mod dport_app_tg1_t0_edge_int_map_reg;
#[doc = "DPORT_APP_TG1_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_t1_edge_int_map_reg](dport_app_tg1_t1_edge_int_map_reg) module"]
pub type DPORT_APP_TG1_T1_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG1_T1_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_T1_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_t1_edge_int_map_reg::R](dport_app_tg1_t1_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_T1_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_t1_edge_int_map_reg::W](dport_app_tg1_t1_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_T1_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_T1_EDGE_INT_MAP_REG"]
pub mod dport_app_tg1_t1_edge_int_map_reg;
#[doc = "DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_wdt_edge_int_map_reg](dport_app_tg1_wdt_edge_int_map_reg) module"]
pub type DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_wdt_edge_int_map_reg::R](dport_app_tg1_wdt_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_wdt_edge_int_map_reg::W](dport_app_tg1_wdt_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG"]
pub mod dport_app_tg1_wdt_edge_int_map_reg;
#[doc = "DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_tg1_lact_edge_int_map_reg](dport_app_tg1_lact_edge_int_map_reg) module"]
pub type DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG =
    crate::Reg<u32, _DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_tg1_lact_edge_int_map_reg::R](dport_app_tg1_lact_edge_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_tg1_lact_edge_int_map_reg::W](dport_app_tg1_lact_edge_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG {}
#[doc = "DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG"]
pub mod dport_app_tg1_lact_edge_int_map_reg;
#[doc = "DPORT_APP_MMU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_mmu_ia_int_map_reg](dport_app_mmu_ia_int_map_reg) module"]
pub type DPORT_APP_MMU_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_MMU_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_MMU_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_mmu_ia_int_map_reg::R](dport_app_mmu_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_MMU_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_mmu_ia_int_map_reg::W](dport_app_mmu_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_MMU_IA_INT_MAP_REG {}
#[doc = "DPORT_APP_MMU_IA_INT_MAP_REG"]
pub mod dport_app_mmu_ia_int_map_reg;
#[doc = "DPORT_APP_MPU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_mpu_ia_int_map_reg](dport_app_mpu_ia_int_map_reg) module"]
pub type DPORT_APP_MPU_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_MPU_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_MPU_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_mpu_ia_int_map_reg::R](dport_app_mpu_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_MPU_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_mpu_ia_int_map_reg::W](dport_app_mpu_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_MPU_IA_INT_MAP_REG {}
#[doc = "DPORT_APP_MPU_IA_INT_MAP_REG"]
pub mod dport_app_mpu_ia_int_map_reg;
#[doc = "DPORT_APP_CACHE_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cache_ia_int_map_reg](dport_app_cache_ia_int_map_reg) module"]
pub type DPORT_APP_CACHE_IA_INT_MAP_REG = crate::Reg<u32, _DPORT_APP_CACHE_IA_INT_MAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CACHE_IA_INT_MAP_REG;
#[doc = "`read()` method returns [dport_app_cache_ia_int_map_reg::R](dport_app_cache_ia_int_map_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CACHE_IA_INT_MAP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cache_ia_int_map_reg::W](dport_app_cache_ia_int_map_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CACHE_IA_INT_MAP_REG {}
#[doc = "DPORT_APP_CACHE_IA_INT_MAP_REG"]
pub mod dport_app_cache_ia_int_map_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_uart_reg](dport_ahblite_mpu_table_uart_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_UART_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_UART_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_UART_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_uart_reg::R](dport_ahblite_mpu_table_uart_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_UART_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_uart_reg::W](dport_ahblite_mpu_table_uart_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_UART_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART_REG"]
pub mod dport_ahblite_mpu_table_uart_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_spi1_reg](dport_ahblite_mpu_table_spi1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SPI1_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SPI1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SPI1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_spi1_reg::R](dport_ahblite_mpu_table_spi1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SPI1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_spi1_reg::W](dport_ahblite_mpu_table_spi1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SPI1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI1_REG"]
pub mod dport_ahblite_mpu_table_spi1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_spi0_reg](dport_ahblite_mpu_table_spi0_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SPI0_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SPI0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SPI0_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_spi0_reg::R](dport_ahblite_mpu_table_spi0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SPI0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_spi0_reg::W](dport_ahblite_mpu_table_spi0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SPI0_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI0_REG"]
pub mod dport_ahblite_mpu_table_spi0_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_GPIO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_gpio_reg](dport_ahblite_mpu_table_gpio_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_GPIO_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_GPIO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_GPIO_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_gpio_reg::R](dport_ahblite_mpu_table_gpio_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_GPIO_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_gpio_reg::W](dport_ahblite_mpu_table_gpio_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_GPIO_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_GPIO_REG"]
pub mod dport_ahblite_mpu_table_gpio_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_fe2_reg](dport_ahblite_mpu_table_fe2_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_FE2_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_FE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_FE2_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_fe2_reg::R](dport_ahblite_mpu_table_fe2_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_FE2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_fe2_reg::W](dport_ahblite_mpu_table_fe2_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_FE2_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE2_REG"]
pub mod dport_ahblite_mpu_table_fe2_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_fe_reg](dport_ahblite_mpu_table_fe_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_FE_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_FE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_FE_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_fe_reg::R](dport_ahblite_mpu_table_fe_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_FE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_fe_reg::W](dport_ahblite_mpu_table_fe_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_FE_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE_REG"]
pub mod dport_ahblite_mpu_table_fe_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMER_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_timer_reg](dport_ahblite_mpu_table_timer_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_TIMER_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_TIMER_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_TIMER_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_timer_reg::R](dport_ahblite_mpu_table_timer_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_TIMER_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_timer_reg::W](dport_ahblite_mpu_table_timer_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_TIMER_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMER_REG"]
pub mod dport_ahblite_mpu_table_timer_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RTC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_rtc_reg](dport_ahblite_mpu_table_rtc_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_RTC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_RTC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_RTC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_rtc_reg::R](dport_ahblite_mpu_table_rtc_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_RTC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_rtc_reg::W](dport_ahblite_mpu_table_rtc_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_RTC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RTC_REG"]
pub mod dport_ahblite_mpu_table_rtc_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_io_mux_reg](dport_ahblite_mpu_table_io_mux_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_io_mux_reg::R](dport_ahblite_mpu_table_io_mux_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_io_mux_reg::W](dport_ahblite_mpu_table_io_mux_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG"]
pub mod dport_ahblite_mpu_table_io_mux_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_WDG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_wdg_reg](dport_ahblite_mpu_table_wdg_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_WDG_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_WDG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_WDG_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_wdg_reg::R](dport_ahblite_mpu_table_wdg_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_WDG_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_wdg_reg::W](dport_ahblite_mpu_table_wdg_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_WDG_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_WDG_REG"]
pub mod dport_ahblite_mpu_table_wdg_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_HINF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_hinf_reg](dport_ahblite_mpu_table_hinf_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_HINF_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_HINF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_HINF_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_hinf_reg::R](dport_ahblite_mpu_table_hinf_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_HINF_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_hinf_reg::W](dport_ahblite_mpu_table_hinf_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_HINF_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_HINF_REG"]
pub mod dport_ahblite_mpu_table_hinf_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_uhci1_reg](dport_ahblite_mpu_table_uhci1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_UHCI1_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_UHCI1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_UHCI1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_uhci1_reg::R](dport_ahblite_mpu_table_uhci1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_UHCI1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_uhci1_reg::W](dport_ahblite_mpu_table_uhci1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_UHCI1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI1_REG"]
pub mod dport_ahblite_mpu_table_uhci1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_MISC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_misc_reg](dport_ahblite_mpu_table_misc_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_MISC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_MISC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_MISC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_misc_reg::R](dport_ahblite_mpu_table_misc_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_MISC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_misc_reg::W](dport_ahblite_mpu_table_misc_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_MISC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_MISC_REG"]
pub mod dport_ahblite_mpu_table_misc_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_i2c_reg](dport_ahblite_mpu_table_i2c_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_I2C_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_I2C_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_I2C_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_i2c_reg::R](dport_ahblite_mpu_table_i2c_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_I2C_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_i2c_reg::W](dport_ahblite_mpu_table_i2c_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_I2C_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_REG"]
pub mod dport_ahblite_mpu_table_i2c_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_i2s0_reg](dport_ahblite_mpu_table_i2s0_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_I2S0_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_I2S0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_I2S0_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_i2s0_reg::R](dport_ahblite_mpu_table_i2s0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_I2S0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_i2s0_reg::W](dport_ahblite_mpu_table_i2s0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_I2S0_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S0_REG"]
pub mod dport_ahblite_mpu_table_i2s0_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_uart1_reg](dport_ahblite_mpu_table_uart1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_UART1_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_UART1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_UART1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_uart1_reg::R](dport_ahblite_mpu_table_uart1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_UART1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_uart1_reg::W](dport_ahblite_mpu_table_uart1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_UART1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART1_REG"]
pub mod dport_ahblite_mpu_table_uart1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_bt_reg](dport_ahblite_mpu_table_bt_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_BT_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_BT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_BT_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_bt_reg::R](dport_ahblite_mpu_table_bt_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_BT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_bt_reg::W](dport_ahblite_mpu_table_bt_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_BT_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_REG"]
pub mod dport_ahblite_mpu_table_bt_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_bt_buffer_reg](dport_ahblite_mpu_table_bt_buffer_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_bt_buffer_reg::R](dport_ahblite_mpu_table_bt_buffer_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_bt_buffer_reg::W](dport_ahblite_mpu_table_bt_buffer_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG"]
pub mod dport_ahblite_mpu_table_bt_buffer_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_i2c_ext0_reg](dport_ahblite_mpu_table_i2c_ext0_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_i2c_ext0_reg::R](dport_ahblite_mpu_table_i2c_ext0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_i2c_ext0_reg::W](dport_ahblite_mpu_table_i2c_ext0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG"]
pub mod dport_ahblite_mpu_table_i2c_ext0_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_uhci0_reg](dport_ahblite_mpu_table_uhci0_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_UHCI0_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_UHCI0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_UHCI0_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_uhci0_reg::R](dport_ahblite_mpu_table_uhci0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_UHCI0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_uhci0_reg::W](dport_ahblite_mpu_table_uhci0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_UHCI0_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI0_REG"]
pub mod dport_ahblite_mpu_table_uhci0_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_slchost_reg](dport_ahblite_mpu_table_slchost_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_slchost_reg::R](dport_ahblite_mpu_table_slchost_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_slchost_reg::W](dport_ahblite_mpu_table_slchost_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG"]
pub mod dport_ahblite_mpu_table_slchost_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RMT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_rmt_reg](dport_ahblite_mpu_table_rmt_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_RMT_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_RMT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_RMT_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_rmt_reg::R](dport_ahblite_mpu_table_rmt_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_RMT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_rmt_reg::W](dport_ahblite_mpu_table_rmt_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_RMT_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RMT_REG"]
pub mod dport_ahblite_mpu_table_rmt_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PCNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pcnt_reg](dport_ahblite_mpu_table_pcnt_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PCNT_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PCNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PCNT_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pcnt_reg::R](dport_ahblite_mpu_table_pcnt_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PCNT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pcnt_reg::W](dport_ahblite_mpu_table_pcnt_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PCNT_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PCNT_REG"]
pub mod dport_ahblite_mpu_table_pcnt_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_slc_reg](dport_ahblite_mpu_table_slc_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SLC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SLC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SLC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_slc_reg::R](dport_ahblite_mpu_table_slc_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SLC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_slc_reg::W](dport_ahblite_mpu_table_slc_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SLC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLC_REG"]
pub mod dport_ahblite_mpu_table_slc_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_LEDC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_ledc_reg](dport_ahblite_mpu_table_ledc_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_LEDC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_LEDC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_LEDC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_ledc_reg::R](dport_ahblite_mpu_table_ledc_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_LEDC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_ledc_reg::W](dport_ahblite_mpu_table_ledc_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_LEDC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_LEDC_REG"]
pub mod dport_ahblite_mpu_table_ledc_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_EFUSE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_efuse_reg](dport_ahblite_mpu_table_efuse_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_EFUSE_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_EFUSE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_EFUSE_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_efuse_reg::R](dport_ahblite_mpu_table_efuse_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_EFUSE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_efuse_reg::W](dport_ahblite_mpu_table_efuse_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_EFUSE_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_EFUSE_REG"]
pub mod dport_ahblite_mpu_table_efuse_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_spi_encrypt_reg](dport_ahblite_mpu_table_spi_encrypt_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_spi_encrypt_reg::R](dport_ahblite_mpu_table_spi_encrypt_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_spi_encrypt_reg::W](dport_ahblite_mpu_table_spi_encrypt_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
pub mod dport_ahblite_mpu_table_spi_encrypt_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BB_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_bb_reg](dport_ahblite_mpu_table_bb_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_BB_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_BB_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_BB_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_bb_reg::R](dport_ahblite_mpu_table_bb_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_BB_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_bb_reg::W](dport_ahblite_mpu_table_bb_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_BB_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BB_REG"]
pub mod dport_ahblite_mpu_table_bb_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pwm0_reg](dport_ahblite_mpu_table_pwm0_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PWM0_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PWM0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PWM0_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pwm0_reg::R](dport_ahblite_mpu_table_pwm0_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PWM0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pwm0_reg::W](dport_ahblite_mpu_table_pwm0_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PWM0_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM0_REG"]
pub mod dport_ahblite_mpu_table_pwm0_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_timergroup_reg](dport_ahblite_mpu_table_timergroup_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_timergroup_reg::R](dport_ahblite_mpu_table_timergroup_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_timergroup_reg::W](dport_ahblite_mpu_table_timergroup_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG"]
pub mod dport_ahblite_mpu_table_timergroup_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_timergroup1_reg](dport_ahblite_mpu_table_timergroup1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_timergroup1_reg::R](dport_ahblite_mpu_table_timergroup1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_timergroup1_reg::W](dport_ahblite_mpu_table_timergroup1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG"]
pub mod dport_ahblite_mpu_table_timergroup1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_spi2_reg](dport_ahblite_mpu_table_spi2_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SPI2_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SPI2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SPI2_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_spi2_reg::R](dport_ahblite_mpu_table_spi2_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SPI2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_spi2_reg::W](dport_ahblite_mpu_table_spi2_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SPI2_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI2_REG"]
pub mod dport_ahblite_mpu_table_spi2_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_spi3_reg](dport_ahblite_mpu_table_spi3_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SPI3_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SPI3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SPI3_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_spi3_reg::R](dport_ahblite_mpu_table_spi3_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SPI3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_spi3_reg::W](dport_ahblite_mpu_table_spi3_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SPI3_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI3_REG"]
pub mod dport_ahblite_mpu_table_spi3_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_apb_ctrl_reg](dport_ahblite_mpu_table_apb_ctrl_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_apb_ctrl_reg::R](dport_ahblite_mpu_table_apb_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_apb_ctrl_reg::W](dport_ahblite_mpu_table_apb_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG"]
pub mod dport_ahblite_mpu_table_apb_ctrl_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_i2c_ext1_reg](dport_ahblite_mpu_table_i2c_ext1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_i2c_ext1_reg::R](dport_ahblite_mpu_table_i2c_ext1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_i2c_ext1_reg::W](dport_ahblite_mpu_table_i2c_ext1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG"]
pub mod dport_ahblite_mpu_table_i2c_ext1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_sdio_host_reg](dport_ahblite_mpu_table_sdio_host_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_sdio_host_reg::R](dport_ahblite_mpu_table_sdio_host_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_sdio_host_reg::W](dport_ahblite_mpu_table_sdio_host_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG"]
pub mod dport_ahblite_mpu_table_sdio_host_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_EMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_emac_reg](dport_ahblite_mpu_table_emac_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_EMAC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_EMAC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_EMAC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_emac_reg::R](dport_ahblite_mpu_table_emac_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_EMAC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_emac_reg::W](dport_ahblite_mpu_table_emac_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_EMAC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_EMAC_REG"]
pub mod dport_ahblite_mpu_table_emac_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_CAN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_can_reg](dport_ahblite_mpu_table_can_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_CAN_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_CAN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_CAN_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_can_reg::R](dport_ahblite_mpu_table_can_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_CAN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_can_reg::W](dport_ahblite_mpu_table_can_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_CAN_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_CAN_REG"]
pub mod dport_ahblite_mpu_table_can_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pwm1_reg](dport_ahblite_mpu_table_pwm1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PWM1_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PWM1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PWM1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pwm1_reg::R](dport_ahblite_mpu_table_pwm1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PWM1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pwm1_reg::W](dport_ahblite_mpu_table_pwm1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PWM1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM1_REG"]
pub mod dport_ahblite_mpu_table_pwm1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_i2s1_reg](dport_ahblite_mpu_table_i2s1_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_I2S1_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_I2S1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_I2S1_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_i2s1_reg::R](dport_ahblite_mpu_table_i2s1_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_I2S1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_i2s1_reg::W](dport_ahblite_mpu_table_i2s1_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_I2S1_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S1_REG"]
pub mod dport_ahblite_mpu_table_i2s1_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_uart2_reg](dport_ahblite_mpu_table_uart2_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_UART2_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_UART2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_UART2_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_uart2_reg::R](dport_ahblite_mpu_table_uart2_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_UART2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_uart2_reg::W](dport_ahblite_mpu_table_uart2_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_UART2_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART2_REG"]
pub mod dport_ahblite_mpu_table_uart2_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pwm2_reg](dport_ahblite_mpu_table_pwm2_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PWM2_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PWM2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PWM2_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pwm2_reg::R](dport_ahblite_mpu_table_pwm2_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PWM2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pwm2_reg::W](dport_ahblite_mpu_table_pwm2_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PWM2_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM2_REG"]
pub mod dport_ahblite_mpu_table_pwm2_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pwm3_reg](dport_ahblite_mpu_table_pwm3_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PWM3_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PWM3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PWM3_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pwm3_reg::R](dport_ahblite_mpu_table_pwm3_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PWM3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pwm3_reg::W](dport_ahblite_mpu_table_pwm3_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PWM3_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM3_REG"]
pub mod dport_ahblite_mpu_table_pwm3_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RWBT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_rwbt_reg](dport_ahblite_mpu_table_rwbt_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_RWBT_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_RWBT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_RWBT_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_rwbt_reg::R](dport_ahblite_mpu_table_rwbt_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_RWBT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_rwbt_reg::W](dport_ahblite_mpu_table_rwbt_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_RWBT_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RWBT_REG"]
pub mod dport_ahblite_mpu_table_rwbt_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BTMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_btmac_reg](dport_ahblite_mpu_table_btmac_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_BTMAC_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_BTMAC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_BTMAC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_btmac_reg::R](dport_ahblite_mpu_table_btmac_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_BTMAC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_btmac_reg::W](dport_ahblite_mpu_table_btmac_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_BTMAC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BTMAC_REG"]
pub mod dport_ahblite_mpu_table_btmac_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_wifimac_reg](dport_ahblite_mpu_table_wifimac_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG =
    crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_wifimac_reg::R](dport_ahblite_mpu_table_wifimac_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_wifimac_reg::W](dport_ahblite_mpu_table_wifimac_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG"]
pub mod dport_ahblite_mpu_table_wifimac_reg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_ahblite_mpu_table_pwr_reg](dport_ahblite_mpu_table_pwr_reg) module"]
pub type DPORT_AHBLITE_MPU_TABLE_PWR_REG = crate::Reg<u32, _DPORT_AHBLITE_MPU_TABLE_PWR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_AHBLITE_MPU_TABLE_PWR_REG;
#[doc = "`read()` method returns [dport_ahblite_mpu_table_pwr_reg::R](dport_ahblite_mpu_table_pwr_reg::R) reader structure"]
impl crate::Readable for DPORT_AHBLITE_MPU_TABLE_PWR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_ahblite_mpu_table_pwr_reg::W](dport_ahblite_mpu_table_pwr_reg::W) writer structure"]
impl crate::Writable for DPORT_AHBLITE_MPU_TABLE_PWR_REG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWR_REG"]
pub mod dport_ahblite_mpu_table_pwr_reg;
#[doc = "DPORT_MEM_ACCESS_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_mem_access_dbug0_reg](dport_mem_access_dbug0_reg) module"]
pub type DPORT_MEM_ACCESS_DBUG0_REG = crate::Reg<u32, _DPORT_MEM_ACCESS_DBUG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_MEM_ACCESS_DBUG0_REG;
#[doc = "`read()` method returns [dport_mem_access_dbug0_reg::R](dport_mem_access_dbug0_reg::R) reader structure"]
impl crate::Readable for DPORT_MEM_ACCESS_DBUG0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_mem_access_dbug0_reg::W](dport_mem_access_dbug0_reg::W) writer structure"]
impl crate::Writable for DPORT_MEM_ACCESS_DBUG0_REG {}
#[doc = "DPORT_MEM_ACCESS_DBUG0_REG"]
pub mod dport_mem_access_dbug0_reg;
#[doc = "DPORT_MEM_ACCESS_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_mem_access_dbug1_reg](dport_mem_access_dbug1_reg) module"]
pub type DPORT_MEM_ACCESS_DBUG1_REG = crate::Reg<u32, _DPORT_MEM_ACCESS_DBUG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_MEM_ACCESS_DBUG1_REG;
#[doc = "`read()` method returns [dport_mem_access_dbug1_reg::R](dport_mem_access_dbug1_reg::R) reader structure"]
impl crate::Readable for DPORT_MEM_ACCESS_DBUG1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_mem_access_dbug1_reg::W](dport_mem_access_dbug1_reg::W) writer structure"]
impl crate::Writable for DPORT_MEM_ACCESS_DBUG1_REG {}
#[doc = "DPORT_MEM_ACCESS_DBUG1_REG"]
pub mod dport_mem_access_dbug1_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug0_reg](dport_pro_dcache_dbug0_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG0_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG0_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug0_reg::R](dport_pro_dcache_dbug0_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug0_reg::W](dport_pro_dcache_dbug0_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG0_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG0_REG"]
pub mod dport_pro_dcache_dbug0_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug1_reg](dport_pro_dcache_dbug1_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG1_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG1_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug1_reg::R](dport_pro_dcache_dbug1_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug1_reg::W](dport_pro_dcache_dbug1_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG1_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG1_REG"]
pub mod dport_pro_dcache_dbug1_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug2_reg](dport_pro_dcache_dbug2_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG2_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG2_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug2_reg::R](dport_pro_dcache_dbug2_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug2_reg::W](dport_pro_dcache_dbug2_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG2_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG2_REG"]
pub mod dport_pro_dcache_dbug2_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug3_reg](dport_pro_dcache_dbug3_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG3_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG3_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug3_reg::R](dport_pro_dcache_dbug3_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug3_reg::W](dport_pro_dcache_dbug3_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG3_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG3_REG"]
pub mod dport_pro_dcache_dbug3_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug4_reg](dport_pro_dcache_dbug4_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG4_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG4_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug4_reg::R](dport_pro_dcache_dbug4_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG4_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug4_reg::W](dport_pro_dcache_dbug4_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG4_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG4_REG"]
pub mod dport_pro_dcache_dbug4_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug5_reg](dport_pro_dcache_dbug5_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG5_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG5_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug5_reg::R](dport_pro_dcache_dbug5_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG5_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug5_reg::W](dport_pro_dcache_dbug5_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG5_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG5_REG"]
pub mod dport_pro_dcache_dbug5_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug6_reg](dport_pro_dcache_dbug6_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG6_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG6_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug6_reg::R](dport_pro_dcache_dbug6_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG6_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug6_reg::W](dport_pro_dcache_dbug6_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG6_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG6_REG"]
pub mod dport_pro_dcache_dbug6_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug7_reg](dport_pro_dcache_dbug7_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG7_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG7_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug7_reg::R](dport_pro_dcache_dbug7_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG7_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug7_reg::W](dport_pro_dcache_dbug7_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG7_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG7_REG"]
pub mod dport_pro_dcache_dbug7_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug8_reg](dport_pro_dcache_dbug8_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG8_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG8_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug8_reg::R](dport_pro_dcache_dbug8_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG8_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug8_reg::W](dport_pro_dcache_dbug8_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG8_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG8_REG"]
pub mod dport_pro_dcache_dbug8_reg;
#[doc = "DPORT_PRO_DCACHE_DBUG9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_dcache_dbug9_reg](dport_pro_dcache_dbug9_reg) module"]
pub type DPORT_PRO_DCACHE_DBUG9_REG = crate::Reg<u32, _DPORT_PRO_DCACHE_DBUG9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_DCACHE_DBUG9_REG;
#[doc = "`read()` method returns [dport_pro_dcache_dbug9_reg::R](dport_pro_dcache_dbug9_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_DCACHE_DBUG9_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_dcache_dbug9_reg::W](dport_pro_dcache_dbug9_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_DCACHE_DBUG9_REG {}
#[doc = "DPORT_PRO_DCACHE_DBUG9_REG"]
pub mod dport_pro_dcache_dbug9_reg;
#[doc = "DPORT_APP_DCACHE_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug0_reg](dport_app_dcache_dbug0_reg) module"]
pub type DPORT_APP_DCACHE_DBUG0_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG0_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug0_reg::R](dport_app_dcache_dbug0_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug0_reg::W](dport_app_dcache_dbug0_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG0_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG0_REG"]
pub mod dport_app_dcache_dbug0_reg;
#[doc = "DPORT_APP_DCACHE_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug1_reg](dport_app_dcache_dbug1_reg) module"]
pub type DPORT_APP_DCACHE_DBUG1_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG1_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug1_reg::R](dport_app_dcache_dbug1_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug1_reg::W](dport_app_dcache_dbug1_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG1_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG1_REG"]
pub mod dport_app_dcache_dbug1_reg;
#[doc = "DPORT_APP_DCACHE_DBUG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug2_reg](dport_app_dcache_dbug2_reg) module"]
pub type DPORT_APP_DCACHE_DBUG2_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG2_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug2_reg::R](dport_app_dcache_dbug2_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug2_reg::W](dport_app_dcache_dbug2_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG2_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG2_REG"]
pub mod dport_app_dcache_dbug2_reg;
#[doc = "DPORT_APP_DCACHE_DBUG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug3_reg](dport_app_dcache_dbug3_reg) module"]
pub type DPORT_APP_DCACHE_DBUG3_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG3_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug3_reg::R](dport_app_dcache_dbug3_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug3_reg::W](dport_app_dcache_dbug3_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG3_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG3_REG"]
pub mod dport_app_dcache_dbug3_reg;
#[doc = "DPORT_APP_DCACHE_DBUG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug4_reg](dport_app_dcache_dbug4_reg) module"]
pub type DPORT_APP_DCACHE_DBUG4_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG4_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug4_reg::R](dport_app_dcache_dbug4_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG4_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug4_reg::W](dport_app_dcache_dbug4_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG4_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG4_REG"]
pub mod dport_app_dcache_dbug4_reg;
#[doc = "DPORT_APP_DCACHE_DBUG5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug5_reg](dport_app_dcache_dbug5_reg) module"]
pub type DPORT_APP_DCACHE_DBUG5_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG5_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug5_reg::R](dport_app_dcache_dbug5_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG5_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug5_reg::W](dport_app_dcache_dbug5_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG5_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG5_REG"]
pub mod dport_app_dcache_dbug5_reg;
#[doc = "DPORT_APP_DCACHE_DBUG6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug6_reg](dport_app_dcache_dbug6_reg) module"]
pub type DPORT_APP_DCACHE_DBUG6_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG6_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug6_reg::R](dport_app_dcache_dbug6_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG6_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug6_reg::W](dport_app_dcache_dbug6_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG6_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG6_REG"]
pub mod dport_app_dcache_dbug6_reg;
#[doc = "DPORT_APP_DCACHE_DBUG7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug7_reg](dport_app_dcache_dbug7_reg) module"]
pub type DPORT_APP_DCACHE_DBUG7_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG7_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug7_reg::R](dport_app_dcache_dbug7_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG7_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug7_reg::W](dport_app_dcache_dbug7_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG7_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG7_REG"]
pub mod dport_app_dcache_dbug7_reg;
#[doc = "DPORT_APP_DCACHE_DBUG8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug8_reg](dport_app_dcache_dbug8_reg) module"]
pub type DPORT_APP_DCACHE_DBUG8_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG8_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug8_reg::R](dport_app_dcache_dbug8_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG8_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug8_reg::W](dport_app_dcache_dbug8_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG8_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG8_REG"]
pub mod dport_app_dcache_dbug8_reg;
#[doc = "DPORT_APP_DCACHE_DBUG9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_dcache_dbug9_reg](dport_app_dcache_dbug9_reg) module"]
pub type DPORT_APP_DCACHE_DBUG9_REG = crate::Reg<u32, _DPORT_APP_DCACHE_DBUG9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_DCACHE_DBUG9_REG;
#[doc = "`read()` method returns [dport_app_dcache_dbug9_reg::R](dport_app_dcache_dbug9_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_DCACHE_DBUG9_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_dcache_dbug9_reg::W](dport_app_dcache_dbug9_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_DCACHE_DBUG9_REG {}
#[doc = "DPORT_APP_DCACHE_DBUG9_REG"]
pub mod dport_app_dcache_dbug9_reg;
#[doc = "DPORT_PRO_CPU_RECORD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_ctrl_reg](dport_pro_cpu_record_ctrl_reg) module"]
pub type DPORT_PRO_CPU_RECORD_CTRL_REG = crate::Reg<u32, _DPORT_PRO_CPU_RECORD_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_CTRL_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_ctrl_reg::R](dport_pro_cpu_record_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_ctrl_reg::W](dport_pro_cpu_record_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_CTRL_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_CTRL_REG"]
pub mod dport_pro_cpu_record_ctrl_reg;
#[doc = "DPORT_PRO_CPU_RECORD_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_status_reg](dport_pro_cpu_record_status_reg) module"]
pub type DPORT_PRO_CPU_RECORD_STATUS_REG = crate::Reg<u32, _DPORT_PRO_CPU_RECORD_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_STATUS_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_status_reg::R](dport_pro_cpu_record_status_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_status_reg::W](dport_pro_cpu_record_status_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_STATUS_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_STATUS_REG"]
pub mod dport_pro_cpu_record_status_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pid_reg](dport_pro_cpu_record_pid_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PID_REG = crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PID_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PID_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pid_reg::R](dport_pro_cpu_record_pid_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PID_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pid_reg::W](dport_pro_cpu_record_pid_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PID_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PID_REG"]
pub mod dport_pro_cpu_record_pid_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGINST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebuginst_reg](dport_pro_cpu_record_pdebuginst_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGINST_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGINST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGINST_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebuginst_reg::R](dport_pro_cpu_record_pdebuginst_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGINST_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebuginst_reg::W](dport_pro_cpu_record_pdebuginst_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGINST_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGINST_REG"]
pub mod dport_pro_cpu_record_pdebuginst_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugstatus_reg](dport_pro_cpu_record_pdebugstatus_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugstatus_reg::R](dport_pro_cpu_record_pdebugstatus_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugstatus_reg::W](dport_pro_cpu_record_pdebugstatus_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG"]
pub mod dport_pro_cpu_record_pdebugstatus_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugdata_reg](dport_pro_cpu_record_pdebugdata_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugdata_reg::R](dport_pro_cpu_record_pdebugdata_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugdata_reg::W](dport_pro_cpu_record_pdebugdata_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
pub mod dport_pro_cpu_record_pdebugdata_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGPC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugpc_reg](dport_pro_cpu_record_pdebugpc_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGPC_REG = crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGPC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGPC_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugpc_reg::R](dport_pro_cpu_record_pdebugpc_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGPC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugpc_reg::W](dport_pro_cpu_record_pdebugpc_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGPC_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGPC_REG"]
pub mod dport_pro_cpu_record_pdebugpc_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugls0stat_reg](dport_pro_cpu_record_pdebugls0stat_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugls0stat_reg::R](dport_pro_cpu_record_pdebugls0stat_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugls0stat_reg::W](dport_pro_cpu_record_pdebugls0stat_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG"]
pub mod dport_pro_cpu_record_pdebugls0stat_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugls0addr_reg](dport_pro_cpu_record_pdebugls0addr_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugls0addr_reg::R](dport_pro_cpu_record_pdebugls0addr_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugls0addr_reg::W](dport_pro_cpu_record_pdebugls0addr_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG"]
pub mod dport_pro_cpu_record_pdebugls0addr_reg;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_cpu_record_pdebugls0data_reg](dport_pro_cpu_record_pdebugls0data_reg) module"]
pub type DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG =
    crate::Reg<u32, _DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG;
#[doc = "`read()` method returns [dport_pro_cpu_record_pdebugls0data_reg::R](dport_pro_cpu_record_pdebugls0data_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_cpu_record_pdebugls0data_reg::W](dport_pro_cpu_record_pdebugls0data_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG"]
pub mod dport_pro_cpu_record_pdebugls0data_reg;
#[doc = "DPORT_APP_CPU_RECORD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_ctrl_reg](dport_app_cpu_record_ctrl_reg) module"]
pub type DPORT_APP_CPU_RECORD_CTRL_REG = crate::Reg<u32, _DPORT_APP_CPU_RECORD_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_CTRL_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_ctrl_reg::R](dport_app_cpu_record_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_ctrl_reg::W](dport_app_cpu_record_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_CTRL_REG {}
#[doc = "DPORT_APP_CPU_RECORD_CTRL_REG"]
pub mod dport_app_cpu_record_ctrl_reg;
#[doc = "DPORT_APP_CPU_RECORD_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_status_reg](dport_app_cpu_record_status_reg) module"]
pub type DPORT_APP_CPU_RECORD_STATUS_REG = crate::Reg<u32, _DPORT_APP_CPU_RECORD_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_STATUS_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_status_reg::R](dport_app_cpu_record_status_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_status_reg::W](dport_app_cpu_record_status_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_STATUS_REG {}
#[doc = "DPORT_APP_CPU_RECORD_STATUS_REG"]
pub mod dport_app_cpu_record_status_reg;
#[doc = "DPORT_APP_CPU_RECORD_PID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pid_reg](dport_app_cpu_record_pid_reg) module"]
pub type DPORT_APP_CPU_RECORD_PID_REG = crate::Reg<u32, _DPORT_APP_CPU_RECORD_PID_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PID_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pid_reg::R](dport_app_cpu_record_pid_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PID_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pid_reg::W](dport_app_cpu_record_pid_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PID_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PID_REG"]
pub mod dport_app_cpu_record_pid_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGINST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebuginst_reg](dport_app_cpu_record_pdebuginst_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGINST_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGINST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGINST_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebuginst_reg::R](dport_app_cpu_record_pdebuginst_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGINST_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebuginst_reg::W](dport_app_cpu_record_pdebuginst_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGINST_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGINST_REG"]
pub mod dport_app_cpu_record_pdebuginst_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugstatus_reg](dport_app_cpu_record_pdebugstatus_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugstatus_reg::R](dport_app_cpu_record_pdebugstatus_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugstatus_reg::W](dport_app_cpu_record_pdebugstatus_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
pub mod dport_app_cpu_record_pdebugstatus_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugdata_reg](dport_app_cpu_record_pdebugdata_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGDATA_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGDATA_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugdata_reg::R](dport_app_cpu_record_pdebugdata_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGDATA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugdata_reg::W](dport_app_cpu_record_pdebugdata_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGDATA_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGDATA_REG"]
pub mod dport_app_cpu_record_pdebugdata_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGPC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugpc_reg](dport_app_cpu_record_pdebugpc_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGPC_REG = crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGPC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGPC_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugpc_reg::R](dport_app_cpu_record_pdebugpc_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGPC_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugpc_reg::W](dport_app_cpu_record_pdebugpc_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGPC_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGPC_REG"]
pub mod dport_app_cpu_record_pdebugpc_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugls0stat_reg](dport_app_cpu_record_pdebugls0stat_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugls0stat_reg::R](dport_app_cpu_record_pdebugls0stat_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugls0stat_reg::W](dport_app_cpu_record_pdebugls0stat_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG"]
pub mod dport_app_cpu_record_pdebugls0stat_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugls0addr_reg](dport_app_cpu_record_pdebugls0addr_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugls0addr_reg::R](dport_app_cpu_record_pdebugls0addr_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugls0addr_reg::W](dport_app_cpu_record_pdebugls0addr_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG"]
pub mod dport_app_cpu_record_pdebugls0addr_reg;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_cpu_record_pdebugls0data_reg](dport_app_cpu_record_pdebugls0data_reg) module"]
pub type DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG =
    crate::Reg<u32, _DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG;
#[doc = "`read()` method returns [dport_app_cpu_record_pdebugls0data_reg::R](dport_app_cpu_record_pdebugls0data_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_cpu_record_pdebugls0data_reg::W](dport_app_cpu_record_pdebugls0data_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG"]
pub mod dport_app_cpu_record_pdebugls0data_reg;
#[doc = "DPORT_RSA_PD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rsa_pd_ctrl_reg](dport_rsa_pd_ctrl_reg) module"]
pub type DPORT_RSA_PD_CTRL_REG = crate::Reg<u32, _DPORT_RSA_PD_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_RSA_PD_CTRL_REG;
#[doc = "`read()` method returns [dport_rsa_pd_ctrl_reg::R](dport_rsa_pd_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_RSA_PD_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rsa_pd_ctrl_reg::W](dport_rsa_pd_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_RSA_PD_CTRL_REG {}
#[doc = "DPORT_RSA_PD_CTRL_REG"]
pub mod dport_rsa_pd_ctrl_reg;
#[doc = "DPORT_ROM_MPU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_mpu_table0_reg](dport_rom_mpu_table0_reg) module"]
pub type DPORT_ROM_MPU_TABLE0_REG = crate::Reg<u32, _DPORT_ROM_MPU_TABLE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_MPU_TABLE0_REG;
#[doc = "`read()` method returns [dport_rom_mpu_table0_reg::R](dport_rom_mpu_table0_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_MPU_TABLE0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_mpu_table0_reg::W](dport_rom_mpu_table0_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_MPU_TABLE0_REG {}
#[doc = "DPORT_ROM_MPU_TABLE0_REG"]
pub mod dport_rom_mpu_table0_reg;
#[doc = "DPORT_ROM_MPU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_mpu_table1_reg](dport_rom_mpu_table1_reg) module"]
pub type DPORT_ROM_MPU_TABLE1_REG = crate::Reg<u32, _DPORT_ROM_MPU_TABLE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_MPU_TABLE1_REG;
#[doc = "`read()` method returns [dport_rom_mpu_table1_reg::R](dport_rom_mpu_table1_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_MPU_TABLE1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_mpu_table1_reg::W](dport_rom_mpu_table1_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_MPU_TABLE1_REG {}
#[doc = "DPORT_ROM_MPU_TABLE1_REG"]
pub mod dport_rom_mpu_table1_reg;
#[doc = "DPORT_ROM_MPU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_mpu_table2_reg](dport_rom_mpu_table2_reg) module"]
pub type DPORT_ROM_MPU_TABLE2_REG = crate::Reg<u32, _DPORT_ROM_MPU_TABLE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_MPU_TABLE2_REG;
#[doc = "`read()` method returns [dport_rom_mpu_table2_reg::R](dport_rom_mpu_table2_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_MPU_TABLE2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_mpu_table2_reg::W](dport_rom_mpu_table2_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_MPU_TABLE2_REG {}
#[doc = "DPORT_ROM_MPU_TABLE2_REG"]
pub mod dport_rom_mpu_table2_reg;
#[doc = "DPORT_ROM_MPU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_rom_mpu_table3_reg](dport_rom_mpu_table3_reg) module"]
pub type DPORT_ROM_MPU_TABLE3_REG = crate::Reg<u32, _DPORT_ROM_MPU_TABLE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_ROM_MPU_TABLE3_REG;
#[doc = "`read()` method returns [dport_rom_mpu_table3_reg::R](dport_rom_mpu_table3_reg::R) reader structure"]
impl crate::Readable for DPORT_ROM_MPU_TABLE3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_rom_mpu_table3_reg::W](dport_rom_mpu_table3_reg::W) writer structure"]
impl crate::Writable for DPORT_ROM_MPU_TABLE3_REG {}
#[doc = "DPORT_ROM_MPU_TABLE3_REG"]
pub mod dport_rom_mpu_table3_reg;
#[doc = "DPORT_SHROM_MPU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table0_reg](dport_shrom_mpu_table0_reg) module"]
pub type DPORT_SHROM_MPU_TABLE0_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE0_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table0_reg::R](dport_shrom_mpu_table0_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table0_reg::W](dport_shrom_mpu_table0_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE0_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE0_REG"]
pub mod dport_shrom_mpu_table0_reg;
#[doc = "DPORT_SHROM_MPU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table1_reg](dport_shrom_mpu_table1_reg) module"]
pub type DPORT_SHROM_MPU_TABLE1_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE1_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table1_reg::R](dport_shrom_mpu_table1_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table1_reg::W](dport_shrom_mpu_table1_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE1_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE1_REG"]
pub mod dport_shrom_mpu_table1_reg;
#[doc = "DPORT_SHROM_MPU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table2_reg](dport_shrom_mpu_table2_reg) module"]
pub type DPORT_SHROM_MPU_TABLE2_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE2_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table2_reg::R](dport_shrom_mpu_table2_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table2_reg::W](dport_shrom_mpu_table2_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE2_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE2_REG"]
pub mod dport_shrom_mpu_table2_reg;
#[doc = "DPORT_SHROM_MPU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table3_reg](dport_shrom_mpu_table3_reg) module"]
pub type DPORT_SHROM_MPU_TABLE3_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE3_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table3_reg::R](dport_shrom_mpu_table3_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table3_reg::W](dport_shrom_mpu_table3_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE3_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE3_REG"]
pub mod dport_shrom_mpu_table3_reg;
#[doc = "DPORT_SHROM_MPU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table4_reg](dport_shrom_mpu_table4_reg) module"]
pub type DPORT_SHROM_MPU_TABLE4_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE4_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table4_reg::R](dport_shrom_mpu_table4_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE4_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table4_reg::W](dport_shrom_mpu_table4_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE4_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE4_REG"]
pub mod dport_shrom_mpu_table4_reg;
#[doc = "DPORT_SHROM_MPU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table5_reg](dport_shrom_mpu_table5_reg) module"]
pub type DPORT_SHROM_MPU_TABLE5_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE5_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table5_reg::R](dport_shrom_mpu_table5_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE5_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table5_reg::W](dport_shrom_mpu_table5_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE5_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE5_REG"]
pub mod dport_shrom_mpu_table5_reg;
#[doc = "DPORT_SHROM_MPU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table6_reg](dport_shrom_mpu_table6_reg) module"]
pub type DPORT_SHROM_MPU_TABLE6_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE6_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table6_reg::R](dport_shrom_mpu_table6_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE6_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table6_reg::W](dport_shrom_mpu_table6_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE6_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE6_REG"]
pub mod dport_shrom_mpu_table6_reg;
#[doc = "DPORT_SHROM_MPU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table7_reg](dport_shrom_mpu_table7_reg) module"]
pub type DPORT_SHROM_MPU_TABLE7_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE7_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table7_reg::R](dport_shrom_mpu_table7_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE7_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table7_reg::W](dport_shrom_mpu_table7_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE7_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE7_REG"]
pub mod dport_shrom_mpu_table7_reg;
#[doc = "DPORT_SHROM_MPU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table8_reg](dport_shrom_mpu_table8_reg) module"]
pub type DPORT_SHROM_MPU_TABLE8_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE8_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table8_reg::R](dport_shrom_mpu_table8_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE8_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table8_reg::W](dport_shrom_mpu_table8_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE8_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE8_REG"]
pub mod dport_shrom_mpu_table8_reg;
#[doc = "DPORT_SHROM_MPU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table9_reg](dport_shrom_mpu_table9_reg) module"]
pub type DPORT_SHROM_MPU_TABLE9_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE9_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table9_reg::R](dport_shrom_mpu_table9_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE9_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table9_reg::W](dport_shrom_mpu_table9_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE9_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE9_REG"]
pub mod dport_shrom_mpu_table9_reg;
#[doc = "DPORT_SHROM_MPU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table10_reg](dport_shrom_mpu_table10_reg) module"]
pub type DPORT_SHROM_MPU_TABLE10_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE10_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table10_reg::R](dport_shrom_mpu_table10_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE10_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table10_reg::W](dport_shrom_mpu_table10_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE10_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE10_REG"]
pub mod dport_shrom_mpu_table10_reg;
#[doc = "DPORT_SHROM_MPU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table11_reg](dport_shrom_mpu_table11_reg) module"]
pub type DPORT_SHROM_MPU_TABLE11_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE11_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table11_reg::R](dport_shrom_mpu_table11_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE11_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table11_reg::W](dport_shrom_mpu_table11_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE11_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE11_REG"]
pub mod dport_shrom_mpu_table11_reg;
#[doc = "DPORT_SHROM_MPU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table12_reg](dport_shrom_mpu_table12_reg) module"]
pub type DPORT_SHROM_MPU_TABLE12_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE12_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table12_reg::R](dport_shrom_mpu_table12_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE12_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table12_reg::W](dport_shrom_mpu_table12_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE12_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE12_REG"]
pub mod dport_shrom_mpu_table12_reg;
#[doc = "DPORT_SHROM_MPU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table13_reg](dport_shrom_mpu_table13_reg) module"]
pub type DPORT_SHROM_MPU_TABLE13_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE13_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table13_reg::R](dport_shrom_mpu_table13_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE13_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table13_reg::W](dport_shrom_mpu_table13_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE13_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE13_REG"]
pub mod dport_shrom_mpu_table13_reg;
#[doc = "DPORT_SHROM_MPU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table14_reg](dport_shrom_mpu_table14_reg) module"]
pub type DPORT_SHROM_MPU_TABLE14_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE14_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table14_reg::R](dport_shrom_mpu_table14_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE14_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table14_reg::W](dport_shrom_mpu_table14_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE14_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE14_REG"]
pub mod dport_shrom_mpu_table14_reg;
#[doc = "DPORT_SHROM_MPU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table15_reg](dport_shrom_mpu_table15_reg) module"]
pub type DPORT_SHROM_MPU_TABLE15_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE15_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table15_reg::R](dport_shrom_mpu_table15_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE15_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table15_reg::W](dport_shrom_mpu_table15_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE15_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE15_REG"]
pub mod dport_shrom_mpu_table15_reg;
#[doc = "DPORT_SHROM_MPU_TABLE16_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table16_reg](dport_shrom_mpu_table16_reg) module"]
pub type DPORT_SHROM_MPU_TABLE16_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE16_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE16_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table16_reg::R](dport_shrom_mpu_table16_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE16_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table16_reg::W](dport_shrom_mpu_table16_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE16_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE16_REG"]
pub mod dport_shrom_mpu_table16_reg;
#[doc = "DPORT_SHROM_MPU_TABLE17_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table17_reg](dport_shrom_mpu_table17_reg) module"]
pub type DPORT_SHROM_MPU_TABLE17_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE17_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE17_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table17_reg::R](dport_shrom_mpu_table17_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE17_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table17_reg::W](dport_shrom_mpu_table17_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE17_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE17_REG"]
pub mod dport_shrom_mpu_table17_reg;
#[doc = "DPORT_SHROM_MPU_TABLE18_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table18_reg](dport_shrom_mpu_table18_reg) module"]
pub type DPORT_SHROM_MPU_TABLE18_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE18_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE18_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table18_reg::R](dport_shrom_mpu_table18_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE18_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table18_reg::W](dport_shrom_mpu_table18_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE18_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE18_REG"]
pub mod dport_shrom_mpu_table18_reg;
#[doc = "DPORT_SHROM_MPU_TABLE19_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table19_reg](dport_shrom_mpu_table19_reg) module"]
pub type DPORT_SHROM_MPU_TABLE19_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE19_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE19_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table19_reg::R](dport_shrom_mpu_table19_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE19_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table19_reg::W](dport_shrom_mpu_table19_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE19_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE19_REG"]
pub mod dport_shrom_mpu_table19_reg;
#[doc = "DPORT_SHROM_MPU_TABLE20_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table20_reg](dport_shrom_mpu_table20_reg) module"]
pub type DPORT_SHROM_MPU_TABLE20_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE20_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE20_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table20_reg::R](dport_shrom_mpu_table20_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE20_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table20_reg::W](dport_shrom_mpu_table20_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE20_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE20_REG"]
pub mod dport_shrom_mpu_table20_reg;
#[doc = "DPORT_SHROM_MPU_TABLE21_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table21_reg](dport_shrom_mpu_table21_reg) module"]
pub type DPORT_SHROM_MPU_TABLE21_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE21_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE21_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table21_reg::R](dport_shrom_mpu_table21_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE21_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table21_reg::W](dport_shrom_mpu_table21_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE21_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE21_REG"]
pub mod dport_shrom_mpu_table21_reg;
#[doc = "DPORT_SHROM_MPU_TABLE22_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table22_reg](dport_shrom_mpu_table22_reg) module"]
pub type DPORT_SHROM_MPU_TABLE22_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE22_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE22_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table22_reg::R](dport_shrom_mpu_table22_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE22_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table22_reg::W](dport_shrom_mpu_table22_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE22_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE22_REG"]
pub mod dport_shrom_mpu_table22_reg;
#[doc = "DPORT_SHROM_MPU_TABLE23_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_shrom_mpu_table23_reg](dport_shrom_mpu_table23_reg) module"]
pub type DPORT_SHROM_MPU_TABLE23_REG = crate::Reg<u32, _DPORT_SHROM_MPU_TABLE23_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SHROM_MPU_TABLE23_REG;
#[doc = "`read()` method returns [dport_shrom_mpu_table23_reg::R](dport_shrom_mpu_table23_reg::R) reader structure"]
impl crate::Readable for DPORT_SHROM_MPU_TABLE23_REG {}
#[doc = "`write(|w| ..)` method takes [dport_shrom_mpu_table23_reg::W](dport_shrom_mpu_table23_reg::W) writer structure"]
impl crate::Writable for DPORT_SHROM_MPU_TABLE23_REG {}
#[doc = "DPORT_SHROM_MPU_TABLE23_REG"]
pub mod dport_shrom_mpu_table23_reg;
#[doc = "DPORT_IMMU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table0_reg](dport_immu_table0_reg) module"]
pub type DPORT_IMMU_TABLE0_REG = crate::Reg<u32, _DPORT_IMMU_TABLE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE0_REG;
#[doc = "`read()` method returns [dport_immu_table0_reg::R](dport_immu_table0_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table0_reg::W](dport_immu_table0_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE0_REG {}
#[doc = "DPORT_IMMU_TABLE0_REG"]
pub mod dport_immu_table0_reg;
#[doc = "DPORT_IMMU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table1_reg](dport_immu_table1_reg) module"]
pub type DPORT_IMMU_TABLE1_REG = crate::Reg<u32, _DPORT_IMMU_TABLE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE1_REG;
#[doc = "`read()` method returns [dport_immu_table1_reg::R](dport_immu_table1_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table1_reg::W](dport_immu_table1_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE1_REG {}
#[doc = "DPORT_IMMU_TABLE1_REG"]
pub mod dport_immu_table1_reg;
#[doc = "DPORT_IMMU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table2_reg](dport_immu_table2_reg) module"]
pub type DPORT_IMMU_TABLE2_REG = crate::Reg<u32, _DPORT_IMMU_TABLE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE2_REG;
#[doc = "`read()` method returns [dport_immu_table2_reg::R](dport_immu_table2_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table2_reg::W](dport_immu_table2_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE2_REG {}
#[doc = "DPORT_IMMU_TABLE2_REG"]
pub mod dport_immu_table2_reg;
#[doc = "DPORT_IMMU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table3_reg](dport_immu_table3_reg) module"]
pub type DPORT_IMMU_TABLE3_REG = crate::Reg<u32, _DPORT_IMMU_TABLE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE3_REG;
#[doc = "`read()` method returns [dport_immu_table3_reg::R](dport_immu_table3_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table3_reg::W](dport_immu_table3_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE3_REG {}
#[doc = "DPORT_IMMU_TABLE3_REG"]
pub mod dport_immu_table3_reg;
#[doc = "DPORT_IMMU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table4_reg](dport_immu_table4_reg) module"]
pub type DPORT_IMMU_TABLE4_REG = crate::Reg<u32, _DPORT_IMMU_TABLE4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE4_REG;
#[doc = "`read()` method returns [dport_immu_table4_reg::R](dport_immu_table4_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE4_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table4_reg::W](dport_immu_table4_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE4_REG {}
#[doc = "DPORT_IMMU_TABLE4_REG"]
pub mod dport_immu_table4_reg;
#[doc = "DPORT_IMMU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table5_reg](dport_immu_table5_reg) module"]
pub type DPORT_IMMU_TABLE5_REG = crate::Reg<u32, _DPORT_IMMU_TABLE5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE5_REG;
#[doc = "`read()` method returns [dport_immu_table5_reg::R](dport_immu_table5_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE5_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table5_reg::W](dport_immu_table5_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE5_REG {}
#[doc = "DPORT_IMMU_TABLE5_REG"]
pub mod dport_immu_table5_reg;
#[doc = "DPORT_IMMU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table6_reg](dport_immu_table6_reg) module"]
pub type DPORT_IMMU_TABLE6_REG = crate::Reg<u32, _DPORT_IMMU_TABLE6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE6_REG;
#[doc = "`read()` method returns [dport_immu_table6_reg::R](dport_immu_table6_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE6_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table6_reg::W](dport_immu_table6_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE6_REG {}
#[doc = "DPORT_IMMU_TABLE6_REG"]
pub mod dport_immu_table6_reg;
#[doc = "DPORT_IMMU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table7_reg](dport_immu_table7_reg) module"]
pub type DPORT_IMMU_TABLE7_REG = crate::Reg<u32, _DPORT_IMMU_TABLE7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE7_REG;
#[doc = "`read()` method returns [dport_immu_table7_reg::R](dport_immu_table7_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE7_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table7_reg::W](dport_immu_table7_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE7_REG {}
#[doc = "DPORT_IMMU_TABLE7_REG"]
pub mod dport_immu_table7_reg;
#[doc = "DPORT_IMMU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table8_reg](dport_immu_table8_reg) module"]
pub type DPORT_IMMU_TABLE8_REG = crate::Reg<u32, _DPORT_IMMU_TABLE8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE8_REG;
#[doc = "`read()` method returns [dport_immu_table8_reg::R](dport_immu_table8_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE8_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table8_reg::W](dport_immu_table8_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE8_REG {}
#[doc = "DPORT_IMMU_TABLE8_REG"]
pub mod dport_immu_table8_reg;
#[doc = "DPORT_IMMU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table9_reg](dport_immu_table9_reg) module"]
pub type DPORT_IMMU_TABLE9_REG = crate::Reg<u32, _DPORT_IMMU_TABLE9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE9_REG;
#[doc = "`read()` method returns [dport_immu_table9_reg::R](dport_immu_table9_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE9_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table9_reg::W](dport_immu_table9_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE9_REG {}
#[doc = "DPORT_IMMU_TABLE9_REG"]
pub mod dport_immu_table9_reg;
#[doc = "DPORT_IMMU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table10_reg](dport_immu_table10_reg) module"]
pub type DPORT_IMMU_TABLE10_REG = crate::Reg<u32, _DPORT_IMMU_TABLE10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE10_REG;
#[doc = "`read()` method returns [dport_immu_table10_reg::R](dport_immu_table10_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE10_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table10_reg::W](dport_immu_table10_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE10_REG {}
#[doc = "DPORT_IMMU_TABLE10_REG"]
pub mod dport_immu_table10_reg;
#[doc = "DPORT_IMMU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table11_reg](dport_immu_table11_reg) module"]
pub type DPORT_IMMU_TABLE11_REG = crate::Reg<u32, _DPORT_IMMU_TABLE11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE11_REG;
#[doc = "`read()` method returns [dport_immu_table11_reg::R](dport_immu_table11_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE11_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table11_reg::W](dport_immu_table11_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE11_REG {}
#[doc = "DPORT_IMMU_TABLE11_REG"]
pub mod dport_immu_table11_reg;
#[doc = "DPORT_IMMU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table12_reg](dport_immu_table12_reg) module"]
pub type DPORT_IMMU_TABLE12_REG = crate::Reg<u32, _DPORT_IMMU_TABLE12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE12_REG;
#[doc = "`read()` method returns [dport_immu_table12_reg::R](dport_immu_table12_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE12_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table12_reg::W](dport_immu_table12_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE12_REG {}
#[doc = "DPORT_IMMU_TABLE12_REG"]
pub mod dport_immu_table12_reg;
#[doc = "DPORT_IMMU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table13_reg](dport_immu_table13_reg) module"]
pub type DPORT_IMMU_TABLE13_REG = crate::Reg<u32, _DPORT_IMMU_TABLE13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE13_REG;
#[doc = "`read()` method returns [dport_immu_table13_reg::R](dport_immu_table13_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE13_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table13_reg::W](dport_immu_table13_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE13_REG {}
#[doc = "DPORT_IMMU_TABLE13_REG"]
pub mod dport_immu_table13_reg;
#[doc = "DPORT_IMMU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table14_reg](dport_immu_table14_reg) module"]
pub type DPORT_IMMU_TABLE14_REG = crate::Reg<u32, _DPORT_IMMU_TABLE14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE14_REG;
#[doc = "`read()` method returns [dport_immu_table14_reg::R](dport_immu_table14_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE14_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table14_reg::W](dport_immu_table14_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE14_REG {}
#[doc = "DPORT_IMMU_TABLE14_REG"]
pub mod dport_immu_table14_reg;
#[doc = "DPORT_IMMU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_immu_table15_reg](dport_immu_table15_reg) module"]
pub type DPORT_IMMU_TABLE15_REG = crate::Reg<u32, _DPORT_IMMU_TABLE15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_IMMU_TABLE15_REG;
#[doc = "`read()` method returns [dport_immu_table15_reg::R](dport_immu_table15_reg::R) reader structure"]
impl crate::Readable for DPORT_IMMU_TABLE15_REG {}
#[doc = "`write(|w| ..)` method takes [dport_immu_table15_reg::W](dport_immu_table15_reg::W) writer structure"]
impl crate::Writable for DPORT_IMMU_TABLE15_REG {}
#[doc = "DPORT_IMMU_TABLE15_REG"]
pub mod dport_immu_table15_reg;
#[doc = "DPORT_DMMU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table0_reg](dport_dmmu_table0_reg) module"]
pub type DPORT_DMMU_TABLE0_REG = crate::Reg<u32, _DPORT_DMMU_TABLE0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE0_REG;
#[doc = "`read()` method returns [dport_dmmu_table0_reg::R](dport_dmmu_table0_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE0_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table0_reg::W](dport_dmmu_table0_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE0_REG {}
#[doc = "DPORT_DMMU_TABLE0_REG"]
pub mod dport_dmmu_table0_reg;
#[doc = "DPORT_DMMU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table1_reg](dport_dmmu_table1_reg) module"]
pub type DPORT_DMMU_TABLE1_REG = crate::Reg<u32, _DPORT_DMMU_TABLE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE1_REG;
#[doc = "`read()` method returns [dport_dmmu_table1_reg::R](dport_dmmu_table1_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE1_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table1_reg::W](dport_dmmu_table1_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE1_REG {}
#[doc = "DPORT_DMMU_TABLE1_REG"]
pub mod dport_dmmu_table1_reg;
#[doc = "DPORT_DMMU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table2_reg](dport_dmmu_table2_reg) module"]
pub type DPORT_DMMU_TABLE2_REG = crate::Reg<u32, _DPORT_DMMU_TABLE2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE2_REG;
#[doc = "`read()` method returns [dport_dmmu_table2_reg::R](dport_dmmu_table2_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE2_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table2_reg::W](dport_dmmu_table2_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE2_REG {}
#[doc = "DPORT_DMMU_TABLE2_REG"]
pub mod dport_dmmu_table2_reg;
#[doc = "DPORT_DMMU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table3_reg](dport_dmmu_table3_reg) module"]
pub type DPORT_DMMU_TABLE3_REG = crate::Reg<u32, _DPORT_DMMU_TABLE3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE3_REG;
#[doc = "`read()` method returns [dport_dmmu_table3_reg::R](dport_dmmu_table3_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE3_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table3_reg::W](dport_dmmu_table3_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE3_REG {}
#[doc = "DPORT_DMMU_TABLE3_REG"]
pub mod dport_dmmu_table3_reg;
#[doc = "DPORT_DMMU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table4_reg](dport_dmmu_table4_reg) module"]
pub type DPORT_DMMU_TABLE4_REG = crate::Reg<u32, _DPORT_DMMU_TABLE4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE4_REG;
#[doc = "`read()` method returns [dport_dmmu_table4_reg::R](dport_dmmu_table4_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE4_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table4_reg::W](dport_dmmu_table4_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE4_REG {}
#[doc = "DPORT_DMMU_TABLE4_REG"]
pub mod dport_dmmu_table4_reg;
#[doc = "DPORT_DMMU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table5_reg](dport_dmmu_table5_reg) module"]
pub type DPORT_DMMU_TABLE5_REG = crate::Reg<u32, _DPORT_DMMU_TABLE5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE5_REG;
#[doc = "`read()` method returns [dport_dmmu_table5_reg::R](dport_dmmu_table5_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE5_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table5_reg::W](dport_dmmu_table5_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE5_REG {}
#[doc = "DPORT_DMMU_TABLE5_REG"]
pub mod dport_dmmu_table5_reg;
#[doc = "DPORT_DMMU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table6_reg](dport_dmmu_table6_reg) module"]
pub type DPORT_DMMU_TABLE6_REG = crate::Reg<u32, _DPORT_DMMU_TABLE6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE6_REG;
#[doc = "`read()` method returns [dport_dmmu_table6_reg::R](dport_dmmu_table6_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE6_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table6_reg::W](dport_dmmu_table6_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE6_REG {}
#[doc = "DPORT_DMMU_TABLE6_REG"]
pub mod dport_dmmu_table6_reg;
#[doc = "DPORT_DMMU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table7_reg](dport_dmmu_table7_reg) module"]
pub type DPORT_DMMU_TABLE7_REG = crate::Reg<u32, _DPORT_DMMU_TABLE7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE7_REG;
#[doc = "`read()` method returns [dport_dmmu_table7_reg::R](dport_dmmu_table7_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE7_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table7_reg::W](dport_dmmu_table7_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE7_REG {}
#[doc = "DPORT_DMMU_TABLE7_REG"]
pub mod dport_dmmu_table7_reg;
#[doc = "DPORT_DMMU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table8_reg](dport_dmmu_table8_reg) module"]
pub type DPORT_DMMU_TABLE8_REG = crate::Reg<u32, _DPORT_DMMU_TABLE8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE8_REG;
#[doc = "`read()` method returns [dport_dmmu_table8_reg::R](dport_dmmu_table8_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE8_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table8_reg::W](dport_dmmu_table8_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE8_REG {}
#[doc = "DPORT_DMMU_TABLE8_REG"]
pub mod dport_dmmu_table8_reg;
#[doc = "DPORT_DMMU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table9_reg](dport_dmmu_table9_reg) module"]
pub type DPORT_DMMU_TABLE9_REG = crate::Reg<u32, _DPORT_DMMU_TABLE9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE9_REG;
#[doc = "`read()` method returns [dport_dmmu_table9_reg::R](dport_dmmu_table9_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE9_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table9_reg::W](dport_dmmu_table9_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE9_REG {}
#[doc = "DPORT_DMMU_TABLE9_REG"]
pub mod dport_dmmu_table9_reg;
#[doc = "DPORT_DMMU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table10_reg](dport_dmmu_table10_reg) module"]
pub type DPORT_DMMU_TABLE10_REG = crate::Reg<u32, _DPORT_DMMU_TABLE10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE10_REG;
#[doc = "`read()` method returns [dport_dmmu_table10_reg::R](dport_dmmu_table10_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE10_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table10_reg::W](dport_dmmu_table10_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE10_REG {}
#[doc = "DPORT_DMMU_TABLE10_REG"]
pub mod dport_dmmu_table10_reg;
#[doc = "DPORT_DMMU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table11_reg](dport_dmmu_table11_reg) module"]
pub type DPORT_DMMU_TABLE11_REG = crate::Reg<u32, _DPORT_DMMU_TABLE11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE11_REG;
#[doc = "`read()` method returns [dport_dmmu_table11_reg::R](dport_dmmu_table11_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE11_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table11_reg::W](dport_dmmu_table11_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE11_REG {}
#[doc = "DPORT_DMMU_TABLE11_REG"]
pub mod dport_dmmu_table11_reg;
#[doc = "DPORT_DMMU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table12_reg](dport_dmmu_table12_reg) module"]
pub type DPORT_DMMU_TABLE12_REG = crate::Reg<u32, _DPORT_DMMU_TABLE12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE12_REG;
#[doc = "`read()` method returns [dport_dmmu_table12_reg::R](dport_dmmu_table12_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE12_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table12_reg::W](dport_dmmu_table12_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE12_REG {}
#[doc = "DPORT_DMMU_TABLE12_REG"]
pub mod dport_dmmu_table12_reg;
#[doc = "DPORT_DMMU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table13_reg](dport_dmmu_table13_reg) module"]
pub type DPORT_DMMU_TABLE13_REG = crate::Reg<u32, _DPORT_DMMU_TABLE13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE13_REG;
#[doc = "`read()` method returns [dport_dmmu_table13_reg::R](dport_dmmu_table13_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE13_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table13_reg::W](dport_dmmu_table13_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE13_REG {}
#[doc = "DPORT_DMMU_TABLE13_REG"]
pub mod dport_dmmu_table13_reg;
#[doc = "DPORT_DMMU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table14_reg](dport_dmmu_table14_reg) module"]
pub type DPORT_DMMU_TABLE14_REG = crate::Reg<u32, _DPORT_DMMU_TABLE14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE14_REG;
#[doc = "`read()` method returns [dport_dmmu_table14_reg::R](dport_dmmu_table14_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE14_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table14_reg::W](dport_dmmu_table14_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE14_REG {}
#[doc = "DPORT_DMMU_TABLE14_REG"]
pub mod dport_dmmu_table14_reg;
#[doc = "DPORT_DMMU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_dmmu_table15_reg](dport_dmmu_table15_reg) module"]
pub type DPORT_DMMU_TABLE15_REG = crate::Reg<u32, _DPORT_DMMU_TABLE15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DMMU_TABLE15_REG;
#[doc = "`read()` method returns [dport_dmmu_table15_reg::R](dport_dmmu_table15_reg::R) reader structure"]
impl crate::Readable for DPORT_DMMU_TABLE15_REG {}
#[doc = "`write(|w| ..)` method takes [dport_dmmu_table15_reg::W](dport_dmmu_table15_reg::W) writer structure"]
impl crate::Writable for DPORT_DMMU_TABLE15_REG {}
#[doc = "DPORT_DMMU_TABLE15_REG"]
pub mod dport_dmmu_table15_reg;
#[doc = "DPORT_PRO_INTRUSION_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_intrusion_ctrl_reg](dport_pro_intrusion_ctrl_reg) module"]
pub type DPORT_PRO_INTRUSION_CTRL_REG = crate::Reg<u32, _DPORT_PRO_INTRUSION_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_INTRUSION_CTRL_REG;
#[doc = "`read()` method returns [dport_pro_intrusion_ctrl_reg::R](dport_pro_intrusion_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_INTRUSION_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_intrusion_ctrl_reg::W](dport_pro_intrusion_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_INTRUSION_CTRL_REG {}
#[doc = "DPORT_PRO_INTRUSION_CTRL_REG"]
pub mod dport_pro_intrusion_ctrl_reg;
#[doc = "DPORT_PRO_INTRUSION_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_intrusion_status_reg](dport_pro_intrusion_status_reg) module"]
pub type DPORT_PRO_INTRUSION_STATUS_REG = crate::Reg<u32, _DPORT_PRO_INTRUSION_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_INTRUSION_STATUS_REG;
#[doc = "`read()` method returns [dport_pro_intrusion_status_reg::R](dport_pro_intrusion_status_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_INTRUSION_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_intrusion_status_reg::W](dport_pro_intrusion_status_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_INTRUSION_STATUS_REG {}
#[doc = "DPORT_PRO_INTRUSION_STATUS_REG"]
pub mod dport_pro_intrusion_status_reg;
#[doc = "DPORT_APP_INTRUSION_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_intrusion_ctrl_reg](dport_app_intrusion_ctrl_reg) module"]
pub type DPORT_APP_INTRUSION_CTRL_REG = crate::Reg<u32, _DPORT_APP_INTRUSION_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_INTRUSION_CTRL_REG;
#[doc = "`read()` method returns [dport_app_intrusion_ctrl_reg::R](dport_app_intrusion_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_INTRUSION_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_intrusion_ctrl_reg::W](dport_app_intrusion_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_INTRUSION_CTRL_REG {}
#[doc = "DPORT_APP_INTRUSION_CTRL_REG"]
pub mod dport_app_intrusion_ctrl_reg;
#[doc = "DPORT_APP_INTRUSION_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_intrusion_status_reg](dport_app_intrusion_status_reg) module"]
pub type DPORT_APP_INTRUSION_STATUS_REG = crate::Reg<u32, _DPORT_APP_INTRUSION_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_INTRUSION_STATUS_REG;
#[doc = "`read()` method returns [dport_app_intrusion_status_reg::R](dport_app_intrusion_status_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_INTRUSION_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_intrusion_status_reg::W](dport_app_intrusion_status_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_INTRUSION_STATUS_REG {}
#[doc = "DPORT_APP_INTRUSION_STATUS_REG"]
pub mod dport_app_intrusion_status_reg;
#[doc = "DPORT_FRONT_END_MEM_PD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_front_end_mem_pd_reg](dport_front_end_mem_pd_reg) module"]
pub type DPORT_FRONT_END_MEM_PD_REG = crate::Reg<u32, _DPORT_FRONT_END_MEM_PD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_FRONT_END_MEM_PD_REG;
#[doc = "`read()` method returns [dport_front_end_mem_pd_reg::R](dport_front_end_mem_pd_reg::R) reader structure"]
impl crate::Readable for DPORT_FRONT_END_MEM_PD_REG {}
#[doc = "`write(|w| ..)` method takes [dport_front_end_mem_pd_reg::W](dport_front_end_mem_pd_reg::W) writer structure"]
impl crate::Writable for DPORT_FRONT_END_MEM_PD_REG {}
#[doc = "DPORT_FRONT_END_MEM_PD_REG"]
pub mod dport_front_end_mem_pd_reg;
#[doc = "DPORT_MMU_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_mmu_ia_int_en_reg](dport_mmu_ia_int_en_reg) module"]
pub type DPORT_MMU_IA_INT_EN_REG = crate::Reg<u32, _DPORT_MMU_IA_INT_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_MMU_IA_INT_EN_REG;
#[doc = "`read()` method returns [dport_mmu_ia_int_en_reg::R](dport_mmu_ia_int_en_reg::R) reader structure"]
impl crate::Readable for DPORT_MMU_IA_INT_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_mmu_ia_int_en_reg::W](dport_mmu_ia_int_en_reg::W) writer structure"]
impl crate::Writable for DPORT_MMU_IA_INT_EN_REG {}
#[doc = "DPORT_MMU_IA_INT_EN_REG"]
pub mod dport_mmu_ia_int_en_reg;
#[doc = "DPORT_MPU_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_mpu_ia_int_en_reg](dport_mpu_ia_int_en_reg) module"]
pub type DPORT_MPU_IA_INT_EN_REG = crate::Reg<u32, _DPORT_MPU_IA_INT_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_MPU_IA_INT_EN_REG;
#[doc = "`read()` method returns [dport_mpu_ia_int_en_reg::R](dport_mpu_ia_int_en_reg::R) reader structure"]
impl crate::Readable for DPORT_MPU_IA_INT_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_mpu_ia_int_en_reg::W](dport_mpu_ia_int_en_reg::W) writer structure"]
impl crate::Writable for DPORT_MPU_IA_INT_EN_REG {}
#[doc = "DPORT_MPU_IA_INT_EN_REG"]
pub mod dport_mpu_ia_int_en_reg;
#[doc = "DPORT_CACHE_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_cache_ia_int_en_reg](dport_cache_ia_int_en_reg) module"]
pub type DPORT_CACHE_IA_INT_EN_REG = crate::Reg<u32, _DPORT_CACHE_IA_INT_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_CACHE_IA_INT_EN_REG;
#[doc = "`read()` method returns [dport_cache_ia_int_en_reg::R](dport_cache_ia_int_en_reg::R) reader structure"]
impl crate::Readable for DPORT_CACHE_IA_INT_EN_REG {}
#[doc = "`write(|w| ..)` method takes [dport_cache_ia_int_en_reg::W](dport_cache_ia_int_en_reg::W) writer structure"]
impl crate::Writable for DPORT_CACHE_IA_INT_EN_REG {}
#[doc = "DPORT_CACHE_IA_INT_EN_REG"]
pub mod dport_cache_ia_int_en_reg;
#[doc = "DPORT_SECURE_BOOT_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_secure_boot_ctrl_reg](dport_secure_boot_ctrl_reg) module"]
pub type DPORT_SECURE_BOOT_CTRL_REG = crate::Reg<u32, _DPORT_SECURE_BOOT_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SECURE_BOOT_CTRL_REG;
#[doc = "`read()` method returns [dport_secure_boot_ctrl_reg::R](dport_secure_boot_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_SECURE_BOOT_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_secure_boot_ctrl_reg::W](dport_secure_boot_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_SECURE_BOOT_CTRL_REG {}
#[doc = "DPORT_SECURE_BOOT_CTRL_REG"]
pub mod dport_secure_boot_ctrl_reg;
#[doc = "DPORT_SPI_DMA_CHAN_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_spi_dma_chan_sel_reg](dport_spi_dma_chan_sel_reg) module"]
pub type DPORT_SPI_DMA_CHAN_SEL_REG = crate::Reg<u32, _DPORT_SPI_DMA_CHAN_SEL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_SPI_DMA_CHAN_SEL_REG;
#[doc = "`read()` method returns [dport_spi_dma_chan_sel_reg::R](dport_spi_dma_chan_sel_reg::R) reader structure"]
impl crate::Readable for DPORT_SPI_DMA_CHAN_SEL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_spi_dma_chan_sel_reg::W](dport_spi_dma_chan_sel_reg::W) writer structure"]
impl crate::Writable for DPORT_SPI_DMA_CHAN_SEL_REG {}
#[doc = "DPORT_SPI_DMA_CHAN_SEL_REG"]
pub mod dport_spi_dma_chan_sel_reg;
#[doc = "DPORT_PRO_VECBASE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_vecbase_ctrl_reg](dport_pro_vecbase_ctrl_reg) module"]
pub type DPORT_PRO_VECBASE_CTRL_REG = crate::Reg<u32, _DPORT_PRO_VECBASE_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_VECBASE_CTRL_REG;
#[doc = "`read()` method returns [dport_pro_vecbase_ctrl_reg::R](dport_pro_vecbase_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_VECBASE_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_vecbase_ctrl_reg::W](dport_pro_vecbase_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_VECBASE_CTRL_REG {}
#[doc = "DPORT_PRO_VECBASE_CTRL_REG"]
pub mod dport_pro_vecbase_ctrl_reg;
#[doc = "DPORT_PRO_VECBASE_SET_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_pro_vecbase_set_reg](dport_pro_vecbase_set_reg) module"]
pub type DPORT_PRO_VECBASE_SET_REG = crate::Reg<u32, _DPORT_PRO_VECBASE_SET_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_PRO_VECBASE_SET_REG;
#[doc = "`read()` method returns [dport_pro_vecbase_set_reg::R](dport_pro_vecbase_set_reg::R) reader structure"]
impl crate::Readable for DPORT_PRO_VECBASE_SET_REG {}
#[doc = "`write(|w| ..)` method takes [dport_pro_vecbase_set_reg::W](dport_pro_vecbase_set_reg::W) writer structure"]
impl crate::Writable for DPORT_PRO_VECBASE_SET_REG {}
#[doc = "DPORT_PRO_VECBASE_SET_REG"]
pub mod dport_pro_vecbase_set_reg;
#[doc = "DPORT_APP_VECBASE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_vecbase_ctrl_reg](dport_app_vecbase_ctrl_reg) module"]
pub type DPORT_APP_VECBASE_CTRL_REG = crate::Reg<u32, _DPORT_APP_VECBASE_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_VECBASE_CTRL_REG;
#[doc = "`read()` method returns [dport_app_vecbase_ctrl_reg::R](dport_app_vecbase_ctrl_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_VECBASE_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_vecbase_ctrl_reg::W](dport_app_vecbase_ctrl_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_VECBASE_CTRL_REG {}
#[doc = "DPORT_APP_VECBASE_CTRL_REG"]
pub mod dport_app_vecbase_ctrl_reg;
#[doc = "DPORT_APP_VECBASE_SET_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_app_vecbase_set_reg](dport_app_vecbase_set_reg) module"]
pub type DPORT_APP_VECBASE_SET_REG = crate::Reg<u32, _DPORT_APP_VECBASE_SET_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_APP_VECBASE_SET_REG;
#[doc = "`read()` method returns [dport_app_vecbase_set_reg::R](dport_app_vecbase_set_reg::R) reader structure"]
impl crate::Readable for DPORT_APP_VECBASE_SET_REG {}
#[doc = "`write(|w| ..)` method takes [dport_app_vecbase_set_reg::W](dport_app_vecbase_set_reg::W) writer structure"]
impl crate::Writable for DPORT_APP_VECBASE_SET_REG {}
#[doc = "DPORT_APP_VECBASE_SET_REG"]
pub mod dport_app_vecbase_set_reg;
#[doc = "DPORT_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dport_date_reg](dport_date_reg) module"]
pub type DPORT_DATE_REG = crate::Reg<u32, _DPORT_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPORT_DATE_REG;
#[doc = "`read()` method returns [dport_date_reg::R](dport_date_reg::R) reader structure"]
impl crate::Readable for DPORT_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [dport_date_reg::W](dport_date_reg::W) writer structure"]
impl crate::Writable for DPORT_DATE_REG {}
#[doc = "DPORT_DATE_REG"]
pub mod dport_date_reg;
