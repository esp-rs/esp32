#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DPORT_PRO_BOOT_REMAP_CTRL_REG"]
    pub pro_boot_remap_ctrl: PRO_BOOT_REMAP_CTRL,
    #[doc = "0x04 - DPORT_APP_BOOT_REMAP_CTRL_REG"]
    pub app_boot_remap_ctrl: APP_BOOT_REMAP_CTRL,
    #[doc = "0x08 - DPORT_ACCESS_CHECK_REG"]
    pub access_check: ACCESS_CHECK,
    #[doc = "0x0c - DPORT_PRO_DPORT_APB_MASK0_REG"]
    pub pro_dport_apb_mask0: PRO_DPORT_APB_MASK0,
    #[doc = "0x10 - DPORT_PRO_DPORT_APB_MASK1_REG"]
    pub pro_dport_apb_mask1: PRO_DPORT_APB_MASK1,
    #[doc = "0x14 - DPORT_APP_DPORT_APB_MASK0_REG"]
    pub app_dport_apb_mask0: APP_DPORT_APB_MASK0,
    #[doc = "0x18 - DPORT_APP_DPORT_APB_MASK1_REG"]
    pub app_dport_apb_mask1: APP_DPORT_APB_MASK1,
    #[doc = "0x1c - DPORT_PERI_CLK_EN_REG"]
    pub peri_clk_en: PERI_CLK_EN,
    #[doc = "0x20 - DPORT_PERI_RST_EN_REG"]
    pub peri_rst_en: PERI_RST_EN,
    #[doc = "0x24 - DPORT_WIFI_BB_CFG_REG"]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x28 - DPORT_WIFI_BB_CFG_2_REG"]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x2c - DPORT_APPCPU_CTRL_A_REG"]
    pub appcpu_ctrl_a: APPCPU_CTRL_A,
    #[doc = "0x30 - DPORT_APPCPU_CTRL_B_REG"]
    pub appcpu_ctrl_b: APPCPU_CTRL_B,
    #[doc = "0x34 - DPORT_APPCPU_CTRL_C_REG"]
    pub appcpu_ctrl_c: APPCPU_CTRL_C,
    #[doc = "0x38 - DPORT_APPCPU_CTRL_D_REG"]
    pub appcpu_ctrl_d: APPCPU_CTRL_D,
    #[doc = "0x3c - DPORT_CPU_PER_CONF_REG"]
    pub cpu_per_conf: CPU_PER_CONF,
    #[doc = "0x40 - DPORT_PRO_CACHE_CTRL_REG"]
    pub pro_cache_ctrl: PRO_CACHE_CTRL,
    #[doc = "0x44 - DPORT_PRO_CACHE_CTRL1_REG"]
    pub pro_cache_ctrl1: PRO_CACHE_CTRL1,
    #[doc = "0x48 - DPORT_PRO_CACHE_LOCK_0_ADDR_REG"]
    pub pro_cache_lock_0_addr: PRO_CACHE_LOCK_0_ADDR,
    #[doc = "0x4c - DPORT_PRO_CACHE_LOCK_1_ADDR_REG"]
    pub pro_cache_lock_1_addr: PRO_CACHE_LOCK_1_ADDR,
    #[doc = "0x50 - DPORT_PRO_CACHE_LOCK_2_ADDR_REG"]
    pub pro_cache_lock_2_addr: PRO_CACHE_LOCK_2_ADDR,
    #[doc = "0x54 - DPORT_PRO_CACHE_LOCK_3_ADDR_REG"]
    pub pro_cache_lock_3_addr: PRO_CACHE_LOCK_3_ADDR,
    #[doc = "0x58 - DPORT_APP_CACHE_CTRL_REG"]
    pub app_cache_ctrl: APP_CACHE_CTRL,
    #[doc = "0x5c - DPORT_APP_CACHE_CTRL1_REG"]
    pub app_cache_ctrl1: APP_CACHE_CTRL1,
    #[doc = "0x60 - DPORT_APP_CACHE_LOCK_0_ADDR_REG"]
    pub app_cache_lock_0_addr: APP_CACHE_LOCK_0_ADDR,
    #[doc = "0x64 - DPORT_APP_CACHE_LOCK_1_ADDR_REG"]
    pub app_cache_lock_1_addr: APP_CACHE_LOCK_1_ADDR,
    #[doc = "0x68 - DPORT_APP_CACHE_LOCK_2_ADDR_REG"]
    pub app_cache_lock_2_addr: APP_CACHE_LOCK_2_ADDR,
    #[doc = "0x6c - DPORT_APP_CACHE_LOCK_3_ADDR_REG"]
    pub app_cache_lock_3_addr: APP_CACHE_LOCK_3_ADDR,
    #[doc = "0x70 - DPORT_TRACEMEM_MUX_MODE_REG"]
    pub tracemem_mux_mode: TRACEMEM_MUX_MODE,
    #[doc = "0x74 - DPORT_PRO_TRACEMEM_ENA_REG"]
    pub pro_tracemem_ena: PRO_TRACEMEM_ENA,
    #[doc = "0x78 - DPORT_APP_TRACEMEM_ENA_REG"]
    pub app_tracemem_ena: APP_TRACEMEM_ENA,
    #[doc = "0x7c - DPORT_CACHE_MUX_MODE_REG"]
    pub cache_mux_mode: CACHE_MUX_MODE,
    #[doc = "0x80 - DPORT_IMMU_PAGE_MODE_REG"]
    pub immu_page_mode: IMMU_PAGE_MODE,
    #[doc = "0x84 - DPORT_DMMU_PAGE_MODE_REG"]
    pub dmmu_page_mode: DMMU_PAGE_MODE,
    #[doc = "0x88 - DPORT_ROM_MPU_ENA_REG"]
    pub rom_mpu_ena: ROM_MPU_ENA,
    #[doc = "0x8c - DPORT_MEM_PD_MASK_REG"]
    pub mem_pd_mask: MEM_PD_MASK,
    #[doc = "0x90 - DPORT_ROM_PD_CTRL_REG"]
    pub rom_pd_ctrl: ROM_PD_CTRL,
    #[doc = "0x94 - DPORT_ROM_FO_CTRL_REG"]
    pub rom_fo_ctrl: ROM_FO_CTRL,
    #[doc = "0x98 - DPORT_SRAM_PD_CTRL_0_REG"]
    pub sram_pd_ctrl_0: SRAM_PD_CTRL_0,
    #[doc = "0x9c - DPORT_SRAM_PD_CTRL_1_REG"]
    pub sram_pd_ctrl_1: SRAM_PD_CTRL_1,
    #[doc = "0xa0 - DPORT_SRAM_FO_CTRL_0_REG"]
    pub sram_fo_ctrl_0: SRAM_FO_CTRL_0,
    #[doc = "0xa4 - DPORT_SRAM_FO_CTRL_1_REG"]
    pub sram_fo_ctrl_1: SRAM_FO_CTRL_1,
    #[doc = "0xa8 - DPORT_IRAM_DRAM_AHB_SEL_REG"]
    pub iram_dram_ahb_sel: IRAM_DRAM_AHB_SEL,
    #[doc = "0xac - DPORT_TAG_FO_CTRL_REG"]
    pub tag_fo_ctrl: TAG_FO_CTRL,
    #[doc = "0xb0 - DPORT_AHB_LITE_MASK_REG"]
    pub ahb_lite_mask: AHB_LITE_MASK,
    #[doc = "0xb4 - DPORT_AHB_MPU_TABLE_0_REG"]
    pub ahb_mpu_table_0: AHB_MPU_TABLE_0,
    #[doc = "0xb8 - DPORT_AHB_MPU_TABLE_1_REG"]
    pub ahb_mpu_table_1: AHB_MPU_TABLE_1,
    #[doc = "0xbc - DPORT_HOST_INF_SEL_REG"]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0xc0 - DPORT_PERIP_CLK_EN_REG"]
    pub perip_clk_en: PERIP_CLK_EN,
    #[doc = "0xc4 - DPORT_PERIP_RST_EN_REG"]
    pub perip_rst_en: PERIP_RST_EN,
    _reserved50: [u8; 4usize],
    #[doc = "0xcc - DPORT_WIFI_CLK_EN_REG"]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0xd0 - DPORT_CORE_RST_EN_REG"]
    pub core_rst_en: CORE_RST_EN,
    #[doc = "0xd4 - DPORT_BT_LPCK_DIV_INT_REG"]
    pub bt_lpck_div_int: BT_LPCK_DIV_INT,
    #[doc = "0xd8 - DPORT_BT_LPCK_DIV_FRAC_REG"]
    pub bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    #[doc = "0xdc - DPORT_CPU_INTR_FROM_CPU_0_REG"]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0xe0 - DPORT_CPU_INTR_FROM_CPU_1_REG"]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0xe4 - DPORT_CPU_INTR_FROM_CPU_2_REG"]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0xe8 - DPORT_CPU_INTR_FROM_CPU_3_REG"]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0xec - DPORT_PRO_INTR_STATUS_0_REG"]
    pub pro_intr_status_0: PRO_INTR_STATUS_0,
    #[doc = "0xf0 - DPORT_PRO_INTR_STATUS_1_REG"]
    pub pro_intr_status_1: PRO_INTR_STATUS_1,
    #[doc = "0xf4 - DPORT_PRO_INTR_STATUS_2_REG"]
    pub pro_intr_status_2: PRO_INTR_STATUS_2,
    #[doc = "0xf8 - DPORT_APP_INTR_STATUS_0_REG"]
    pub app_intr_status_0: APP_INTR_STATUS_0,
    #[doc = "0xfc - DPORT_APP_INTR_STATUS_1_REG"]
    pub app_intr_status_1: APP_INTR_STATUS_1,
    #[doc = "0x100 - DPORT_APP_INTR_STATUS_2_REG"]
    pub app_intr_status_2: APP_INTR_STATUS_2,
    #[doc = "0x104 - DPORT_PRO_MAC_INTR_MAP_REG"]
    pub pro_mac_intr_map: PRO_MAC_INTR_MAP,
    #[doc = "0x108 - DPORT_PRO_MAC_NMI_MAP_REG"]
    pub pro_mac_nmi_map: PRO_MAC_NMI_MAP,
    #[doc = "0x10c - DPORT_PRO_BB_INT_MAP_REG"]
    pub pro_bb_int_map: PRO_BB_INT_MAP,
    #[doc = "0x110 - DPORT_PRO_BT_MAC_INT_MAP_REG"]
    pub pro_bt_mac_int_map: PRO_BT_MAC_INT_MAP,
    #[doc = "0x114 - DPORT_PRO_BT_BB_INT_MAP_REG"]
    pub pro_bt_bb_int_map: PRO_BT_BB_INT_MAP,
    #[doc = "0x118 - DPORT_PRO_BT_BB_NMI_MAP_REG"]
    pub pro_bt_bb_nmi_map: PRO_BT_BB_NMI_MAP,
    #[doc = "0x11c - DPORT_PRO_RWBT_IRQ_MAP_REG"]
    pub pro_rwbt_irq_map: PRO_RWBT_IRQ_MAP,
    #[doc = "0x120 - DPORT_PRO_RWBLE_IRQ_MAP_REG"]
    pub pro_rwble_irq_map: PRO_RWBLE_IRQ_MAP,
    #[doc = "0x124 - DPORT_PRO_RWBT_NMI_MAP_REG"]
    pub pro_rwbt_nmi_map: PRO_RWBT_NMI_MAP,
    #[doc = "0x128 - DPORT_PRO_RWBLE_NMI_MAP_REG"]
    pub pro_rwble_nmi_map: PRO_RWBLE_NMI_MAP,
    #[doc = "0x12c - DPORT_PRO_SLC0_INTR_MAP_REG"]
    pub pro_slc0_intr_map: PRO_SLC0_INTR_MAP,
    #[doc = "0x130 - DPORT_PRO_SLC1_INTR_MAP_REG"]
    pub pro_slc1_intr_map: PRO_SLC1_INTR_MAP,
    #[doc = "0x134 - DPORT_PRO_UHCI0_INTR_MAP_REG"]
    pub pro_uhci0_intr_map: PRO_UHCI0_INTR_MAP,
    #[doc = "0x138 - DPORT_PRO_UHCI1_INTR_MAP_REG"]
    pub pro_uhci1_intr_map: PRO_UHCI1_INTR_MAP,
    #[doc = "0x13c - DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG"]
    pub pro_tg_t0_level_int_map: PRO_TG_T0_LEVEL_INT_MAP,
    #[doc = "0x140 - DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG"]
    pub pro_tg_t1_level_int_map: PRO_TG_T1_LEVEL_INT_MAP,
    #[doc = "0x144 - DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG"]
    pub pro_tg_wdt_level_int_map: PRO_TG_WDT_LEVEL_INT_MAP,
    #[doc = "0x148 - DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG"]
    pub pro_tg_lact_level_int_map: PRO_TG_LACT_LEVEL_INT_MAP,
    #[doc = "0x14c - DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG"]
    pub pro_tg1_t0_level_int_map: PRO_TG1_T0_LEVEL_INT_MAP,
    #[doc = "0x150 - DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG"]
    pub pro_tg1_t1_level_int_map: PRO_TG1_T1_LEVEL_INT_MAP,
    #[doc = "0x154 - DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG"]
    pub pro_tg1_wdt_level_int_map: PRO_TG1_WDT_LEVEL_INT_MAP,
    #[doc = "0x158 - DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG"]
    pub pro_tg1_lact_level_int_map: PRO_TG1_LACT_LEVEL_INT_MAP,
    #[doc = "0x15c - DPORT_PRO_GPIO_INTERRUPT_MAP_REG"]
    pub pro_gpio_interrupt_map: PRO_GPIO_INTERRUPT_MAP,
    #[doc = "0x160 - DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG"]
    pub pro_gpio_interrupt_nmi_map: PRO_GPIO_INTERRUPT_NMI_MAP,
    #[doc = "0x164 - DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG"]
    pub pro_cpu_intr_from_cpu_0_map: PRO_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x168 - DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG"]
    pub pro_cpu_intr_from_cpu_1_map: PRO_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x16c - DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG"]
    pub pro_cpu_intr_from_cpu_2_map: PRO_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x170 - DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG"]
    pub pro_cpu_intr_from_cpu_3_map: PRO_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x174 - DPORT_PRO_SPI_INTR_0_MAP_REG"]
    pub pro_spi_intr_0_map: PRO_SPI_INTR_0_MAP,
    #[doc = "0x178 - DPORT_PRO_SPI_INTR_1_MAP_REG"]
    pub pro_spi_intr_1_map: PRO_SPI_INTR_1_MAP,
    #[doc = "0x17c - DPORT_PRO_SPI_INTR_2_MAP_REG"]
    pub pro_spi_intr_2_map: PRO_SPI_INTR_2_MAP,
    #[doc = "0x180 - DPORT_PRO_SPI_INTR_3_MAP_REG"]
    pub pro_spi_intr_3_map: PRO_SPI_INTR_3_MAP,
    #[doc = "0x184 - DPORT_PRO_I2S0_INT_MAP_REG"]
    pub pro_i2s0_int_map: PRO_I2S0_INT_MAP,
    #[doc = "0x188 - DPORT_PRO_I2S1_INT_MAP_REG"]
    pub pro_i2s1_int_map: PRO_I2S1_INT_MAP,
    #[doc = "0x18c - DPORT_PRO_UART_INTR_MAP_REG"]
    pub pro_uart_intr_map: PRO_UART_INTR_MAP,
    #[doc = "0x190 - DPORT_PRO_UART1_INTR_MAP_REG"]
    pub pro_uart1_intr_map: PRO_UART1_INTR_MAP,
    #[doc = "0x194 - DPORT_PRO_UART2_INTR_MAP_REG"]
    pub pro_uart2_intr_map: PRO_UART2_INTR_MAP,
    #[doc = "0x198 - DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG"]
    pub pro_sdio_host_interrupt_map: PRO_SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x19c - DPORT_PRO_EMAC_INT_MAP_REG"]
    pub pro_emac_int_map: PRO_EMAC_INT_MAP,
    #[doc = "0x1a0 - DPORT_PRO_PWM0_INTR_MAP_REG"]
    pub pro_pwm0_intr_map: PRO_PWM0_INTR_MAP,
    #[doc = "0x1a4 - DPORT_PRO_PWM1_INTR_MAP_REG"]
    pub pro_pwm1_intr_map: PRO_PWM1_INTR_MAP,
    #[doc = "0x1a8 - DPORT_PRO_PWM2_INTR_MAP_REG"]
    pub pro_pwm2_intr_map: PRO_PWM2_INTR_MAP,
    #[doc = "0x1ac - DPORT_PRO_PWM3_INTR_MAP_REG"]
    pub pro_pwm3_intr_map: PRO_PWM3_INTR_MAP,
    #[doc = "0x1b0 - DPORT_PRO_LEDC_INT_MAP_REG"]
    pub pro_ledc_int_map: PRO_LEDC_INT_MAP,
    #[doc = "0x1b4 - DPORT_PRO_EFUSE_INT_MAP_REG"]
    pub pro_efuse_int_map: PRO_EFUSE_INT_MAP,
    #[doc = "0x1b8 - DPORT_PRO_CAN_INT_MAP_REG"]
    pub pro_can_int_map: PRO_CAN_INT_MAP,
    #[doc = "0x1bc - DPORT_PRO_RTC_CORE_INTR_MAP_REG"]
    pub pro_rtc_core_intr_map: PRO_RTC_CORE_INTR_MAP,
    #[doc = "0x1c0 - DPORT_PRO_RMT_INTR_MAP_REG"]
    pub pro_rmt_intr_map: PRO_RMT_INTR_MAP,
    #[doc = "0x1c4 - DPORT_PRO_PCNT_INTR_MAP_REG"]
    pub pro_pcnt_intr_map: PRO_PCNT_INTR_MAP,
    #[doc = "0x1c8 - DPORT_PRO_I2C_EXT0_INTR_MAP_REG"]
    pub pro_i2c_ext0_intr_map: PRO_I2C_EXT0_INTR_MAP,
    #[doc = "0x1cc - DPORT_PRO_I2C_EXT1_INTR_MAP_REG"]
    pub pro_i2c_ext1_intr_map: PRO_I2C_EXT1_INTR_MAP,
    #[doc = "0x1d0 - DPORT_PRO_RSA_INTR_MAP_REG"]
    pub pro_rsa_intr_map: PRO_RSA_INTR_MAP,
    #[doc = "0x1d4 - DPORT_PRO_SPI1_DMA_INT_MAP_REG"]
    pub pro_spi1_dma_int_map: PRO_SPI1_DMA_INT_MAP,
    #[doc = "0x1d8 - DPORT_PRO_SPI2_DMA_INT_MAP_REG"]
    pub pro_spi2_dma_int_map: PRO_SPI2_DMA_INT_MAP,
    #[doc = "0x1dc - DPORT_PRO_SPI3_DMA_INT_MAP_REG"]
    pub pro_spi3_dma_int_map: PRO_SPI3_DMA_INT_MAP,
    #[doc = "0x1e0 - DPORT_PRO_WDG_INT_MAP_REG"]
    pub pro_wdg_int_map: PRO_WDG_INT_MAP,
    #[doc = "0x1e4 - DPORT_PRO_TIMER_INT1_MAP_REG"]
    pub pro_timer_int1_map: PRO_TIMER_INT1_MAP,
    #[doc = "0x1e8 - DPORT_PRO_TIMER_INT2_MAP_REG"]
    pub pro_timer_int2_map: PRO_TIMER_INT2_MAP,
    #[doc = "0x1ec - DPORT_PRO_TG_T0_EDGE_INT_MAP_REG"]
    pub pro_tg_t0_edge_int_map: PRO_TG_T0_EDGE_INT_MAP,
    #[doc = "0x1f0 - DPORT_PRO_TG_T1_EDGE_INT_MAP_REG"]
    pub pro_tg_t1_edge_int_map: PRO_TG_T1_EDGE_INT_MAP,
    #[doc = "0x1f4 - DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG"]
    pub pro_tg_wdt_edge_int_map: PRO_TG_WDT_EDGE_INT_MAP,
    #[doc = "0x1f8 - DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG"]
    pub pro_tg_lact_edge_int_map: PRO_TG_LACT_EDGE_INT_MAP,
    #[doc = "0x1fc - DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG"]
    pub pro_tg1_t0_edge_int_map: PRO_TG1_T0_EDGE_INT_MAP,
    #[doc = "0x200 - DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG"]
    pub pro_tg1_t1_edge_int_map: PRO_TG1_T1_EDGE_INT_MAP,
    #[doc = "0x204 - DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG"]
    pub pro_tg1_wdt_edge_int_map: PRO_TG1_WDT_EDGE_INT_MAP,
    #[doc = "0x208 - DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG"]
    pub pro_tg1_lact_edge_int_map: PRO_TG1_LACT_EDGE_INT_MAP,
    #[doc = "0x20c - DPORT_PRO_MMU_IA_INT_MAP_REG"]
    pub pro_mmu_ia_int_map: PRO_MMU_IA_INT_MAP,
    #[doc = "0x210 - DPORT_PRO_MPU_IA_INT_MAP_REG"]
    pub pro_mpu_ia_int_map: PRO_MPU_IA_INT_MAP,
    #[doc = "0x214 - DPORT_PRO_CACHE_IA_INT_MAP_REG"]
    pub pro_cache_ia_int_map: PRO_CACHE_IA_INT_MAP,
    #[doc = "0x218 - DPORT_APP_MAC_INTR_MAP_REG"]
    pub app_mac_intr_map: APP_MAC_INTR_MAP,
    #[doc = "0x21c - DPORT_APP_MAC_NMI_MAP_REG"]
    pub app_mac_nmi_map: APP_MAC_NMI_MAP,
    #[doc = "0x220 - DPORT_APP_BB_INT_MAP_REG"]
    pub app_bb_int_map: APP_BB_INT_MAP,
    #[doc = "0x224 - DPORT_APP_BT_MAC_INT_MAP_REG"]
    pub app_bt_mac_int_map: APP_BT_MAC_INT_MAP,
    #[doc = "0x228 - DPORT_APP_BT_BB_INT_MAP_REG"]
    pub app_bt_bb_int_map: APP_BT_BB_INT_MAP,
    #[doc = "0x22c - DPORT_APP_BT_BB_NMI_MAP_REG"]
    pub app_bt_bb_nmi_map: APP_BT_BB_NMI_MAP,
    #[doc = "0x230 - DPORT_APP_RWBT_IRQ_MAP_REG"]
    pub app_rwbt_irq_map: APP_RWBT_IRQ_MAP,
    #[doc = "0x234 - DPORT_APP_RWBLE_IRQ_MAP_REG"]
    pub app_rwble_irq_map: APP_RWBLE_IRQ_MAP,
    #[doc = "0x238 - DPORT_APP_RWBT_NMI_MAP_REG"]
    pub app_rwbt_nmi_map: APP_RWBT_NMI_MAP,
    #[doc = "0x23c - DPORT_APP_RWBLE_NMI_MAP_REG"]
    pub app_rwble_nmi_map: APP_RWBLE_NMI_MAP,
    #[doc = "0x240 - DPORT_APP_SLC0_INTR_MAP_REG"]
    pub app_slc0_intr_map: APP_SLC0_INTR_MAP,
    #[doc = "0x244 - DPORT_APP_SLC1_INTR_MAP_REG"]
    pub app_slc1_intr_map: APP_SLC1_INTR_MAP,
    #[doc = "0x248 - DPORT_APP_UHCI0_INTR_MAP_REG"]
    pub app_uhci0_intr_map: APP_UHCI0_INTR_MAP,
    #[doc = "0x24c - DPORT_APP_UHCI1_INTR_MAP_REG"]
    pub app_uhci1_intr_map: APP_UHCI1_INTR_MAP,
    #[doc = "0x250 - DPORT_APP_TG_T0_LEVEL_INT_MAP_REG"]
    pub app_tg_t0_level_int_map: APP_TG_T0_LEVEL_INT_MAP,
    #[doc = "0x254 - DPORT_APP_TG_T1_LEVEL_INT_MAP_REG"]
    pub app_tg_t1_level_int_map: APP_TG_T1_LEVEL_INT_MAP,
    #[doc = "0x258 - DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG"]
    pub app_tg_wdt_level_int_map: APP_TG_WDT_LEVEL_INT_MAP,
    #[doc = "0x25c - DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG"]
    pub app_tg_lact_level_int_map: APP_TG_LACT_LEVEL_INT_MAP,
    #[doc = "0x260 - DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG"]
    pub app_tg1_t0_level_int_map: APP_TG1_T0_LEVEL_INT_MAP,
    #[doc = "0x264 - DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG"]
    pub app_tg1_t1_level_int_map: APP_TG1_T1_LEVEL_INT_MAP,
    #[doc = "0x268 - DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG"]
    pub app_tg1_wdt_level_int_map: APP_TG1_WDT_LEVEL_INT_MAP,
    #[doc = "0x26c - DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG"]
    pub app_tg1_lact_level_int_map: APP_TG1_LACT_LEVEL_INT_MAP,
    #[doc = "0x270 - DPORT_APP_GPIO_INTERRUPT_MAP_REG"]
    pub app_gpio_interrupt_map: APP_GPIO_INTERRUPT_MAP,
    #[doc = "0x274 - DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG"]
    pub app_gpio_interrupt_nmi_map: APP_GPIO_INTERRUPT_NMI_MAP,
    #[doc = "0x278 - DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG"]
    pub app_cpu_intr_from_cpu_0_map: APP_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x27c - DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG"]
    pub app_cpu_intr_from_cpu_1_map: APP_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x280 - DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG"]
    pub app_cpu_intr_from_cpu_2_map: APP_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x284 - DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG"]
    pub app_cpu_intr_from_cpu_3_map: APP_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x288 - DPORT_APP_SPI_INTR_0_MAP_REG"]
    pub app_spi_intr_0_map: APP_SPI_INTR_0_MAP,
    #[doc = "0x28c - DPORT_APP_SPI_INTR_1_MAP_REG"]
    pub app_spi_intr_1_map: APP_SPI_INTR_1_MAP,
    #[doc = "0x290 - DPORT_APP_SPI_INTR_2_MAP_REG"]
    pub app_spi_intr_2_map: APP_SPI_INTR_2_MAP,
    #[doc = "0x294 - DPORT_APP_SPI_INTR_3_MAP_REG"]
    pub app_spi_intr_3_map: APP_SPI_INTR_3_MAP,
    #[doc = "0x298 - DPORT_APP_I2S0_INT_MAP_REG"]
    pub app_i2s0_int_map: APP_I2S0_INT_MAP,
    #[doc = "0x29c - DPORT_APP_I2S1_INT_MAP_REG"]
    pub app_i2s1_int_map: APP_I2S1_INT_MAP,
    #[doc = "0x2a0 - DPORT_APP_UART_INTR_MAP_REG"]
    pub app_uart_intr_map: APP_UART_INTR_MAP,
    #[doc = "0x2a4 - DPORT_APP_UART1_INTR_MAP_REG"]
    pub app_uart1_intr_map: APP_UART1_INTR_MAP,
    #[doc = "0x2a8 - DPORT_APP_UART2_INTR_MAP_REG"]
    pub app_uart2_intr_map: APP_UART2_INTR_MAP,
    #[doc = "0x2ac - DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG"]
    pub app_sdio_host_interrupt_map: APP_SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x2b0 - DPORT_APP_EMAC_INT_MAP_REG"]
    pub app_emac_int_map: APP_EMAC_INT_MAP,
    #[doc = "0x2b4 - DPORT_APP_PWM0_INTR_MAP_REG"]
    pub app_pwm0_intr_map: APP_PWM0_INTR_MAP,
    #[doc = "0x2b8 - DPORT_APP_PWM1_INTR_MAP_REG"]
    pub app_pwm1_intr_map: APP_PWM1_INTR_MAP,
    #[doc = "0x2bc - DPORT_APP_PWM2_INTR_MAP_REG"]
    pub app_pwm2_intr_map: APP_PWM2_INTR_MAP,
    #[doc = "0x2c0 - DPORT_APP_PWM3_INTR_MAP_REG"]
    pub app_pwm3_intr_map: APP_PWM3_INTR_MAP,
    #[doc = "0x2c4 - DPORT_APP_LEDC_INT_MAP_REG"]
    pub app_ledc_int_map: APP_LEDC_INT_MAP,
    #[doc = "0x2c8 - DPORT_APP_EFUSE_INT_MAP_REG"]
    pub app_efuse_int_map: APP_EFUSE_INT_MAP,
    #[doc = "0x2cc - DPORT_APP_CAN_INT_MAP_REG"]
    pub app_can_int_map: APP_CAN_INT_MAP,
    #[doc = "0x2d0 - DPORT_APP_RTC_CORE_INTR_MAP_REG"]
    pub app_rtc_core_intr_map: APP_RTC_CORE_INTR_MAP,
    #[doc = "0x2d4 - DPORT_APP_RMT_INTR_MAP_REG"]
    pub app_rmt_intr_map: APP_RMT_INTR_MAP,
    #[doc = "0x2d8 - DPORT_APP_PCNT_INTR_MAP_REG"]
    pub app_pcnt_intr_map: APP_PCNT_INTR_MAP,
    #[doc = "0x2dc - DPORT_APP_I2C_EXT0_INTR_MAP_REG"]
    pub app_i2c_ext0_intr_map: APP_I2C_EXT0_INTR_MAP,
    #[doc = "0x2e0 - DPORT_APP_I2C_EXT1_INTR_MAP_REG"]
    pub app_i2c_ext1_intr_map: APP_I2C_EXT1_INTR_MAP,
    #[doc = "0x2e4 - DPORT_APP_RSA_INTR_MAP_REG"]
    pub app_rsa_intr_map: APP_RSA_INTR_MAP,
    #[doc = "0x2e8 - DPORT_APP_SPI1_DMA_INT_MAP_REG"]
    pub app_spi1_dma_int_map: APP_SPI1_DMA_INT_MAP,
    #[doc = "0x2ec - DPORT_APP_SPI2_DMA_INT_MAP_REG"]
    pub app_spi2_dma_int_map: APP_SPI2_DMA_INT_MAP,
    #[doc = "0x2f0 - DPORT_APP_SPI3_DMA_INT_MAP_REG"]
    pub app_spi3_dma_int_map: APP_SPI3_DMA_INT_MAP,
    #[doc = "0x2f4 - DPORT_APP_WDG_INT_MAP_REG"]
    pub app_wdg_int_map: APP_WDG_INT_MAP,
    #[doc = "0x2f8 - DPORT_APP_TIMER_INT1_MAP_REG"]
    pub app_timer_int1_map: APP_TIMER_INT1_MAP,
    #[doc = "0x2fc - DPORT_APP_TIMER_INT2_MAP_REG"]
    pub app_timer_int2_map: APP_TIMER_INT2_MAP,
    #[doc = "0x300 - DPORT_APP_TG_T0_EDGE_INT_MAP_REG"]
    pub app_tg_t0_edge_int_map: APP_TG_T0_EDGE_INT_MAP,
    #[doc = "0x304 - DPORT_APP_TG_T1_EDGE_INT_MAP_REG"]
    pub app_tg_t1_edge_int_map: APP_TG_T1_EDGE_INT_MAP,
    #[doc = "0x308 - DPORT_APP_TG_WDT_EDGE_INT_MAP_REG"]
    pub app_tg_wdt_edge_int_map: APP_TG_WDT_EDGE_INT_MAP,
    #[doc = "0x30c - DPORT_APP_TG_LACT_EDGE_INT_MAP_REG"]
    pub app_tg_lact_edge_int_map: APP_TG_LACT_EDGE_INT_MAP,
    #[doc = "0x310 - DPORT_APP_TG1_T0_EDGE_INT_MAP_REG"]
    pub app_tg1_t0_edge_int_map: APP_TG1_T0_EDGE_INT_MAP,
    #[doc = "0x314 - DPORT_APP_TG1_T1_EDGE_INT_MAP_REG"]
    pub app_tg1_t1_edge_int_map: APP_TG1_T1_EDGE_INT_MAP,
    #[doc = "0x318 - DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG"]
    pub app_tg1_wdt_edge_int_map: APP_TG1_WDT_EDGE_INT_MAP,
    #[doc = "0x31c - DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG"]
    pub app_tg1_lact_edge_int_map: APP_TG1_LACT_EDGE_INT_MAP,
    #[doc = "0x320 - DPORT_APP_MMU_IA_INT_MAP_REG"]
    pub app_mmu_ia_int_map: APP_MMU_IA_INT_MAP,
    #[doc = "0x324 - DPORT_APP_MPU_IA_INT_MAP_REG"]
    pub app_mpu_ia_int_map: APP_MPU_IA_INT_MAP,
    #[doc = "0x328 - DPORT_APP_CACHE_IA_INT_MAP_REG"]
    pub app_cache_ia_int_map: APP_CACHE_IA_INT_MAP,
    #[doc = "0x32c - DPORT_AHBLITE_MPU_TABLE_UART_REG"]
    pub ahblite_mpu_table_uart: AHBLITE_MPU_TABLE_UART,
    #[doc = "0x330 - DPORT_AHBLITE_MPU_TABLE_SPI1_REG"]
    pub ahblite_mpu_table_spi1: AHBLITE_MPU_TABLE_SPI1,
    #[doc = "0x334 - DPORT_AHBLITE_MPU_TABLE_SPI0_REG"]
    pub ahblite_mpu_table_spi0: AHBLITE_MPU_TABLE_SPI0,
    #[doc = "0x338 - DPORT_AHBLITE_MPU_TABLE_GPIO_REG"]
    pub ahblite_mpu_table_gpio: AHBLITE_MPU_TABLE_GPIO,
    #[doc = "0x33c - DPORT_AHBLITE_MPU_TABLE_FE2_REG"]
    pub ahblite_mpu_table_fe2: AHBLITE_MPU_TABLE_FE2,
    #[doc = "0x340 - DPORT_AHBLITE_MPU_TABLE_FE_REG"]
    pub ahblite_mpu_table_fe: AHBLITE_MPU_TABLE_FE,
    #[doc = "0x344 - DPORT_AHBLITE_MPU_TABLE_TIMER_REG"]
    pub ahblite_mpu_table_timer: AHBLITE_MPU_TABLE_TIMER,
    #[doc = "0x348 - DPORT_AHBLITE_MPU_TABLE_RTC_REG"]
    pub ahblite_mpu_table_rtc: AHBLITE_MPU_TABLE_RTC,
    #[doc = "0x34c - DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG"]
    pub ahblite_mpu_table_io_mux: AHBLITE_MPU_TABLE_IO_MUX,
    #[doc = "0x350 - DPORT_AHBLITE_MPU_TABLE_WDG_REG"]
    pub ahblite_mpu_table_wdg: AHBLITE_MPU_TABLE_WDG,
    #[doc = "0x354 - DPORT_AHBLITE_MPU_TABLE_HINF_REG"]
    pub ahblite_mpu_table_hinf: AHBLITE_MPU_TABLE_HINF,
    #[doc = "0x358 - DPORT_AHBLITE_MPU_TABLE_UHCI1_REG"]
    pub ahblite_mpu_table_uhci1: AHBLITE_MPU_TABLE_UHCI1,
    #[doc = "0x35c - DPORT_AHBLITE_MPU_TABLE_MISC_REG"]
    pub ahblite_mpu_table_misc: AHBLITE_MPU_TABLE_MISC,
    #[doc = "0x360 - DPORT_AHBLITE_MPU_TABLE_I2C_REG"]
    pub ahblite_mpu_table_i2c: AHBLITE_MPU_TABLE_I2C,
    #[doc = "0x364 - DPORT_AHBLITE_MPU_TABLE_I2S0_REG"]
    pub ahblite_mpu_table_i2s0: AHBLITE_MPU_TABLE_I2S0,
    #[doc = "0x368 - DPORT_AHBLITE_MPU_TABLE_UART1_REG"]
    pub ahblite_mpu_table_uart1: AHBLITE_MPU_TABLE_UART1,
    #[doc = "0x36c - DPORT_AHBLITE_MPU_TABLE_BT_REG"]
    pub ahblite_mpu_table_bt: AHBLITE_MPU_TABLE_BT,
    #[doc = "0x370 - DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG"]
    pub ahblite_mpu_table_bt_buffer: AHBLITE_MPU_TABLE_BT_BUFFER,
    #[doc = "0x374 - DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG"]
    pub ahblite_mpu_table_i2c_ext0: AHBLITE_MPU_TABLE_I2C_EXT0,
    #[doc = "0x378 - DPORT_AHBLITE_MPU_TABLE_UHCI0_REG"]
    pub ahblite_mpu_table_uhci0: AHBLITE_MPU_TABLE_UHCI0,
    #[doc = "0x37c - DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG"]
    pub ahblite_mpu_table_slchost: AHBLITE_MPU_TABLE_SLCHOST,
    #[doc = "0x380 - DPORT_AHBLITE_MPU_TABLE_RMT_REG"]
    pub ahblite_mpu_table_rmt: AHBLITE_MPU_TABLE_RMT,
    #[doc = "0x384 - DPORT_AHBLITE_MPU_TABLE_PCNT_REG"]
    pub ahblite_mpu_table_pcnt: AHBLITE_MPU_TABLE_PCNT,
    #[doc = "0x388 - DPORT_AHBLITE_MPU_TABLE_SLC_REG"]
    pub ahblite_mpu_table_slc: AHBLITE_MPU_TABLE_SLC,
    #[doc = "0x38c - DPORT_AHBLITE_MPU_TABLE_LEDC_REG"]
    pub ahblite_mpu_table_ledc: AHBLITE_MPU_TABLE_LEDC,
    #[doc = "0x390 - DPORT_AHBLITE_MPU_TABLE_EFUSE_REG"]
    pub ahblite_mpu_table_efuse: AHBLITE_MPU_TABLE_EFUSE,
    #[doc = "0x394 - DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
    pub ahblite_mpu_table_spi_encrypt: AHBLITE_MPU_TABLE_SPI_ENCRYPT,
    #[doc = "0x398 - DPORT_AHBLITE_MPU_TABLE_BB_REG"]
    pub ahblite_mpu_table_bb: AHBLITE_MPU_TABLE_BB,
    #[doc = "0x39c - DPORT_AHBLITE_MPU_TABLE_PWM0_REG"]
    pub ahblite_mpu_table_pwm0: AHBLITE_MPU_TABLE_PWM0,
    #[doc = "0x3a0 - DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG"]
    pub ahblite_mpu_table_timergroup: AHBLITE_MPU_TABLE_TIMERGROUP,
    #[doc = "0x3a4 - DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG"]
    pub ahblite_mpu_table_timergroup1: AHBLITE_MPU_TABLE_TIMERGROUP1,
    #[doc = "0x3a8 - DPORT_AHBLITE_MPU_TABLE_SPI2_REG"]
    pub ahblite_mpu_table_spi2: AHBLITE_MPU_TABLE_SPI2,
    #[doc = "0x3ac - DPORT_AHBLITE_MPU_TABLE_SPI3_REG"]
    pub ahblite_mpu_table_spi3: AHBLITE_MPU_TABLE_SPI3,
    #[doc = "0x3b0 - DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG"]
    pub ahblite_mpu_table_apb_ctrl: AHBLITE_MPU_TABLE_APB_CTRL,
    #[doc = "0x3b4 - DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG"]
    pub ahblite_mpu_table_i2c_ext1: AHBLITE_MPU_TABLE_I2C_EXT1,
    #[doc = "0x3b8 - DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG"]
    pub ahblite_mpu_table_sdio_host: AHBLITE_MPU_TABLE_SDIO_HOST,
    #[doc = "0x3bc - DPORT_AHBLITE_MPU_TABLE_EMAC_REG"]
    pub ahblite_mpu_table_emac: AHBLITE_MPU_TABLE_EMAC,
    #[doc = "0x3c0 - DPORT_AHBLITE_MPU_TABLE_CAN_REG"]
    pub ahblite_mpu_table_can: AHBLITE_MPU_TABLE_CAN,
    #[doc = "0x3c4 - DPORT_AHBLITE_MPU_TABLE_PWM1_REG"]
    pub ahblite_mpu_table_pwm1: AHBLITE_MPU_TABLE_PWM1,
    #[doc = "0x3c8 - DPORT_AHBLITE_MPU_TABLE_I2S1_REG"]
    pub ahblite_mpu_table_i2s1: AHBLITE_MPU_TABLE_I2S1,
    #[doc = "0x3cc - DPORT_AHBLITE_MPU_TABLE_UART2_REG"]
    pub ahblite_mpu_table_uart2: AHBLITE_MPU_TABLE_UART2,
    #[doc = "0x3d0 - DPORT_AHBLITE_MPU_TABLE_PWM2_REG"]
    pub ahblite_mpu_table_pwm2: AHBLITE_MPU_TABLE_PWM2,
    #[doc = "0x3d4 - DPORT_AHBLITE_MPU_TABLE_PWM3_REG"]
    pub ahblite_mpu_table_pwm3: AHBLITE_MPU_TABLE_PWM3,
    #[doc = "0x3d8 - DPORT_AHBLITE_MPU_TABLE_RWBT_REG"]
    pub ahblite_mpu_table_rwbt: AHBLITE_MPU_TABLE_RWBT,
    #[doc = "0x3dc - DPORT_AHBLITE_MPU_TABLE_BTMAC_REG"]
    pub ahblite_mpu_table_btmac: AHBLITE_MPU_TABLE_BTMAC,
    #[doc = "0x3e0 - DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG"]
    pub ahblite_mpu_table_wifimac: AHBLITE_MPU_TABLE_WIFIMAC,
    #[doc = "0x3e4 - DPORT_AHBLITE_MPU_TABLE_PWR_REG"]
    pub ahblite_mpu_table_pwr: AHBLITE_MPU_TABLE_PWR,
    #[doc = "0x3e8 - DPORT_MEM_ACCESS_DBUG0_REG"]
    pub mem_access_dbug0: MEM_ACCESS_DBUG0,
    #[doc = "0x3ec - DPORT_MEM_ACCESS_DBUG1_REG"]
    pub mem_access_dbug1: MEM_ACCESS_DBUG1,
    #[doc = "0x3f0 - DPORT_PRO_DCACHE_DBUG0_REG"]
    pub pro_dcache_dbug0: PRO_DCACHE_DBUG0,
    #[doc = "0x3f4 - DPORT_PRO_DCACHE_DBUG1_REG"]
    pub pro_dcache_dbug1: PRO_DCACHE_DBUG1,
    #[doc = "0x3f8 - DPORT_PRO_DCACHE_DBUG2_REG"]
    pub pro_dcache_dbug2: PRO_DCACHE_DBUG2,
    #[doc = "0x3fc - DPORT_PRO_DCACHE_DBUG3_REG"]
    pub pro_dcache_dbug3: PRO_DCACHE_DBUG3,
    #[doc = "0x400 - DPORT_PRO_DCACHE_DBUG4_REG"]
    pub pro_dcache_dbug4: PRO_DCACHE_DBUG4,
    #[doc = "0x404 - DPORT_PRO_DCACHE_DBUG5_REG"]
    pub pro_dcache_dbug5: PRO_DCACHE_DBUG5,
    #[doc = "0x408 - DPORT_PRO_DCACHE_DBUG6_REG"]
    pub pro_dcache_dbug6: PRO_DCACHE_DBUG6,
    #[doc = "0x40c - DPORT_PRO_DCACHE_DBUG7_REG"]
    pub pro_dcache_dbug7: PRO_DCACHE_DBUG7,
    #[doc = "0x410 - DPORT_PRO_DCACHE_DBUG8_REG"]
    pub pro_dcache_dbug8: PRO_DCACHE_DBUG8,
    #[doc = "0x414 - DPORT_PRO_DCACHE_DBUG9_REG"]
    pub pro_dcache_dbug9: PRO_DCACHE_DBUG9,
    #[doc = "0x418 - DPORT_APP_DCACHE_DBUG0_REG"]
    pub app_dcache_dbug0: APP_DCACHE_DBUG0,
    #[doc = "0x41c - DPORT_APP_DCACHE_DBUG1_REG"]
    pub app_dcache_dbug1: APP_DCACHE_DBUG1,
    #[doc = "0x420 - DPORT_APP_DCACHE_DBUG2_REG"]
    pub app_dcache_dbug2: APP_DCACHE_DBUG2,
    #[doc = "0x424 - DPORT_APP_DCACHE_DBUG3_REG"]
    pub app_dcache_dbug3: APP_DCACHE_DBUG3,
    #[doc = "0x428 - DPORT_APP_DCACHE_DBUG4_REG"]
    pub app_dcache_dbug4: APP_DCACHE_DBUG4,
    #[doc = "0x42c - DPORT_APP_DCACHE_DBUG5_REG"]
    pub app_dcache_dbug5: APP_DCACHE_DBUG5,
    #[doc = "0x430 - DPORT_APP_DCACHE_DBUG6_REG"]
    pub app_dcache_dbug6: APP_DCACHE_DBUG6,
    #[doc = "0x434 - DPORT_APP_DCACHE_DBUG7_REG"]
    pub app_dcache_dbug7: APP_DCACHE_DBUG7,
    #[doc = "0x438 - DPORT_APP_DCACHE_DBUG8_REG"]
    pub app_dcache_dbug8: APP_DCACHE_DBUG8,
    #[doc = "0x43c - DPORT_APP_DCACHE_DBUG9_REG"]
    pub app_dcache_dbug9: APP_DCACHE_DBUG9,
    #[doc = "0x440 - DPORT_PRO_CPU_RECORD_CTRL_REG"]
    pub pro_cpu_record_ctrl: PRO_CPU_RECORD_CTRL,
    #[doc = "0x444 - DPORT_PRO_CPU_RECORD_STATUS_REG"]
    pub pro_cpu_record_status: PRO_CPU_RECORD_STATUS,
    #[doc = "0x448 - DPORT_PRO_CPU_RECORD_PID_REG"]
    pub pro_cpu_record_pid: PRO_CPU_RECORD_PID,
    #[doc = "0x44c - DPORT_PRO_CPU_RECORD_PDEBUGINST_REG"]
    pub pro_cpu_record_pdebuginst: PRO_CPU_RECORD_PDEBUGINST,
    #[doc = "0x450 - DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG"]
    pub pro_cpu_record_pdebugstatus: PRO_CPU_RECORD_PDEBUGSTATUS,
    #[doc = "0x454 - DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
    pub pro_cpu_record_pdebugdata: PRO_CPU_RECORD_PDEBUGDATA,
    #[doc = "0x458 - DPORT_PRO_CPU_RECORD_PDEBUGPC_REG"]
    pub pro_cpu_record_pdebugpc: PRO_CPU_RECORD_PDEBUGPC,
    #[doc = "0x45c - DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG"]
    pub pro_cpu_record_pdebugls0stat: PRO_CPU_RECORD_PDEBUGLS0STAT,
    #[doc = "0x460 - DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG"]
    pub pro_cpu_record_pdebugls0addr: PRO_CPU_RECORD_PDEBUGLS0ADDR,
    #[doc = "0x464 - DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG"]
    pub pro_cpu_record_pdebugls0data: PRO_CPU_RECORD_PDEBUGLS0DATA,
    #[doc = "0x468 - DPORT_APP_CPU_RECORD_CTRL_REG"]
    pub app_cpu_record_ctrl: APP_CPU_RECORD_CTRL,
    #[doc = "0x46c - DPORT_APP_CPU_RECORD_STATUS_REG"]
    pub app_cpu_record_status: APP_CPU_RECORD_STATUS,
    #[doc = "0x470 - DPORT_APP_CPU_RECORD_PID_REG"]
    pub app_cpu_record_pid: APP_CPU_RECORD_PID,
    #[doc = "0x474 - DPORT_APP_CPU_RECORD_PDEBUGINST_REG"]
    pub app_cpu_record_pdebuginst: APP_CPU_RECORD_PDEBUGINST,
    #[doc = "0x478 - DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
    pub app_cpu_record_pdebugstatus: APP_CPU_RECORD_PDEBUGSTATUS,
    #[doc = "0x47c - DPORT_APP_CPU_RECORD_PDEBUGDATA_REG"]
    pub app_cpu_record_pdebugdata: APP_CPU_RECORD_PDEBUGDATA,
    #[doc = "0x480 - DPORT_APP_CPU_RECORD_PDEBUGPC_REG"]
    pub app_cpu_record_pdebugpc: APP_CPU_RECORD_PDEBUGPC,
    #[doc = "0x484 - DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG"]
    pub app_cpu_record_pdebugls0stat: APP_CPU_RECORD_PDEBUGLS0STAT,
    #[doc = "0x488 - DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG"]
    pub app_cpu_record_pdebugls0addr: APP_CPU_RECORD_PDEBUGLS0ADDR,
    #[doc = "0x48c - DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG"]
    pub app_cpu_record_pdebugls0data: APP_CPU_RECORD_PDEBUGLS0DATA,
    #[doc = "0x490 - DPORT_RSA_PD_CTRL_REG"]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0x494 - DPORT_ROM_MPU_TABLE0_REG"]
    pub rom_mpu_table0: ROM_MPU_TABLE0,
    #[doc = "0x498 - DPORT_ROM_MPU_TABLE1_REG"]
    pub rom_mpu_table1: ROM_MPU_TABLE1,
    #[doc = "0x49c - DPORT_ROM_MPU_TABLE2_REG"]
    pub rom_mpu_table2: ROM_MPU_TABLE2,
    #[doc = "0x4a0 - DPORT_ROM_MPU_TABLE3_REG"]
    pub rom_mpu_table3: ROM_MPU_TABLE3,
    #[doc = "0x4a4 - DPORT_SHROM_MPU_TABLE0_REG"]
    pub shrom_mpu_table0: SHROM_MPU_TABLE0,
    #[doc = "0x4a8 - DPORT_SHROM_MPU_TABLE1_REG"]
    pub shrom_mpu_table1: SHROM_MPU_TABLE1,
    #[doc = "0x4ac - DPORT_SHROM_MPU_TABLE2_REG"]
    pub shrom_mpu_table2: SHROM_MPU_TABLE2,
    #[doc = "0x4b0 - DPORT_SHROM_MPU_TABLE3_REG"]
    pub shrom_mpu_table3: SHROM_MPU_TABLE3,
    #[doc = "0x4b4 - DPORT_SHROM_MPU_TABLE4_REG"]
    pub shrom_mpu_table4: SHROM_MPU_TABLE4,
    #[doc = "0x4b8 - DPORT_SHROM_MPU_TABLE5_REG"]
    pub shrom_mpu_table5: SHROM_MPU_TABLE5,
    #[doc = "0x4bc - DPORT_SHROM_MPU_TABLE6_REG"]
    pub shrom_mpu_table6: SHROM_MPU_TABLE6,
    #[doc = "0x4c0 - DPORT_SHROM_MPU_TABLE7_REG"]
    pub shrom_mpu_table7: SHROM_MPU_TABLE7,
    #[doc = "0x4c4 - DPORT_SHROM_MPU_TABLE8_REG"]
    pub shrom_mpu_table8: SHROM_MPU_TABLE8,
    #[doc = "0x4c8 - DPORT_SHROM_MPU_TABLE9_REG"]
    pub shrom_mpu_table9: SHROM_MPU_TABLE9,
    #[doc = "0x4cc - DPORT_SHROM_MPU_TABLE10_REG"]
    pub shrom_mpu_table10: SHROM_MPU_TABLE10,
    #[doc = "0x4d0 - DPORT_SHROM_MPU_TABLE11_REG"]
    pub shrom_mpu_table11: SHROM_MPU_TABLE11,
    #[doc = "0x4d4 - DPORT_SHROM_MPU_TABLE12_REG"]
    pub shrom_mpu_table12: SHROM_MPU_TABLE12,
    #[doc = "0x4d8 - DPORT_SHROM_MPU_TABLE13_REG"]
    pub shrom_mpu_table13: SHROM_MPU_TABLE13,
    #[doc = "0x4dc - DPORT_SHROM_MPU_TABLE14_REG"]
    pub shrom_mpu_table14: SHROM_MPU_TABLE14,
    #[doc = "0x4e0 - DPORT_SHROM_MPU_TABLE15_REG"]
    pub shrom_mpu_table15: SHROM_MPU_TABLE15,
    #[doc = "0x4e4 - DPORT_SHROM_MPU_TABLE16_REG"]
    pub shrom_mpu_table16: SHROM_MPU_TABLE16,
    #[doc = "0x4e8 - DPORT_SHROM_MPU_TABLE17_REG"]
    pub shrom_mpu_table17: SHROM_MPU_TABLE17,
    #[doc = "0x4ec - DPORT_SHROM_MPU_TABLE18_REG"]
    pub shrom_mpu_table18: SHROM_MPU_TABLE18,
    #[doc = "0x4f0 - DPORT_SHROM_MPU_TABLE19_REG"]
    pub shrom_mpu_table19: SHROM_MPU_TABLE19,
    #[doc = "0x4f4 - DPORT_SHROM_MPU_TABLE20_REG"]
    pub shrom_mpu_table20: SHROM_MPU_TABLE20,
    #[doc = "0x4f8 - DPORT_SHROM_MPU_TABLE21_REG"]
    pub shrom_mpu_table21: SHROM_MPU_TABLE21,
    #[doc = "0x4fc - DPORT_SHROM_MPU_TABLE22_REG"]
    pub shrom_mpu_table22: SHROM_MPU_TABLE22,
    #[doc = "0x500 - DPORT_SHROM_MPU_TABLE23_REG"]
    pub shrom_mpu_table23: SHROM_MPU_TABLE23,
    #[doc = "0x504 - DPORT_IMMU_TABLE0_REG"]
    pub immu_table0: IMMU_TABLE0,
    #[doc = "0x508 - DPORT_IMMU_TABLE1_REG"]
    pub immu_table1: IMMU_TABLE1,
    #[doc = "0x50c - DPORT_IMMU_TABLE2_REG"]
    pub immu_table2: IMMU_TABLE2,
    #[doc = "0x510 - DPORT_IMMU_TABLE3_REG"]
    pub immu_table3: IMMU_TABLE3,
    #[doc = "0x514 - DPORT_IMMU_TABLE4_REG"]
    pub immu_table4: IMMU_TABLE4,
    #[doc = "0x518 - DPORT_IMMU_TABLE5_REG"]
    pub immu_table5: IMMU_TABLE5,
    #[doc = "0x51c - DPORT_IMMU_TABLE6_REG"]
    pub immu_table6: IMMU_TABLE6,
    #[doc = "0x520 - DPORT_IMMU_TABLE7_REG"]
    pub immu_table7: IMMU_TABLE7,
    #[doc = "0x524 - DPORT_IMMU_TABLE8_REG"]
    pub immu_table8: IMMU_TABLE8,
    #[doc = "0x528 - DPORT_IMMU_TABLE9_REG"]
    pub immu_table9: IMMU_TABLE9,
    #[doc = "0x52c - DPORT_IMMU_TABLE10_REG"]
    pub immu_table10: IMMU_TABLE10,
    #[doc = "0x530 - DPORT_IMMU_TABLE11_REG"]
    pub immu_table11: IMMU_TABLE11,
    #[doc = "0x534 - DPORT_IMMU_TABLE12_REG"]
    pub immu_table12: IMMU_TABLE12,
    #[doc = "0x538 - DPORT_IMMU_TABLE13_REG"]
    pub immu_table13: IMMU_TABLE13,
    #[doc = "0x53c - DPORT_IMMU_TABLE14_REG"]
    pub immu_table14: IMMU_TABLE14,
    #[doc = "0x540 - DPORT_IMMU_TABLE15_REG"]
    pub immu_table15: IMMU_TABLE15,
    #[doc = "0x544 - DPORT_DMMU_TABLE0_REG"]
    pub dmmu_table0: DMMU_TABLE0,
    #[doc = "0x548 - DPORT_DMMU_TABLE1_REG"]
    pub dmmu_table1: DMMU_TABLE1,
    #[doc = "0x54c - DPORT_DMMU_TABLE2_REG"]
    pub dmmu_table2: DMMU_TABLE2,
    #[doc = "0x550 - DPORT_DMMU_TABLE3_REG"]
    pub dmmu_table3: DMMU_TABLE3,
    #[doc = "0x554 - DPORT_DMMU_TABLE4_REG"]
    pub dmmu_table4: DMMU_TABLE4,
    #[doc = "0x558 - DPORT_DMMU_TABLE5_REG"]
    pub dmmu_table5: DMMU_TABLE5,
    #[doc = "0x55c - DPORT_DMMU_TABLE6_REG"]
    pub dmmu_table6: DMMU_TABLE6,
    #[doc = "0x560 - DPORT_DMMU_TABLE7_REG"]
    pub dmmu_table7: DMMU_TABLE7,
    #[doc = "0x564 - DPORT_DMMU_TABLE8_REG"]
    pub dmmu_table8: DMMU_TABLE8,
    #[doc = "0x568 - DPORT_DMMU_TABLE9_REG"]
    pub dmmu_table9: DMMU_TABLE9,
    #[doc = "0x56c - DPORT_DMMU_TABLE10_REG"]
    pub dmmu_table10: DMMU_TABLE10,
    #[doc = "0x570 - DPORT_DMMU_TABLE11_REG"]
    pub dmmu_table11: DMMU_TABLE11,
    #[doc = "0x574 - DPORT_DMMU_TABLE12_REG"]
    pub dmmu_table12: DMMU_TABLE12,
    #[doc = "0x578 - DPORT_DMMU_TABLE13_REG"]
    pub dmmu_table13: DMMU_TABLE13,
    #[doc = "0x57c - DPORT_DMMU_TABLE14_REG"]
    pub dmmu_table14: DMMU_TABLE14,
    #[doc = "0x580 - DPORT_DMMU_TABLE15_REG"]
    pub dmmu_table15: DMMU_TABLE15,
    #[doc = "0x584 - DPORT_PRO_INTRUSION_CTRL_REG"]
    pub pro_intrusion_ctrl: PRO_INTRUSION_CTRL,
    #[doc = "0x588 - DPORT_PRO_INTRUSION_STATUS_REG"]
    pub pro_intrusion_status: PRO_INTRUSION_STATUS,
    #[doc = "0x58c - DPORT_APP_INTRUSION_CTRL_REG"]
    pub app_intrusion_ctrl: APP_INTRUSION_CTRL,
    #[doc = "0x590 - DPORT_APP_INTRUSION_STATUS_REG"]
    pub app_intrusion_status: APP_INTRUSION_STATUS,
    #[doc = "0x594 - DPORT_FRONT_END_MEM_PD_REG"]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    #[doc = "0x598 - DPORT_MMU_IA_INT_EN_REG"]
    pub mmu_ia_int_en: MMU_IA_INT_EN,
    #[doc = "0x59c - DPORT_MPU_IA_INT_EN_REG"]
    pub mpu_ia_int_en: MPU_IA_INT_EN,
    #[doc = "0x5a0 - DPORT_CACHE_IA_INT_EN_REG"]
    pub cache_ia_int_en: CACHE_IA_INT_EN,
    #[doc = "0x5a4 - DPORT_SECURE_BOOT_CTRL_REG"]
    pub secure_boot_ctrl: SECURE_BOOT_CTRL,
    #[doc = "0x5a8 - DPORT_SPI_DMA_CHAN_SEL_REG"]
    pub spi_dma_chan_sel: SPI_DMA_CHAN_SEL,
    #[doc = "0x5ac - DPORT_PRO_VECBASE_CTRL_REG"]
    pub pro_vecbase_ctrl: PRO_VECBASE_CTRL,
    #[doc = "0x5b0 - DPORT_PRO_VECBASE_SET_REG"]
    pub pro_vecbase_set: PRO_VECBASE_SET,
    #[doc = "0x5b4 - DPORT_APP_VECBASE_CTRL_REG"]
    pub app_vecbase_ctrl: APP_VECBASE_CTRL,
    #[doc = "0x5b8 - DPORT_APP_VECBASE_SET_REG"]
    pub app_vecbase_set: APP_VECBASE_SET,
    _reserved366: [u8; 2624usize],
    #[doc = "0xffc - DPORT_DATE_REG"]
    pub date: DATE,
}
#[doc = "DPORT_PRO_BOOT_REMAP_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_boot_remap_ctrl](pro_boot_remap_ctrl) module"]
pub type PRO_BOOT_REMAP_CTRL = crate::Reg<u32, _PRO_BOOT_REMAP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_BOOT_REMAP_CTRL;
#[doc = "`read()` method returns [pro_boot_remap_ctrl::R](pro_boot_remap_ctrl::R) reader structure"]
impl crate::Readable for PRO_BOOT_REMAP_CTRL {}
#[doc = "`write(|w| ..)` method takes [pro_boot_remap_ctrl::W](pro_boot_remap_ctrl::W) writer structure"]
impl crate::Writable for PRO_BOOT_REMAP_CTRL {}
#[doc = "DPORT_PRO_BOOT_REMAP_CTRL_REG"]
pub mod pro_boot_remap_ctrl;
#[doc = "DPORT_APP_BOOT_REMAP_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_boot_remap_ctrl](app_boot_remap_ctrl) module"]
pub type APP_BOOT_REMAP_CTRL = crate::Reg<u32, _APP_BOOT_REMAP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_BOOT_REMAP_CTRL;
#[doc = "`read()` method returns [app_boot_remap_ctrl::R](app_boot_remap_ctrl::R) reader structure"]
impl crate::Readable for APP_BOOT_REMAP_CTRL {}
#[doc = "`write(|w| ..)` method takes [app_boot_remap_ctrl::W](app_boot_remap_ctrl::W) writer structure"]
impl crate::Writable for APP_BOOT_REMAP_CTRL {}
#[doc = "DPORT_APP_BOOT_REMAP_CTRL_REG"]
pub mod app_boot_remap_ctrl;
#[doc = "DPORT_ACCESS_CHECK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [access_check](access_check) module"]
pub type ACCESS_CHECK = crate::Reg<u32, _ACCESS_CHECK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCESS_CHECK;
#[doc = "`read()` method returns [access_check::R](access_check::R) reader structure"]
impl crate::Readable for ACCESS_CHECK {}
#[doc = "`write(|w| ..)` method takes [access_check::W](access_check::W) writer structure"]
impl crate::Writable for ACCESS_CHECK {}
#[doc = "DPORT_ACCESS_CHECK_REG"]
pub mod access_check;
#[doc = "DPORT_PRO_DPORT_APB_MASK0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dport_apb_mask0](pro_dport_apb_mask0) module"]
pub type PRO_DPORT_APB_MASK0 = crate::Reg<u32, _PRO_DPORT_APB_MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DPORT_APB_MASK0;
#[doc = "`read()` method returns [pro_dport_apb_mask0::R](pro_dport_apb_mask0::R) reader structure"]
impl crate::Readable for PRO_DPORT_APB_MASK0 {}
#[doc = "`write(|w| ..)` method takes [pro_dport_apb_mask0::W](pro_dport_apb_mask0::W) writer structure"]
impl crate::Writable for PRO_DPORT_APB_MASK0 {}
#[doc = "DPORT_PRO_DPORT_APB_MASK0_REG"]
pub mod pro_dport_apb_mask0;
#[doc = "DPORT_PRO_DPORT_APB_MASK1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dport_apb_mask1](pro_dport_apb_mask1) module"]
pub type PRO_DPORT_APB_MASK1 = crate::Reg<u32, _PRO_DPORT_APB_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DPORT_APB_MASK1;
#[doc = "`read()` method returns [pro_dport_apb_mask1::R](pro_dport_apb_mask1::R) reader structure"]
impl crate::Readable for PRO_DPORT_APB_MASK1 {}
#[doc = "`write(|w| ..)` method takes [pro_dport_apb_mask1::W](pro_dport_apb_mask1::W) writer structure"]
impl crate::Writable for PRO_DPORT_APB_MASK1 {}
#[doc = "DPORT_PRO_DPORT_APB_MASK1_REG"]
pub mod pro_dport_apb_mask1;
#[doc = "DPORT_APP_DPORT_APB_MASK0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dport_apb_mask0](app_dport_apb_mask0) module"]
pub type APP_DPORT_APB_MASK0 = crate::Reg<u32, _APP_DPORT_APB_MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DPORT_APB_MASK0;
#[doc = "`read()` method returns [app_dport_apb_mask0::R](app_dport_apb_mask0::R) reader structure"]
impl crate::Readable for APP_DPORT_APB_MASK0 {}
#[doc = "`write(|w| ..)` method takes [app_dport_apb_mask0::W](app_dport_apb_mask0::W) writer structure"]
impl crate::Writable for APP_DPORT_APB_MASK0 {}
#[doc = "DPORT_APP_DPORT_APB_MASK0_REG"]
pub mod app_dport_apb_mask0;
#[doc = "DPORT_APP_DPORT_APB_MASK1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dport_apb_mask1](app_dport_apb_mask1) module"]
pub type APP_DPORT_APB_MASK1 = crate::Reg<u32, _APP_DPORT_APB_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DPORT_APB_MASK1;
#[doc = "`read()` method returns [app_dport_apb_mask1::R](app_dport_apb_mask1::R) reader structure"]
impl crate::Readable for APP_DPORT_APB_MASK1 {}
#[doc = "`write(|w| ..)` method takes [app_dport_apb_mask1::W](app_dport_apb_mask1::W) writer structure"]
impl crate::Writable for APP_DPORT_APB_MASK1 {}
#[doc = "DPORT_APP_DPORT_APB_MASK1_REG"]
pub mod app_dport_apb_mask1;
#[doc = "DPORT_PERI_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [peri_clk_en](peri_clk_en) module"]
pub type PERI_CLK_EN = crate::Reg<u32, _PERI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_CLK_EN;
#[doc = "`read()` method returns [peri_clk_en::R](peri_clk_en::R) reader structure"]
impl crate::Readable for PERI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [peri_clk_en::W](peri_clk_en::W) writer structure"]
impl crate::Writable for PERI_CLK_EN {}
#[doc = "DPORT_PERI_CLK_EN_REG"]
pub mod peri_clk_en;
#[doc = "DPORT_PERI_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [peri_rst_en](peri_rst_en) module"]
pub type PERI_RST_EN = crate::Reg<u32, _PERI_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_RST_EN;
#[doc = "`read()` method returns [peri_rst_en::R](peri_rst_en::R) reader structure"]
impl crate::Readable for PERI_RST_EN {}
#[doc = "`write(|w| ..)` method takes [peri_rst_en::W](peri_rst_en::W) writer structure"]
impl crate::Writable for PERI_RST_EN {}
#[doc = "DPORT_PERI_RST_EN_REG"]
pub mod peri_rst_en;
#[doc = "DPORT_WIFI_BB_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wifi_bb_cfg](wifi_bb_cfg) module"]
pub type WIFI_BB_CFG = crate::Reg<u32, _WIFI_BB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_BB_CFG;
#[doc = "`read()` method returns [wifi_bb_cfg::R](wifi_bb_cfg::R) reader structure"]
impl crate::Readable for WIFI_BB_CFG {}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg::W](wifi_bb_cfg::W) writer structure"]
impl crate::Writable for WIFI_BB_CFG {}
#[doc = "DPORT_WIFI_BB_CFG_REG"]
pub mod wifi_bb_cfg;
#[doc = "DPORT_WIFI_BB_CFG_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wifi_bb_cfg_2](wifi_bb_cfg_2) module"]
pub type WIFI_BB_CFG_2 = crate::Reg<u32, _WIFI_BB_CFG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_BB_CFG_2;
#[doc = "`read()` method returns [wifi_bb_cfg_2::R](wifi_bb_cfg_2::R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_2 {}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg_2::W](wifi_bb_cfg_2::W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_2 {}
#[doc = "DPORT_WIFI_BB_CFG_2_REG"]
pub mod wifi_bb_cfg_2;
#[doc = "DPORT_APPCPU_CTRL_A_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [appcpu_ctrl_a](appcpu_ctrl_a) module"]
pub type APPCPU_CTRL_A = crate::Reg<u32, _APPCPU_CTRL_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPCPU_CTRL_A;
#[doc = "`read()` method returns [appcpu_ctrl_a::R](appcpu_ctrl_a::R) reader structure"]
impl crate::Readable for APPCPU_CTRL_A {}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_a::W](appcpu_ctrl_a::W) writer structure"]
impl crate::Writable for APPCPU_CTRL_A {}
#[doc = "DPORT_APPCPU_CTRL_A_REG"]
pub mod appcpu_ctrl_a;
#[doc = "DPORT_APPCPU_CTRL_B_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [appcpu_ctrl_b](appcpu_ctrl_b) module"]
pub type APPCPU_CTRL_B = crate::Reg<u32, _APPCPU_CTRL_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPCPU_CTRL_B;
#[doc = "`read()` method returns [appcpu_ctrl_b::R](appcpu_ctrl_b::R) reader structure"]
impl crate::Readable for APPCPU_CTRL_B {}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_b::W](appcpu_ctrl_b::W) writer structure"]
impl crate::Writable for APPCPU_CTRL_B {}
#[doc = "DPORT_APPCPU_CTRL_B_REG"]
pub mod appcpu_ctrl_b;
#[doc = "DPORT_APPCPU_CTRL_C_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [appcpu_ctrl_c](appcpu_ctrl_c) module"]
pub type APPCPU_CTRL_C = crate::Reg<u32, _APPCPU_CTRL_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPCPU_CTRL_C;
#[doc = "`read()` method returns [appcpu_ctrl_c::R](appcpu_ctrl_c::R) reader structure"]
impl crate::Readable for APPCPU_CTRL_C {}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_c::W](appcpu_ctrl_c::W) writer structure"]
impl crate::Writable for APPCPU_CTRL_C {}
#[doc = "DPORT_APPCPU_CTRL_C_REG"]
pub mod appcpu_ctrl_c;
#[doc = "DPORT_APPCPU_CTRL_D_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [appcpu_ctrl_d](appcpu_ctrl_d) module"]
pub type APPCPU_CTRL_D = crate::Reg<u32, _APPCPU_CTRL_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPCPU_CTRL_D;
#[doc = "`read()` method returns [appcpu_ctrl_d::R](appcpu_ctrl_d::R) reader structure"]
impl crate::Readable for APPCPU_CTRL_D {}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_d::W](appcpu_ctrl_d::W) writer structure"]
impl crate::Writable for APPCPU_CTRL_D {}
#[doc = "DPORT_APPCPU_CTRL_D_REG"]
pub mod appcpu_ctrl_d;
#[doc = "DPORT_CPU_PER_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_per_conf](cpu_per_conf) module"]
pub type CPU_PER_CONF = crate::Reg<u32, _CPU_PER_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_PER_CONF;
#[doc = "`read()` method returns [cpu_per_conf::R](cpu_per_conf::R) reader structure"]
impl crate::Readable for CPU_PER_CONF {}
#[doc = "`write(|w| ..)` method takes [cpu_per_conf::W](cpu_per_conf::W) writer structure"]
impl crate::Writable for CPU_PER_CONF {}
#[doc = "DPORT_CPU_PER_CONF_REG"]
pub mod cpu_per_conf;
#[doc = "DPORT_PRO_CACHE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_ctrl](pro_cache_ctrl) module"]
pub type PRO_CACHE_CTRL = crate::Reg<u32, _PRO_CACHE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_CTRL;
#[doc = "`read()` method returns [pro_cache_ctrl::R](pro_cache_ctrl::R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL {}
#[doc = "`write(|w| ..)` method takes [pro_cache_ctrl::W](pro_cache_ctrl::W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL {}
#[doc = "DPORT_PRO_CACHE_CTRL_REG"]
pub mod pro_cache_ctrl;
#[doc = "DPORT_PRO_CACHE_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_ctrl1](pro_cache_ctrl1) module"]
pub type PRO_CACHE_CTRL1 = crate::Reg<u32, _PRO_CACHE_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_CTRL1;
#[doc = "`read()` method returns [pro_cache_ctrl1::R](pro_cache_ctrl1::R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [pro_cache_ctrl1::W](pro_cache_ctrl1::W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL1 {}
#[doc = "DPORT_PRO_CACHE_CTRL1_REG"]
pub mod pro_cache_ctrl1;
#[doc = "DPORT_PRO_CACHE_LOCK_0_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_lock_0_addr](pro_cache_lock_0_addr) module"]
pub type PRO_CACHE_LOCK_0_ADDR = crate::Reg<u32, _PRO_CACHE_LOCK_0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_LOCK_0_ADDR;
#[doc = "`read()` method returns [pro_cache_lock_0_addr::R](pro_cache_lock_0_addr::R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_0_ADDR {}
#[doc = "`write(|w| ..)` method takes [pro_cache_lock_0_addr::W](pro_cache_lock_0_addr::W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_0_ADDR {}
#[doc = "DPORT_PRO_CACHE_LOCK_0_ADDR_REG"]
pub mod pro_cache_lock_0_addr;
#[doc = "DPORT_PRO_CACHE_LOCK_1_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_lock_1_addr](pro_cache_lock_1_addr) module"]
pub type PRO_CACHE_LOCK_1_ADDR = crate::Reg<u32, _PRO_CACHE_LOCK_1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_LOCK_1_ADDR;
#[doc = "`read()` method returns [pro_cache_lock_1_addr::R](pro_cache_lock_1_addr::R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_1_ADDR {}
#[doc = "`write(|w| ..)` method takes [pro_cache_lock_1_addr::W](pro_cache_lock_1_addr::W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_1_ADDR {}
#[doc = "DPORT_PRO_CACHE_LOCK_1_ADDR_REG"]
pub mod pro_cache_lock_1_addr;
#[doc = "DPORT_PRO_CACHE_LOCK_2_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_lock_2_addr](pro_cache_lock_2_addr) module"]
pub type PRO_CACHE_LOCK_2_ADDR = crate::Reg<u32, _PRO_CACHE_LOCK_2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_LOCK_2_ADDR;
#[doc = "`read()` method returns [pro_cache_lock_2_addr::R](pro_cache_lock_2_addr::R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_2_ADDR {}
#[doc = "`write(|w| ..)` method takes [pro_cache_lock_2_addr::W](pro_cache_lock_2_addr::W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_2_ADDR {}
#[doc = "DPORT_PRO_CACHE_LOCK_2_ADDR_REG"]
pub mod pro_cache_lock_2_addr;
#[doc = "DPORT_PRO_CACHE_LOCK_3_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_lock_3_addr](pro_cache_lock_3_addr) module"]
pub type PRO_CACHE_LOCK_3_ADDR = crate::Reg<u32, _PRO_CACHE_LOCK_3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_LOCK_3_ADDR;
#[doc = "`read()` method returns [pro_cache_lock_3_addr::R](pro_cache_lock_3_addr::R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_3_ADDR {}
#[doc = "`write(|w| ..)` method takes [pro_cache_lock_3_addr::W](pro_cache_lock_3_addr::W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_3_ADDR {}
#[doc = "DPORT_PRO_CACHE_LOCK_3_ADDR_REG"]
pub mod pro_cache_lock_3_addr;
#[doc = "DPORT_APP_CACHE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_ctrl](app_cache_ctrl) module"]
pub type APP_CACHE_CTRL = crate::Reg<u32, _APP_CACHE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_CTRL;
#[doc = "`read()` method returns [app_cache_ctrl::R](app_cache_ctrl::R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL {}
#[doc = "`write(|w| ..)` method takes [app_cache_ctrl::W](app_cache_ctrl::W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL {}
#[doc = "DPORT_APP_CACHE_CTRL_REG"]
pub mod app_cache_ctrl;
#[doc = "DPORT_APP_CACHE_CTRL1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_ctrl1](app_cache_ctrl1) module"]
pub type APP_CACHE_CTRL1 = crate::Reg<u32, _APP_CACHE_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_CTRL1;
#[doc = "`read()` method returns [app_cache_ctrl1::R](app_cache_ctrl1::R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [app_cache_ctrl1::W](app_cache_ctrl1::W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL1 {}
#[doc = "DPORT_APP_CACHE_CTRL1_REG"]
pub mod app_cache_ctrl1;
#[doc = "DPORT_APP_CACHE_LOCK_0_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_lock_0_addr](app_cache_lock_0_addr) module"]
pub type APP_CACHE_LOCK_0_ADDR = crate::Reg<u32, _APP_CACHE_LOCK_0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_LOCK_0_ADDR;
#[doc = "`read()` method returns [app_cache_lock_0_addr::R](app_cache_lock_0_addr::R) reader structure"]
impl crate::Readable for APP_CACHE_LOCK_0_ADDR {}
#[doc = "`write(|w| ..)` method takes [app_cache_lock_0_addr::W](app_cache_lock_0_addr::W) writer structure"]
impl crate::Writable for APP_CACHE_LOCK_0_ADDR {}
#[doc = "DPORT_APP_CACHE_LOCK_0_ADDR_REG"]
pub mod app_cache_lock_0_addr;
#[doc = "DPORT_APP_CACHE_LOCK_1_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_lock_1_addr](app_cache_lock_1_addr) module"]
pub type APP_CACHE_LOCK_1_ADDR = crate::Reg<u32, _APP_CACHE_LOCK_1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_LOCK_1_ADDR;
#[doc = "`read()` method returns [app_cache_lock_1_addr::R](app_cache_lock_1_addr::R) reader structure"]
impl crate::Readable for APP_CACHE_LOCK_1_ADDR {}
#[doc = "`write(|w| ..)` method takes [app_cache_lock_1_addr::W](app_cache_lock_1_addr::W) writer structure"]
impl crate::Writable for APP_CACHE_LOCK_1_ADDR {}
#[doc = "DPORT_APP_CACHE_LOCK_1_ADDR_REG"]
pub mod app_cache_lock_1_addr;
#[doc = "DPORT_APP_CACHE_LOCK_2_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_lock_2_addr](app_cache_lock_2_addr) module"]
pub type APP_CACHE_LOCK_2_ADDR = crate::Reg<u32, _APP_CACHE_LOCK_2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_LOCK_2_ADDR;
#[doc = "`read()` method returns [app_cache_lock_2_addr::R](app_cache_lock_2_addr::R) reader structure"]
impl crate::Readable for APP_CACHE_LOCK_2_ADDR {}
#[doc = "`write(|w| ..)` method takes [app_cache_lock_2_addr::W](app_cache_lock_2_addr::W) writer structure"]
impl crate::Writable for APP_CACHE_LOCK_2_ADDR {}
#[doc = "DPORT_APP_CACHE_LOCK_2_ADDR_REG"]
pub mod app_cache_lock_2_addr;
#[doc = "DPORT_APP_CACHE_LOCK_3_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_lock_3_addr](app_cache_lock_3_addr) module"]
pub type APP_CACHE_LOCK_3_ADDR = crate::Reg<u32, _APP_CACHE_LOCK_3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_LOCK_3_ADDR;
#[doc = "`read()` method returns [app_cache_lock_3_addr::R](app_cache_lock_3_addr::R) reader structure"]
impl crate::Readable for APP_CACHE_LOCK_3_ADDR {}
#[doc = "`write(|w| ..)` method takes [app_cache_lock_3_addr::W](app_cache_lock_3_addr::W) writer structure"]
impl crate::Writable for APP_CACHE_LOCK_3_ADDR {}
#[doc = "DPORT_APP_CACHE_LOCK_3_ADDR_REG"]
pub mod app_cache_lock_3_addr;
#[doc = "DPORT_TRACEMEM_MUX_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tracemem_mux_mode](tracemem_mux_mode) module"]
pub type TRACEMEM_MUX_MODE = crate::Reg<u32, _TRACEMEM_MUX_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACEMEM_MUX_MODE;
#[doc = "`read()` method returns [tracemem_mux_mode::R](tracemem_mux_mode::R) reader structure"]
impl crate::Readable for TRACEMEM_MUX_MODE {}
#[doc = "`write(|w| ..)` method takes [tracemem_mux_mode::W](tracemem_mux_mode::W) writer structure"]
impl crate::Writable for TRACEMEM_MUX_MODE {}
#[doc = "DPORT_TRACEMEM_MUX_MODE_REG"]
pub mod tracemem_mux_mode;
#[doc = "DPORT_PRO_TRACEMEM_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tracemem_ena](pro_tracemem_ena) module"]
pub type PRO_TRACEMEM_ENA = crate::Reg<u32, _PRO_TRACEMEM_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TRACEMEM_ENA;
#[doc = "`read()` method returns [pro_tracemem_ena::R](pro_tracemem_ena::R) reader structure"]
impl crate::Readable for PRO_TRACEMEM_ENA {}
#[doc = "`write(|w| ..)` method takes [pro_tracemem_ena::W](pro_tracemem_ena::W) writer structure"]
impl crate::Writable for PRO_TRACEMEM_ENA {}
#[doc = "DPORT_PRO_TRACEMEM_ENA_REG"]
pub mod pro_tracemem_ena;
#[doc = "DPORT_APP_TRACEMEM_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tracemem_ena](app_tracemem_ena) module"]
pub type APP_TRACEMEM_ENA = crate::Reg<u32, _APP_TRACEMEM_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TRACEMEM_ENA;
#[doc = "`read()` method returns [app_tracemem_ena::R](app_tracemem_ena::R) reader structure"]
impl crate::Readable for APP_TRACEMEM_ENA {}
#[doc = "`write(|w| ..)` method takes [app_tracemem_ena::W](app_tracemem_ena::W) writer structure"]
impl crate::Writable for APP_TRACEMEM_ENA {}
#[doc = "DPORT_APP_TRACEMEM_ENA_REG"]
pub mod app_tracemem_ena;
#[doc = "DPORT_CACHE_MUX_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cache_mux_mode](cache_mux_mode) module"]
pub type CACHE_MUX_MODE = crate::Reg<u32, _CACHE_MUX_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_MUX_MODE;
#[doc = "`read()` method returns [cache_mux_mode::R](cache_mux_mode::R) reader structure"]
impl crate::Readable for CACHE_MUX_MODE {}
#[doc = "`write(|w| ..)` method takes [cache_mux_mode::W](cache_mux_mode::W) writer structure"]
impl crate::Writable for CACHE_MUX_MODE {}
#[doc = "DPORT_CACHE_MUX_MODE_REG"]
pub mod cache_mux_mode;
#[doc = "DPORT_IMMU_PAGE_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_page_mode](immu_page_mode) module"]
pub type IMMU_PAGE_MODE = crate::Reg<u32, _IMMU_PAGE_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_PAGE_MODE;
#[doc = "`read()` method returns [immu_page_mode::R](immu_page_mode::R) reader structure"]
impl crate::Readable for IMMU_PAGE_MODE {}
#[doc = "`write(|w| ..)` method takes [immu_page_mode::W](immu_page_mode::W) writer structure"]
impl crate::Writable for IMMU_PAGE_MODE {}
#[doc = "DPORT_IMMU_PAGE_MODE_REG"]
pub mod immu_page_mode;
#[doc = "DPORT_DMMU_PAGE_MODE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_page_mode](dmmu_page_mode) module"]
pub type DMMU_PAGE_MODE = crate::Reg<u32, _DMMU_PAGE_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_PAGE_MODE;
#[doc = "`read()` method returns [dmmu_page_mode::R](dmmu_page_mode::R) reader structure"]
impl crate::Readable for DMMU_PAGE_MODE {}
#[doc = "`write(|w| ..)` method takes [dmmu_page_mode::W](dmmu_page_mode::W) writer structure"]
impl crate::Writable for DMMU_PAGE_MODE {}
#[doc = "DPORT_DMMU_PAGE_MODE_REG"]
pub mod dmmu_page_mode;
#[doc = "DPORT_ROM_MPU_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_mpu_ena](rom_mpu_ena) module"]
pub type ROM_MPU_ENA = crate::Reg<u32, _ROM_MPU_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_MPU_ENA;
#[doc = "`read()` method returns [rom_mpu_ena::R](rom_mpu_ena::R) reader structure"]
impl crate::Readable for ROM_MPU_ENA {}
#[doc = "`write(|w| ..)` method takes [rom_mpu_ena::W](rom_mpu_ena::W) writer structure"]
impl crate::Writable for ROM_MPU_ENA {}
#[doc = "DPORT_ROM_MPU_ENA_REG"]
pub mod rom_mpu_ena;
#[doc = "DPORT_MEM_PD_MASK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mem_pd_mask](mem_pd_mask) module"]
pub type MEM_PD_MASK = crate::Reg<u32, _MEM_PD_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_PD_MASK;
#[doc = "`read()` method returns [mem_pd_mask::R](mem_pd_mask::R) reader structure"]
impl crate::Readable for MEM_PD_MASK {}
#[doc = "`write(|w| ..)` method takes [mem_pd_mask::W](mem_pd_mask::W) writer structure"]
impl crate::Writable for MEM_PD_MASK {}
#[doc = "DPORT_MEM_PD_MASK_REG"]
pub mod mem_pd_mask;
#[doc = "DPORT_ROM_PD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_pd_ctrl](rom_pd_ctrl) module"]
pub type ROM_PD_CTRL = crate::Reg<u32, _ROM_PD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PD_CTRL;
#[doc = "`read()` method returns [rom_pd_ctrl::R](rom_pd_ctrl::R) reader structure"]
impl crate::Readable for ROM_PD_CTRL {}
#[doc = "`write(|w| ..)` method takes [rom_pd_ctrl::W](rom_pd_ctrl::W) writer structure"]
impl crate::Writable for ROM_PD_CTRL {}
#[doc = "DPORT_ROM_PD_CTRL_REG"]
pub mod rom_pd_ctrl;
#[doc = "DPORT_ROM_FO_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_fo_ctrl](rom_fo_ctrl) module"]
pub type ROM_FO_CTRL = crate::Reg<u32, _ROM_FO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_FO_CTRL;
#[doc = "`read()` method returns [rom_fo_ctrl::R](rom_fo_ctrl::R) reader structure"]
impl crate::Readable for ROM_FO_CTRL {}
#[doc = "`write(|w| ..)` method takes [rom_fo_ctrl::W](rom_fo_ctrl::W) writer structure"]
impl crate::Writable for ROM_FO_CTRL {}
#[doc = "DPORT_ROM_FO_CTRL_REG"]
pub mod rom_fo_ctrl;
#[doc = "DPORT_SRAM_PD_CTRL_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_pd_ctrl_0](sram_pd_ctrl_0) module"]
pub type SRAM_PD_CTRL_0 = crate::Reg<u32, _SRAM_PD_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_PD_CTRL_0;
#[doc = "`read()` method returns [sram_pd_ctrl_0::R](sram_pd_ctrl_0::R) reader structure"]
impl crate::Readable for SRAM_PD_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [sram_pd_ctrl_0::W](sram_pd_ctrl_0::W) writer structure"]
impl crate::Writable for SRAM_PD_CTRL_0 {}
#[doc = "DPORT_SRAM_PD_CTRL_0_REG"]
pub mod sram_pd_ctrl_0;
#[doc = "DPORT_SRAM_PD_CTRL_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_pd_ctrl_1](sram_pd_ctrl_1) module"]
pub type SRAM_PD_CTRL_1 = crate::Reg<u32, _SRAM_PD_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_PD_CTRL_1;
#[doc = "`read()` method returns [sram_pd_ctrl_1::R](sram_pd_ctrl_1::R) reader structure"]
impl crate::Readable for SRAM_PD_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [sram_pd_ctrl_1::W](sram_pd_ctrl_1::W) writer structure"]
impl crate::Writable for SRAM_PD_CTRL_1 {}
#[doc = "DPORT_SRAM_PD_CTRL_1_REG"]
pub mod sram_pd_ctrl_1;
#[doc = "DPORT_SRAM_FO_CTRL_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_fo_ctrl_0](sram_fo_ctrl_0) module"]
pub type SRAM_FO_CTRL_0 = crate::Reg<u32, _SRAM_FO_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_FO_CTRL_0;
#[doc = "`read()` method returns [sram_fo_ctrl_0::R](sram_fo_ctrl_0::R) reader structure"]
impl crate::Readable for SRAM_FO_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [sram_fo_ctrl_0::W](sram_fo_ctrl_0::W) writer structure"]
impl crate::Writable for SRAM_FO_CTRL_0 {}
#[doc = "DPORT_SRAM_FO_CTRL_0_REG"]
pub mod sram_fo_ctrl_0;
#[doc = "DPORT_SRAM_FO_CTRL_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_fo_ctrl_1](sram_fo_ctrl_1) module"]
pub type SRAM_FO_CTRL_1 = crate::Reg<u32, _SRAM_FO_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_FO_CTRL_1;
#[doc = "`read()` method returns [sram_fo_ctrl_1::R](sram_fo_ctrl_1::R) reader structure"]
impl crate::Readable for SRAM_FO_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [sram_fo_ctrl_1::W](sram_fo_ctrl_1::W) writer structure"]
impl crate::Writable for SRAM_FO_CTRL_1 {}
#[doc = "DPORT_SRAM_FO_CTRL_1_REG"]
pub mod sram_fo_ctrl_1;
#[doc = "DPORT_IRAM_DRAM_AHB_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iram_dram_ahb_sel](iram_dram_ahb_sel) module"]
pub type IRAM_DRAM_AHB_SEL = crate::Reg<u32, _IRAM_DRAM_AHB_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRAM_DRAM_AHB_SEL;
#[doc = "`read()` method returns [iram_dram_ahb_sel::R](iram_dram_ahb_sel::R) reader structure"]
impl crate::Readable for IRAM_DRAM_AHB_SEL {}
#[doc = "`write(|w| ..)` method takes [iram_dram_ahb_sel::W](iram_dram_ahb_sel::W) writer structure"]
impl crate::Writable for IRAM_DRAM_AHB_SEL {}
#[doc = "DPORT_IRAM_DRAM_AHB_SEL_REG"]
pub mod iram_dram_ahb_sel;
#[doc = "DPORT_TAG_FO_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tag_fo_ctrl](tag_fo_ctrl) module"]
pub type TAG_FO_CTRL = crate::Reg<u32, _TAG_FO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_FO_CTRL;
#[doc = "`read()` method returns [tag_fo_ctrl::R](tag_fo_ctrl::R) reader structure"]
impl crate::Readable for TAG_FO_CTRL {}
#[doc = "`write(|w| ..)` method takes [tag_fo_ctrl::W](tag_fo_ctrl::W) writer structure"]
impl crate::Writable for TAG_FO_CTRL {}
#[doc = "DPORT_TAG_FO_CTRL_REG"]
pub mod tag_fo_ctrl;
#[doc = "DPORT_AHB_LITE_MASK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_lite_mask](ahb_lite_mask) module"]
pub type AHB_LITE_MASK = crate::Reg<u32, _AHB_LITE_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_LITE_MASK;
#[doc = "`read()` method returns [ahb_lite_mask::R](ahb_lite_mask::R) reader structure"]
impl crate::Readable for AHB_LITE_MASK {}
#[doc = "`write(|w| ..)` method takes [ahb_lite_mask::W](ahb_lite_mask::W) writer structure"]
impl crate::Writable for AHB_LITE_MASK {}
#[doc = "DPORT_AHB_LITE_MASK_REG"]
pub mod ahb_lite_mask;
#[doc = "DPORT_AHB_MPU_TABLE_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_mpu_table_0](ahb_mpu_table_0) module"]
pub type AHB_MPU_TABLE_0 = crate::Reg<u32, _AHB_MPU_TABLE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_MPU_TABLE_0;
#[doc = "`read()` method returns [ahb_mpu_table_0::R](ahb_mpu_table_0::R) reader structure"]
impl crate::Readable for AHB_MPU_TABLE_0 {}
#[doc = "`write(|w| ..)` method takes [ahb_mpu_table_0::W](ahb_mpu_table_0::W) writer structure"]
impl crate::Writable for AHB_MPU_TABLE_0 {}
#[doc = "DPORT_AHB_MPU_TABLE_0_REG"]
pub mod ahb_mpu_table_0;
#[doc = "DPORT_AHB_MPU_TABLE_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_mpu_table_1](ahb_mpu_table_1) module"]
pub type AHB_MPU_TABLE_1 = crate::Reg<u32, _AHB_MPU_TABLE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_MPU_TABLE_1;
#[doc = "`read()` method returns [ahb_mpu_table_1::R](ahb_mpu_table_1::R) reader structure"]
impl crate::Readable for AHB_MPU_TABLE_1 {}
#[doc = "`write(|w| ..)` method takes [ahb_mpu_table_1::W](ahb_mpu_table_1::W) writer structure"]
impl crate::Writable for AHB_MPU_TABLE_1 {}
#[doc = "DPORT_AHB_MPU_TABLE_1_REG"]
pub mod ahb_mpu_table_1;
#[doc = "DPORT_HOST_INF_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_inf_sel](host_inf_sel) module"]
pub type HOST_INF_SEL = crate::Reg<u32, _HOST_INF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_INF_SEL;
#[doc = "`read()` method returns [host_inf_sel::R](host_inf_sel::R) reader structure"]
impl crate::Readable for HOST_INF_SEL {}
#[doc = "`write(|w| ..)` method takes [host_inf_sel::W](host_inf_sel::W) writer structure"]
impl crate::Writable for HOST_INF_SEL {}
#[doc = "DPORT_HOST_INF_SEL_REG"]
pub mod host_inf_sel;
#[doc = "DPORT_PERIP_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [perip_clk_en](perip_clk_en) module"]
pub type PERIP_CLK_EN = crate::Reg<u32, _PERIP_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIP_CLK_EN;
#[doc = "`read()` method returns [perip_clk_en::R](perip_clk_en::R) reader structure"]
impl crate::Readable for PERIP_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [perip_clk_en::W](perip_clk_en::W) writer structure"]
impl crate::Writable for PERIP_CLK_EN {}
#[doc = "DPORT_PERIP_CLK_EN_REG"]
pub mod perip_clk_en;
#[doc = "DPORT_PERIP_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [perip_rst_en](perip_rst_en) module"]
pub type PERIP_RST_EN = crate::Reg<u32, _PERIP_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIP_RST_EN;
#[doc = "`read()` method returns [perip_rst_en::R](perip_rst_en::R) reader structure"]
impl crate::Readable for PERIP_RST_EN {}
#[doc = "`write(|w| ..)` method takes [perip_rst_en::W](perip_rst_en::W) writer structure"]
impl crate::Writable for PERIP_RST_EN {}
#[doc = "DPORT_PERIP_RST_EN_REG"]
pub mod perip_rst_en;
#[doc = "DPORT_WIFI_CLK_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wifi_clk_en](wifi_clk_en) module"]
pub type WIFI_CLK_EN = crate::Reg<u32, _WIFI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_CLK_EN;
#[doc = "`read()` method returns [wifi_clk_en::R](wifi_clk_en::R) reader structure"]
impl crate::Readable for WIFI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [wifi_clk_en::W](wifi_clk_en::W) writer structure"]
impl crate::Writable for WIFI_CLK_EN {}
#[doc = "DPORT_WIFI_CLK_EN_REG"]
pub mod wifi_clk_en;
#[doc = "DPORT_CORE_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [core_rst_en](core_rst_en) module"]
pub type CORE_RST_EN = crate::Reg<u32, _CORE_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORE_RST_EN;
#[doc = "`read()` method returns [core_rst_en::R](core_rst_en::R) reader structure"]
impl crate::Readable for CORE_RST_EN {}
#[doc = "`write(|w| ..)` method takes [core_rst_en::W](core_rst_en::W) writer structure"]
impl crate::Writable for CORE_RST_EN {}
#[doc = "DPORT_CORE_RST_EN_REG"]
pub mod core_rst_en;
#[doc = "DPORT_BT_LPCK_DIV_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bt_lpck_div_int](bt_lpck_div_int) module"]
pub type BT_LPCK_DIV_INT = crate::Reg<u32, _BT_LPCK_DIV_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_LPCK_DIV_INT;
#[doc = "`read()` method returns [bt_lpck_div_int::R](bt_lpck_div_int::R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_INT {}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_int::W](bt_lpck_div_int::W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_INT {}
#[doc = "DPORT_BT_LPCK_DIV_INT_REG"]
pub mod bt_lpck_div_int;
#[doc = "DPORT_BT_LPCK_DIV_FRAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bt_lpck_div_frac](bt_lpck_div_frac) module"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<u32, _BT_LPCK_DIV_FRAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_LPCK_DIV_FRAC;
#[doc = "`read()` method returns [bt_lpck_div_frac::R](bt_lpck_div_frac::R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_FRAC {}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_frac::W](bt_lpck_div_frac::W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_FRAC {}
#[doc = "DPORT_BT_LPCK_DIV_FRAC_REG"]
pub mod bt_lpck_div_frac;
#[doc = "DPORT_CPU_INTR_FROM_CPU_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_intr_from_cpu_0](cpu_intr_from_cpu_0) module"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<u32, _CPU_INTR_FROM_CPU_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_INTR_FROM_CPU_0;
#[doc = "`read()` method returns [cpu_intr_from_cpu_0::R](cpu_intr_from_cpu_0::R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_0 {}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_0::W](cpu_intr_from_cpu_0::W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_0 {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_0_REG"]
pub mod cpu_intr_from_cpu_0;
#[doc = "DPORT_CPU_INTR_FROM_CPU_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_intr_from_cpu_1](cpu_intr_from_cpu_1) module"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<u32, _CPU_INTR_FROM_CPU_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_INTR_FROM_CPU_1;
#[doc = "`read()` method returns [cpu_intr_from_cpu_1::R](cpu_intr_from_cpu_1::R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_1 {}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_1::W](cpu_intr_from_cpu_1::W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_1 {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_1_REG"]
pub mod cpu_intr_from_cpu_1;
#[doc = "DPORT_CPU_INTR_FROM_CPU_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_intr_from_cpu_2](cpu_intr_from_cpu_2) module"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<u32, _CPU_INTR_FROM_CPU_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_INTR_FROM_CPU_2;
#[doc = "`read()` method returns [cpu_intr_from_cpu_2::R](cpu_intr_from_cpu_2::R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_2 {}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_2::W](cpu_intr_from_cpu_2::W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_2 {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_2_REG"]
pub mod cpu_intr_from_cpu_2;
#[doc = "DPORT_CPU_INTR_FROM_CPU_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu_intr_from_cpu_3](cpu_intr_from_cpu_3) module"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<u32, _CPU_INTR_FROM_CPU_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_INTR_FROM_CPU_3;
#[doc = "`read()` method returns [cpu_intr_from_cpu_3::R](cpu_intr_from_cpu_3::R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_3 {}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_3::W](cpu_intr_from_cpu_3::W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_3 {}
#[doc = "DPORT_CPU_INTR_FROM_CPU_3_REG"]
pub mod cpu_intr_from_cpu_3;
#[doc = "DPORT_PRO_INTR_STATUS_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_intr_status_0](pro_intr_status_0) module"]
pub type PRO_INTR_STATUS_0 = crate::Reg<u32, _PRO_INTR_STATUS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_INTR_STATUS_0;
#[doc = "`read()` method returns [pro_intr_status_0::R](pro_intr_status_0::R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_0 {}
#[doc = "`write(|w| ..)` method takes [pro_intr_status_0::W](pro_intr_status_0::W) writer structure"]
impl crate::Writable for PRO_INTR_STATUS_0 {}
#[doc = "DPORT_PRO_INTR_STATUS_0_REG"]
pub mod pro_intr_status_0;
#[doc = "DPORT_PRO_INTR_STATUS_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_intr_status_1](pro_intr_status_1) module"]
pub type PRO_INTR_STATUS_1 = crate::Reg<u32, _PRO_INTR_STATUS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_INTR_STATUS_1;
#[doc = "`read()` method returns [pro_intr_status_1::R](pro_intr_status_1::R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_1 {}
#[doc = "`write(|w| ..)` method takes [pro_intr_status_1::W](pro_intr_status_1::W) writer structure"]
impl crate::Writable for PRO_INTR_STATUS_1 {}
#[doc = "DPORT_PRO_INTR_STATUS_1_REG"]
pub mod pro_intr_status_1;
#[doc = "DPORT_PRO_INTR_STATUS_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_intr_status_2](pro_intr_status_2) module"]
pub type PRO_INTR_STATUS_2 = crate::Reg<u32, _PRO_INTR_STATUS_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_INTR_STATUS_2;
#[doc = "`read()` method returns [pro_intr_status_2::R](pro_intr_status_2::R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_2 {}
#[doc = "`write(|w| ..)` method takes [pro_intr_status_2::W](pro_intr_status_2::W) writer structure"]
impl crate::Writable for PRO_INTR_STATUS_2 {}
#[doc = "DPORT_PRO_INTR_STATUS_2_REG"]
pub mod pro_intr_status_2;
#[doc = "DPORT_APP_INTR_STATUS_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_intr_status_0](app_intr_status_0) module"]
pub type APP_INTR_STATUS_0 = crate::Reg<u32, _APP_INTR_STATUS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_INTR_STATUS_0;
#[doc = "`read()` method returns [app_intr_status_0::R](app_intr_status_0::R) reader structure"]
impl crate::Readable for APP_INTR_STATUS_0 {}
#[doc = "`write(|w| ..)` method takes [app_intr_status_0::W](app_intr_status_0::W) writer structure"]
impl crate::Writable for APP_INTR_STATUS_0 {}
#[doc = "DPORT_APP_INTR_STATUS_0_REG"]
pub mod app_intr_status_0;
#[doc = "DPORT_APP_INTR_STATUS_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_intr_status_1](app_intr_status_1) module"]
pub type APP_INTR_STATUS_1 = crate::Reg<u32, _APP_INTR_STATUS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_INTR_STATUS_1;
#[doc = "`read()` method returns [app_intr_status_1::R](app_intr_status_1::R) reader structure"]
impl crate::Readable for APP_INTR_STATUS_1 {}
#[doc = "`write(|w| ..)` method takes [app_intr_status_1::W](app_intr_status_1::W) writer structure"]
impl crate::Writable for APP_INTR_STATUS_1 {}
#[doc = "DPORT_APP_INTR_STATUS_1_REG"]
pub mod app_intr_status_1;
#[doc = "DPORT_APP_INTR_STATUS_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_intr_status_2](app_intr_status_2) module"]
pub type APP_INTR_STATUS_2 = crate::Reg<u32, _APP_INTR_STATUS_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_INTR_STATUS_2;
#[doc = "`read()` method returns [app_intr_status_2::R](app_intr_status_2::R) reader structure"]
impl crate::Readable for APP_INTR_STATUS_2 {}
#[doc = "`write(|w| ..)` method takes [app_intr_status_2::W](app_intr_status_2::W) writer structure"]
impl crate::Writable for APP_INTR_STATUS_2 {}
#[doc = "DPORT_APP_INTR_STATUS_2_REG"]
pub mod app_intr_status_2;
#[doc = "DPORT_PRO_MAC_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_mac_intr_map](pro_mac_intr_map) module"]
pub type PRO_MAC_INTR_MAP = crate::Reg<u32, _PRO_MAC_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_MAC_INTR_MAP;
#[doc = "`read()` method returns [pro_mac_intr_map::R](pro_mac_intr_map::R) reader structure"]
impl crate::Readable for PRO_MAC_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_mac_intr_map::W](pro_mac_intr_map::W) writer structure"]
impl crate::Writable for PRO_MAC_INTR_MAP {}
#[doc = "DPORT_PRO_MAC_INTR_MAP_REG"]
pub mod pro_mac_intr_map;
#[doc = "DPORT_PRO_MAC_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_mac_nmi_map](pro_mac_nmi_map) module"]
pub type PRO_MAC_NMI_MAP = crate::Reg<u32, _PRO_MAC_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_MAC_NMI_MAP;
#[doc = "`read()` method returns [pro_mac_nmi_map::R](pro_mac_nmi_map::R) reader structure"]
impl crate::Readable for PRO_MAC_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_mac_nmi_map::W](pro_mac_nmi_map::W) writer structure"]
impl crate::Writable for PRO_MAC_NMI_MAP {}
#[doc = "DPORT_PRO_MAC_NMI_MAP_REG"]
pub mod pro_mac_nmi_map;
#[doc = "DPORT_PRO_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_bb_int_map](pro_bb_int_map) module"]
pub type PRO_BB_INT_MAP = crate::Reg<u32, _PRO_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_BB_INT_MAP;
#[doc = "`read()` method returns [pro_bb_int_map::R](pro_bb_int_map::R) reader structure"]
impl crate::Readable for PRO_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_bb_int_map::W](pro_bb_int_map::W) writer structure"]
impl crate::Writable for PRO_BB_INT_MAP {}
#[doc = "DPORT_PRO_BB_INT_MAP_REG"]
pub mod pro_bb_int_map;
#[doc = "DPORT_PRO_BT_MAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_bt_mac_int_map](pro_bt_mac_int_map) module"]
pub type PRO_BT_MAC_INT_MAP = crate::Reg<u32, _PRO_BT_MAC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_BT_MAC_INT_MAP;
#[doc = "`read()` method returns [pro_bt_mac_int_map::R](pro_bt_mac_int_map::R) reader structure"]
impl crate::Readable for PRO_BT_MAC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_bt_mac_int_map::W](pro_bt_mac_int_map::W) writer structure"]
impl crate::Writable for PRO_BT_MAC_INT_MAP {}
#[doc = "DPORT_PRO_BT_MAC_INT_MAP_REG"]
pub mod pro_bt_mac_int_map;
#[doc = "DPORT_PRO_BT_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_bt_bb_int_map](pro_bt_bb_int_map) module"]
pub type PRO_BT_BB_INT_MAP = crate::Reg<u32, _PRO_BT_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_BT_BB_INT_MAP;
#[doc = "`read()` method returns [pro_bt_bb_int_map::R](pro_bt_bb_int_map::R) reader structure"]
impl crate::Readable for PRO_BT_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_bt_bb_int_map::W](pro_bt_bb_int_map::W) writer structure"]
impl crate::Writable for PRO_BT_BB_INT_MAP {}
#[doc = "DPORT_PRO_BT_BB_INT_MAP_REG"]
pub mod pro_bt_bb_int_map;
#[doc = "DPORT_PRO_BT_BB_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_bt_bb_nmi_map](pro_bt_bb_nmi_map) module"]
pub type PRO_BT_BB_NMI_MAP = crate::Reg<u32, _PRO_BT_BB_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_BT_BB_NMI_MAP;
#[doc = "`read()` method returns [pro_bt_bb_nmi_map::R](pro_bt_bb_nmi_map::R) reader structure"]
impl crate::Readable for PRO_BT_BB_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_bt_bb_nmi_map::W](pro_bt_bb_nmi_map::W) writer structure"]
impl crate::Writable for PRO_BT_BB_NMI_MAP {}
#[doc = "DPORT_PRO_BT_BB_NMI_MAP_REG"]
pub mod pro_bt_bb_nmi_map;
#[doc = "DPORT_PRO_RWBT_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rwbt_irq_map](pro_rwbt_irq_map) module"]
pub type PRO_RWBT_IRQ_MAP = crate::Reg<u32, _PRO_RWBT_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RWBT_IRQ_MAP;
#[doc = "`read()` method returns [pro_rwbt_irq_map::R](pro_rwbt_irq_map::R) reader structure"]
impl crate::Readable for PRO_RWBT_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rwbt_irq_map::W](pro_rwbt_irq_map::W) writer structure"]
impl crate::Writable for PRO_RWBT_IRQ_MAP {}
#[doc = "DPORT_PRO_RWBT_IRQ_MAP_REG"]
pub mod pro_rwbt_irq_map;
#[doc = "DPORT_PRO_RWBLE_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rwble_irq_map](pro_rwble_irq_map) module"]
pub type PRO_RWBLE_IRQ_MAP = crate::Reg<u32, _PRO_RWBLE_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RWBLE_IRQ_MAP;
#[doc = "`read()` method returns [pro_rwble_irq_map::R](pro_rwble_irq_map::R) reader structure"]
impl crate::Readable for PRO_RWBLE_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rwble_irq_map::W](pro_rwble_irq_map::W) writer structure"]
impl crate::Writable for PRO_RWBLE_IRQ_MAP {}
#[doc = "DPORT_PRO_RWBLE_IRQ_MAP_REG"]
pub mod pro_rwble_irq_map;
#[doc = "DPORT_PRO_RWBT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rwbt_nmi_map](pro_rwbt_nmi_map) module"]
pub type PRO_RWBT_NMI_MAP = crate::Reg<u32, _PRO_RWBT_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RWBT_NMI_MAP;
#[doc = "`read()` method returns [pro_rwbt_nmi_map::R](pro_rwbt_nmi_map::R) reader structure"]
impl crate::Readable for PRO_RWBT_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rwbt_nmi_map::W](pro_rwbt_nmi_map::W) writer structure"]
impl crate::Writable for PRO_RWBT_NMI_MAP {}
#[doc = "DPORT_PRO_RWBT_NMI_MAP_REG"]
pub mod pro_rwbt_nmi_map;
#[doc = "DPORT_PRO_RWBLE_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rwble_nmi_map](pro_rwble_nmi_map) module"]
pub type PRO_RWBLE_NMI_MAP = crate::Reg<u32, _PRO_RWBLE_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RWBLE_NMI_MAP;
#[doc = "`read()` method returns [pro_rwble_nmi_map::R](pro_rwble_nmi_map::R) reader structure"]
impl crate::Readable for PRO_RWBLE_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rwble_nmi_map::W](pro_rwble_nmi_map::W) writer structure"]
impl crate::Writable for PRO_RWBLE_NMI_MAP {}
#[doc = "DPORT_PRO_RWBLE_NMI_MAP_REG"]
pub mod pro_rwble_nmi_map;
#[doc = "DPORT_PRO_SLC0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_slc0_intr_map](pro_slc0_intr_map) module"]
pub type PRO_SLC0_INTR_MAP = crate::Reg<u32, _PRO_SLC0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SLC0_INTR_MAP;
#[doc = "`read()` method returns [pro_slc0_intr_map::R](pro_slc0_intr_map::R) reader structure"]
impl crate::Readable for PRO_SLC0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_slc0_intr_map::W](pro_slc0_intr_map::W) writer structure"]
impl crate::Writable for PRO_SLC0_INTR_MAP {}
#[doc = "DPORT_PRO_SLC0_INTR_MAP_REG"]
pub mod pro_slc0_intr_map;
#[doc = "DPORT_PRO_SLC1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_slc1_intr_map](pro_slc1_intr_map) module"]
pub type PRO_SLC1_INTR_MAP = crate::Reg<u32, _PRO_SLC1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SLC1_INTR_MAP;
#[doc = "`read()` method returns [pro_slc1_intr_map::R](pro_slc1_intr_map::R) reader structure"]
impl crate::Readable for PRO_SLC1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_slc1_intr_map::W](pro_slc1_intr_map::W) writer structure"]
impl crate::Writable for PRO_SLC1_INTR_MAP {}
#[doc = "DPORT_PRO_SLC1_INTR_MAP_REG"]
pub mod pro_slc1_intr_map;
#[doc = "DPORT_PRO_UHCI0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_uhci0_intr_map](pro_uhci0_intr_map) module"]
pub type PRO_UHCI0_INTR_MAP = crate::Reg<u32, _PRO_UHCI0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_UHCI0_INTR_MAP;
#[doc = "`read()` method returns [pro_uhci0_intr_map::R](pro_uhci0_intr_map::R) reader structure"]
impl crate::Readable for PRO_UHCI0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_uhci0_intr_map::W](pro_uhci0_intr_map::W) writer structure"]
impl crate::Writable for PRO_UHCI0_INTR_MAP {}
#[doc = "DPORT_PRO_UHCI0_INTR_MAP_REG"]
pub mod pro_uhci0_intr_map;
#[doc = "DPORT_PRO_UHCI1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_uhci1_intr_map](pro_uhci1_intr_map) module"]
pub type PRO_UHCI1_INTR_MAP = crate::Reg<u32, _PRO_UHCI1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_UHCI1_INTR_MAP;
#[doc = "`read()` method returns [pro_uhci1_intr_map::R](pro_uhci1_intr_map::R) reader structure"]
impl crate::Readable for PRO_UHCI1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_uhci1_intr_map::W](pro_uhci1_intr_map::W) writer structure"]
impl crate::Writable for PRO_UHCI1_INTR_MAP {}
#[doc = "DPORT_PRO_UHCI1_INTR_MAP_REG"]
pub mod pro_uhci1_intr_map;
#[doc = "DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_t0_level_int_map](pro_tg_t0_level_int_map) module"]
pub type PRO_TG_T0_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG_T0_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_T0_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg_t0_level_int_map::R](pro_tg_t0_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_T0_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_t0_level_int_map::W](pro_tg_t0_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_T0_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG_T0_LEVEL_INT_MAP_REG"]
pub mod pro_tg_t0_level_int_map;
#[doc = "DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_t1_level_int_map](pro_tg_t1_level_int_map) module"]
pub type PRO_TG_T1_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG_T1_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_T1_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg_t1_level_int_map::R](pro_tg_t1_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_T1_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_t1_level_int_map::W](pro_tg_t1_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_T1_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG_T1_LEVEL_INT_MAP_REG"]
pub mod pro_tg_t1_level_int_map;
#[doc = "DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_wdt_level_int_map](pro_tg_wdt_level_int_map) module"]
pub type PRO_TG_WDT_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG_WDT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_WDT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg_wdt_level_int_map::R](pro_tg_wdt_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_WDT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_wdt_level_int_map::W](pro_tg_wdt_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_WDT_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG_WDT_LEVEL_INT_MAP_REG"]
pub mod pro_tg_wdt_level_int_map;
#[doc = "DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_lact_level_int_map](pro_tg_lact_level_int_map) module"]
pub type PRO_TG_LACT_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG_LACT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_LACT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg_lact_level_int_map::R](pro_tg_lact_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_LACT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_lact_level_int_map::W](pro_tg_lact_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_LACT_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG_LACT_LEVEL_INT_MAP_REG"]
pub mod pro_tg_lact_level_int_map;
#[doc = "DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_t0_level_int_map](pro_tg1_t0_level_int_map) module"]
pub type PRO_TG1_T0_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG1_T0_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_T0_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_t0_level_int_map::R](pro_tg1_t0_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_T0_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_t0_level_int_map::W](pro_tg1_t0_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_T0_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG1_T0_LEVEL_INT_MAP_REG"]
pub mod pro_tg1_t0_level_int_map;
#[doc = "DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_t1_level_int_map](pro_tg1_t1_level_int_map) module"]
pub type PRO_TG1_T1_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG1_T1_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_T1_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_t1_level_int_map::R](pro_tg1_t1_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_T1_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_t1_level_int_map::W](pro_tg1_t1_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_T1_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG1_T1_LEVEL_INT_MAP_REG"]
pub mod pro_tg1_t1_level_int_map;
#[doc = "DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_wdt_level_int_map](pro_tg1_wdt_level_int_map) module"]
pub type PRO_TG1_WDT_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG1_WDT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_WDT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_wdt_level_int_map::R](pro_tg1_wdt_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_WDT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_wdt_level_int_map::W](pro_tg1_wdt_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_WDT_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG1_WDT_LEVEL_INT_MAP_REG"]
pub mod pro_tg1_wdt_level_int_map;
#[doc = "DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_lact_level_int_map](pro_tg1_lact_level_int_map) module"]
pub type PRO_TG1_LACT_LEVEL_INT_MAP = crate::Reg<u32, _PRO_TG1_LACT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_LACT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_lact_level_int_map::R](pro_tg1_lact_level_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_LACT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_lact_level_int_map::W](pro_tg1_lact_level_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_LACT_LEVEL_INT_MAP {}
#[doc = "DPORT_PRO_TG1_LACT_LEVEL_INT_MAP_REG"]
pub mod pro_tg1_lact_level_int_map;
#[doc = "DPORT_PRO_GPIO_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_gpio_interrupt_map](pro_gpio_interrupt_map) module"]
pub type PRO_GPIO_INTERRUPT_MAP = crate::Reg<u32, _PRO_GPIO_INTERRUPT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_GPIO_INTERRUPT_MAP;
#[doc = "`read()` method returns [pro_gpio_interrupt_map::R](pro_gpio_interrupt_map::R) reader structure"]
impl crate::Readable for PRO_GPIO_INTERRUPT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_gpio_interrupt_map::W](pro_gpio_interrupt_map::W) writer structure"]
impl crate::Writable for PRO_GPIO_INTERRUPT_MAP {}
#[doc = "DPORT_PRO_GPIO_INTERRUPT_MAP_REG"]
pub mod pro_gpio_interrupt_map;
#[doc = "DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_gpio_interrupt_nmi_map](pro_gpio_interrupt_nmi_map) module"]
pub type PRO_GPIO_INTERRUPT_NMI_MAP = crate::Reg<u32, _PRO_GPIO_INTERRUPT_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_GPIO_INTERRUPT_NMI_MAP;
#[doc = "`read()` method returns [pro_gpio_interrupt_nmi_map::R](pro_gpio_interrupt_nmi_map::R) reader structure"]
impl crate::Readable for PRO_GPIO_INTERRUPT_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_gpio_interrupt_nmi_map::W](pro_gpio_interrupt_nmi_map::W) writer structure"]
impl crate::Writable for PRO_GPIO_INTERRUPT_NMI_MAP {}
#[doc = "DPORT_PRO_GPIO_INTERRUPT_NMI_MAP_REG"]
pub mod pro_gpio_interrupt_nmi_map;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_intr_from_cpu_0_map](pro_cpu_intr_from_cpu_0_map) module"]
pub type PRO_CPU_INTR_FROM_CPU_0_MAP = crate::Reg<u32, _PRO_CPU_INTR_FROM_CPU_0_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_INTR_FROM_CPU_0_MAP;
#[doc = "`read()` method returns [pro_cpu_intr_from_cpu_0_map::R](pro_cpu_intr_from_cpu_0_map::R) reader structure"]
impl crate::Readable for PRO_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_intr_from_cpu_0_map::W](pro_cpu_intr_from_cpu_0_map::W) writer structure"]
impl crate::Writable for PRO_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_0_MAP_REG"]
pub mod pro_cpu_intr_from_cpu_0_map;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_intr_from_cpu_1_map](pro_cpu_intr_from_cpu_1_map) module"]
pub type PRO_CPU_INTR_FROM_CPU_1_MAP = crate::Reg<u32, _PRO_CPU_INTR_FROM_CPU_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_INTR_FROM_CPU_1_MAP;
#[doc = "`read()` method returns [pro_cpu_intr_from_cpu_1_map::R](pro_cpu_intr_from_cpu_1_map::R) reader structure"]
impl crate::Readable for PRO_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_intr_from_cpu_1_map::W](pro_cpu_intr_from_cpu_1_map::W) writer structure"]
impl crate::Writable for PRO_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_1_MAP_REG"]
pub mod pro_cpu_intr_from_cpu_1_map;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_intr_from_cpu_2_map](pro_cpu_intr_from_cpu_2_map) module"]
pub type PRO_CPU_INTR_FROM_CPU_2_MAP = crate::Reg<u32, _PRO_CPU_INTR_FROM_CPU_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_INTR_FROM_CPU_2_MAP;
#[doc = "`read()` method returns [pro_cpu_intr_from_cpu_2_map::R](pro_cpu_intr_from_cpu_2_map::R) reader structure"]
impl crate::Readable for PRO_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_intr_from_cpu_2_map::W](pro_cpu_intr_from_cpu_2_map::W) writer structure"]
impl crate::Writable for PRO_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_2_MAP_REG"]
pub mod pro_cpu_intr_from_cpu_2_map;
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_intr_from_cpu_3_map](pro_cpu_intr_from_cpu_3_map) module"]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP = crate::Reg<u32, _PRO_CPU_INTR_FROM_CPU_3_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_INTR_FROM_CPU_3_MAP;
#[doc = "`read()` method returns [pro_cpu_intr_from_cpu_3_map::R](pro_cpu_intr_from_cpu_3_map::R) reader structure"]
impl crate::Readable for PRO_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_intr_from_cpu_3_map::W](pro_cpu_intr_from_cpu_3_map::W) writer structure"]
impl crate::Writable for PRO_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "DPORT_PRO_CPU_INTR_FROM_CPU_3_MAP_REG"]
pub mod pro_cpu_intr_from_cpu_3_map;
#[doc = "DPORT_PRO_SPI_INTR_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi_intr_0_map](pro_spi_intr_0_map) module"]
pub type PRO_SPI_INTR_0_MAP = crate::Reg<u32, _PRO_SPI_INTR_0_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI_INTR_0_MAP;
#[doc = "`read()` method returns [pro_spi_intr_0_map::R](pro_spi_intr_0_map::R) reader structure"]
impl crate::Readable for PRO_SPI_INTR_0_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi_intr_0_map::W](pro_spi_intr_0_map::W) writer structure"]
impl crate::Writable for PRO_SPI_INTR_0_MAP {}
#[doc = "DPORT_PRO_SPI_INTR_0_MAP_REG"]
pub mod pro_spi_intr_0_map;
#[doc = "DPORT_PRO_SPI_INTR_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi_intr_1_map](pro_spi_intr_1_map) module"]
pub type PRO_SPI_INTR_1_MAP = crate::Reg<u32, _PRO_SPI_INTR_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI_INTR_1_MAP;
#[doc = "`read()` method returns [pro_spi_intr_1_map::R](pro_spi_intr_1_map::R) reader structure"]
impl crate::Readable for PRO_SPI_INTR_1_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi_intr_1_map::W](pro_spi_intr_1_map::W) writer structure"]
impl crate::Writable for PRO_SPI_INTR_1_MAP {}
#[doc = "DPORT_PRO_SPI_INTR_1_MAP_REG"]
pub mod pro_spi_intr_1_map;
#[doc = "DPORT_PRO_SPI_INTR_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi_intr_2_map](pro_spi_intr_2_map) module"]
pub type PRO_SPI_INTR_2_MAP = crate::Reg<u32, _PRO_SPI_INTR_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI_INTR_2_MAP;
#[doc = "`read()` method returns [pro_spi_intr_2_map::R](pro_spi_intr_2_map::R) reader structure"]
impl crate::Readable for PRO_SPI_INTR_2_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi_intr_2_map::W](pro_spi_intr_2_map::W) writer structure"]
impl crate::Writable for PRO_SPI_INTR_2_MAP {}
#[doc = "DPORT_PRO_SPI_INTR_2_MAP_REG"]
pub mod pro_spi_intr_2_map;
#[doc = "DPORT_PRO_SPI_INTR_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi_intr_3_map](pro_spi_intr_3_map) module"]
pub type PRO_SPI_INTR_3_MAP = crate::Reg<u32, _PRO_SPI_INTR_3_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI_INTR_3_MAP;
#[doc = "`read()` method returns [pro_spi_intr_3_map::R](pro_spi_intr_3_map::R) reader structure"]
impl crate::Readable for PRO_SPI_INTR_3_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi_intr_3_map::W](pro_spi_intr_3_map::W) writer structure"]
impl crate::Writable for PRO_SPI_INTR_3_MAP {}
#[doc = "DPORT_PRO_SPI_INTR_3_MAP_REG"]
pub mod pro_spi_intr_3_map;
#[doc = "DPORT_PRO_I2S0_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_i2s0_int_map](pro_i2s0_int_map) module"]
pub type PRO_I2S0_INT_MAP = crate::Reg<u32, _PRO_I2S0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_I2S0_INT_MAP;
#[doc = "`read()` method returns [pro_i2s0_int_map::R](pro_i2s0_int_map::R) reader structure"]
impl crate::Readable for PRO_I2S0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_i2s0_int_map::W](pro_i2s0_int_map::W) writer structure"]
impl crate::Writable for PRO_I2S0_INT_MAP {}
#[doc = "DPORT_PRO_I2S0_INT_MAP_REG"]
pub mod pro_i2s0_int_map;
#[doc = "DPORT_PRO_I2S1_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_i2s1_int_map](pro_i2s1_int_map) module"]
pub type PRO_I2S1_INT_MAP = crate::Reg<u32, _PRO_I2S1_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_I2S1_INT_MAP;
#[doc = "`read()` method returns [pro_i2s1_int_map::R](pro_i2s1_int_map::R) reader structure"]
impl crate::Readable for PRO_I2S1_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_i2s1_int_map::W](pro_i2s1_int_map::W) writer structure"]
impl crate::Writable for PRO_I2S1_INT_MAP {}
#[doc = "DPORT_PRO_I2S1_INT_MAP_REG"]
pub mod pro_i2s1_int_map;
#[doc = "DPORT_PRO_UART_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_uart_intr_map](pro_uart_intr_map) module"]
pub type PRO_UART_INTR_MAP = crate::Reg<u32, _PRO_UART_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_UART_INTR_MAP;
#[doc = "`read()` method returns [pro_uart_intr_map::R](pro_uart_intr_map::R) reader structure"]
impl crate::Readable for PRO_UART_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_uart_intr_map::W](pro_uart_intr_map::W) writer structure"]
impl crate::Writable for PRO_UART_INTR_MAP {}
#[doc = "DPORT_PRO_UART_INTR_MAP_REG"]
pub mod pro_uart_intr_map;
#[doc = "DPORT_PRO_UART1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_uart1_intr_map](pro_uart1_intr_map) module"]
pub type PRO_UART1_INTR_MAP = crate::Reg<u32, _PRO_UART1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_UART1_INTR_MAP;
#[doc = "`read()` method returns [pro_uart1_intr_map::R](pro_uart1_intr_map::R) reader structure"]
impl crate::Readable for PRO_UART1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_uart1_intr_map::W](pro_uart1_intr_map::W) writer structure"]
impl crate::Writable for PRO_UART1_INTR_MAP {}
#[doc = "DPORT_PRO_UART1_INTR_MAP_REG"]
pub mod pro_uart1_intr_map;
#[doc = "DPORT_PRO_UART2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_uart2_intr_map](pro_uart2_intr_map) module"]
pub type PRO_UART2_INTR_MAP = crate::Reg<u32, _PRO_UART2_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_UART2_INTR_MAP;
#[doc = "`read()` method returns [pro_uart2_intr_map::R](pro_uart2_intr_map::R) reader structure"]
impl crate::Readable for PRO_UART2_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_uart2_intr_map::W](pro_uart2_intr_map::W) writer structure"]
impl crate::Writable for PRO_UART2_INTR_MAP {}
#[doc = "DPORT_PRO_UART2_INTR_MAP_REG"]
pub mod pro_uart2_intr_map;
#[doc = "DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_sdio_host_interrupt_map](pro_sdio_host_interrupt_map) module"]
pub type PRO_SDIO_HOST_INTERRUPT_MAP = crate::Reg<u32, _PRO_SDIO_HOST_INTERRUPT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SDIO_HOST_INTERRUPT_MAP;
#[doc = "`read()` method returns [pro_sdio_host_interrupt_map::R](pro_sdio_host_interrupt_map::R) reader structure"]
impl crate::Readable for PRO_SDIO_HOST_INTERRUPT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_sdio_host_interrupt_map::W](pro_sdio_host_interrupt_map::W) writer structure"]
impl crate::Writable for PRO_SDIO_HOST_INTERRUPT_MAP {}
#[doc = "DPORT_PRO_SDIO_HOST_INTERRUPT_MAP_REG"]
pub mod pro_sdio_host_interrupt_map;
#[doc = "DPORT_PRO_EMAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_emac_int_map](pro_emac_int_map) module"]
pub type PRO_EMAC_INT_MAP = crate::Reg<u32, _PRO_EMAC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_EMAC_INT_MAP;
#[doc = "`read()` method returns [pro_emac_int_map::R](pro_emac_int_map::R) reader structure"]
impl crate::Readable for PRO_EMAC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_emac_int_map::W](pro_emac_int_map::W) writer structure"]
impl crate::Writable for PRO_EMAC_INT_MAP {}
#[doc = "DPORT_PRO_EMAC_INT_MAP_REG"]
pub mod pro_emac_int_map;
#[doc = "DPORT_PRO_PWM0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_pwm0_intr_map](pro_pwm0_intr_map) module"]
pub type PRO_PWM0_INTR_MAP = crate::Reg<u32, _PRO_PWM0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_PWM0_INTR_MAP;
#[doc = "`read()` method returns [pro_pwm0_intr_map::R](pro_pwm0_intr_map::R) reader structure"]
impl crate::Readable for PRO_PWM0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_pwm0_intr_map::W](pro_pwm0_intr_map::W) writer structure"]
impl crate::Writable for PRO_PWM0_INTR_MAP {}
#[doc = "DPORT_PRO_PWM0_INTR_MAP_REG"]
pub mod pro_pwm0_intr_map;
#[doc = "DPORT_PRO_PWM1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_pwm1_intr_map](pro_pwm1_intr_map) module"]
pub type PRO_PWM1_INTR_MAP = crate::Reg<u32, _PRO_PWM1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_PWM1_INTR_MAP;
#[doc = "`read()` method returns [pro_pwm1_intr_map::R](pro_pwm1_intr_map::R) reader structure"]
impl crate::Readable for PRO_PWM1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_pwm1_intr_map::W](pro_pwm1_intr_map::W) writer structure"]
impl crate::Writable for PRO_PWM1_INTR_MAP {}
#[doc = "DPORT_PRO_PWM1_INTR_MAP_REG"]
pub mod pro_pwm1_intr_map;
#[doc = "DPORT_PRO_PWM2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_pwm2_intr_map](pro_pwm2_intr_map) module"]
pub type PRO_PWM2_INTR_MAP = crate::Reg<u32, _PRO_PWM2_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_PWM2_INTR_MAP;
#[doc = "`read()` method returns [pro_pwm2_intr_map::R](pro_pwm2_intr_map::R) reader structure"]
impl crate::Readable for PRO_PWM2_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_pwm2_intr_map::W](pro_pwm2_intr_map::W) writer structure"]
impl crate::Writable for PRO_PWM2_INTR_MAP {}
#[doc = "DPORT_PRO_PWM2_INTR_MAP_REG"]
pub mod pro_pwm2_intr_map;
#[doc = "DPORT_PRO_PWM3_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_pwm3_intr_map](pro_pwm3_intr_map) module"]
pub type PRO_PWM3_INTR_MAP = crate::Reg<u32, _PRO_PWM3_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_PWM3_INTR_MAP;
#[doc = "`read()` method returns [pro_pwm3_intr_map::R](pro_pwm3_intr_map::R) reader structure"]
impl crate::Readable for PRO_PWM3_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_pwm3_intr_map::W](pro_pwm3_intr_map::W) writer structure"]
impl crate::Writable for PRO_PWM3_INTR_MAP {}
#[doc = "DPORT_PRO_PWM3_INTR_MAP_REG"]
pub mod pro_pwm3_intr_map;
#[doc = "DPORT_PRO_LEDC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_ledc_int_map](pro_ledc_int_map) module"]
pub type PRO_LEDC_INT_MAP = crate::Reg<u32, _PRO_LEDC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_LEDC_INT_MAP;
#[doc = "`read()` method returns [pro_ledc_int_map::R](pro_ledc_int_map::R) reader structure"]
impl crate::Readable for PRO_LEDC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_ledc_int_map::W](pro_ledc_int_map::W) writer structure"]
impl crate::Writable for PRO_LEDC_INT_MAP {}
#[doc = "DPORT_PRO_LEDC_INT_MAP_REG"]
pub mod pro_ledc_int_map;
#[doc = "DPORT_PRO_EFUSE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_efuse_int_map](pro_efuse_int_map) module"]
pub type PRO_EFUSE_INT_MAP = crate::Reg<u32, _PRO_EFUSE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_EFUSE_INT_MAP;
#[doc = "`read()` method returns [pro_efuse_int_map::R](pro_efuse_int_map::R) reader structure"]
impl crate::Readable for PRO_EFUSE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_efuse_int_map::W](pro_efuse_int_map::W) writer structure"]
impl crate::Writable for PRO_EFUSE_INT_MAP {}
#[doc = "DPORT_PRO_EFUSE_INT_MAP_REG"]
pub mod pro_efuse_int_map;
#[doc = "DPORT_PRO_CAN_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_can_int_map](pro_can_int_map) module"]
pub type PRO_CAN_INT_MAP = crate::Reg<u32, _PRO_CAN_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CAN_INT_MAP;
#[doc = "`read()` method returns [pro_can_int_map::R](pro_can_int_map::R) reader structure"]
impl crate::Readable for PRO_CAN_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_can_int_map::W](pro_can_int_map::W) writer structure"]
impl crate::Writable for PRO_CAN_INT_MAP {}
#[doc = "DPORT_PRO_CAN_INT_MAP_REG"]
pub mod pro_can_int_map;
#[doc = "DPORT_PRO_RTC_CORE_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rtc_core_intr_map](pro_rtc_core_intr_map) module"]
pub type PRO_RTC_CORE_INTR_MAP = crate::Reg<u32, _PRO_RTC_CORE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RTC_CORE_INTR_MAP;
#[doc = "`read()` method returns [pro_rtc_core_intr_map::R](pro_rtc_core_intr_map::R) reader structure"]
impl crate::Readable for PRO_RTC_CORE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rtc_core_intr_map::W](pro_rtc_core_intr_map::W) writer structure"]
impl crate::Writable for PRO_RTC_CORE_INTR_MAP {}
#[doc = "DPORT_PRO_RTC_CORE_INTR_MAP_REG"]
pub mod pro_rtc_core_intr_map;
#[doc = "DPORT_PRO_RMT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rmt_intr_map](pro_rmt_intr_map) module"]
pub type PRO_RMT_INTR_MAP = crate::Reg<u32, _PRO_RMT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RMT_INTR_MAP;
#[doc = "`read()` method returns [pro_rmt_intr_map::R](pro_rmt_intr_map::R) reader structure"]
impl crate::Readable for PRO_RMT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rmt_intr_map::W](pro_rmt_intr_map::W) writer structure"]
impl crate::Writable for PRO_RMT_INTR_MAP {}
#[doc = "DPORT_PRO_RMT_INTR_MAP_REG"]
pub mod pro_rmt_intr_map;
#[doc = "DPORT_PRO_PCNT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_pcnt_intr_map](pro_pcnt_intr_map) module"]
pub type PRO_PCNT_INTR_MAP = crate::Reg<u32, _PRO_PCNT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_PCNT_INTR_MAP;
#[doc = "`read()` method returns [pro_pcnt_intr_map::R](pro_pcnt_intr_map::R) reader structure"]
impl crate::Readable for PRO_PCNT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_pcnt_intr_map::W](pro_pcnt_intr_map::W) writer structure"]
impl crate::Writable for PRO_PCNT_INTR_MAP {}
#[doc = "DPORT_PRO_PCNT_INTR_MAP_REG"]
pub mod pro_pcnt_intr_map;
#[doc = "DPORT_PRO_I2C_EXT0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_i2c_ext0_intr_map](pro_i2c_ext0_intr_map) module"]
pub type PRO_I2C_EXT0_INTR_MAP = crate::Reg<u32, _PRO_I2C_EXT0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_I2C_EXT0_INTR_MAP;
#[doc = "`read()` method returns [pro_i2c_ext0_intr_map::R](pro_i2c_ext0_intr_map::R) reader structure"]
impl crate::Readable for PRO_I2C_EXT0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_i2c_ext0_intr_map::W](pro_i2c_ext0_intr_map::W) writer structure"]
impl crate::Writable for PRO_I2C_EXT0_INTR_MAP {}
#[doc = "DPORT_PRO_I2C_EXT0_INTR_MAP_REG"]
pub mod pro_i2c_ext0_intr_map;
#[doc = "DPORT_PRO_I2C_EXT1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_i2c_ext1_intr_map](pro_i2c_ext1_intr_map) module"]
pub type PRO_I2C_EXT1_INTR_MAP = crate::Reg<u32, _PRO_I2C_EXT1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_I2C_EXT1_INTR_MAP;
#[doc = "`read()` method returns [pro_i2c_ext1_intr_map::R](pro_i2c_ext1_intr_map::R) reader structure"]
impl crate::Readable for PRO_I2C_EXT1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_i2c_ext1_intr_map::W](pro_i2c_ext1_intr_map::W) writer structure"]
impl crate::Writable for PRO_I2C_EXT1_INTR_MAP {}
#[doc = "DPORT_PRO_I2C_EXT1_INTR_MAP_REG"]
pub mod pro_i2c_ext1_intr_map;
#[doc = "DPORT_PRO_RSA_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_rsa_intr_map](pro_rsa_intr_map) module"]
pub type PRO_RSA_INTR_MAP = crate::Reg<u32, _PRO_RSA_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_RSA_INTR_MAP;
#[doc = "`read()` method returns [pro_rsa_intr_map::R](pro_rsa_intr_map::R) reader structure"]
impl crate::Readable for PRO_RSA_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_rsa_intr_map::W](pro_rsa_intr_map::W) writer structure"]
impl crate::Writable for PRO_RSA_INTR_MAP {}
#[doc = "DPORT_PRO_RSA_INTR_MAP_REG"]
pub mod pro_rsa_intr_map;
#[doc = "DPORT_PRO_SPI1_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi1_dma_int_map](pro_spi1_dma_int_map) module"]
pub type PRO_SPI1_DMA_INT_MAP = crate::Reg<u32, _PRO_SPI1_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI1_DMA_INT_MAP;
#[doc = "`read()` method returns [pro_spi1_dma_int_map::R](pro_spi1_dma_int_map::R) reader structure"]
impl crate::Readable for PRO_SPI1_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi1_dma_int_map::W](pro_spi1_dma_int_map::W) writer structure"]
impl crate::Writable for PRO_SPI1_DMA_INT_MAP {}
#[doc = "DPORT_PRO_SPI1_DMA_INT_MAP_REG"]
pub mod pro_spi1_dma_int_map;
#[doc = "DPORT_PRO_SPI2_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi2_dma_int_map](pro_spi2_dma_int_map) module"]
pub type PRO_SPI2_DMA_INT_MAP = crate::Reg<u32, _PRO_SPI2_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI2_DMA_INT_MAP;
#[doc = "`read()` method returns [pro_spi2_dma_int_map::R](pro_spi2_dma_int_map::R) reader structure"]
impl crate::Readable for PRO_SPI2_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi2_dma_int_map::W](pro_spi2_dma_int_map::W) writer structure"]
impl crate::Writable for PRO_SPI2_DMA_INT_MAP {}
#[doc = "DPORT_PRO_SPI2_DMA_INT_MAP_REG"]
pub mod pro_spi2_dma_int_map;
#[doc = "DPORT_PRO_SPI3_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_spi3_dma_int_map](pro_spi3_dma_int_map) module"]
pub type PRO_SPI3_DMA_INT_MAP = crate::Reg<u32, _PRO_SPI3_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_SPI3_DMA_INT_MAP;
#[doc = "`read()` method returns [pro_spi3_dma_int_map::R](pro_spi3_dma_int_map::R) reader structure"]
impl crate::Readable for PRO_SPI3_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_spi3_dma_int_map::W](pro_spi3_dma_int_map::W) writer structure"]
impl crate::Writable for PRO_SPI3_DMA_INT_MAP {}
#[doc = "DPORT_PRO_SPI3_DMA_INT_MAP_REG"]
pub mod pro_spi3_dma_int_map;
#[doc = "DPORT_PRO_WDG_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_wdg_int_map](pro_wdg_int_map) module"]
pub type PRO_WDG_INT_MAP = crate::Reg<u32, _PRO_WDG_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_WDG_INT_MAP;
#[doc = "`read()` method returns [pro_wdg_int_map::R](pro_wdg_int_map::R) reader structure"]
impl crate::Readable for PRO_WDG_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_wdg_int_map::W](pro_wdg_int_map::W) writer structure"]
impl crate::Writable for PRO_WDG_INT_MAP {}
#[doc = "DPORT_PRO_WDG_INT_MAP_REG"]
pub mod pro_wdg_int_map;
#[doc = "DPORT_PRO_TIMER_INT1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_timer_int1_map](pro_timer_int1_map) module"]
pub type PRO_TIMER_INT1_MAP = crate::Reg<u32, _PRO_TIMER_INT1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TIMER_INT1_MAP;
#[doc = "`read()` method returns [pro_timer_int1_map::R](pro_timer_int1_map::R) reader structure"]
impl crate::Readable for PRO_TIMER_INT1_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_timer_int1_map::W](pro_timer_int1_map::W) writer structure"]
impl crate::Writable for PRO_TIMER_INT1_MAP {}
#[doc = "DPORT_PRO_TIMER_INT1_MAP_REG"]
pub mod pro_timer_int1_map;
#[doc = "DPORT_PRO_TIMER_INT2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_timer_int2_map](pro_timer_int2_map) module"]
pub type PRO_TIMER_INT2_MAP = crate::Reg<u32, _PRO_TIMER_INT2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TIMER_INT2_MAP;
#[doc = "`read()` method returns [pro_timer_int2_map::R](pro_timer_int2_map::R) reader structure"]
impl crate::Readable for PRO_TIMER_INT2_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_timer_int2_map::W](pro_timer_int2_map::W) writer structure"]
impl crate::Writable for PRO_TIMER_INT2_MAP {}
#[doc = "DPORT_PRO_TIMER_INT2_MAP_REG"]
pub mod pro_timer_int2_map;
#[doc = "DPORT_PRO_TG_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_t0_edge_int_map](pro_tg_t0_edge_int_map) module"]
pub type PRO_TG_T0_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG_T0_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_T0_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg_t0_edge_int_map::R](pro_tg_t0_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_T0_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_t0_edge_int_map::W](pro_tg_t0_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_T0_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG_T0_EDGE_INT_MAP_REG"]
pub mod pro_tg_t0_edge_int_map;
#[doc = "DPORT_PRO_TG_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_t1_edge_int_map](pro_tg_t1_edge_int_map) module"]
pub type PRO_TG_T1_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG_T1_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_T1_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg_t1_edge_int_map::R](pro_tg_t1_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_T1_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_t1_edge_int_map::W](pro_tg_t1_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_T1_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG_T1_EDGE_INT_MAP_REG"]
pub mod pro_tg_t1_edge_int_map;
#[doc = "DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_wdt_edge_int_map](pro_tg_wdt_edge_int_map) module"]
pub type PRO_TG_WDT_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG_WDT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_WDT_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg_wdt_edge_int_map::R](pro_tg_wdt_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_WDT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_wdt_edge_int_map::W](pro_tg_wdt_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_WDT_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG_WDT_EDGE_INT_MAP_REG"]
pub mod pro_tg_wdt_edge_int_map;
#[doc = "DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg_lact_edge_int_map](pro_tg_lact_edge_int_map) module"]
pub type PRO_TG_LACT_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG_LACT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG_LACT_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg_lact_edge_int_map::R](pro_tg_lact_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG_LACT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg_lact_edge_int_map::W](pro_tg_lact_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG_LACT_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG_LACT_EDGE_INT_MAP_REG"]
pub mod pro_tg_lact_edge_int_map;
#[doc = "DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_t0_edge_int_map](pro_tg1_t0_edge_int_map) module"]
pub type PRO_TG1_T0_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG1_T0_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_T0_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_t0_edge_int_map::R](pro_tg1_t0_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_T0_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_t0_edge_int_map::W](pro_tg1_t0_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_T0_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG1_T0_EDGE_INT_MAP_REG"]
pub mod pro_tg1_t0_edge_int_map;
#[doc = "DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_t1_edge_int_map](pro_tg1_t1_edge_int_map) module"]
pub type PRO_TG1_T1_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG1_T1_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_T1_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_t1_edge_int_map::R](pro_tg1_t1_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_T1_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_t1_edge_int_map::W](pro_tg1_t1_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_T1_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG1_T1_EDGE_INT_MAP_REG"]
pub mod pro_tg1_t1_edge_int_map;
#[doc = "DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_wdt_edge_int_map](pro_tg1_wdt_edge_int_map) module"]
pub type PRO_TG1_WDT_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG1_WDT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_WDT_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_wdt_edge_int_map::R](pro_tg1_wdt_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_WDT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_wdt_edge_int_map::W](pro_tg1_wdt_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_WDT_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG1_WDT_EDGE_INT_MAP_REG"]
pub mod pro_tg1_wdt_edge_int_map;
#[doc = "DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_tg1_lact_edge_int_map](pro_tg1_lact_edge_int_map) module"]
pub type PRO_TG1_LACT_EDGE_INT_MAP = crate::Reg<u32, _PRO_TG1_LACT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_TG1_LACT_EDGE_INT_MAP;
#[doc = "`read()` method returns [pro_tg1_lact_edge_int_map::R](pro_tg1_lact_edge_int_map::R) reader structure"]
impl crate::Readable for PRO_TG1_LACT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_tg1_lact_edge_int_map::W](pro_tg1_lact_edge_int_map::W) writer structure"]
impl crate::Writable for PRO_TG1_LACT_EDGE_INT_MAP {}
#[doc = "DPORT_PRO_TG1_LACT_EDGE_INT_MAP_REG"]
pub mod pro_tg1_lact_edge_int_map;
#[doc = "DPORT_PRO_MMU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_mmu_ia_int_map](pro_mmu_ia_int_map) module"]
pub type PRO_MMU_IA_INT_MAP = crate::Reg<u32, _PRO_MMU_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_MMU_IA_INT_MAP;
#[doc = "`read()` method returns [pro_mmu_ia_int_map::R](pro_mmu_ia_int_map::R) reader structure"]
impl crate::Readable for PRO_MMU_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_mmu_ia_int_map::W](pro_mmu_ia_int_map::W) writer structure"]
impl crate::Writable for PRO_MMU_IA_INT_MAP {}
#[doc = "DPORT_PRO_MMU_IA_INT_MAP_REG"]
pub mod pro_mmu_ia_int_map;
#[doc = "DPORT_PRO_MPU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_mpu_ia_int_map](pro_mpu_ia_int_map) module"]
pub type PRO_MPU_IA_INT_MAP = crate::Reg<u32, _PRO_MPU_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_MPU_IA_INT_MAP;
#[doc = "`read()` method returns [pro_mpu_ia_int_map::R](pro_mpu_ia_int_map::R) reader structure"]
impl crate::Readable for PRO_MPU_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_mpu_ia_int_map::W](pro_mpu_ia_int_map::W) writer structure"]
impl crate::Writable for PRO_MPU_IA_INT_MAP {}
#[doc = "DPORT_PRO_MPU_IA_INT_MAP_REG"]
pub mod pro_mpu_ia_int_map;
#[doc = "DPORT_PRO_CACHE_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cache_ia_int_map](pro_cache_ia_int_map) module"]
pub type PRO_CACHE_IA_INT_MAP = crate::Reg<u32, _PRO_CACHE_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CACHE_IA_INT_MAP;
#[doc = "`read()` method returns [pro_cache_ia_int_map::R](pro_cache_ia_int_map::R) reader structure"]
impl crate::Readable for PRO_CACHE_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [pro_cache_ia_int_map::W](pro_cache_ia_int_map::W) writer structure"]
impl crate::Writable for PRO_CACHE_IA_INT_MAP {}
#[doc = "DPORT_PRO_CACHE_IA_INT_MAP_REG"]
pub mod pro_cache_ia_int_map;
#[doc = "DPORT_APP_MAC_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_mac_intr_map](app_mac_intr_map) module"]
pub type APP_MAC_INTR_MAP = crate::Reg<u32, _APP_MAC_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_MAC_INTR_MAP;
#[doc = "`read()` method returns [app_mac_intr_map::R](app_mac_intr_map::R) reader structure"]
impl crate::Readable for APP_MAC_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_mac_intr_map::W](app_mac_intr_map::W) writer structure"]
impl crate::Writable for APP_MAC_INTR_MAP {}
#[doc = "DPORT_APP_MAC_INTR_MAP_REG"]
pub mod app_mac_intr_map;
#[doc = "DPORT_APP_MAC_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_mac_nmi_map](app_mac_nmi_map) module"]
pub type APP_MAC_NMI_MAP = crate::Reg<u32, _APP_MAC_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_MAC_NMI_MAP;
#[doc = "`read()` method returns [app_mac_nmi_map::R](app_mac_nmi_map::R) reader structure"]
impl crate::Readable for APP_MAC_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [app_mac_nmi_map::W](app_mac_nmi_map::W) writer structure"]
impl crate::Writable for APP_MAC_NMI_MAP {}
#[doc = "DPORT_APP_MAC_NMI_MAP_REG"]
pub mod app_mac_nmi_map;
#[doc = "DPORT_APP_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_bb_int_map](app_bb_int_map) module"]
pub type APP_BB_INT_MAP = crate::Reg<u32, _APP_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_BB_INT_MAP;
#[doc = "`read()` method returns [app_bb_int_map::R](app_bb_int_map::R) reader structure"]
impl crate::Readable for APP_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_bb_int_map::W](app_bb_int_map::W) writer structure"]
impl crate::Writable for APP_BB_INT_MAP {}
#[doc = "DPORT_APP_BB_INT_MAP_REG"]
pub mod app_bb_int_map;
#[doc = "DPORT_APP_BT_MAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_bt_mac_int_map](app_bt_mac_int_map) module"]
pub type APP_BT_MAC_INT_MAP = crate::Reg<u32, _APP_BT_MAC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_BT_MAC_INT_MAP;
#[doc = "`read()` method returns [app_bt_mac_int_map::R](app_bt_mac_int_map::R) reader structure"]
impl crate::Readable for APP_BT_MAC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_bt_mac_int_map::W](app_bt_mac_int_map::W) writer structure"]
impl crate::Writable for APP_BT_MAC_INT_MAP {}
#[doc = "DPORT_APP_BT_MAC_INT_MAP_REG"]
pub mod app_bt_mac_int_map;
#[doc = "DPORT_APP_BT_BB_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_bt_bb_int_map](app_bt_bb_int_map) module"]
pub type APP_BT_BB_INT_MAP = crate::Reg<u32, _APP_BT_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_BT_BB_INT_MAP;
#[doc = "`read()` method returns [app_bt_bb_int_map::R](app_bt_bb_int_map::R) reader structure"]
impl crate::Readable for APP_BT_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_bt_bb_int_map::W](app_bt_bb_int_map::W) writer structure"]
impl crate::Writable for APP_BT_BB_INT_MAP {}
#[doc = "DPORT_APP_BT_BB_INT_MAP_REG"]
pub mod app_bt_bb_int_map;
#[doc = "DPORT_APP_BT_BB_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_bt_bb_nmi_map](app_bt_bb_nmi_map) module"]
pub type APP_BT_BB_NMI_MAP = crate::Reg<u32, _APP_BT_BB_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_BT_BB_NMI_MAP;
#[doc = "`read()` method returns [app_bt_bb_nmi_map::R](app_bt_bb_nmi_map::R) reader structure"]
impl crate::Readable for APP_BT_BB_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [app_bt_bb_nmi_map::W](app_bt_bb_nmi_map::W) writer structure"]
impl crate::Writable for APP_BT_BB_NMI_MAP {}
#[doc = "DPORT_APP_BT_BB_NMI_MAP_REG"]
pub mod app_bt_bb_nmi_map;
#[doc = "DPORT_APP_RWBT_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rwbt_irq_map](app_rwbt_irq_map) module"]
pub type APP_RWBT_IRQ_MAP = crate::Reg<u32, _APP_RWBT_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RWBT_IRQ_MAP;
#[doc = "`read()` method returns [app_rwbt_irq_map::R](app_rwbt_irq_map::R) reader structure"]
impl crate::Readable for APP_RWBT_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rwbt_irq_map::W](app_rwbt_irq_map::W) writer structure"]
impl crate::Writable for APP_RWBT_IRQ_MAP {}
#[doc = "DPORT_APP_RWBT_IRQ_MAP_REG"]
pub mod app_rwbt_irq_map;
#[doc = "DPORT_APP_RWBLE_IRQ_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rwble_irq_map](app_rwble_irq_map) module"]
pub type APP_RWBLE_IRQ_MAP = crate::Reg<u32, _APP_RWBLE_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RWBLE_IRQ_MAP;
#[doc = "`read()` method returns [app_rwble_irq_map::R](app_rwble_irq_map::R) reader structure"]
impl crate::Readable for APP_RWBLE_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rwble_irq_map::W](app_rwble_irq_map::W) writer structure"]
impl crate::Writable for APP_RWBLE_IRQ_MAP {}
#[doc = "DPORT_APP_RWBLE_IRQ_MAP_REG"]
pub mod app_rwble_irq_map;
#[doc = "DPORT_APP_RWBT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rwbt_nmi_map](app_rwbt_nmi_map) module"]
pub type APP_RWBT_NMI_MAP = crate::Reg<u32, _APP_RWBT_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RWBT_NMI_MAP;
#[doc = "`read()` method returns [app_rwbt_nmi_map::R](app_rwbt_nmi_map::R) reader structure"]
impl crate::Readable for APP_RWBT_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rwbt_nmi_map::W](app_rwbt_nmi_map::W) writer structure"]
impl crate::Writable for APP_RWBT_NMI_MAP {}
#[doc = "DPORT_APP_RWBT_NMI_MAP_REG"]
pub mod app_rwbt_nmi_map;
#[doc = "DPORT_APP_RWBLE_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rwble_nmi_map](app_rwble_nmi_map) module"]
pub type APP_RWBLE_NMI_MAP = crate::Reg<u32, _APP_RWBLE_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RWBLE_NMI_MAP;
#[doc = "`read()` method returns [app_rwble_nmi_map::R](app_rwble_nmi_map::R) reader structure"]
impl crate::Readable for APP_RWBLE_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rwble_nmi_map::W](app_rwble_nmi_map::W) writer structure"]
impl crate::Writable for APP_RWBLE_NMI_MAP {}
#[doc = "DPORT_APP_RWBLE_NMI_MAP_REG"]
pub mod app_rwble_nmi_map;
#[doc = "DPORT_APP_SLC0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_slc0_intr_map](app_slc0_intr_map) module"]
pub type APP_SLC0_INTR_MAP = crate::Reg<u32, _APP_SLC0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SLC0_INTR_MAP;
#[doc = "`read()` method returns [app_slc0_intr_map::R](app_slc0_intr_map::R) reader structure"]
impl crate::Readable for APP_SLC0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_slc0_intr_map::W](app_slc0_intr_map::W) writer structure"]
impl crate::Writable for APP_SLC0_INTR_MAP {}
#[doc = "DPORT_APP_SLC0_INTR_MAP_REG"]
pub mod app_slc0_intr_map;
#[doc = "DPORT_APP_SLC1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_slc1_intr_map](app_slc1_intr_map) module"]
pub type APP_SLC1_INTR_MAP = crate::Reg<u32, _APP_SLC1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SLC1_INTR_MAP;
#[doc = "`read()` method returns [app_slc1_intr_map::R](app_slc1_intr_map::R) reader structure"]
impl crate::Readable for APP_SLC1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_slc1_intr_map::W](app_slc1_intr_map::W) writer structure"]
impl crate::Writable for APP_SLC1_INTR_MAP {}
#[doc = "DPORT_APP_SLC1_INTR_MAP_REG"]
pub mod app_slc1_intr_map;
#[doc = "DPORT_APP_UHCI0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_uhci0_intr_map](app_uhci0_intr_map) module"]
pub type APP_UHCI0_INTR_MAP = crate::Reg<u32, _APP_UHCI0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_UHCI0_INTR_MAP;
#[doc = "`read()` method returns [app_uhci0_intr_map::R](app_uhci0_intr_map::R) reader structure"]
impl crate::Readable for APP_UHCI0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_uhci0_intr_map::W](app_uhci0_intr_map::W) writer structure"]
impl crate::Writable for APP_UHCI0_INTR_MAP {}
#[doc = "DPORT_APP_UHCI0_INTR_MAP_REG"]
pub mod app_uhci0_intr_map;
#[doc = "DPORT_APP_UHCI1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_uhci1_intr_map](app_uhci1_intr_map) module"]
pub type APP_UHCI1_INTR_MAP = crate::Reg<u32, _APP_UHCI1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_UHCI1_INTR_MAP;
#[doc = "`read()` method returns [app_uhci1_intr_map::R](app_uhci1_intr_map::R) reader structure"]
impl crate::Readable for APP_UHCI1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_uhci1_intr_map::W](app_uhci1_intr_map::W) writer structure"]
impl crate::Writable for APP_UHCI1_INTR_MAP {}
#[doc = "DPORT_APP_UHCI1_INTR_MAP_REG"]
pub mod app_uhci1_intr_map;
#[doc = "DPORT_APP_TG_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_t0_level_int_map](app_tg_t0_level_int_map) module"]
pub type APP_TG_T0_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG_T0_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_T0_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg_t0_level_int_map::R](app_tg_t0_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG_T0_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_t0_level_int_map::W](app_tg_t0_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG_T0_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG_T0_LEVEL_INT_MAP_REG"]
pub mod app_tg_t0_level_int_map;
#[doc = "DPORT_APP_TG_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_t1_level_int_map](app_tg_t1_level_int_map) module"]
pub type APP_TG_T1_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG_T1_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_T1_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg_t1_level_int_map::R](app_tg_t1_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG_T1_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_t1_level_int_map::W](app_tg_t1_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG_T1_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG_T1_LEVEL_INT_MAP_REG"]
pub mod app_tg_t1_level_int_map;
#[doc = "DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_wdt_level_int_map](app_tg_wdt_level_int_map) module"]
pub type APP_TG_WDT_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG_WDT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_WDT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg_wdt_level_int_map::R](app_tg_wdt_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG_WDT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_wdt_level_int_map::W](app_tg_wdt_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG_WDT_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG_WDT_LEVEL_INT_MAP_REG"]
pub mod app_tg_wdt_level_int_map;
#[doc = "DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_lact_level_int_map](app_tg_lact_level_int_map) module"]
pub type APP_TG_LACT_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG_LACT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_LACT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg_lact_level_int_map::R](app_tg_lact_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG_LACT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_lact_level_int_map::W](app_tg_lact_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG_LACT_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG_LACT_LEVEL_INT_MAP_REG"]
pub mod app_tg_lact_level_int_map;
#[doc = "DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_t0_level_int_map](app_tg1_t0_level_int_map) module"]
pub type APP_TG1_T0_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG1_T0_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_T0_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg1_t0_level_int_map::R](app_tg1_t0_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_T0_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_t0_level_int_map::W](app_tg1_t0_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_T0_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG1_T0_LEVEL_INT_MAP_REG"]
pub mod app_tg1_t0_level_int_map;
#[doc = "DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_t1_level_int_map](app_tg1_t1_level_int_map) module"]
pub type APP_TG1_T1_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG1_T1_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_T1_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg1_t1_level_int_map::R](app_tg1_t1_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_T1_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_t1_level_int_map::W](app_tg1_t1_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_T1_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG1_T1_LEVEL_INT_MAP_REG"]
pub mod app_tg1_t1_level_int_map;
#[doc = "DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_wdt_level_int_map](app_tg1_wdt_level_int_map) module"]
pub type APP_TG1_WDT_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG1_WDT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_WDT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg1_wdt_level_int_map::R](app_tg1_wdt_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_WDT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_wdt_level_int_map::W](app_tg1_wdt_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_WDT_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG1_WDT_LEVEL_INT_MAP_REG"]
pub mod app_tg1_wdt_level_int_map;
#[doc = "DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_lact_level_int_map](app_tg1_lact_level_int_map) module"]
pub type APP_TG1_LACT_LEVEL_INT_MAP = crate::Reg<u32, _APP_TG1_LACT_LEVEL_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_LACT_LEVEL_INT_MAP;
#[doc = "`read()` method returns [app_tg1_lact_level_int_map::R](app_tg1_lact_level_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_LACT_LEVEL_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_lact_level_int_map::W](app_tg1_lact_level_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_LACT_LEVEL_INT_MAP {}
#[doc = "DPORT_APP_TG1_LACT_LEVEL_INT_MAP_REG"]
pub mod app_tg1_lact_level_int_map;
#[doc = "DPORT_APP_GPIO_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_gpio_interrupt_map](app_gpio_interrupt_map) module"]
pub type APP_GPIO_INTERRUPT_MAP = crate::Reg<u32, _APP_GPIO_INTERRUPT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_GPIO_INTERRUPT_MAP;
#[doc = "`read()` method returns [app_gpio_interrupt_map::R](app_gpio_interrupt_map::R) reader structure"]
impl crate::Readable for APP_GPIO_INTERRUPT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_gpio_interrupt_map::W](app_gpio_interrupt_map::W) writer structure"]
impl crate::Writable for APP_GPIO_INTERRUPT_MAP {}
#[doc = "DPORT_APP_GPIO_INTERRUPT_MAP_REG"]
pub mod app_gpio_interrupt_map;
#[doc = "DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_gpio_interrupt_nmi_map](app_gpio_interrupt_nmi_map) module"]
pub type APP_GPIO_INTERRUPT_NMI_MAP = crate::Reg<u32, _APP_GPIO_INTERRUPT_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_GPIO_INTERRUPT_NMI_MAP;
#[doc = "`read()` method returns [app_gpio_interrupt_nmi_map::R](app_gpio_interrupt_nmi_map::R) reader structure"]
impl crate::Readable for APP_GPIO_INTERRUPT_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [app_gpio_interrupt_nmi_map::W](app_gpio_interrupt_nmi_map::W) writer structure"]
impl crate::Writable for APP_GPIO_INTERRUPT_NMI_MAP {}
#[doc = "DPORT_APP_GPIO_INTERRUPT_NMI_MAP_REG"]
pub mod app_gpio_interrupt_nmi_map;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_intr_from_cpu_0_map](app_cpu_intr_from_cpu_0_map) module"]
pub type APP_CPU_INTR_FROM_CPU_0_MAP = crate::Reg<u32, _APP_CPU_INTR_FROM_CPU_0_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_INTR_FROM_CPU_0_MAP;
#[doc = "`read()` method returns [app_cpu_intr_from_cpu_0_map::R](app_cpu_intr_from_cpu_0_map::R) reader structure"]
impl crate::Readable for APP_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "`write(|w| ..)` method takes [app_cpu_intr_from_cpu_0_map::W](app_cpu_intr_from_cpu_0_map::W) writer structure"]
impl crate::Writable for APP_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_0_MAP_REG"]
pub mod app_cpu_intr_from_cpu_0_map;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_intr_from_cpu_1_map](app_cpu_intr_from_cpu_1_map) module"]
pub type APP_CPU_INTR_FROM_CPU_1_MAP = crate::Reg<u32, _APP_CPU_INTR_FROM_CPU_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_INTR_FROM_CPU_1_MAP;
#[doc = "`read()` method returns [app_cpu_intr_from_cpu_1_map::R](app_cpu_intr_from_cpu_1_map::R) reader structure"]
impl crate::Readable for APP_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "`write(|w| ..)` method takes [app_cpu_intr_from_cpu_1_map::W](app_cpu_intr_from_cpu_1_map::W) writer structure"]
impl crate::Writable for APP_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_1_MAP_REG"]
pub mod app_cpu_intr_from_cpu_1_map;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_intr_from_cpu_2_map](app_cpu_intr_from_cpu_2_map) module"]
pub type APP_CPU_INTR_FROM_CPU_2_MAP = crate::Reg<u32, _APP_CPU_INTR_FROM_CPU_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_INTR_FROM_CPU_2_MAP;
#[doc = "`read()` method returns [app_cpu_intr_from_cpu_2_map::R](app_cpu_intr_from_cpu_2_map::R) reader structure"]
impl crate::Readable for APP_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "`write(|w| ..)` method takes [app_cpu_intr_from_cpu_2_map::W](app_cpu_intr_from_cpu_2_map::W) writer structure"]
impl crate::Writable for APP_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_2_MAP_REG"]
pub mod app_cpu_intr_from_cpu_2_map;
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_intr_from_cpu_3_map](app_cpu_intr_from_cpu_3_map) module"]
pub type APP_CPU_INTR_FROM_CPU_3_MAP = crate::Reg<u32, _APP_CPU_INTR_FROM_CPU_3_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_INTR_FROM_CPU_3_MAP;
#[doc = "`read()` method returns [app_cpu_intr_from_cpu_3_map::R](app_cpu_intr_from_cpu_3_map::R) reader structure"]
impl crate::Readable for APP_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "`write(|w| ..)` method takes [app_cpu_intr_from_cpu_3_map::W](app_cpu_intr_from_cpu_3_map::W) writer structure"]
impl crate::Writable for APP_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "DPORT_APP_CPU_INTR_FROM_CPU_3_MAP_REG"]
pub mod app_cpu_intr_from_cpu_3_map;
#[doc = "DPORT_APP_SPI_INTR_0_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi_intr_0_map](app_spi_intr_0_map) module"]
pub type APP_SPI_INTR_0_MAP = crate::Reg<u32, _APP_SPI_INTR_0_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI_INTR_0_MAP;
#[doc = "`read()` method returns [app_spi_intr_0_map::R](app_spi_intr_0_map::R) reader structure"]
impl crate::Readable for APP_SPI_INTR_0_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi_intr_0_map::W](app_spi_intr_0_map::W) writer structure"]
impl crate::Writable for APP_SPI_INTR_0_MAP {}
#[doc = "DPORT_APP_SPI_INTR_0_MAP_REG"]
pub mod app_spi_intr_0_map;
#[doc = "DPORT_APP_SPI_INTR_1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi_intr_1_map](app_spi_intr_1_map) module"]
pub type APP_SPI_INTR_1_MAP = crate::Reg<u32, _APP_SPI_INTR_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI_INTR_1_MAP;
#[doc = "`read()` method returns [app_spi_intr_1_map::R](app_spi_intr_1_map::R) reader structure"]
impl crate::Readable for APP_SPI_INTR_1_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi_intr_1_map::W](app_spi_intr_1_map::W) writer structure"]
impl crate::Writable for APP_SPI_INTR_1_MAP {}
#[doc = "DPORT_APP_SPI_INTR_1_MAP_REG"]
pub mod app_spi_intr_1_map;
#[doc = "DPORT_APP_SPI_INTR_2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi_intr_2_map](app_spi_intr_2_map) module"]
pub type APP_SPI_INTR_2_MAP = crate::Reg<u32, _APP_SPI_INTR_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI_INTR_2_MAP;
#[doc = "`read()` method returns [app_spi_intr_2_map::R](app_spi_intr_2_map::R) reader structure"]
impl crate::Readable for APP_SPI_INTR_2_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi_intr_2_map::W](app_spi_intr_2_map::W) writer structure"]
impl crate::Writable for APP_SPI_INTR_2_MAP {}
#[doc = "DPORT_APP_SPI_INTR_2_MAP_REG"]
pub mod app_spi_intr_2_map;
#[doc = "DPORT_APP_SPI_INTR_3_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi_intr_3_map](app_spi_intr_3_map) module"]
pub type APP_SPI_INTR_3_MAP = crate::Reg<u32, _APP_SPI_INTR_3_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI_INTR_3_MAP;
#[doc = "`read()` method returns [app_spi_intr_3_map::R](app_spi_intr_3_map::R) reader structure"]
impl crate::Readable for APP_SPI_INTR_3_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi_intr_3_map::W](app_spi_intr_3_map::W) writer structure"]
impl crate::Writable for APP_SPI_INTR_3_MAP {}
#[doc = "DPORT_APP_SPI_INTR_3_MAP_REG"]
pub mod app_spi_intr_3_map;
#[doc = "DPORT_APP_I2S0_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_i2s0_int_map](app_i2s0_int_map) module"]
pub type APP_I2S0_INT_MAP = crate::Reg<u32, _APP_I2S0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_I2S0_INT_MAP;
#[doc = "`read()` method returns [app_i2s0_int_map::R](app_i2s0_int_map::R) reader structure"]
impl crate::Readable for APP_I2S0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_i2s0_int_map::W](app_i2s0_int_map::W) writer structure"]
impl crate::Writable for APP_I2S0_INT_MAP {}
#[doc = "DPORT_APP_I2S0_INT_MAP_REG"]
pub mod app_i2s0_int_map;
#[doc = "DPORT_APP_I2S1_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_i2s1_int_map](app_i2s1_int_map) module"]
pub type APP_I2S1_INT_MAP = crate::Reg<u32, _APP_I2S1_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_I2S1_INT_MAP;
#[doc = "`read()` method returns [app_i2s1_int_map::R](app_i2s1_int_map::R) reader structure"]
impl crate::Readable for APP_I2S1_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_i2s1_int_map::W](app_i2s1_int_map::W) writer structure"]
impl crate::Writable for APP_I2S1_INT_MAP {}
#[doc = "DPORT_APP_I2S1_INT_MAP_REG"]
pub mod app_i2s1_int_map;
#[doc = "DPORT_APP_UART_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_uart_intr_map](app_uart_intr_map) module"]
pub type APP_UART_INTR_MAP = crate::Reg<u32, _APP_UART_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_UART_INTR_MAP;
#[doc = "`read()` method returns [app_uart_intr_map::R](app_uart_intr_map::R) reader structure"]
impl crate::Readable for APP_UART_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_uart_intr_map::W](app_uart_intr_map::W) writer structure"]
impl crate::Writable for APP_UART_INTR_MAP {}
#[doc = "DPORT_APP_UART_INTR_MAP_REG"]
pub mod app_uart_intr_map;
#[doc = "DPORT_APP_UART1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_uart1_intr_map](app_uart1_intr_map) module"]
pub type APP_UART1_INTR_MAP = crate::Reg<u32, _APP_UART1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_UART1_INTR_MAP;
#[doc = "`read()` method returns [app_uart1_intr_map::R](app_uart1_intr_map::R) reader structure"]
impl crate::Readable for APP_UART1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_uart1_intr_map::W](app_uart1_intr_map::W) writer structure"]
impl crate::Writable for APP_UART1_INTR_MAP {}
#[doc = "DPORT_APP_UART1_INTR_MAP_REG"]
pub mod app_uart1_intr_map;
#[doc = "DPORT_APP_UART2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_uart2_intr_map](app_uart2_intr_map) module"]
pub type APP_UART2_INTR_MAP = crate::Reg<u32, _APP_UART2_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_UART2_INTR_MAP;
#[doc = "`read()` method returns [app_uart2_intr_map::R](app_uart2_intr_map::R) reader structure"]
impl crate::Readable for APP_UART2_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_uart2_intr_map::W](app_uart2_intr_map::W) writer structure"]
impl crate::Writable for APP_UART2_INTR_MAP {}
#[doc = "DPORT_APP_UART2_INTR_MAP_REG"]
pub mod app_uart2_intr_map;
#[doc = "DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_sdio_host_interrupt_map](app_sdio_host_interrupt_map) module"]
pub type APP_SDIO_HOST_INTERRUPT_MAP = crate::Reg<u32, _APP_SDIO_HOST_INTERRUPT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SDIO_HOST_INTERRUPT_MAP;
#[doc = "`read()` method returns [app_sdio_host_interrupt_map::R](app_sdio_host_interrupt_map::R) reader structure"]
impl crate::Readable for APP_SDIO_HOST_INTERRUPT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_sdio_host_interrupt_map::W](app_sdio_host_interrupt_map::W) writer structure"]
impl crate::Writable for APP_SDIO_HOST_INTERRUPT_MAP {}
#[doc = "DPORT_APP_SDIO_HOST_INTERRUPT_MAP_REG"]
pub mod app_sdio_host_interrupt_map;
#[doc = "DPORT_APP_EMAC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_emac_int_map](app_emac_int_map) module"]
pub type APP_EMAC_INT_MAP = crate::Reg<u32, _APP_EMAC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_EMAC_INT_MAP;
#[doc = "`read()` method returns [app_emac_int_map::R](app_emac_int_map::R) reader structure"]
impl crate::Readable for APP_EMAC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_emac_int_map::W](app_emac_int_map::W) writer structure"]
impl crate::Writable for APP_EMAC_INT_MAP {}
#[doc = "DPORT_APP_EMAC_INT_MAP_REG"]
pub mod app_emac_int_map;
#[doc = "DPORT_APP_PWM0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_pwm0_intr_map](app_pwm0_intr_map) module"]
pub type APP_PWM0_INTR_MAP = crate::Reg<u32, _APP_PWM0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_PWM0_INTR_MAP;
#[doc = "`read()` method returns [app_pwm0_intr_map::R](app_pwm0_intr_map::R) reader structure"]
impl crate::Readable for APP_PWM0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_pwm0_intr_map::W](app_pwm0_intr_map::W) writer structure"]
impl crate::Writable for APP_PWM0_INTR_MAP {}
#[doc = "DPORT_APP_PWM0_INTR_MAP_REG"]
pub mod app_pwm0_intr_map;
#[doc = "DPORT_APP_PWM1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_pwm1_intr_map](app_pwm1_intr_map) module"]
pub type APP_PWM1_INTR_MAP = crate::Reg<u32, _APP_PWM1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_PWM1_INTR_MAP;
#[doc = "`read()` method returns [app_pwm1_intr_map::R](app_pwm1_intr_map::R) reader structure"]
impl crate::Readable for APP_PWM1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_pwm1_intr_map::W](app_pwm1_intr_map::W) writer structure"]
impl crate::Writable for APP_PWM1_INTR_MAP {}
#[doc = "DPORT_APP_PWM1_INTR_MAP_REG"]
pub mod app_pwm1_intr_map;
#[doc = "DPORT_APP_PWM2_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_pwm2_intr_map](app_pwm2_intr_map) module"]
pub type APP_PWM2_INTR_MAP = crate::Reg<u32, _APP_PWM2_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_PWM2_INTR_MAP;
#[doc = "`read()` method returns [app_pwm2_intr_map::R](app_pwm2_intr_map::R) reader structure"]
impl crate::Readable for APP_PWM2_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_pwm2_intr_map::W](app_pwm2_intr_map::W) writer structure"]
impl crate::Writable for APP_PWM2_INTR_MAP {}
#[doc = "DPORT_APP_PWM2_INTR_MAP_REG"]
pub mod app_pwm2_intr_map;
#[doc = "DPORT_APP_PWM3_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_pwm3_intr_map](app_pwm3_intr_map) module"]
pub type APP_PWM3_INTR_MAP = crate::Reg<u32, _APP_PWM3_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_PWM3_INTR_MAP;
#[doc = "`read()` method returns [app_pwm3_intr_map::R](app_pwm3_intr_map::R) reader structure"]
impl crate::Readable for APP_PWM3_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_pwm3_intr_map::W](app_pwm3_intr_map::W) writer structure"]
impl crate::Writable for APP_PWM3_INTR_MAP {}
#[doc = "DPORT_APP_PWM3_INTR_MAP_REG"]
pub mod app_pwm3_intr_map;
#[doc = "DPORT_APP_LEDC_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_ledc_int_map](app_ledc_int_map) module"]
pub type APP_LEDC_INT_MAP = crate::Reg<u32, _APP_LEDC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_LEDC_INT_MAP;
#[doc = "`read()` method returns [app_ledc_int_map::R](app_ledc_int_map::R) reader structure"]
impl crate::Readable for APP_LEDC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_ledc_int_map::W](app_ledc_int_map::W) writer structure"]
impl crate::Writable for APP_LEDC_INT_MAP {}
#[doc = "DPORT_APP_LEDC_INT_MAP_REG"]
pub mod app_ledc_int_map;
#[doc = "DPORT_APP_EFUSE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_efuse_int_map](app_efuse_int_map) module"]
pub type APP_EFUSE_INT_MAP = crate::Reg<u32, _APP_EFUSE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_EFUSE_INT_MAP;
#[doc = "`read()` method returns [app_efuse_int_map::R](app_efuse_int_map::R) reader structure"]
impl crate::Readable for APP_EFUSE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_efuse_int_map::W](app_efuse_int_map::W) writer structure"]
impl crate::Writable for APP_EFUSE_INT_MAP {}
#[doc = "DPORT_APP_EFUSE_INT_MAP_REG"]
pub mod app_efuse_int_map;
#[doc = "DPORT_APP_CAN_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_can_int_map](app_can_int_map) module"]
pub type APP_CAN_INT_MAP = crate::Reg<u32, _APP_CAN_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CAN_INT_MAP;
#[doc = "`read()` method returns [app_can_int_map::R](app_can_int_map::R) reader structure"]
impl crate::Readable for APP_CAN_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_can_int_map::W](app_can_int_map::W) writer structure"]
impl crate::Writable for APP_CAN_INT_MAP {}
#[doc = "DPORT_APP_CAN_INT_MAP_REG"]
pub mod app_can_int_map;
#[doc = "DPORT_APP_RTC_CORE_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rtc_core_intr_map](app_rtc_core_intr_map) module"]
pub type APP_RTC_CORE_INTR_MAP = crate::Reg<u32, _APP_RTC_CORE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RTC_CORE_INTR_MAP;
#[doc = "`read()` method returns [app_rtc_core_intr_map::R](app_rtc_core_intr_map::R) reader structure"]
impl crate::Readable for APP_RTC_CORE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rtc_core_intr_map::W](app_rtc_core_intr_map::W) writer structure"]
impl crate::Writable for APP_RTC_CORE_INTR_MAP {}
#[doc = "DPORT_APP_RTC_CORE_INTR_MAP_REG"]
pub mod app_rtc_core_intr_map;
#[doc = "DPORT_APP_RMT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rmt_intr_map](app_rmt_intr_map) module"]
pub type APP_RMT_INTR_MAP = crate::Reg<u32, _APP_RMT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RMT_INTR_MAP;
#[doc = "`read()` method returns [app_rmt_intr_map::R](app_rmt_intr_map::R) reader structure"]
impl crate::Readable for APP_RMT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rmt_intr_map::W](app_rmt_intr_map::W) writer structure"]
impl crate::Writable for APP_RMT_INTR_MAP {}
#[doc = "DPORT_APP_RMT_INTR_MAP_REG"]
pub mod app_rmt_intr_map;
#[doc = "DPORT_APP_PCNT_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_pcnt_intr_map](app_pcnt_intr_map) module"]
pub type APP_PCNT_INTR_MAP = crate::Reg<u32, _APP_PCNT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_PCNT_INTR_MAP;
#[doc = "`read()` method returns [app_pcnt_intr_map::R](app_pcnt_intr_map::R) reader structure"]
impl crate::Readable for APP_PCNT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_pcnt_intr_map::W](app_pcnt_intr_map::W) writer structure"]
impl crate::Writable for APP_PCNT_INTR_MAP {}
#[doc = "DPORT_APP_PCNT_INTR_MAP_REG"]
pub mod app_pcnt_intr_map;
#[doc = "DPORT_APP_I2C_EXT0_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_i2c_ext0_intr_map](app_i2c_ext0_intr_map) module"]
pub type APP_I2C_EXT0_INTR_MAP = crate::Reg<u32, _APP_I2C_EXT0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_I2C_EXT0_INTR_MAP;
#[doc = "`read()` method returns [app_i2c_ext0_intr_map::R](app_i2c_ext0_intr_map::R) reader structure"]
impl crate::Readable for APP_I2C_EXT0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_i2c_ext0_intr_map::W](app_i2c_ext0_intr_map::W) writer structure"]
impl crate::Writable for APP_I2C_EXT0_INTR_MAP {}
#[doc = "DPORT_APP_I2C_EXT0_INTR_MAP_REG"]
pub mod app_i2c_ext0_intr_map;
#[doc = "DPORT_APP_I2C_EXT1_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_i2c_ext1_intr_map](app_i2c_ext1_intr_map) module"]
pub type APP_I2C_EXT1_INTR_MAP = crate::Reg<u32, _APP_I2C_EXT1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_I2C_EXT1_INTR_MAP;
#[doc = "`read()` method returns [app_i2c_ext1_intr_map::R](app_i2c_ext1_intr_map::R) reader structure"]
impl crate::Readable for APP_I2C_EXT1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_i2c_ext1_intr_map::W](app_i2c_ext1_intr_map::W) writer structure"]
impl crate::Writable for APP_I2C_EXT1_INTR_MAP {}
#[doc = "DPORT_APP_I2C_EXT1_INTR_MAP_REG"]
pub mod app_i2c_ext1_intr_map;
#[doc = "DPORT_APP_RSA_INTR_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_rsa_intr_map](app_rsa_intr_map) module"]
pub type APP_RSA_INTR_MAP = crate::Reg<u32, _APP_RSA_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_RSA_INTR_MAP;
#[doc = "`read()` method returns [app_rsa_intr_map::R](app_rsa_intr_map::R) reader structure"]
impl crate::Readable for APP_RSA_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [app_rsa_intr_map::W](app_rsa_intr_map::W) writer structure"]
impl crate::Writable for APP_RSA_INTR_MAP {}
#[doc = "DPORT_APP_RSA_INTR_MAP_REG"]
pub mod app_rsa_intr_map;
#[doc = "DPORT_APP_SPI1_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi1_dma_int_map](app_spi1_dma_int_map) module"]
pub type APP_SPI1_DMA_INT_MAP = crate::Reg<u32, _APP_SPI1_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI1_DMA_INT_MAP;
#[doc = "`read()` method returns [app_spi1_dma_int_map::R](app_spi1_dma_int_map::R) reader structure"]
impl crate::Readable for APP_SPI1_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi1_dma_int_map::W](app_spi1_dma_int_map::W) writer structure"]
impl crate::Writable for APP_SPI1_DMA_INT_MAP {}
#[doc = "DPORT_APP_SPI1_DMA_INT_MAP_REG"]
pub mod app_spi1_dma_int_map;
#[doc = "DPORT_APP_SPI2_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi2_dma_int_map](app_spi2_dma_int_map) module"]
pub type APP_SPI2_DMA_INT_MAP = crate::Reg<u32, _APP_SPI2_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI2_DMA_INT_MAP;
#[doc = "`read()` method returns [app_spi2_dma_int_map::R](app_spi2_dma_int_map::R) reader structure"]
impl crate::Readable for APP_SPI2_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi2_dma_int_map::W](app_spi2_dma_int_map::W) writer structure"]
impl crate::Writable for APP_SPI2_DMA_INT_MAP {}
#[doc = "DPORT_APP_SPI2_DMA_INT_MAP_REG"]
pub mod app_spi2_dma_int_map;
#[doc = "DPORT_APP_SPI3_DMA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_spi3_dma_int_map](app_spi3_dma_int_map) module"]
pub type APP_SPI3_DMA_INT_MAP = crate::Reg<u32, _APP_SPI3_DMA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_SPI3_DMA_INT_MAP;
#[doc = "`read()` method returns [app_spi3_dma_int_map::R](app_spi3_dma_int_map::R) reader structure"]
impl crate::Readable for APP_SPI3_DMA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_spi3_dma_int_map::W](app_spi3_dma_int_map::W) writer structure"]
impl crate::Writable for APP_SPI3_DMA_INT_MAP {}
#[doc = "DPORT_APP_SPI3_DMA_INT_MAP_REG"]
pub mod app_spi3_dma_int_map;
#[doc = "DPORT_APP_WDG_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_wdg_int_map](app_wdg_int_map) module"]
pub type APP_WDG_INT_MAP = crate::Reg<u32, _APP_WDG_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_WDG_INT_MAP;
#[doc = "`read()` method returns [app_wdg_int_map::R](app_wdg_int_map::R) reader structure"]
impl crate::Readable for APP_WDG_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_wdg_int_map::W](app_wdg_int_map::W) writer structure"]
impl crate::Writable for APP_WDG_INT_MAP {}
#[doc = "DPORT_APP_WDG_INT_MAP_REG"]
pub mod app_wdg_int_map;
#[doc = "DPORT_APP_TIMER_INT1_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_timer_int1_map](app_timer_int1_map) module"]
pub type APP_TIMER_INT1_MAP = crate::Reg<u32, _APP_TIMER_INT1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TIMER_INT1_MAP;
#[doc = "`read()` method returns [app_timer_int1_map::R](app_timer_int1_map::R) reader structure"]
impl crate::Readable for APP_TIMER_INT1_MAP {}
#[doc = "`write(|w| ..)` method takes [app_timer_int1_map::W](app_timer_int1_map::W) writer structure"]
impl crate::Writable for APP_TIMER_INT1_MAP {}
#[doc = "DPORT_APP_TIMER_INT1_MAP_REG"]
pub mod app_timer_int1_map;
#[doc = "DPORT_APP_TIMER_INT2_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_timer_int2_map](app_timer_int2_map) module"]
pub type APP_TIMER_INT2_MAP = crate::Reg<u32, _APP_TIMER_INT2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TIMER_INT2_MAP;
#[doc = "`read()` method returns [app_timer_int2_map::R](app_timer_int2_map::R) reader structure"]
impl crate::Readable for APP_TIMER_INT2_MAP {}
#[doc = "`write(|w| ..)` method takes [app_timer_int2_map::W](app_timer_int2_map::W) writer structure"]
impl crate::Writable for APP_TIMER_INT2_MAP {}
#[doc = "DPORT_APP_TIMER_INT2_MAP_REG"]
pub mod app_timer_int2_map;
#[doc = "DPORT_APP_TG_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_t0_edge_int_map](app_tg_t0_edge_int_map) module"]
pub type APP_TG_T0_EDGE_INT_MAP = crate::Reg<u32, _APP_TG_T0_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_T0_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg_t0_edge_int_map::R](app_tg_t0_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG_T0_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_t0_edge_int_map::W](app_tg_t0_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG_T0_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG_T0_EDGE_INT_MAP_REG"]
pub mod app_tg_t0_edge_int_map;
#[doc = "DPORT_APP_TG_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_t1_edge_int_map](app_tg_t1_edge_int_map) module"]
pub type APP_TG_T1_EDGE_INT_MAP = crate::Reg<u32, _APP_TG_T1_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_T1_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg_t1_edge_int_map::R](app_tg_t1_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG_T1_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_t1_edge_int_map::W](app_tg_t1_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG_T1_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG_T1_EDGE_INT_MAP_REG"]
pub mod app_tg_t1_edge_int_map;
#[doc = "DPORT_APP_TG_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_wdt_edge_int_map](app_tg_wdt_edge_int_map) module"]
pub type APP_TG_WDT_EDGE_INT_MAP = crate::Reg<u32, _APP_TG_WDT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_WDT_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg_wdt_edge_int_map::R](app_tg_wdt_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG_WDT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_wdt_edge_int_map::W](app_tg_wdt_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG_WDT_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG_WDT_EDGE_INT_MAP_REG"]
pub mod app_tg_wdt_edge_int_map;
#[doc = "DPORT_APP_TG_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg_lact_edge_int_map](app_tg_lact_edge_int_map) module"]
pub type APP_TG_LACT_EDGE_INT_MAP = crate::Reg<u32, _APP_TG_LACT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG_LACT_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg_lact_edge_int_map::R](app_tg_lact_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG_LACT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg_lact_edge_int_map::W](app_tg_lact_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG_LACT_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG_LACT_EDGE_INT_MAP_REG"]
pub mod app_tg_lact_edge_int_map;
#[doc = "DPORT_APP_TG1_T0_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_t0_edge_int_map](app_tg1_t0_edge_int_map) module"]
pub type APP_TG1_T0_EDGE_INT_MAP = crate::Reg<u32, _APP_TG1_T0_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_T0_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg1_t0_edge_int_map::R](app_tg1_t0_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_T0_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_t0_edge_int_map::W](app_tg1_t0_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_T0_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG1_T0_EDGE_INT_MAP_REG"]
pub mod app_tg1_t0_edge_int_map;
#[doc = "DPORT_APP_TG1_T1_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_t1_edge_int_map](app_tg1_t1_edge_int_map) module"]
pub type APP_TG1_T1_EDGE_INT_MAP = crate::Reg<u32, _APP_TG1_T1_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_T1_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg1_t1_edge_int_map::R](app_tg1_t1_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_T1_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_t1_edge_int_map::W](app_tg1_t1_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_T1_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG1_T1_EDGE_INT_MAP_REG"]
pub mod app_tg1_t1_edge_int_map;
#[doc = "DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_wdt_edge_int_map](app_tg1_wdt_edge_int_map) module"]
pub type APP_TG1_WDT_EDGE_INT_MAP = crate::Reg<u32, _APP_TG1_WDT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_WDT_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg1_wdt_edge_int_map::R](app_tg1_wdt_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_WDT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_wdt_edge_int_map::W](app_tg1_wdt_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_WDT_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG1_WDT_EDGE_INT_MAP_REG"]
pub mod app_tg1_wdt_edge_int_map;
#[doc = "DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_tg1_lact_edge_int_map](app_tg1_lact_edge_int_map) module"]
pub type APP_TG1_LACT_EDGE_INT_MAP = crate::Reg<u32, _APP_TG1_LACT_EDGE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_TG1_LACT_EDGE_INT_MAP;
#[doc = "`read()` method returns [app_tg1_lact_edge_int_map::R](app_tg1_lact_edge_int_map::R) reader structure"]
impl crate::Readable for APP_TG1_LACT_EDGE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_tg1_lact_edge_int_map::W](app_tg1_lact_edge_int_map::W) writer structure"]
impl crate::Writable for APP_TG1_LACT_EDGE_INT_MAP {}
#[doc = "DPORT_APP_TG1_LACT_EDGE_INT_MAP_REG"]
pub mod app_tg1_lact_edge_int_map;
#[doc = "DPORT_APP_MMU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_mmu_ia_int_map](app_mmu_ia_int_map) module"]
pub type APP_MMU_IA_INT_MAP = crate::Reg<u32, _APP_MMU_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_MMU_IA_INT_MAP;
#[doc = "`read()` method returns [app_mmu_ia_int_map::R](app_mmu_ia_int_map::R) reader structure"]
impl crate::Readable for APP_MMU_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_mmu_ia_int_map::W](app_mmu_ia_int_map::W) writer structure"]
impl crate::Writable for APP_MMU_IA_INT_MAP {}
#[doc = "DPORT_APP_MMU_IA_INT_MAP_REG"]
pub mod app_mmu_ia_int_map;
#[doc = "DPORT_APP_MPU_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_mpu_ia_int_map](app_mpu_ia_int_map) module"]
pub type APP_MPU_IA_INT_MAP = crate::Reg<u32, _APP_MPU_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_MPU_IA_INT_MAP;
#[doc = "`read()` method returns [app_mpu_ia_int_map::R](app_mpu_ia_int_map::R) reader structure"]
impl crate::Readable for APP_MPU_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_mpu_ia_int_map::W](app_mpu_ia_int_map::W) writer structure"]
impl crate::Writable for APP_MPU_IA_INT_MAP {}
#[doc = "DPORT_APP_MPU_IA_INT_MAP_REG"]
pub mod app_mpu_ia_int_map;
#[doc = "DPORT_APP_CACHE_IA_INT_MAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cache_ia_int_map](app_cache_ia_int_map) module"]
pub type APP_CACHE_IA_INT_MAP = crate::Reg<u32, _APP_CACHE_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CACHE_IA_INT_MAP;
#[doc = "`read()` method returns [app_cache_ia_int_map::R](app_cache_ia_int_map::R) reader structure"]
impl crate::Readable for APP_CACHE_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [app_cache_ia_int_map::W](app_cache_ia_int_map::W) writer structure"]
impl crate::Writable for APP_CACHE_IA_INT_MAP {}
#[doc = "DPORT_APP_CACHE_IA_INT_MAP_REG"]
pub mod app_cache_ia_int_map;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_uart](ahblite_mpu_table_uart) module"]
pub type AHBLITE_MPU_TABLE_UART = crate::Reg<u32, _AHBLITE_MPU_TABLE_UART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_UART;
#[doc = "`read()` method returns [ahblite_mpu_table_uart::R](ahblite_mpu_table_uart::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_UART {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_uart::W](ahblite_mpu_table_uart::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_UART {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART_REG"]
pub mod ahblite_mpu_table_uart;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_spi1](ahblite_mpu_table_spi1) module"]
pub type AHBLITE_MPU_TABLE_SPI1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_SPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SPI1;
#[doc = "`read()` method returns [ahblite_mpu_table_spi1::R](ahblite_mpu_table_spi1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SPI1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_spi1::W](ahblite_mpu_table_spi1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SPI1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI1_REG"]
pub mod ahblite_mpu_table_spi1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_spi0](ahblite_mpu_table_spi0) module"]
pub type AHBLITE_MPU_TABLE_SPI0 = crate::Reg<u32, _AHBLITE_MPU_TABLE_SPI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SPI0;
#[doc = "`read()` method returns [ahblite_mpu_table_spi0::R](ahblite_mpu_table_spi0::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SPI0 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_spi0::W](ahblite_mpu_table_spi0::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SPI0 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI0_REG"]
pub mod ahblite_mpu_table_spi0;
#[doc = "DPORT_AHBLITE_MPU_TABLE_GPIO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_gpio](ahblite_mpu_table_gpio) module"]
pub type AHBLITE_MPU_TABLE_GPIO = crate::Reg<u32, _AHBLITE_MPU_TABLE_GPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_GPIO;
#[doc = "`read()` method returns [ahblite_mpu_table_gpio::R](ahblite_mpu_table_gpio::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_GPIO {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_gpio::W](ahblite_mpu_table_gpio::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_GPIO {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_GPIO_REG"]
pub mod ahblite_mpu_table_gpio;
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_fe2](ahblite_mpu_table_fe2) module"]
pub type AHBLITE_MPU_TABLE_FE2 = crate::Reg<u32, _AHBLITE_MPU_TABLE_FE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_FE2;
#[doc = "`read()` method returns [ahblite_mpu_table_fe2::R](ahblite_mpu_table_fe2::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_FE2 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_fe2::W](ahblite_mpu_table_fe2::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_FE2 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE2_REG"]
pub mod ahblite_mpu_table_fe2;
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_fe](ahblite_mpu_table_fe) module"]
pub type AHBLITE_MPU_TABLE_FE = crate::Reg<u32, _AHBLITE_MPU_TABLE_FE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_FE;
#[doc = "`read()` method returns [ahblite_mpu_table_fe::R](ahblite_mpu_table_fe::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_FE {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_fe::W](ahblite_mpu_table_fe::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_FE {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_FE_REG"]
pub mod ahblite_mpu_table_fe;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMER_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_timer](ahblite_mpu_table_timer) module"]
pub type AHBLITE_MPU_TABLE_TIMER = crate::Reg<u32, _AHBLITE_MPU_TABLE_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_TIMER;
#[doc = "`read()` method returns [ahblite_mpu_table_timer::R](ahblite_mpu_table_timer::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_TIMER {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_timer::W](ahblite_mpu_table_timer::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_TIMER {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMER_REG"]
pub mod ahblite_mpu_table_timer;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RTC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_rtc](ahblite_mpu_table_rtc) module"]
pub type AHBLITE_MPU_TABLE_RTC = crate::Reg<u32, _AHBLITE_MPU_TABLE_RTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_RTC;
#[doc = "`read()` method returns [ahblite_mpu_table_rtc::R](ahblite_mpu_table_rtc::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_RTC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_rtc::W](ahblite_mpu_table_rtc::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_RTC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RTC_REG"]
pub mod ahblite_mpu_table_rtc;
#[doc = "DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_io_mux](ahblite_mpu_table_io_mux) module"]
pub type AHBLITE_MPU_TABLE_IO_MUX = crate::Reg<u32, _AHBLITE_MPU_TABLE_IO_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_IO_MUX;
#[doc = "`read()` method returns [ahblite_mpu_table_io_mux::R](ahblite_mpu_table_io_mux::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_IO_MUX {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_io_mux::W](ahblite_mpu_table_io_mux::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_IO_MUX {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_IO_MUX_REG"]
pub mod ahblite_mpu_table_io_mux;
#[doc = "DPORT_AHBLITE_MPU_TABLE_WDG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_wdg](ahblite_mpu_table_wdg) module"]
pub type AHBLITE_MPU_TABLE_WDG = crate::Reg<u32, _AHBLITE_MPU_TABLE_WDG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_WDG;
#[doc = "`read()` method returns [ahblite_mpu_table_wdg::R](ahblite_mpu_table_wdg::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_WDG {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_wdg::W](ahblite_mpu_table_wdg::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_WDG {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_WDG_REG"]
pub mod ahblite_mpu_table_wdg;
#[doc = "DPORT_AHBLITE_MPU_TABLE_HINF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_hinf](ahblite_mpu_table_hinf) module"]
pub type AHBLITE_MPU_TABLE_HINF = crate::Reg<u32, _AHBLITE_MPU_TABLE_HINF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_HINF;
#[doc = "`read()` method returns [ahblite_mpu_table_hinf::R](ahblite_mpu_table_hinf::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_HINF {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_hinf::W](ahblite_mpu_table_hinf::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_HINF {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_HINF_REG"]
pub mod ahblite_mpu_table_hinf;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_uhci1](ahblite_mpu_table_uhci1) module"]
pub type AHBLITE_MPU_TABLE_UHCI1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_UHCI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_UHCI1;
#[doc = "`read()` method returns [ahblite_mpu_table_uhci1::R](ahblite_mpu_table_uhci1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_UHCI1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_uhci1::W](ahblite_mpu_table_uhci1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_UHCI1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI1_REG"]
pub mod ahblite_mpu_table_uhci1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_MISC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_misc](ahblite_mpu_table_misc) module"]
pub type AHBLITE_MPU_TABLE_MISC = crate::Reg<u32, _AHBLITE_MPU_TABLE_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_MISC;
#[doc = "`read()` method returns [ahblite_mpu_table_misc::R](ahblite_mpu_table_misc::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_MISC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_misc::W](ahblite_mpu_table_misc::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_MISC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_MISC_REG"]
pub mod ahblite_mpu_table_misc;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_i2c](ahblite_mpu_table_i2c) module"]
pub type AHBLITE_MPU_TABLE_I2C = crate::Reg<u32, _AHBLITE_MPU_TABLE_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_I2C;
#[doc = "`read()` method returns [ahblite_mpu_table_i2c::R](ahblite_mpu_table_i2c::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2C {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2c::W](ahblite_mpu_table_i2c::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2C {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_REG"]
pub mod ahblite_mpu_table_i2c;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_i2s0](ahblite_mpu_table_i2s0) module"]
pub type AHBLITE_MPU_TABLE_I2S0 = crate::Reg<u32, _AHBLITE_MPU_TABLE_I2S0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_I2S0;
#[doc = "`read()` method returns [ahblite_mpu_table_i2s0::R](ahblite_mpu_table_i2s0::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2S0 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2s0::W](ahblite_mpu_table_i2s0::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2S0 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S0_REG"]
pub mod ahblite_mpu_table_i2s0;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_uart1](ahblite_mpu_table_uart1) module"]
pub type AHBLITE_MPU_TABLE_UART1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_UART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_UART1;
#[doc = "`read()` method returns [ahblite_mpu_table_uart1::R](ahblite_mpu_table_uart1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_UART1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_uart1::W](ahblite_mpu_table_uart1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_UART1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART1_REG"]
pub mod ahblite_mpu_table_uart1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_bt](ahblite_mpu_table_bt) module"]
pub type AHBLITE_MPU_TABLE_BT = crate::Reg<u32, _AHBLITE_MPU_TABLE_BT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_BT;
#[doc = "`read()` method returns [ahblite_mpu_table_bt::R](ahblite_mpu_table_bt::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_BT {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_bt::W](ahblite_mpu_table_bt::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_BT {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_REG"]
pub mod ahblite_mpu_table_bt;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_bt_buffer](ahblite_mpu_table_bt_buffer) module"]
pub type AHBLITE_MPU_TABLE_BT_BUFFER = crate::Reg<u32, _AHBLITE_MPU_TABLE_BT_BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_BT_BUFFER;
#[doc = "`read()` method returns [ahblite_mpu_table_bt_buffer::R](ahblite_mpu_table_bt_buffer::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_BT_BUFFER {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_bt_buffer::W](ahblite_mpu_table_bt_buffer::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_BT_BUFFER {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BT_BUFFER_REG"]
pub mod ahblite_mpu_table_bt_buffer;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_i2c_ext0](ahblite_mpu_table_i2c_ext0) module"]
pub type AHBLITE_MPU_TABLE_I2C_EXT0 = crate::Reg<u32, _AHBLITE_MPU_TABLE_I2C_EXT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_I2C_EXT0;
#[doc = "`read()` method returns [ahblite_mpu_table_i2c_ext0::R](ahblite_mpu_table_i2c_ext0::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2C_EXT0 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2c_ext0::W](ahblite_mpu_table_i2c_ext0::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2C_EXT0 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT0_REG"]
pub mod ahblite_mpu_table_i2c_ext0;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_uhci0](ahblite_mpu_table_uhci0) module"]
pub type AHBLITE_MPU_TABLE_UHCI0 = crate::Reg<u32, _AHBLITE_MPU_TABLE_UHCI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_UHCI0;
#[doc = "`read()` method returns [ahblite_mpu_table_uhci0::R](ahblite_mpu_table_uhci0::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_UHCI0 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_uhci0::W](ahblite_mpu_table_uhci0::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_UHCI0 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UHCI0_REG"]
pub mod ahblite_mpu_table_uhci0;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_slchost](ahblite_mpu_table_slchost) module"]
pub type AHBLITE_MPU_TABLE_SLCHOST = crate::Reg<u32, _AHBLITE_MPU_TABLE_SLCHOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SLCHOST;
#[doc = "`read()` method returns [ahblite_mpu_table_slchost::R](ahblite_mpu_table_slchost::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SLCHOST {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_slchost::W](ahblite_mpu_table_slchost::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SLCHOST {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLCHOST_REG"]
pub mod ahblite_mpu_table_slchost;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RMT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_rmt](ahblite_mpu_table_rmt) module"]
pub type AHBLITE_MPU_TABLE_RMT = crate::Reg<u32, _AHBLITE_MPU_TABLE_RMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_RMT;
#[doc = "`read()` method returns [ahblite_mpu_table_rmt::R](ahblite_mpu_table_rmt::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_RMT {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_rmt::W](ahblite_mpu_table_rmt::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_RMT {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RMT_REG"]
pub mod ahblite_mpu_table_rmt;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PCNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pcnt](ahblite_mpu_table_pcnt) module"]
pub type AHBLITE_MPU_TABLE_PCNT = crate::Reg<u32, _AHBLITE_MPU_TABLE_PCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PCNT;
#[doc = "`read()` method returns [ahblite_mpu_table_pcnt::R](ahblite_mpu_table_pcnt::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PCNT {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pcnt::W](ahblite_mpu_table_pcnt::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PCNT {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PCNT_REG"]
pub mod ahblite_mpu_table_pcnt;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_slc](ahblite_mpu_table_slc) module"]
pub type AHBLITE_MPU_TABLE_SLC = crate::Reg<u32, _AHBLITE_MPU_TABLE_SLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SLC;
#[doc = "`read()` method returns [ahblite_mpu_table_slc::R](ahblite_mpu_table_slc::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SLC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_slc::W](ahblite_mpu_table_slc::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SLC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SLC_REG"]
pub mod ahblite_mpu_table_slc;
#[doc = "DPORT_AHBLITE_MPU_TABLE_LEDC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_ledc](ahblite_mpu_table_ledc) module"]
pub type AHBLITE_MPU_TABLE_LEDC = crate::Reg<u32, _AHBLITE_MPU_TABLE_LEDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_LEDC;
#[doc = "`read()` method returns [ahblite_mpu_table_ledc::R](ahblite_mpu_table_ledc::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_LEDC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_ledc::W](ahblite_mpu_table_ledc::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_LEDC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_LEDC_REG"]
pub mod ahblite_mpu_table_ledc;
#[doc = "DPORT_AHBLITE_MPU_TABLE_EFUSE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_efuse](ahblite_mpu_table_efuse) module"]
pub type AHBLITE_MPU_TABLE_EFUSE = crate::Reg<u32, _AHBLITE_MPU_TABLE_EFUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_EFUSE;
#[doc = "`read()` method returns [ahblite_mpu_table_efuse::R](ahblite_mpu_table_efuse::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_EFUSE {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_efuse::W](ahblite_mpu_table_efuse::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_EFUSE {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_EFUSE_REG"]
pub mod ahblite_mpu_table_efuse;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_spi_encrypt](ahblite_mpu_table_spi_encrypt) module"]
pub type AHBLITE_MPU_TABLE_SPI_ENCRYPT = crate::Reg<u32, _AHBLITE_MPU_TABLE_SPI_ENCRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SPI_ENCRYPT;
#[doc = "`read()` method returns [ahblite_mpu_table_spi_encrypt::R](ahblite_mpu_table_spi_encrypt::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SPI_ENCRYPT {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_spi_encrypt::W](ahblite_mpu_table_spi_encrypt::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SPI_ENCRYPT {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
pub mod ahblite_mpu_table_spi_encrypt;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BB_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_bb](ahblite_mpu_table_bb) module"]
pub type AHBLITE_MPU_TABLE_BB = crate::Reg<u32, _AHBLITE_MPU_TABLE_BB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_BB;
#[doc = "`read()` method returns [ahblite_mpu_table_bb::R](ahblite_mpu_table_bb::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_BB {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_bb::W](ahblite_mpu_table_bb::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_BB {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BB_REG"]
pub mod ahblite_mpu_table_bb;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pwm0](ahblite_mpu_table_pwm0) module"]
pub type AHBLITE_MPU_TABLE_PWM0 = crate::Reg<u32, _AHBLITE_MPU_TABLE_PWM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PWM0;
#[doc = "`read()` method returns [ahblite_mpu_table_pwm0::R](ahblite_mpu_table_pwm0::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PWM0 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pwm0::W](ahblite_mpu_table_pwm0::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PWM0 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM0_REG"]
pub mod ahblite_mpu_table_pwm0;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_timergroup](ahblite_mpu_table_timergroup) module"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP = crate::Reg<u32, _AHBLITE_MPU_TABLE_TIMERGROUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_TIMERGROUP;
#[doc = "`read()` method returns [ahblite_mpu_table_timergroup::R](ahblite_mpu_table_timergroup::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_TIMERGROUP {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_timergroup::W](ahblite_mpu_table_timergroup::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_TIMERGROUP {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP_REG"]
pub mod ahblite_mpu_table_timergroup;
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_timergroup1](ahblite_mpu_table_timergroup1) module"]
pub type AHBLITE_MPU_TABLE_TIMERGROUP1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_TIMERGROUP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_TIMERGROUP1;
#[doc = "`read()` method returns [ahblite_mpu_table_timergroup1::R](ahblite_mpu_table_timergroup1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_TIMERGROUP1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_timergroup1::W](ahblite_mpu_table_timergroup1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_TIMERGROUP1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_TIMERGROUP1_REG"]
pub mod ahblite_mpu_table_timergroup1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_spi2](ahblite_mpu_table_spi2) module"]
pub type AHBLITE_MPU_TABLE_SPI2 = crate::Reg<u32, _AHBLITE_MPU_TABLE_SPI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SPI2;
#[doc = "`read()` method returns [ahblite_mpu_table_spi2::R](ahblite_mpu_table_spi2::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SPI2 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_spi2::W](ahblite_mpu_table_spi2::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SPI2 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI2_REG"]
pub mod ahblite_mpu_table_spi2;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_spi3](ahblite_mpu_table_spi3) module"]
pub type AHBLITE_MPU_TABLE_SPI3 = crate::Reg<u32, _AHBLITE_MPU_TABLE_SPI3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SPI3;
#[doc = "`read()` method returns [ahblite_mpu_table_spi3::R](ahblite_mpu_table_spi3::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SPI3 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_spi3::W](ahblite_mpu_table_spi3::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SPI3 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SPI3_REG"]
pub mod ahblite_mpu_table_spi3;
#[doc = "DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_apb_ctrl](ahblite_mpu_table_apb_ctrl) module"]
pub type AHBLITE_MPU_TABLE_APB_CTRL = crate::Reg<u32, _AHBLITE_MPU_TABLE_APB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_APB_CTRL;
#[doc = "`read()` method returns [ahblite_mpu_table_apb_ctrl::R](ahblite_mpu_table_apb_ctrl::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_APB_CTRL {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_apb_ctrl::W](ahblite_mpu_table_apb_ctrl::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_APB_CTRL {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_APB_CTRL_REG"]
pub mod ahblite_mpu_table_apb_ctrl;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_i2c_ext1](ahblite_mpu_table_i2c_ext1) module"]
pub type AHBLITE_MPU_TABLE_I2C_EXT1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_I2C_EXT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_I2C_EXT1;
#[doc = "`read()` method returns [ahblite_mpu_table_i2c_ext1::R](ahblite_mpu_table_i2c_ext1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2C_EXT1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2c_ext1::W](ahblite_mpu_table_i2c_ext1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2C_EXT1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2C_EXT1_REG"]
pub mod ahblite_mpu_table_i2c_ext1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_sdio_host](ahblite_mpu_table_sdio_host) module"]
pub type AHBLITE_MPU_TABLE_SDIO_HOST = crate::Reg<u32, _AHBLITE_MPU_TABLE_SDIO_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_SDIO_HOST;
#[doc = "`read()` method returns [ahblite_mpu_table_sdio_host::R](ahblite_mpu_table_sdio_host::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_SDIO_HOST {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_sdio_host::W](ahblite_mpu_table_sdio_host::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_SDIO_HOST {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_SDIO_HOST_REG"]
pub mod ahblite_mpu_table_sdio_host;
#[doc = "DPORT_AHBLITE_MPU_TABLE_EMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_emac](ahblite_mpu_table_emac) module"]
pub type AHBLITE_MPU_TABLE_EMAC = crate::Reg<u32, _AHBLITE_MPU_TABLE_EMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_EMAC;
#[doc = "`read()` method returns [ahblite_mpu_table_emac::R](ahblite_mpu_table_emac::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_EMAC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_emac::W](ahblite_mpu_table_emac::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_EMAC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_EMAC_REG"]
pub mod ahblite_mpu_table_emac;
#[doc = "DPORT_AHBLITE_MPU_TABLE_CAN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_can](ahblite_mpu_table_can) module"]
pub type AHBLITE_MPU_TABLE_CAN = crate::Reg<u32, _AHBLITE_MPU_TABLE_CAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_CAN;
#[doc = "`read()` method returns [ahblite_mpu_table_can::R](ahblite_mpu_table_can::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_CAN {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_can::W](ahblite_mpu_table_can::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_CAN {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_CAN_REG"]
pub mod ahblite_mpu_table_can;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pwm1](ahblite_mpu_table_pwm1) module"]
pub type AHBLITE_MPU_TABLE_PWM1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_PWM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PWM1;
#[doc = "`read()` method returns [ahblite_mpu_table_pwm1::R](ahblite_mpu_table_pwm1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PWM1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pwm1::W](ahblite_mpu_table_pwm1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PWM1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM1_REG"]
pub mod ahblite_mpu_table_pwm1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_i2s1](ahblite_mpu_table_i2s1) module"]
pub type AHBLITE_MPU_TABLE_I2S1 = crate::Reg<u32, _AHBLITE_MPU_TABLE_I2S1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_I2S1;
#[doc = "`read()` method returns [ahblite_mpu_table_i2s1::R](ahblite_mpu_table_i2s1::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2S1 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2s1::W](ahblite_mpu_table_i2s1::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2S1 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_I2S1_REG"]
pub mod ahblite_mpu_table_i2s1;
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_uart2](ahblite_mpu_table_uart2) module"]
pub type AHBLITE_MPU_TABLE_UART2 = crate::Reg<u32, _AHBLITE_MPU_TABLE_UART2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_UART2;
#[doc = "`read()` method returns [ahblite_mpu_table_uart2::R](ahblite_mpu_table_uart2::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_UART2 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_uart2::W](ahblite_mpu_table_uart2::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_UART2 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_UART2_REG"]
pub mod ahblite_mpu_table_uart2;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pwm2](ahblite_mpu_table_pwm2) module"]
pub type AHBLITE_MPU_TABLE_PWM2 = crate::Reg<u32, _AHBLITE_MPU_TABLE_PWM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PWM2;
#[doc = "`read()` method returns [ahblite_mpu_table_pwm2::R](ahblite_mpu_table_pwm2::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PWM2 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pwm2::W](ahblite_mpu_table_pwm2::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PWM2 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM2_REG"]
pub mod ahblite_mpu_table_pwm2;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pwm3](ahblite_mpu_table_pwm3) module"]
pub type AHBLITE_MPU_TABLE_PWM3 = crate::Reg<u32, _AHBLITE_MPU_TABLE_PWM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PWM3;
#[doc = "`read()` method returns [ahblite_mpu_table_pwm3::R](ahblite_mpu_table_pwm3::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PWM3 {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pwm3::W](ahblite_mpu_table_pwm3::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PWM3 {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWM3_REG"]
pub mod ahblite_mpu_table_pwm3;
#[doc = "DPORT_AHBLITE_MPU_TABLE_RWBT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_rwbt](ahblite_mpu_table_rwbt) module"]
pub type AHBLITE_MPU_TABLE_RWBT = crate::Reg<u32, _AHBLITE_MPU_TABLE_RWBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_RWBT;
#[doc = "`read()` method returns [ahblite_mpu_table_rwbt::R](ahblite_mpu_table_rwbt::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_RWBT {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_rwbt::W](ahblite_mpu_table_rwbt::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_RWBT {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_RWBT_REG"]
pub mod ahblite_mpu_table_rwbt;
#[doc = "DPORT_AHBLITE_MPU_TABLE_BTMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_btmac](ahblite_mpu_table_btmac) module"]
pub type AHBLITE_MPU_TABLE_BTMAC = crate::Reg<u32, _AHBLITE_MPU_TABLE_BTMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_BTMAC;
#[doc = "`read()` method returns [ahblite_mpu_table_btmac::R](ahblite_mpu_table_btmac::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_BTMAC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_btmac::W](ahblite_mpu_table_btmac::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_BTMAC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_BTMAC_REG"]
pub mod ahblite_mpu_table_btmac;
#[doc = "DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_wifimac](ahblite_mpu_table_wifimac) module"]
pub type AHBLITE_MPU_TABLE_WIFIMAC = crate::Reg<u32, _AHBLITE_MPU_TABLE_WIFIMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_WIFIMAC;
#[doc = "`read()` method returns [ahblite_mpu_table_wifimac::R](ahblite_mpu_table_wifimac::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_WIFIMAC {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_wifimac::W](ahblite_mpu_table_wifimac::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_WIFIMAC {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_WIFIMAC_REG"]
pub mod ahblite_mpu_table_wifimac;
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahblite_mpu_table_pwr](ahblite_mpu_table_pwr) module"]
pub type AHBLITE_MPU_TABLE_PWR = crate::Reg<u32, _AHBLITE_MPU_TABLE_PWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLITE_MPU_TABLE_PWR;
#[doc = "`read()` method returns [ahblite_mpu_table_pwr::R](ahblite_mpu_table_pwr::R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_PWR {}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_pwr::W](ahblite_mpu_table_pwr::W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_PWR {}
#[doc = "DPORT_AHBLITE_MPU_TABLE_PWR_REG"]
pub mod ahblite_mpu_table_pwr;
#[doc = "DPORT_MEM_ACCESS_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mem_access_dbug0](mem_access_dbug0) module"]
pub type MEM_ACCESS_DBUG0 = crate::Reg<u32, _MEM_ACCESS_DBUG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_ACCESS_DBUG0;
#[doc = "`read()` method returns [mem_access_dbug0::R](mem_access_dbug0::R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG0 {}
#[doc = "`write(|w| ..)` method takes [mem_access_dbug0::W](mem_access_dbug0::W) writer structure"]
impl crate::Writable for MEM_ACCESS_DBUG0 {}
#[doc = "DPORT_MEM_ACCESS_DBUG0_REG"]
pub mod mem_access_dbug0;
#[doc = "DPORT_MEM_ACCESS_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mem_access_dbug1](mem_access_dbug1) module"]
pub type MEM_ACCESS_DBUG1 = crate::Reg<u32, _MEM_ACCESS_DBUG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_ACCESS_DBUG1;
#[doc = "`read()` method returns [mem_access_dbug1::R](mem_access_dbug1::R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG1 {}
#[doc = "`write(|w| ..)` method takes [mem_access_dbug1::W](mem_access_dbug1::W) writer structure"]
impl crate::Writable for MEM_ACCESS_DBUG1 {}
#[doc = "DPORT_MEM_ACCESS_DBUG1_REG"]
pub mod mem_access_dbug1;
#[doc = "DPORT_PRO_DCACHE_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug0](pro_dcache_dbug0) module"]
pub type PRO_DCACHE_DBUG0 = crate::Reg<u32, _PRO_DCACHE_DBUG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG0;
#[doc = "`read()` method returns [pro_dcache_dbug0::R](pro_dcache_dbug0::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG0 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug0::W](pro_dcache_dbug0::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG0 {}
#[doc = "DPORT_PRO_DCACHE_DBUG0_REG"]
pub mod pro_dcache_dbug0;
#[doc = "DPORT_PRO_DCACHE_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug1](pro_dcache_dbug1) module"]
pub type PRO_DCACHE_DBUG1 = crate::Reg<u32, _PRO_DCACHE_DBUG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG1;
#[doc = "`read()` method returns [pro_dcache_dbug1::R](pro_dcache_dbug1::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG1 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug1::W](pro_dcache_dbug1::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG1 {}
#[doc = "DPORT_PRO_DCACHE_DBUG1_REG"]
pub mod pro_dcache_dbug1;
#[doc = "DPORT_PRO_DCACHE_DBUG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug2](pro_dcache_dbug2) module"]
pub type PRO_DCACHE_DBUG2 = crate::Reg<u32, _PRO_DCACHE_DBUG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG2;
#[doc = "`read()` method returns [pro_dcache_dbug2::R](pro_dcache_dbug2::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG2 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug2::W](pro_dcache_dbug2::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG2 {}
#[doc = "DPORT_PRO_DCACHE_DBUG2_REG"]
pub mod pro_dcache_dbug2;
#[doc = "DPORT_PRO_DCACHE_DBUG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug3](pro_dcache_dbug3) module"]
pub type PRO_DCACHE_DBUG3 = crate::Reg<u32, _PRO_DCACHE_DBUG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG3;
#[doc = "`read()` method returns [pro_dcache_dbug3::R](pro_dcache_dbug3::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG3 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug3::W](pro_dcache_dbug3::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG3 {}
#[doc = "DPORT_PRO_DCACHE_DBUG3_REG"]
pub mod pro_dcache_dbug3;
#[doc = "DPORT_PRO_DCACHE_DBUG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug4](pro_dcache_dbug4) module"]
pub type PRO_DCACHE_DBUG4 = crate::Reg<u32, _PRO_DCACHE_DBUG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG4;
#[doc = "`read()` method returns [pro_dcache_dbug4::R](pro_dcache_dbug4::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG4 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug4::W](pro_dcache_dbug4::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG4 {}
#[doc = "DPORT_PRO_DCACHE_DBUG4_REG"]
pub mod pro_dcache_dbug4;
#[doc = "DPORT_PRO_DCACHE_DBUG5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug5](pro_dcache_dbug5) module"]
pub type PRO_DCACHE_DBUG5 = crate::Reg<u32, _PRO_DCACHE_DBUG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG5;
#[doc = "`read()` method returns [pro_dcache_dbug5::R](pro_dcache_dbug5::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG5 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug5::W](pro_dcache_dbug5::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG5 {}
#[doc = "DPORT_PRO_DCACHE_DBUG5_REG"]
pub mod pro_dcache_dbug5;
#[doc = "DPORT_PRO_DCACHE_DBUG6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug6](pro_dcache_dbug6) module"]
pub type PRO_DCACHE_DBUG6 = crate::Reg<u32, _PRO_DCACHE_DBUG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG6;
#[doc = "`read()` method returns [pro_dcache_dbug6::R](pro_dcache_dbug6::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG6 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug6::W](pro_dcache_dbug6::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG6 {}
#[doc = "DPORT_PRO_DCACHE_DBUG6_REG"]
pub mod pro_dcache_dbug6;
#[doc = "DPORT_PRO_DCACHE_DBUG7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug7](pro_dcache_dbug7) module"]
pub type PRO_DCACHE_DBUG7 = crate::Reg<u32, _PRO_DCACHE_DBUG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG7;
#[doc = "`read()` method returns [pro_dcache_dbug7::R](pro_dcache_dbug7::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG7 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug7::W](pro_dcache_dbug7::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG7 {}
#[doc = "DPORT_PRO_DCACHE_DBUG7_REG"]
pub mod pro_dcache_dbug7;
#[doc = "DPORT_PRO_DCACHE_DBUG8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug8](pro_dcache_dbug8) module"]
pub type PRO_DCACHE_DBUG8 = crate::Reg<u32, _PRO_DCACHE_DBUG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG8;
#[doc = "`read()` method returns [pro_dcache_dbug8::R](pro_dcache_dbug8::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG8 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug8::W](pro_dcache_dbug8::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG8 {}
#[doc = "DPORT_PRO_DCACHE_DBUG8_REG"]
pub mod pro_dcache_dbug8;
#[doc = "DPORT_PRO_DCACHE_DBUG9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_dcache_dbug9](pro_dcache_dbug9) module"]
pub type PRO_DCACHE_DBUG9 = crate::Reg<u32, _PRO_DCACHE_DBUG9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_DCACHE_DBUG9;
#[doc = "`read()` method returns [pro_dcache_dbug9::R](pro_dcache_dbug9::R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG9 {}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug9::W](pro_dcache_dbug9::W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG9 {}
#[doc = "DPORT_PRO_DCACHE_DBUG9_REG"]
pub mod pro_dcache_dbug9;
#[doc = "DPORT_APP_DCACHE_DBUG0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug0](app_dcache_dbug0) module"]
pub type APP_DCACHE_DBUG0 = crate::Reg<u32, _APP_DCACHE_DBUG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG0;
#[doc = "`read()` method returns [app_dcache_dbug0::R](app_dcache_dbug0::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG0 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug0::W](app_dcache_dbug0::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG0 {}
#[doc = "DPORT_APP_DCACHE_DBUG0_REG"]
pub mod app_dcache_dbug0;
#[doc = "DPORT_APP_DCACHE_DBUG1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug1](app_dcache_dbug1) module"]
pub type APP_DCACHE_DBUG1 = crate::Reg<u32, _APP_DCACHE_DBUG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG1;
#[doc = "`read()` method returns [app_dcache_dbug1::R](app_dcache_dbug1::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG1 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug1::W](app_dcache_dbug1::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG1 {}
#[doc = "DPORT_APP_DCACHE_DBUG1_REG"]
pub mod app_dcache_dbug1;
#[doc = "DPORT_APP_DCACHE_DBUG2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug2](app_dcache_dbug2) module"]
pub type APP_DCACHE_DBUG2 = crate::Reg<u32, _APP_DCACHE_DBUG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG2;
#[doc = "`read()` method returns [app_dcache_dbug2::R](app_dcache_dbug2::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG2 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug2::W](app_dcache_dbug2::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG2 {}
#[doc = "DPORT_APP_DCACHE_DBUG2_REG"]
pub mod app_dcache_dbug2;
#[doc = "DPORT_APP_DCACHE_DBUG3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug3](app_dcache_dbug3) module"]
pub type APP_DCACHE_DBUG3 = crate::Reg<u32, _APP_DCACHE_DBUG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG3;
#[doc = "`read()` method returns [app_dcache_dbug3::R](app_dcache_dbug3::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG3 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug3::W](app_dcache_dbug3::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG3 {}
#[doc = "DPORT_APP_DCACHE_DBUG3_REG"]
pub mod app_dcache_dbug3;
#[doc = "DPORT_APP_DCACHE_DBUG4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug4](app_dcache_dbug4) module"]
pub type APP_DCACHE_DBUG4 = crate::Reg<u32, _APP_DCACHE_DBUG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG4;
#[doc = "`read()` method returns [app_dcache_dbug4::R](app_dcache_dbug4::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG4 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug4::W](app_dcache_dbug4::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG4 {}
#[doc = "DPORT_APP_DCACHE_DBUG4_REG"]
pub mod app_dcache_dbug4;
#[doc = "DPORT_APP_DCACHE_DBUG5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug5](app_dcache_dbug5) module"]
pub type APP_DCACHE_DBUG5 = crate::Reg<u32, _APP_DCACHE_DBUG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG5;
#[doc = "`read()` method returns [app_dcache_dbug5::R](app_dcache_dbug5::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG5 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug5::W](app_dcache_dbug5::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG5 {}
#[doc = "DPORT_APP_DCACHE_DBUG5_REG"]
pub mod app_dcache_dbug5;
#[doc = "DPORT_APP_DCACHE_DBUG6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug6](app_dcache_dbug6) module"]
pub type APP_DCACHE_DBUG6 = crate::Reg<u32, _APP_DCACHE_DBUG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG6;
#[doc = "`read()` method returns [app_dcache_dbug6::R](app_dcache_dbug6::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG6 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug6::W](app_dcache_dbug6::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG6 {}
#[doc = "DPORT_APP_DCACHE_DBUG6_REG"]
pub mod app_dcache_dbug6;
#[doc = "DPORT_APP_DCACHE_DBUG7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug7](app_dcache_dbug7) module"]
pub type APP_DCACHE_DBUG7 = crate::Reg<u32, _APP_DCACHE_DBUG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG7;
#[doc = "`read()` method returns [app_dcache_dbug7::R](app_dcache_dbug7::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG7 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug7::W](app_dcache_dbug7::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG7 {}
#[doc = "DPORT_APP_DCACHE_DBUG7_REG"]
pub mod app_dcache_dbug7;
#[doc = "DPORT_APP_DCACHE_DBUG8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug8](app_dcache_dbug8) module"]
pub type APP_DCACHE_DBUG8 = crate::Reg<u32, _APP_DCACHE_DBUG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG8;
#[doc = "`read()` method returns [app_dcache_dbug8::R](app_dcache_dbug8::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG8 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug8::W](app_dcache_dbug8::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG8 {}
#[doc = "DPORT_APP_DCACHE_DBUG8_REG"]
pub mod app_dcache_dbug8;
#[doc = "DPORT_APP_DCACHE_DBUG9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_dcache_dbug9](app_dcache_dbug9) module"]
pub type APP_DCACHE_DBUG9 = crate::Reg<u32, _APP_DCACHE_DBUG9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_DCACHE_DBUG9;
#[doc = "`read()` method returns [app_dcache_dbug9::R](app_dcache_dbug9::R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG9 {}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug9::W](app_dcache_dbug9::W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG9 {}
#[doc = "DPORT_APP_DCACHE_DBUG9_REG"]
pub mod app_dcache_dbug9;
#[doc = "DPORT_PRO_CPU_RECORD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_ctrl](pro_cpu_record_ctrl) module"]
pub type PRO_CPU_RECORD_CTRL = crate::Reg<u32, _PRO_CPU_RECORD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_CTRL;
#[doc = "`read()` method returns [pro_cpu_record_ctrl::R](pro_cpu_record_ctrl::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_CTRL {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_ctrl::W](pro_cpu_record_ctrl::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_CTRL {}
#[doc = "DPORT_PRO_CPU_RECORD_CTRL_REG"]
pub mod pro_cpu_record_ctrl;
#[doc = "DPORT_PRO_CPU_RECORD_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_status](pro_cpu_record_status) module"]
pub type PRO_CPU_RECORD_STATUS = crate::Reg<u32, _PRO_CPU_RECORD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_STATUS;
#[doc = "`read()` method returns [pro_cpu_record_status::R](pro_cpu_record_status::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_STATUS {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_status::W](pro_cpu_record_status::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_STATUS {}
#[doc = "DPORT_PRO_CPU_RECORD_STATUS_REG"]
pub mod pro_cpu_record_status;
#[doc = "DPORT_PRO_CPU_RECORD_PID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pid](pro_cpu_record_pid) module"]
pub type PRO_CPU_RECORD_PID = crate::Reg<u32, _PRO_CPU_RECORD_PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PID;
#[doc = "`read()` method returns [pro_cpu_record_pid::R](pro_cpu_record_pid::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PID {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pid::W](pro_cpu_record_pid::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PID {}
#[doc = "DPORT_PRO_CPU_RECORD_PID_REG"]
pub mod pro_cpu_record_pid;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGINST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebuginst](pro_cpu_record_pdebuginst) module"]
pub type PRO_CPU_RECORD_PDEBUGINST = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGINST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGINST;
#[doc = "`read()` method returns [pro_cpu_record_pdebuginst::R](pro_cpu_record_pdebuginst::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGINST {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebuginst::W](pro_cpu_record_pdebuginst::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGINST {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGINST_REG"]
pub mod pro_cpu_record_pdebuginst;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugstatus](pro_cpu_record_pdebugstatus) module"]
pub type PRO_CPU_RECORD_PDEBUGSTATUS = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGSTATUS;
#[doc = "`read()` method returns [pro_cpu_record_pdebugstatus::R](pro_cpu_record_pdebugstatus::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGSTATUS {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugstatus::W](pro_cpu_record_pdebugstatus::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGSTATUS {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGSTATUS_REG"]
pub mod pro_cpu_record_pdebugstatus;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugdata](pro_cpu_record_pdebugdata) module"]
pub type PRO_CPU_RECORD_PDEBUGDATA = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGDATA;
#[doc = "`read()` method returns [pro_cpu_record_pdebugdata::R](pro_cpu_record_pdebugdata::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGDATA {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugdata::W](pro_cpu_record_pdebugdata::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGDATA {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
pub mod pro_cpu_record_pdebugdata;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGPC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugpc](pro_cpu_record_pdebugpc) module"]
pub type PRO_CPU_RECORD_PDEBUGPC = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGPC;
#[doc = "`read()` method returns [pro_cpu_record_pdebugpc::R](pro_cpu_record_pdebugpc::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGPC {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugpc::W](pro_cpu_record_pdebugpc::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGPC {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGPC_REG"]
pub mod pro_cpu_record_pdebugpc;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugls0stat](pro_cpu_record_pdebugls0stat) module"]
pub type PRO_CPU_RECORD_PDEBUGLS0STAT = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGLS0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGLS0STAT;
#[doc = "`read()` method returns [pro_cpu_record_pdebugls0stat::R](pro_cpu_record_pdebugls0stat::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0STAT {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugls0stat::W](pro_cpu_record_pdebugls0stat::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGLS0STAT {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0STAT_REG"]
pub mod pro_cpu_record_pdebugls0stat;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugls0addr](pro_cpu_record_pdebugls0addr) module"]
pub type PRO_CPU_RECORD_PDEBUGLS0ADDR = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGLS0ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGLS0ADDR;
#[doc = "`read()` method returns [pro_cpu_record_pdebugls0addr::R](pro_cpu_record_pdebugls0addr::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0ADDR {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugls0addr::W](pro_cpu_record_pdebugls0addr::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGLS0ADDR {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0ADDR_REG"]
pub mod pro_cpu_record_pdebugls0addr;
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_cpu_record_pdebugls0data](pro_cpu_record_pdebugls0data) module"]
pub type PRO_CPU_RECORD_PDEBUGLS0DATA = crate::Reg<u32, _PRO_CPU_RECORD_PDEBUGLS0DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_CPU_RECORD_PDEBUGLS0DATA;
#[doc = "`read()` method returns [pro_cpu_record_pdebugls0data::R](pro_cpu_record_pdebugls0data::R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0DATA {}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugls0data::W](pro_cpu_record_pdebugls0data::W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGLS0DATA {}
#[doc = "DPORT_PRO_CPU_RECORD_PDEBUGLS0DATA_REG"]
pub mod pro_cpu_record_pdebugls0data;
#[doc = "DPORT_APP_CPU_RECORD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_ctrl](app_cpu_record_ctrl) module"]
pub type APP_CPU_RECORD_CTRL = crate::Reg<u32, _APP_CPU_RECORD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_CTRL;
#[doc = "`read()` method returns [app_cpu_record_ctrl::R](app_cpu_record_ctrl::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_CTRL {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_ctrl::W](app_cpu_record_ctrl::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_CTRL {}
#[doc = "DPORT_APP_CPU_RECORD_CTRL_REG"]
pub mod app_cpu_record_ctrl;
#[doc = "DPORT_APP_CPU_RECORD_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_status](app_cpu_record_status) module"]
pub type APP_CPU_RECORD_STATUS = crate::Reg<u32, _APP_CPU_RECORD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_STATUS;
#[doc = "`read()` method returns [app_cpu_record_status::R](app_cpu_record_status::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_STATUS {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_status::W](app_cpu_record_status::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_STATUS {}
#[doc = "DPORT_APP_CPU_RECORD_STATUS_REG"]
pub mod app_cpu_record_status;
#[doc = "DPORT_APP_CPU_RECORD_PID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pid](app_cpu_record_pid) module"]
pub type APP_CPU_RECORD_PID = crate::Reg<u32, _APP_CPU_RECORD_PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PID;
#[doc = "`read()` method returns [app_cpu_record_pid::R](app_cpu_record_pid::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PID {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pid::W](app_cpu_record_pid::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PID {}
#[doc = "DPORT_APP_CPU_RECORD_PID_REG"]
pub mod app_cpu_record_pid;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGINST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebuginst](app_cpu_record_pdebuginst) module"]
pub type APP_CPU_RECORD_PDEBUGINST = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGINST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGINST;
#[doc = "`read()` method returns [app_cpu_record_pdebuginst::R](app_cpu_record_pdebuginst::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGINST {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebuginst::W](app_cpu_record_pdebuginst::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGINST {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGINST_REG"]
pub mod app_cpu_record_pdebuginst;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugstatus](app_cpu_record_pdebugstatus) module"]
pub type APP_CPU_RECORD_PDEBUGSTATUS = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGSTATUS;
#[doc = "`read()` method returns [app_cpu_record_pdebugstatus::R](app_cpu_record_pdebugstatus::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGSTATUS {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugstatus::W](app_cpu_record_pdebugstatus::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGSTATUS {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
pub mod app_cpu_record_pdebugstatus;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugdata](app_cpu_record_pdebugdata) module"]
pub type APP_CPU_RECORD_PDEBUGDATA = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGDATA;
#[doc = "`read()` method returns [app_cpu_record_pdebugdata::R](app_cpu_record_pdebugdata::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGDATA {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugdata::W](app_cpu_record_pdebugdata::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGDATA {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGDATA_REG"]
pub mod app_cpu_record_pdebugdata;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGPC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugpc](app_cpu_record_pdebugpc) module"]
pub type APP_CPU_RECORD_PDEBUGPC = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGPC;
#[doc = "`read()` method returns [app_cpu_record_pdebugpc::R](app_cpu_record_pdebugpc::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGPC {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugpc::W](app_cpu_record_pdebugpc::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGPC {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGPC_REG"]
pub mod app_cpu_record_pdebugpc;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugls0stat](app_cpu_record_pdebugls0stat) module"]
pub type APP_CPU_RECORD_PDEBUGLS0STAT = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGLS0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGLS0STAT;
#[doc = "`read()` method returns [app_cpu_record_pdebugls0stat::R](app_cpu_record_pdebugls0stat::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGLS0STAT {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugls0stat::W](app_cpu_record_pdebugls0stat::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGLS0STAT {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0STAT_REG"]
pub mod app_cpu_record_pdebugls0stat;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugls0addr](app_cpu_record_pdebugls0addr) module"]
pub type APP_CPU_RECORD_PDEBUGLS0ADDR = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGLS0ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGLS0ADDR;
#[doc = "`read()` method returns [app_cpu_record_pdebugls0addr::R](app_cpu_record_pdebugls0addr::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGLS0ADDR {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugls0addr::W](app_cpu_record_pdebugls0addr::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGLS0ADDR {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0ADDR_REG"]
pub mod app_cpu_record_pdebugls0addr;
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_cpu_record_pdebugls0data](app_cpu_record_pdebugls0data) module"]
pub type APP_CPU_RECORD_PDEBUGLS0DATA = crate::Reg<u32, _APP_CPU_RECORD_PDEBUGLS0DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_CPU_RECORD_PDEBUGLS0DATA;
#[doc = "`read()` method returns [app_cpu_record_pdebugls0data::R](app_cpu_record_pdebugls0data::R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGLS0DATA {}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_pdebugls0data::W](app_cpu_record_pdebugls0data::W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_PDEBUGLS0DATA {}
#[doc = "DPORT_APP_CPU_RECORD_PDEBUGLS0DATA_REG"]
pub mod app_cpu_record_pdebugls0data;
#[doc = "DPORT_RSA_PD_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rsa_pd_ctrl](rsa_pd_ctrl) module"]
pub type RSA_PD_CTRL = crate::Reg<u32, _RSA_PD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSA_PD_CTRL;
#[doc = "`read()` method returns [rsa_pd_ctrl::R](rsa_pd_ctrl::R) reader structure"]
impl crate::Readable for RSA_PD_CTRL {}
#[doc = "`write(|w| ..)` method takes [rsa_pd_ctrl::W](rsa_pd_ctrl::W) writer structure"]
impl crate::Writable for RSA_PD_CTRL {}
#[doc = "DPORT_RSA_PD_CTRL_REG"]
pub mod rsa_pd_ctrl;
#[doc = "DPORT_ROM_MPU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_mpu_table0](rom_mpu_table0) module"]
pub type ROM_MPU_TABLE0 = crate::Reg<u32, _ROM_MPU_TABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_MPU_TABLE0;
#[doc = "`read()` method returns [rom_mpu_table0::R](rom_mpu_table0::R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE0 {}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table0::W](rom_mpu_table0::W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE0 {}
#[doc = "DPORT_ROM_MPU_TABLE0_REG"]
pub mod rom_mpu_table0;
#[doc = "DPORT_ROM_MPU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_mpu_table1](rom_mpu_table1) module"]
pub type ROM_MPU_TABLE1 = crate::Reg<u32, _ROM_MPU_TABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_MPU_TABLE1;
#[doc = "`read()` method returns [rom_mpu_table1::R](rom_mpu_table1::R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE1 {}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table1::W](rom_mpu_table1::W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE1 {}
#[doc = "DPORT_ROM_MPU_TABLE1_REG"]
pub mod rom_mpu_table1;
#[doc = "DPORT_ROM_MPU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_mpu_table2](rom_mpu_table2) module"]
pub type ROM_MPU_TABLE2 = crate::Reg<u32, _ROM_MPU_TABLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_MPU_TABLE2;
#[doc = "`read()` method returns [rom_mpu_table2::R](rom_mpu_table2::R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE2 {}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table2::W](rom_mpu_table2::W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE2 {}
#[doc = "DPORT_ROM_MPU_TABLE2_REG"]
pub mod rom_mpu_table2;
#[doc = "DPORT_ROM_MPU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_mpu_table3](rom_mpu_table3) module"]
pub type ROM_MPU_TABLE3 = crate::Reg<u32, _ROM_MPU_TABLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_MPU_TABLE3;
#[doc = "`read()` method returns [rom_mpu_table3::R](rom_mpu_table3::R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE3 {}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table3::W](rom_mpu_table3::W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE3 {}
#[doc = "DPORT_ROM_MPU_TABLE3_REG"]
pub mod rom_mpu_table3;
#[doc = "DPORT_SHROM_MPU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table0](shrom_mpu_table0) module"]
pub type SHROM_MPU_TABLE0 = crate::Reg<u32, _SHROM_MPU_TABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE0;
#[doc = "`read()` method returns [shrom_mpu_table0::R](shrom_mpu_table0::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE0 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table0::W](shrom_mpu_table0::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE0 {}
#[doc = "DPORT_SHROM_MPU_TABLE0_REG"]
pub mod shrom_mpu_table0;
#[doc = "DPORT_SHROM_MPU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table1](shrom_mpu_table1) module"]
pub type SHROM_MPU_TABLE1 = crate::Reg<u32, _SHROM_MPU_TABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE1;
#[doc = "`read()` method returns [shrom_mpu_table1::R](shrom_mpu_table1::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE1 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table1::W](shrom_mpu_table1::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE1 {}
#[doc = "DPORT_SHROM_MPU_TABLE1_REG"]
pub mod shrom_mpu_table1;
#[doc = "DPORT_SHROM_MPU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table2](shrom_mpu_table2) module"]
pub type SHROM_MPU_TABLE2 = crate::Reg<u32, _SHROM_MPU_TABLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE2;
#[doc = "`read()` method returns [shrom_mpu_table2::R](shrom_mpu_table2::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE2 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table2::W](shrom_mpu_table2::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE2 {}
#[doc = "DPORT_SHROM_MPU_TABLE2_REG"]
pub mod shrom_mpu_table2;
#[doc = "DPORT_SHROM_MPU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table3](shrom_mpu_table3) module"]
pub type SHROM_MPU_TABLE3 = crate::Reg<u32, _SHROM_MPU_TABLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE3;
#[doc = "`read()` method returns [shrom_mpu_table3::R](shrom_mpu_table3::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE3 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table3::W](shrom_mpu_table3::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE3 {}
#[doc = "DPORT_SHROM_MPU_TABLE3_REG"]
pub mod shrom_mpu_table3;
#[doc = "DPORT_SHROM_MPU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table4](shrom_mpu_table4) module"]
pub type SHROM_MPU_TABLE4 = crate::Reg<u32, _SHROM_MPU_TABLE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE4;
#[doc = "`read()` method returns [shrom_mpu_table4::R](shrom_mpu_table4::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE4 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table4::W](shrom_mpu_table4::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE4 {}
#[doc = "DPORT_SHROM_MPU_TABLE4_REG"]
pub mod shrom_mpu_table4;
#[doc = "DPORT_SHROM_MPU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table5](shrom_mpu_table5) module"]
pub type SHROM_MPU_TABLE5 = crate::Reg<u32, _SHROM_MPU_TABLE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE5;
#[doc = "`read()` method returns [shrom_mpu_table5::R](shrom_mpu_table5::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE5 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table5::W](shrom_mpu_table5::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE5 {}
#[doc = "DPORT_SHROM_MPU_TABLE5_REG"]
pub mod shrom_mpu_table5;
#[doc = "DPORT_SHROM_MPU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table6](shrom_mpu_table6) module"]
pub type SHROM_MPU_TABLE6 = crate::Reg<u32, _SHROM_MPU_TABLE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE6;
#[doc = "`read()` method returns [shrom_mpu_table6::R](shrom_mpu_table6::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE6 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table6::W](shrom_mpu_table6::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE6 {}
#[doc = "DPORT_SHROM_MPU_TABLE6_REG"]
pub mod shrom_mpu_table6;
#[doc = "DPORT_SHROM_MPU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table7](shrom_mpu_table7) module"]
pub type SHROM_MPU_TABLE7 = crate::Reg<u32, _SHROM_MPU_TABLE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE7;
#[doc = "`read()` method returns [shrom_mpu_table7::R](shrom_mpu_table7::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE7 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table7::W](shrom_mpu_table7::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE7 {}
#[doc = "DPORT_SHROM_MPU_TABLE7_REG"]
pub mod shrom_mpu_table7;
#[doc = "DPORT_SHROM_MPU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table8](shrom_mpu_table8) module"]
pub type SHROM_MPU_TABLE8 = crate::Reg<u32, _SHROM_MPU_TABLE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE8;
#[doc = "`read()` method returns [shrom_mpu_table8::R](shrom_mpu_table8::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE8 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table8::W](shrom_mpu_table8::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE8 {}
#[doc = "DPORT_SHROM_MPU_TABLE8_REG"]
pub mod shrom_mpu_table8;
#[doc = "DPORT_SHROM_MPU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table9](shrom_mpu_table9) module"]
pub type SHROM_MPU_TABLE9 = crate::Reg<u32, _SHROM_MPU_TABLE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE9;
#[doc = "`read()` method returns [shrom_mpu_table9::R](shrom_mpu_table9::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE9 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table9::W](shrom_mpu_table9::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE9 {}
#[doc = "DPORT_SHROM_MPU_TABLE9_REG"]
pub mod shrom_mpu_table9;
#[doc = "DPORT_SHROM_MPU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table10](shrom_mpu_table10) module"]
pub type SHROM_MPU_TABLE10 = crate::Reg<u32, _SHROM_MPU_TABLE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE10;
#[doc = "`read()` method returns [shrom_mpu_table10::R](shrom_mpu_table10::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE10 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table10::W](shrom_mpu_table10::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE10 {}
#[doc = "DPORT_SHROM_MPU_TABLE10_REG"]
pub mod shrom_mpu_table10;
#[doc = "DPORT_SHROM_MPU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table11](shrom_mpu_table11) module"]
pub type SHROM_MPU_TABLE11 = crate::Reg<u32, _SHROM_MPU_TABLE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE11;
#[doc = "`read()` method returns [shrom_mpu_table11::R](shrom_mpu_table11::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE11 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table11::W](shrom_mpu_table11::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE11 {}
#[doc = "DPORT_SHROM_MPU_TABLE11_REG"]
pub mod shrom_mpu_table11;
#[doc = "DPORT_SHROM_MPU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table12](shrom_mpu_table12) module"]
pub type SHROM_MPU_TABLE12 = crate::Reg<u32, _SHROM_MPU_TABLE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE12;
#[doc = "`read()` method returns [shrom_mpu_table12::R](shrom_mpu_table12::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE12 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table12::W](shrom_mpu_table12::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE12 {}
#[doc = "DPORT_SHROM_MPU_TABLE12_REG"]
pub mod shrom_mpu_table12;
#[doc = "DPORT_SHROM_MPU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table13](shrom_mpu_table13) module"]
pub type SHROM_MPU_TABLE13 = crate::Reg<u32, _SHROM_MPU_TABLE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE13;
#[doc = "`read()` method returns [shrom_mpu_table13::R](shrom_mpu_table13::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE13 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table13::W](shrom_mpu_table13::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE13 {}
#[doc = "DPORT_SHROM_MPU_TABLE13_REG"]
pub mod shrom_mpu_table13;
#[doc = "DPORT_SHROM_MPU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table14](shrom_mpu_table14) module"]
pub type SHROM_MPU_TABLE14 = crate::Reg<u32, _SHROM_MPU_TABLE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE14;
#[doc = "`read()` method returns [shrom_mpu_table14::R](shrom_mpu_table14::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE14 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table14::W](shrom_mpu_table14::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE14 {}
#[doc = "DPORT_SHROM_MPU_TABLE14_REG"]
pub mod shrom_mpu_table14;
#[doc = "DPORT_SHROM_MPU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table15](shrom_mpu_table15) module"]
pub type SHROM_MPU_TABLE15 = crate::Reg<u32, _SHROM_MPU_TABLE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE15;
#[doc = "`read()` method returns [shrom_mpu_table15::R](shrom_mpu_table15::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE15 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table15::W](shrom_mpu_table15::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE15 {}
#[doc = "DPORT_SHROM_MPU_TABLE15_REG"]
pub mod shrom_mpu_table15;
#[doc = "DPORT_SHROM_MPU_TABLE16_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table16](shrom_mpu_table16) module"]
pub type SHROM_MPU_TABLE16 = crate::Reg<u32, _SHROM_MPU_TABLE16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE16;
#[doc = "`read()` method returns [shrom_mpu_table16::R](shrom_mpu_table16::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE16 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table16::W](shrom_mpu_table16::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE16 {}
#[doc = "DPORT_SHROM_MPU_TABLE16_REG"]
pub mod shrom_mpu_table16;
#[doc = "DPORT_SHROM_MPU_TABLE17_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table17](shrom_mpu_table17) module"]
pub type SHROM_MPU_TABLE17 = crate::Reg<u32, _SHROM_MPU_TABLE17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE17;
#[doc = "`read()` method returns [shrom_mpu_table17::R](shrom_mpu_table17::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE17 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table17::W](shrom_mpu_table17::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE17 {}
#[doc = "DPORT_SHROM_MPU_TABLE17_REG"]
pub mod shrom_mpu_table17;
#[doc = "DPORT_SHROM_MPU_TABLE18_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table18](shrom_mpu_table18) module"]
pub type SHROM_MPU_TABLE18 = crate::Reg<u32, _SHROM_MPU_TABLE18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE18;
#[doc = "`read()` method returns [shrom_mpu_table18::R](shrom_mpu_table18::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE18 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table18::W](shrom_mpu_table18::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE18 {}
#[doc = "DPORT_SHROM_MPU_TABLE18_REG"]
pub mod shrom_mpu_table18;
#[doc = "DPORT_SHROM_MPU_TABLE19_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table19](shrom_mpu_table19) module"]
pub type SHROM_MPU_TABLE19 = crate::Reg<u32, _SHROM_MPU_TABLE19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE19;
#[doc = "`read()` method returns [shrom_mpu_table19::R](shrom_mpu_table19::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE19 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table19::W](shrom_mpu_table19::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE19 {}
#[doc = "DPORT_SHROM_MPU_TABLE19_REG"]
pub mod shrom_mpu_table19;
#[doc = "DPORT_SHROM_MPU_TABLE20_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table20](shrom_mpu_table20) module"]
pub type SHROM_MPU_TABLE20 = crate::Reg<u32, _SHROM_MPU_TABLE20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE20;
#[doc = "`read()` method returns [shrom_mpu_table20::R](shrom_mpu_table20::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE20 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table20::W](shrom_mpu_table20::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE20 {}
#[doc = "DPORT_SHROM_MPU_TABLE20_REG"]
pub mod shrom_mpu_table20;
#[doc = "DPORT_SHROM_MPU_TABLE21_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table21](shrom_mpu_table21) module"]
pub type SHROM_MPU_TABLE21 = crate::Reg<u32, _SHROM_MPU_TABLE21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE21;
#[doc = "`read()` method returns [shrom_mpu_table21::R](shrom_mpu_table21::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE21 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table21::W](shrom_mpu_table21::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE21 {}
#[doc = "DPORT_SHROM_MPU_TABLE21_REG"]
pub mod shrom_mpu_table21;
#[doc = "DPORT_SHROM_MPU_TABLE22_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table22](shrom_mpu_table22) module"]
pub type SHROM_MPU_TABLE22 = crate::Reg<u32, _SHROM_MPU_TABLE22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE22;
#[doc = "`read()` method returns [shrom_mpu_table22::R](shrom_mpu_table22::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE22 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table22::W](shrom_mpu_table22::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE22 {}
#[doc = "DPORT_SHROM_MPU_TABLE22_REG"]
pub mod shrom_mpu_table22;
#[doc = "DPORT_SHROM_MPU_TABLE23_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shrom_mpu_table23](shrom_mpu_table23) module"]
pub type SHROM_MPU_TABLE23 = crate::Reg<u32, _SHROM_MPU_TABLE23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHROM_MPU_TABLE23;
#[doc = "`read()` method returns [shrom_mpu_table23::R](shrom_mpu_table23::R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE23 {}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table23::W](shrom_mpu_table23::W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE23 {}
#[doc = "DPORT_SHROM_MPU_TABLE23_REG"]
pub mod shrom_mpu_table23;
#[doc = "DPORT_IMMU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table0](immu_table0) module"]
pub type IMMU_TABLE0 = crate::Reg<u32, _IMMU_TABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE0;
#[doc = "`read()` method returns [immu_table0::R](immu_table0::R) reader structure"]
impl crate::Readable for IMMU_TABLE0 {}
#[doc = "`write(|w| ..)` method takes [immu_table0::W](immu_table0::W) writer structure"]
impl crate::Writable for IMMU_TABLE0 {}
#[doc = "DPORT_IMMU_TABLE0_REG"]
pub mod immu_table0;
#[doc = "DPORT_IMMU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table1](immu_table1) module"]
pub type IMMU_TABLE1 = crate::Reg<u32, _IMMU_TABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE1;
#[doc = "`read()` method returns [immu_table1::R](immu_table1::R) reader structure"]
impl crate::Readable for IMMU_TABLE1 {}
#[doc = "`write(|w| ..)` method takes [immu_table1::W](immu_table1::W) writer structure"]
impl crate::Writable for IMMU_TABLE1 {}
#[doc = "DPORT_IMMU_TABLE1_REG"]
pub mod immu_table1;
#[doc = "DPORT_IMMU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table2](immu_table2) module"]
pub type IMMU_TABLE2 = crate::Reg<u32, _IMMU_TABLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE2;
#[doc = "`read()` method returns [immu_table2::R](immu_table2::R) reader structure"]
impl crate::Readable for IMMU_TABLE2 {}
#[doc = "`write(|w| ..)` method takes [immu_table2::W](immu_table2::W) writer structure"]
impl crate::Writable for IMMU_TABLE2 {}
#[doc = "DPORT_IMMU_TABLE2_REG"]
pub mod immu_table2;
#[doc = "DPORT_IMMU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table3](immu_table3) module"]
pub type IMMU_TABLE3 = crate::Reg<u32, _IMMU_TABLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE3;
#[doc = "`read()` method returns [immu_table3::R](immu_table3::R) reader structure"]
impl crate::Readable for IMMU_TABLE3 {}
#[doc = "`write(|w| ..)` method takes [immu_table3::W](immu_table3::W) writer structure"]
impl crate::Writable for IMMU_TABLE3 {}
#[doc = "DPORT_IMMU_TABLE3_REG"]
pub mod immu_table3;
#[doc = "DPORT_IMMU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table4](immu_table4) module"]
pub type IMMU_TABLE4 = crate::Reg<u32, _IMMU_TABLE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE4;
#[doc = "`read()` method returns [immu_table4::R](immu_table4::R) reader structure"]
impl crate::Readable for IMMU_TABLE4 {}
#[doc = "`write(|w| ..)` method takes [immu_table4::W](immu_table4::W) writer structure"]
impl crate::Writable for IMMU_TABLE4 {}
#[doc = "DPORT_IMMU_TABLE4_REG"]
pub mod immu_table4;
#[doc = "DPORT_IMMU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table5](immu_table5) module"]
pub type IMMU_TABLE5 = crate::Reg<u32, _IMMU_TABLE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE5;
#[doc = "`read()` method returns [immu_table5::R](immu_table5::R) reader structure"]
impl crate::Readable for IMMU_TABLE5 {}
#[doc = "`write(|w| ..)` method takes [immu_table5::W](immu_table5::W) writer structure"]
impl crate::Writable for IMMU_TABLE5 {}
#[doc = "DPORT_IMMU_TABLE5_REG"]
pub mod immu_table5;
#[doc = "DPORT_IMMU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table6](immu_table6) module"]
pub type IMMU_TABLE6 = crate::Reg<u32, _IMMU_TABLE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE6;
#[doc = "`read()` method returns [immu_table6::R](immu_table6::R) reader structure"]
impl crate::Readable for IMMU_TABLE6 {}
#[doc = "`write(|w| ..)` method takes [immu_table6::W](immu_table6::W) writer structure"]
impl crate::Writable for IMMU_TABLE6 {}
#[doc = "DPORT_IMMU_TABLE6_REG"]
pub mod immu_table6;
#[doc = "DPORT_IMMU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table7](immu_table7) module"]
pub type IMMU_TABLE7 = crate::Reg<u32, _IMMU_TABLE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE7;
#[doc = "`read()` method returns [immu_table7::R](immu_table7::R) reader structure"]
impl crate::Readable for IMMU_TABLE7 {}
#[doc = "`write(|w| ..)` method takes [immu_table7::W](immu_table7::W) writer structure"]
impl crate::Writable for IMMU_TABLE7 {}
#[doc = "DPORT_IMMU_TABLE7_REG"]
pub mod immu_table7;
#[doc = "DPORT_IMMU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table8](immu_table8) module"]
pub type IMMU_TABLE8 = crate::Reg<u32, _IMMU_TABLE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE8;
#[doc = "`read()` method returns [immu_table8::R](immu_table8::R) reader structure"]
impl crate::Readable for IMMU_TABLE8 {}
#[doc = "`write(|w| ..)` method takes [immu_table8::W](immu_table8::W) writer structure"]
impl crate::Writable for IMMU_TABLE8 {}
#[doc = "DPORT_IMMU_TABLE8_REG"]
pub mod immu_table8;
#[doc = "DPORT_IMMU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table9](immu_table9) module"]
pub type IMMU_TABLE9 = crate::Reg<u32, _IMMU_TABLE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE9;
#[doc = "`read()` method returns [immu_table9::R](immu_table9::R) reader structure"]
impl crate::Readable for IMMU_TABLE9 {}
#[doc = "`write(|w| ..)` method takes [immu_table9::W](immu_table9::W) writer structure"]
impl crate::Writable for IMMU_TABLE9 {}
#[doc = "DPORT_IMMU_TABLE9_REG"]
pub mod immu_table9;
#[doc = "DPORT_IMMU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table10](immu_table10) module"]
pub type IMMU_TABLE10 = crate::Reg<u32, _IMMU_TABLE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE10;
#[doc = "`read()` method returns [immu_table10::R](immu_table10::R) reader structure"]
impl crate::Readable for IMMU_TABLE10 {}
#[doc = "`write(|w| ..)` method takes [immu_table10::W](immu_table10::W) writer structure"]
impl crate::Writable for IMMU_TABLE10 {}
#[doc = "DPORT_IMMU_TABLE10_REG"]
pub mod immu_table10;
#[doc = "DPORT_IMMU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table11](immu_table11) module"]
pub type IMMU_TABLE11 = crate::Reg<u32, _IMMU_TABLE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE11;
#[doc = "`read()` method returns [immu_table11::R](immu_table11::R) reader structure"]
impl crate::Readable for IMMU_TABLE11 {}
#[doc = "`write(|w| ..)` method takes [immu_table11::W](immu_table11::W) writer structure"]
impl crate::Writable for IMMU_TABLE11 {}
#[doc = "DPORT_IMMU_TABLE11_REG"]
pub mod immu_table11;
#[doc = "DPORT_IMMU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table12](immu_table12) module"]
pub type IMMU_TABLE12 = crate::Reg<u32, _IMMU_TABLE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE12;
#[doc = "`read()` method returns [immu_table12::R](immu_table12::R) reader structure"]
impl crate::Readable for IMMU_TABLE12 {}
#[doc = "`write(|w| ..)` method takes [immu_table12::W](immu_table12::W) writer structure"]
impl crate::Writable for IMMU_TABLE12 {}
#[doc = "DPORT_IMMU_TABLE12_REG"]
pub mod immu_table12;
#[doc = "DPORT_IMMU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table13](immu_table13) module"]
pub type IMMU_TABLE13 = crate::Reg<u32, _IMMU_TABLE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE13;
#[doc = "`read()` method returns [immu_table13::R](immu_table13::R) reader structure"]
impl crate::Readable for IMMU_TABLE13 {}
#[doc = "`write(|w| ..)` method takes [immu_table13::W](immu_table13::W) writer structure"]
impl crate::Writable for IMMU_TABLE13 {}
#[doc = "DPORT_IMMU_TABLE13_REG"]
pub mod immu_table13;
#[doc = "DPORT_IMMU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table14](immu_table14) module"]
pub type IMMU_TABLE14 = crate::Reg<u32, _IMMU_TABLE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE14;
#[doc = "`read()` method returns [immu_table14::R](immu_table14::R) reader structure"]
impl crate::Readable for IMMU_TABLE14 {}
#[doc = "`write(|w| ..)` method takes [immu_table14::W](immu_table14::W) writer structure"]
impl crate::Writable for IMMU_TABLE14 {}
#[doc = "DPORT_IMMU_TABLE14_REG"]
pub mod immu_table14;
#[doc = "DPORT_IMMU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [immu_table15](immu_table15) module"]
pub type IMMU_TABLE15 = crate::Reg<u32, _IMMU_TABLE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMMU_TABLE15;
#[doc = "`read()` method returns [immu_table15::R](immu_table15::R) reader structure"]
impl crate::Readable for IMMU_TABLE15 {}
#[doc = "`write(|w| ..)` method takes [immu_table15::W](immu_table15::W) writer structure"]
impl crate::Writable for IMMU_TABLE15 {}
#[doc = "DPORT_IMMU_TABLE15_REG"]
pub mod immu_table15;
#[doc = "DPORT_DMMU_TABLE0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table0](dmmu_table0) module"]
pub type DMMU_TABLE0 = crate::Reg<u32, _DMMU_TABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE0;
#[doc = "`read()` method returns [dmmu_table0::R](dmmu_table0::R) reader structure"]
impl crate::Readable for DMMU_TABLE0 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table0::W](dmmu_table0::W) writer structure"]
impl crate::Writable for DMMU_TABLE0 {}
#[doc = "DPORT_DMMU_TABLE0_REG"]
pub mod dmmu_table0;
#[doc = "DPORT_DMMU_TABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table1](dmmu_table1) module"]
pub type DMMU_TABLE1 = crate::Reg<u32, _DMMU_TABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE1;
#[doc = "`read()` method returns [dmmu_table1::R](dmmu_table1::R) reader structure"]
impl crate::Readable for DMMU_TABLE1 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table1::W](dmmu_table1::W) writer structure"]
impl crate::Writable for DMMU_TABLE1 {}
#[doc = "DPORT_DMMU_TABLE1_REG"]
pub mod dmmu_table1;
#[doc = "DPORT_DMMU_TABLE2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table2](dmmu_table2) module"]
pub type DMMU_TABLE2 = crate::Reg<u32, _DMMU_TABLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE2;
#[doc = "`read()` method returns [dmmu_table2::R](dmmu_table2::R) reader structure"]
impl crate::Readable for DMMU_TABLE2 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table2::W](dmmu_table2::W) writer structure"]
impl crate::Writable for DMMU_TABLE2 {}
#[doc = "DPORT_DMMU_TABLE2_REG"]
pub mod dmmu_table2;
#[doc = "DPORT_DMMU_TABLE3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table3](dmmu_table3) module"]
pub type DMMU_TABLE3 = crate::Reg<u32, _DMMU_TABLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE3;
#[doc = "`read()` method returns [dmmu_table3::R](dmmu_table3::R) reader structure"]
impl crate::Readable for DMMU_TABLE3 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table3::W](dmmu_table3::W) writer structure"]
impl crate::Writable for DMMU_TABLE3 {}
#[doc = "DPORT_DMMU_TABLE3_REG"]
pub mod dmmu_table3;
#[doc = "DPORT_DMMU_TABLE4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table4](dmmu_table4) module"]
pub type DMMU_TABLE4 = crate::Reg<u32, _DMMU_TABLE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE4;
#[doc = "`read()` method returns [dmmu_table4::R](dmmu_table4::R) reader structure"]
impl crate::Readable for DMMU_TABLE4 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table4::W](dmmu_table4::W) writer structure"]
impl crate::Writable for DMMU_TABLE4 {}
#[doc = "DPORT_DMMU_TABLE4_REG"]
pub mod dmmu_table4;
#[doc = "DPORT_DMMU_TABLE5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table5](dmmu_table5) module"]
pub type DMMU_TABLE5 = crate::Reg<u32, _DMMU_TABLE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE5;
#[doc = "`read()` method returns [dmmu_table5::R](dmmu_table5::R) reader structure"]
impl crate::Readable for DMMU_TABLE5 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table5::W](dmmu_table5::W) writer structure"]
impl crate::Writable for DMMU_TABLE5 {}
#[doc = "DPORT_DMMU_TABLE5_REG"]
pub mod dmmu_table5;
#[doc = "DPORT_DMMU_TABLE6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table6](dmmu_table6) module"]
pub type DMMU_TABLE6 = crate::Reg<u32, _DMMU_TABLE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE6;
#[doc = "`read()` method returns [dmmu_table6::R](dmmu_table6::R) reader structure"]
impl crate::Readable for DMMU_TABLE6 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table6::W](dmmu_table6::W) writer structure"]
impl crate::Writable for DMMU_TABLE6 {}
#[doc = "DPORT_DMMU_TABLE6_REG"]
pub mod dmmu_table6;
#[doc = "DPORT_DMMU_TABLE7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table7](dmmu_table7) module"]
pub type DMMU_TABLE7 = crate::Reg<u32, _DMMU_TABLE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE7;
#[doc = "`read()` method returns [dmmu_table7::R](dmmu_table7::R) reader structure"]
impl crate::Readable for DMMU_TABLE7 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table7::W](dmmu_table7::W) writer structure"]
impl crate::Writable for DMMU_TABLE7 {}
#[doc = "DPORT_DMMU_TABLE7_REG"]
pub mod dmmu_table7;
#[doc = "DPORT_DMMU_TABLE8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table8](dmmu_table8) module"]
pub type DMMU_TABLE8 = crate::Reg<u32, _DMMU_TABLE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE8;
#[doc = "`read()` method returns [dmmu_table8::R](dmmu_table8::R) reader structure"]
impl crate::Readable for DMMU_TABLE8 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table8::W](dmmu_table8::W) writer structure"]
impl crate::Writable for DMMU_TABLE8 {}
#[doc = "DPORT_DMMU_TABLE8_REG"]
pub mod dmmu_table8;
#[doc = "DPORT_DMMU_TABLE9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table9](dmmu_table9) module"]
pub type DMMU_TABLE9 = crate::Reg<u32, _DMMU_TABLE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE9;
#[doc = "`read()` method returns [dmmu_table9::R](dmmu_table9::R) reader structure"]
impl crate::Readable for DMMU_TABLE9 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table9::W](dmmu_table9::W) writer structure"]
impl crate::Writable for DMMU_TABLE9 {}
#[doc = "DPORT_DMMU_TABLE9_REG"]
pub mod dmmu_table9;
#[doc = "DPORT_DMMU_TABLE10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table10](dmmu_table10) module"]
pub type DMMU_TABLE10 = crate::Reg<u32, _DMMU_TABLE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE10;
#[doc = "`read()` method returns [dmmu_table10::R](dmmu_table10::R) reader structure"]
impl crate::Readable for DMMU_TABLE10 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table10::W](dmmu_table10::W) writer structure"]
impl crate::Writable for DMMU_TABLE10 {}
#[doc = "DPORT_DMMU_TABLE10_REG"]
pub mod dmmu_table10;
#[doc = "DPORT_DMMU_TABLE11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table11](dmmu_table11) module"]
pub type DMMU_TABLE11 = crate::Reg<u32, _DMMU_TABLE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE11;
#[doc = "`read()` method returns [dmmu_table11::R](dmmu_table11::R) reader structure"]
impl crate::Readable for DMMU_TABLE11 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table11::W](dmmu_table11::W) writer structure"]
impl crate::Writable for DMMU_TABLE11 {}
#[doc = "DPORT_DMMU_TABLE11_REG"]
pub mod dmmu_table11;
#[doc = "DPORT_DMMU_TABLE12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table12](dmmu_table12) module"]
pub type DMMU_TABLE12 = crate::Reg<u32, _DMMU_TABLE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE12;
#[doc = "`read()` method returns [dmmu_table12::R](dmmu_table12::R) reader structure"]
impl crate::Readable for DMMU_TABLE12 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table12::W](dmmu_table12::W) writer structure"]
impl crate::Writable for DMMU_TABLE12 {}
#[doc = "DPORT_DMMU_TABLE12_REG"]
pub mod dmmu_table12;
#[doc = "DPORT_DMMU_TABLE13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table13](dmmu_table13) module"]
pub type DMMU_TABLE13 = crate::Reg<u32, _DMMU_TABLE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE13;
#[doc = "`read()` method returns [dmmu_table13::R](dmmu_table13::R) reader structure"]
impl crate::Readable for DMMU_TABLE13 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table13::W](dmmu_table13::W) writer structure"]
impl crate::Writable for DMMU_TABLE13 {}
#[doc = "DPORT_DMMU_TABLE13_REG"]
pub mod dmmu_table13;
#[doc = "DPORT_DMMU_TABLE14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table14](dmmu_table14) module"]
pub type DMMU_TABLE14 = crate::Reg<u32, _DMMU_TABLE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE14;
#[doc = "`read()` method returns [dmmu_table14::R](dmmu_table14::R) reader structure"]
impl crate::Readable for DMMU_TABLE14 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table14::W](dmmu_table14::W) writer structure"]
impl crate::Writable for DMMU_TABLE14 {}
#[doc = "DPORT_DMMU_TABLE14_REG"]
pub mod dmmu_table14;
#[doc = "DPORT_DMMU_TABLE15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmmu_table15](dmmu_table15) module"]
pub type DMMU_TABLE15 = crate::Reg<u32, _DMMU_TABLE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMMU_TABLE15;
#[doc = "`read()` method returns [dmmu_table15::R](dmmu_table15::R) reader structure"]
impl crate::Readable for DMMU_TABLE15 {}
#[doc = "`write(|w| ..)` method takes [dmmu_table15::W](dmmu_table15::W) writer structure"]
impl crate::Writable for DMMU_TABLE15 {}
#[doc = "DPORT_DMMU_TABLE15_REG"]
pub mod dmmu_table15;
#[doc = "DPORT_PRO_INTRUSION_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_intrusion_ctrl](pro_intrusion_ctrl) module"]
pub type PRO_INTRUSION_CTRL = crate::Reg<u32, _PRO_INTRUSION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_INTRUSION_CTRL;
#[doc = "`read()` method returns [pro_intrusion_ctrl::R](pro_intrusion_ctrl::R) reader structure"]
impl crate::Readable for PRO_INTRUSION_CTRL {}
#[doc = "`write(|w| ..)` method takes [pro_intrusion_ctrl::W](pro_intrusion_ctrl::W) writer structure"]
impl crate::Writable for PRO_INTRUSION_CTRL {}
#[doc = "DPORT_PRO_INTRUSION_CTRL_REG"]
pub mod pro_intrusion_ctrl;
#[doc = "DPORT_PRO_INTRUSION_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_intrusion_status](pro_intrusion_status) module"]
pub type PRO_INTRUSION_STATUS = crate::Reg<u32, _PRO_INTRUSION_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_INTRUSION_STATUS;
#[doc = "`read()` method returns [pro_intrusion_status::R](pro_intrusion_status::R) reader structure"]
impl crate::Readable for PRO_INTRUSION_STATUS {}
#[doc = "`write(|w| ..)` method takes [pro_intrusion_status::W](pro_intrusion_status::W) writer structure"]
impl crate::Writable for PRO_INTRUSION_STATUS {}
#[doc = "DPORT_PRO_INTRUSION_STATUS_REG"]
pub mod pro_intrusion_status;
#[doc = "DPORT_APP_INTRUSION_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_intrusion_ctrl](app_intrusion_ctrl) module"]
pub type APP_INTRUSION_CTRL = crate::Reg<u32, _APP_INTRUSION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_INTRUSION_CTRL;
#[doc = "`read()` method returns [app_intrusion_ctrl::R](app_intrusion_ctrl::R) reader structure"]
impl crate::Readable for APP_INTRUSION_CTRL {}
#[doc = "`write(|w| ..)` method takes [app_intrusion_ctrl::W](app_intrusion_ctrl::W) writer structure"]
impl crate::Writable for APP_INTRUSION_CTRL {}
#[doc = "DPORT_APP_INTRUSION_CTRL_REG"]
pub mod app_intrusion_ctrl;
#[doc = "DPORT_APP_INTRUSION_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_intrusion_status](app_intrusion_status) module"]
pub type APP_INTRUSION_STATUS = crate::Reg<u32, _APP_INTRUSION_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_INTRUSION_STATUS;
#[doc = "`read()` method returns [app_intrusion_status::R](app_intrusion_status::R) reader structure"]
impl crate::Readable for APP_INTRUSION_STATUS {}
#[doc = "`write(|w| ..)` method takes [app_intrusion_status::W](app_intrusion_status::W) writer structure"]
impl crate::Writable for APP_INTRUSION_STATUS {}
#[doc = "DPORT_APP_INTRUSION_STATUS_REG"]
pub mod app_intrusion_status;
#[doc = "DPORT_FRONT_END_MEM_PD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [front_end_mem_pd](front_end_mem_pd) module"]
pub type FRONT_END_MEM_PD = crate::Reg<u32, _FRONT_END_MEM_PD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRONT_END_MEM_PD;
#[doc = "`read()` method returns [front_end_mem_pd::R](front_end_mem_pd::R) reader structure"]
impl crate::Readable for FRONT_END_MEM_PD {}
#[doc = "`write(|w| ..)` method takes [front_end_mem_pd::W](front_end_mem_pd::W) writer structure"]
impl crate::Writable for FRONT_END_MEM_PD {}
#[doc = "DPORT_FRONT_END_MEM_PD_REG"]
pub mod front_end_mem_pd;
#[doc = "DPORT_MMU_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmu_ia_int_en](mmu_ia_int_en) module"]
pub type MMU_IA_INT_EN = crate::Reg<u32, _MMU_IA_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMU_IA_INT_EN;
#[doc = "`read()` method returns [mmu_ia_int_en::R](mmu_ia_int_en::R) reader structure"]
impl crate::Readable for MMU_IA_INT_EN {}
#[doc = "`write(|w| ..)` method takes [mmu_ia_int_en::W](mmu_ia_int_en::W) writer structure"]
impl crate::Writable for MMU_IA_INT_EN {}
#[doc = "DPORT_MMU_IA_INT_EN_REG"]
pub mod mmu_ia_int_en;
#[doc = "DPORT_MPU_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpu_ia_int_en](mpu_ia_int_en) module"]
pub type MPU_IA_INT_EN = crate::Reg<u32, _MPU_IA_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_IA_INT_EN;
#[doc = "`read()` method returns [mpu_ia_int_en::R](mpu_ia_int_en::R) reader structure"]
impl crate::Readable for MPU_IA_INT_EN {}
#[doc = "`write(|w| ..)` method takes [mpu_ia_int_en::W](mpu_ia_int_en::W) writer structure"]
impl crate::Writable for MPU_IA_INT_EN {}
#[doc = "DPORT_MPU_IA_INT_EN_REG"]
pub mod mpu_ia_int_en;
#[doc = "DPORT_CACHE_IA_INT_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cache_ia_int_en](cache_ia_int_en) module"]
pub type CACHE_IA_INT_EN = crate::Reg<u32, _CACHE_IA_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACHE_IA_INT_EN;
#[doc = "`read()` method returns [cache_ia_int_en::R](cache_ia_int_en::R) reader structure"]
impl crate::Readable for CACHE_IA_INT_EN {}
#[doc = "`write(|w| ..)` method takes [cache_ia_int_en::W](cache_ia_int_en::W) writer structure"]
impl crate::Writable for CACHE_IA_INT_EN {}
#[doc = "DPORT_CACHE_IA_INT_EN_REG"]
pub mod cache_ia_int_en;
#[doc = "DPORT_SECURE_BOOT_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secure_boot_ctrl](secure_boot_ctrl) module"]
pub type SECURE_BOOT_CTRL = crate::Reg<u32, _SECURE_BOOT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECURE_BOOT_CTRL;
#[doc = "`read()` method returns [secure_boot_ctrl::R](secure_boot_ctrl::R) reader structure"]
impl crate::Readable for SECURE_BOOT_CTRL {}
#[doc = "`write(|w| ..)` method takes [secure_boot_ctrl::W](secure_boot_ctrl::W) writer structure"]
impl crate::Writable for SECURE_BOOT_CTRL {}
#[doc = "DPORT_SECURE_BOOT_CTRL_REG"]
pub mod secure_boot_ctrl;
#[doc = "DPORT_SPI_DMA_CHAN_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_dma_chan_sel](spi_dma_chan_sel) module"]
pub type SPI_DMA_CHAN_SEL = crate::Reg<u32, _SPI_DMA_CHAN_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_CHAN_SEL;
#[doc = "`read()` method returns [spi_dma_chan_sel::R](spi_dma_chan_sel::R) reader structure"]
impl crate::Readable for SPI_DMA_CHAN_SEL {}
#[doc = "`write(|w| ..)` method takes [spi_dma_chan_sel::W](spi_dma_chan_sel::W) writer structure"]
impl crate::Writable for SPI_DMA_CHAN_SEL {}
#[doc = "DPORT_SPI_DMA_CHAN_SEL_REG"]
pub mod spi_dma_chan_sel;
#[doc = "DPORT_PRO_VECBASE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_vecbase_ctrl](pro_vecbase_ctrl) module"]
pub type PRO_VECBASE_CTRL = crate::Reg<u32, _PRO_VECBASE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_VECBASE_CTRL;
#[doc = "`read()` method returns [pro_vecbase_ctrl::R](pro_vecbase_ctrl::R) reader structure"]
impl crate::Readable for PRO_VECBASE_CTRL {}
#[doc = "`write(|w| ..)` method takes [pro_vecbase_ctrl::W](pro_vecbase_ctrl::W) writer structure"]
impl crate::Writable for PRO_VECBASE_CTRL {}
#[doc = "DPORT_PRO_VECBASE_CTRL_REG"]
pub mod pro_vecbase_ctrl;
#[doc = "DPORT_PRO_VECBASE_SET_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pro_vecbase_set](pro_vecbase_set) module"]
pub type PRO_VECBASE_SET = crate::Reg<u32, _PRO_VECBASE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRO_VECBASE_SET;
#[doc = "`read()` method returns [pro_vecbase_set::R](pro_vecbase_set::R) reader structure"]
impl crate::Readable for PRO_VECBASE_SET {}
#[doc = "`write(|w| ..)` method takes [pro_vecbase_set::W](pro_vecbase_set::W) writer structure"]
impl crate::Writable for PRO_VECBASE_SET {}
#[doc = "DPORT_PRO_VECBASE_SET_REG"]
pub mod pro_vecbase_set;
#[doc = "DPORT_APP_VECBASE_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_vecbase_ctrl](app_vecbase_ctrl) module"]
pub type APP_VECBASE_CTRL = crate::Reg<u32, _APP_VECBASE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_VECBASE_CTRL;
#[doc = "`read()` method returns [app_vecbase_ctrl::R](app_vecbase_ctrl::R) reader structure"]
impl crate::Readable for APP_VECBASE_CTRL {}
#[doc = "`write(|w| ..)` method takes [app_vecbase_ctrl::W](app_vecbase_ctrl::W) writer structure"]
impl crate::Writable for APP_VECBASE_CTRL {}
#[doc = "DPORT_APP_VECBASE_CTRL_REG"]
pub mod app_vecbase_ctrl;
#[doc = "DPORT_APP_VECBASE_SET_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [app_vecbase_set](app_vecbase_set) module"]
pub type APP_VECBASE_SET = crate::Reg<u32, _APP_VECBASE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APP_VECBASE_SET;
#[doc = "`read()` method returns [app_vecbase_set::R](app_vecbase_set::R) reader structure"]
impl crate::Readable for APP_VECBASE_SET {}
#[doc = "`write(|w| ..)` method takes [app_vecbase_set::W](app_vecbase_set::W) writer structure"]
impl crate::Writable for APP_VECBASE_SET {}
#[doc = "DPORT_APP_VECBASE_SET_REG"]
pub mod app_vecbase_set;
#[doc = "DPORT_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "DPORT_DATE_REG"]
pub mod date;
