#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_BT_SELECT"]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - GPIO_OUT"]
    pub out: OUT,
    #[doc = "0x08 - GPIO_OUT_W1TS"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - GPIO_OUT_W1TC"]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x10 - GPIO_OUT1"]
    pub out1: OUT1,
    #[doc = "0x14 - GPIO_OUT1_W1TS"]
    pub out1_w1ts: OUT1_W1TS,
    #[doc = "0x18 - GPIO_OUT1_W1TC"]
    pub out1_w1tc: OUT1_W1TC,
    #[doc = "0x1c - GPIO_SDIO_SELECT"]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - GPIO_ENABLE"]
    pub enable: ENABLE,
    #[doc = "0x24 - GPIO_ENABLE_W1TS"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - GPIO_ENABLE_W1TC"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x2c - GPIO_ENABLE1"]
    pub enable1: ENABLE1,
    #[doc = "0x30 - GPIO_ENABLE1_W1TS"]
    pub enable1_w1ts: ENABLE1_W1TS,
    #[doc = "0x34 - GPIO_ENABLE1_W1TC"]
    pub enable1_w1tc: ENABLE1_W1TC,
    #[doc = "0x38 - GPIO_STRAP"]
    pub strap: STRAP,
    #[doc = "0x3c - GPIO_IN"]
    pub in_: IN,
    #[doc = "0x40 - GPIO_IN1"]
    pub in1: IN1,
    #[doc = "0x44 - GPIO_STATUS"]
    pub status: STATUS,
    #[doc = "0x48 - GPIO_STATUS_W1TS"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - GPIO_STATUS_W1TC"]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x50 - GPIO_STATUS1"]
    pub status1: STATUS1,
    #[doc = "0x54 - GPIO_STATUS1_W1TS"]
    pub status1_w1ts: STATUS1_W1TS,
    #[doc = "0x58 - GPIO_STATUS1_W1TC"]
    pub status1_w1tc: STATUS1_W1TC,
    _reserved23: [u8; 4usize],
    #[doc = "0x60 - GPIO_ACPU_INT"]
    pub acpu_int: ACPU_INT,
    #[doc = "0x64 - GPIO_ACPU_NMI_INT"]
    pub acpu_nmi_int: ACPU_NMI_INT,
    #[doc = "0x68 - GPIO_PCPU_INT"]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x6c - GPIO_PCPU_NMI_INT"]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x70 - GPIO_CPUSDIO_INT"]
    pub cpusdio_int: CPUSDIO_INT,
    #[doc = "0x74 - GPIO_ACPU_INT1"]
    pub acpu_int1: ACPU_INT1,
    #[doc = "0x78 - GPIO_ACPU_NMI_INT1"]
    pub acpu_nmi_int1: ACPU_NMI_INT1,
    #[doc = "0x7c - GPIO_PCPU_INT1"]
    pub pcpu_int1: PCPU_INT1,
    #[doc = "0x80 - GPIO_PCPU_NMI_INT1"]
    pub pcpu_nmi_int1: PCPU_NMI_INT1,
    #[doc = "0x84 - GPIO_CPUSDIO_INT1"]
    pub cpusdio_int1: CPUSDIO_INT1,
    #[doc = "0x88 - GPIO_PIN0"]
    pub pin0: PIN0,
    #[doc = "0x8c - GPIO_PIN1"]
    pub pin1: PIN1,
    #[doc = "0x90 - GPIO_PIN2"]
    pub pin2: PIN2,
    #[doc = "0x94 - GPIO_PIN3"]
    pub pin3: PIN3,
    #[doc = "0x98 - GPIO_PIN4"]
    pub pin4: PIN4,
    #[doc = "0x9c - GPIO_PIN5"]
    pub pin5: PIN5,
    #[doc = "0xa0 - GPIO_PIN6"]
    pub pin6: PIN6,
    #[doc = "0xa4 - GPIO_PIN7"]
    pub pin7: PIN7,
    #[doc = "0xa8 - GPIO_PIN8"]
    pub pin8: PIN8,
    #[doc = "0xac - GPIO_PIN9"]
    pub pin9: PIN9,
    #[doc = "0xb0 - GPIO_PIN10"]
    pub pin10: PIN10,
    #[doc = "0xb4 - GPIO_PIN11"]
    pub pin11: PIN11,
    #[doc = "0xb8 - GPIO_PIN12"]
    pub pin12: PIN12,
    #[doc = "0xbc - GPIO_PIN13"]
    pub pin13: PIN13,
    #[doc = "0xc0 - GPIO_PIN14"]
    pub pin14: PIN14,
    #[doc = "0xc4 - GPIO_PIN15"]
    pub pin15: PIN15,
    #[doc = "0xc8 - GPIO_PIN16"]
    pub pin16: PIN16,
    #[doc = "0xcc - GPIO_PIN17"]
    pub pin17: PIN17,
    #[doc = "0xd0 - GPIO_PIN18"]
    pub pin18: PIN18,
    #[doc = "0xd4 - GPIO_PIN19"]
    pub pin19: PIN19,
    #[doc = "0xd8 - GPIO_PIN20"]
    pub pin20: PIN20,
    #[doc = "0xdc - GPIO_PIN21"]
    pub pin21: PIN21,
    #[doc = "0xe0 - GPIO_PIN22"]
    pub pin22: PIN22,
    #[doc = "0xe4 - GPIO_PIN23"]
    pub pin23: PIN23,
    #[doc = "0xe8 - GPIO_PIN24"]
    pub pin24: PIN24,
    #[doc = "0xec - GPIO_PIN25"]
    pub pin25: PIN25,
    #[doc = "0xf0 - GPIO_PIN26"]
    pub pin26: PIN26,
    #[doc = "0xf4 - GPIO_PIN27"]
    pub pin27: PIN27,
    #[doc = "0xf8 - GPIO_PIN28"]
    pub pin28: PIN28,
    #[doc = "0xfc - GPIO_PIN29"]
    pub pin29: PIN29,
    #[doc = "0x100 - GPIO_PIN30"]
    pub pin30: PIN30,
    #[doc = "0x104 - GPIO_PIN31"]
    pub pin31: PIN31,
    #[doc = "0x108 - GPIO_PIN32"]
    pub pin32: PIN32,
    #[doc = "0x10c - GPIO_PIN33"]
    pub pin33: PIN33,
    #[doc = "0x110 - GPIO_PIN34"]
    pub pin34: PIN34,
    #[doc = "0x114 - GPIO_PIN35"]
    pub pin35: PIN35,
    #[doc = "0x118 - GPIO_PIN36"]
    pub pin36: PIN36,
    #[doc = "0x11c - GPIO_PIN37"]
    pub pin37: PIN37,
    #[doc = "0x120 - GPIO_PIN38"]
    pub pin38: PIN38,
    #[doc = "0x124 - GPIO_PIN39"]
    pub pin39: PIN39,
    #[doc = "0x128 - GPIO_cali_conf"]
    pub cali_conf: CALI_CONF,
    #[doc = "0x12c - GPIO_cali_data"]
    pub cali_data: CALI_DATA,
    #[doc = "0x130 - GPIO_FUNC0_IN_SEL_CFG"]
    pub func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    #[doc = "0x134 - GPIO_FUNC1_IN_SEL_CFG"]
    pub func1_in_sel_cfg: FUNC1_IN_SEL_CFG,
    #[doc = "0x138 - GPIO_FUNC2_IN_SEL_CFG"]
    pub func2_in_sel_cfg: FUNC2_IN_SEL_CFG,
    #[doc = "0x13c - GPIO_FUNC3_IN_SEL_CFG"]
    pub func3_in_sel_cfg: FUNC3_IN_SEL_CFG,
    #[doc = "0x140 - GPIO_FUNC4_IN_SEL_CFG"]
    pub func4_in_sel_cfg: FUNC4_IN_SEL_CFG,
    #[doc = "0x144 - GPIO_FUNC5_IN_SEL_CFG"]
    pub func5_in_sel_cfg: FUNC5_IN_SEL_CFG,
    #[doc = "0x148 - GPIO_FUNC6_IN_SEL_CFG"]
    pub func6_in_sel_cfg: FUNC6_IN_SEL_CFG,
    #[doc = "0x14c - GPIO_FUNC7_IN_SEL_CFG"]
    pub func7_in_sel_cfg: FUNC7_IN_SEL_CFG,
    #[doc = "0x150 - GPIO_FUNC8_IN_SEL_CFG"]
    pub func8_in_sel_cfg: FUNC8_IN_SEL_CFG,
    #[doc = "0x154 - GPIO_FUNC9_IN_SEL_CFG"]
    pub func9_in_sel_cfg: FUNC9_IN_SEL_CFG,
    #[doc = "0x158 - GPIO_FUNC10_IN_SEL_CFG"]
    pub func10_in_sel_cfg: FUNC10_IN_SEL_CFG,
    #[doc = "0x15c - GPIO_FUNC11_IN_SEL_CFG"]
    pub func11_in_sel_cfg: FUNC11_IN_SEL_CFG,
    #[doc = "0x160 - GPIO_FUNC12_IN_SEL_CFG"]
    pub func12_in_sel_cfg: FUNC12_IN_SEL_CFG,
    #[doc = "0x164 - GPIO_FUNC13_IN_SEL_CFG"]
    pub func13_in_sel_cfg: FUNC13_IN_SEL_CFG,
    #[doc = "0x168 - GPIO_FUNC14_IN_SEL_CFG"]
    pub func14_in_sel_cfg: FUNC14_IN_SEL_CFG,
    #[doc = "0x16c - GPIO_FUNC15_IN_SEL_CFG"]
    pub func15_in_sel_cfg: FUNC15_IN_SEL_CFG,
    #[doc = "0x170 - GPIO_FUNC16_IN_SEL_CFG"]
    pub func16_in_sel_cfg: FUNC16_IN_SEL_CFG,
    #[doc = "0x174 - GPIO_FUNC17_IN_SEL_CFG"]
    pub func17_in_sel_cfg: FUNC17_IN_SEL_CFG,
    #[doc = "0x178 - GPIO_FUNC18_IN_SEL_CFG"]
    pub func18_in_sel_cfg: FUNC18_IN_SEL_CFG,
    #[doc = "0x17c - GPIO_FUNC19_IN_SEL_CFG"]
    pub func19_in_sel_cfg: FUNC19_IN_SEL_CFG,
    #[doc = "0x180 - GPIO_FUNC20_IN_SEL_CFG"]
    pub func20_in_sel_cfg: FUNC20_IN_SEL_CFG,
    #[doc = "0x184 - GPIO_FUNC21_IN_SEL_CFG"]
    pub func21_in_sel_cfg: FUNC21_IN_SEL_CFG,
    #[doc = "0x188 - GPIO_FUNC22_IN_SEL_CFG"]
    pub func22_in_sel_cfg: FUNC22_IN_SEL_CFG,
    #[doc = "0x18c - GPIO_FUNC23_IN_SEL_CFG"]
    pub func23_in_sel_cfg: FUNC23_IN_SEL_CFG,
    #[doc = "0x190 - GPIO_FUNC24_IN_SEL_CFG"]
    pub func24_in_sel_cfg: FUNC24_IN_SEL_CFG,
    #[doc = "0x194 - GPIO_FUNC25_IN_SEL_CFG"]
    pub func25_in_sel_cfg: FUNC25_IN_SEL_CFG,
    #[doc = "0x198 - GPIO_FUNC26_IN_SEL_CFG"]
    pub func26_in_sel_cfg: FUNC26_IN_SEL_CFG,
    #[doc = "0x19c - GPIO_FUNC27_IN_SEL_CFG"]
    pub func27_in_sel_cfg: FUNC27_IN_SEL_CFG,
    #[doc = "0x1a0 - GPIO_FUNC28_IN_SEL_CFG"]
    pub func28_in_sel_cfg: FUNC28_IN_SEL_CFG,
    #[doc = "0x1a4 - GPIO_FUNC29_IN_SEL_CFG"]
    pub func29_in_sel_cfg: FUNC29_IN_SEL_CFG,
    #[doc = "0x1a8 - GPIO_FUNC30_IN_SEL_CFG"]
    pub func30_in_sel_cfg: FUNC30_IN_SEL_CFG,
    #[doc = "0x1ac - GPIO_FUNC31_IN_SEL_CFG"]
    pub func31_in_sel_cfg: FUNC31_IN_SEL_CFG,
    #[doc = "0x1b0 - GPIO_FUNC32_IN_SEL_CFG"]
    pub func32_in_sel_cfg: FUNC32_IN_SEL_CFG,
    #[doc = "0x1b4 - GPIO_FUNC33_IN_SEL_CFG"]
    pub func33_in_sel_cfg: FUNC33_IN_SEL_CFG,
    #[doc = "0x1b8 - GPIO_FUNC34_IN_SEL_CFG"]
    pub func34_in_sel_cfg: FUNC34_IN_SEL_CFG,
    #[doc = "0x1bc - GPIO_FUNC35_IN_SEL_CFG"]
    pub func35_in_sel_cfg: FUNC35_IN_SEL_CFG,
    #[doc = "0x1c0 - GPIO_FUNC36_IN_SEL_CFG"]
    pub func36_in_sel_cfg: FUNC36_IN_SEL_CFG,
    #[doc = "0x1c4 - GPIO_FUNC37_IN_SEL_CFG"]
    pub func37_in_sel_cfg: FUNC37_IN_SEL_CFG,
    #[doc = "0x1c8 - GPIO_FUNC38_IN_SEL_CFG"]
    pub func38_in_sel_cfg: FUNC38_IN_SEL_CFG,
    #[doc = "0x1cc - GPIO_FUNC39_IN_SEL_CFG"]
    pub func39_in_sel_cfg: FUNC39_IN_SEL_CFG,
    #[doc = "0x1d0 - GPIO_FUNC40_IN_SEL_CFG"]
    pub func40_in_sel_cfg: FUNC40_IN_SEL_CFG,
    #[doc = "0x1d4 - GPIO_FUNC41_IN_SEL_CFG"]
    pub func41_in_sel_cfg: FUNC41_IN_SEL_CFG,
    #[doc = "0x1d8 - GPIO_FUNC42_IN_SEL_CFG"]
    pub func42_in_sel_cfg: FUNC42_IN_SEL_CFG,
    #[doc = "0x1dc - GPIO_FUNC43_IN_SEL_CFG"]
    pub func43_in_sel_cfg: FUNC43_IN_SEL_CFG,
    #[doc = "0x1e0 - GPIO_FUNC44_IN_SEL_CFG"]
    pub func44_in_sel_cfg: FUNC44_IN_SEL_CFG,
    #[doc = "0x1e4 - GPIO_FUNC45_IN_SEL_CFG"]
    pub func45_in_sel_cfg: FUNC45_IN_SEL_CFG,
    #[doc = "0x1e8 - GPIO_FUNC46_IN_SEL_CFG"]
    pub func46_in_sel_cfg: FUNC46_IN_SEL_CFG,
    #[doc = "0x1ec - GPIO_FUNC47_IN_SEL_CFG"]
    pub func47_in_sel_cfg: FUNC47_IN_SEL_CFG,
    #[doc = "0x1f0 - GPIO_FUNC48_IN_SEL_CFG"]
    pub func48_in_sel_cfg: FUNC48_IN_SEL_CFG,
    #[doc = "0x1f4 - GPIO_FUNC49_IN_SEL_CFG"]
    pub func49_in_sel_cfg: FUNC49_IN_SEL_CFG,
    #[doc = "0x1f8 - GPIO_FUNC50_IN_SEL_CFG"]
    pub func50_in_sel_cfg: FUNC50_IN_SEL_CFG,
    #[doc = "0x1fc - GPIO_FUNC51_IN_SEL_CFG"]
    pub func51_in_sel_cfg: FUNC51_IN_SEL_CFG,
    #[doc = "0x200 - GPIO_FUNC52_IN_SEL_CFG"]
    pub func52_in_sel_cfg: FUNC52_IN_SEL_CFG,
    #[doc = "0x204 - GPIO_FUNC53_IN_SEL_CFG"]
    pub func53_in_sel_cfg: FUNC53_IN_SEL_CFG,
    #[doc = "0x208 - GPIO_FUNC54_IN_SEL_CFG"]
    pub func54_in_sel_cfg: FUNC54_IN_SEL_CFG,
    #[doc = "0x20c - GPIO_FUNC55_IN_SEL_CFG"]
    pub func55_in_sel_cfg: FUNC55_IN_SEL_CFG,
    #[doc = "0x210 - GPIO_FUNC56_IN_SEL_CFG"]
    pub func56_in_sel_cfg: FUNC56_IN_SEL_CFG,
    #[doc = "0x214 - GPIO_FUNC57_IN_SEL_CFG"]
    pub func57_in_sel_cfg: FUNC57_IN_SEL_CFG,
    #[doc = "0x218 - GPIO_FUNC58_IN_SEL_CFG"]
    pub func58_in_sel_cfg: FUNC58_IN_SEL_CFG,
    #[doc = "0x21c - GPIO_FUNC59_IN_SEL_CFG"]
    pub func59_in_sel_cfg: FUNC59_IN_SEL_CFG,
    #[doc = "0x220 - GPIO_FUNC60_IN_SEL_CFG"]
    pub func60_in_sel_cfg: FUNC60_IN_SEL_CFG,
    #[doc = "0x224 - GPIO_FUNC61_IN_SEL_CFG"]
    pub func61_in_sel_cfg: FUNC61_IN_SEL_CFG,
    #[doc = "0x228 - GPIO_FUNC62_IN_SEL_CFG"]
    pub func62_in_sel_cfg: FUNC62_IN_SEL_CFG,
    #[doc = "0x22c - GPIO_FUNC63_IN_SEL_CFG"]
    pub func63_in_sel_cfg: FUNC63_IN_SEL_CFG,
    #[doc = "0x230 - GPIO_FUNC64_IN_SEL_CFG"]
    pub func64_in_sel_cfg: FUNC64_IN_SEL_CFG,
    #[doc = "0x234 - GPIO_FUNC65_IN_SEL_CFG"]
    pub func65_in_sel_cfg: FUNC65_IN_SEL_CFG,
    #[doc = "0x238 - GPIO_FUNC66_IN_SEL_CFG"]
    pub func66_in_sel_cfg: FUNC66_IN_SEL_CFG,
    #[doc = "0x23c - GPIO_FUNC67_IN_SEL_CFG"]
    pub func67_in_sel_cfg: FUNC67_IN_SEL_CFG,
    #[doc = "0x240 - GPIO_FUNC68_IN_SEL_CFG"]
    pub func68_in_sel_cfg: FUNC68_IN_SEL_CFG,
    #[doc = "0x244 - GPIO_FUNC69_IN_SEL_CFG"]
    pub func69_in_sel_cfg: FUNC69_IN_SEL_CFG,
    #[doc = "0x248 - GPIO_FUNC70_IN_SEL_CFG"]
    pub func70_in_sel_cfg: FUNC70_IN_SEL_CFG,
    #[doc = "0x24c - GPIO_FUNC71_IN_SEL_CFG"]
    pub func71_in_sel_cfg: FUNC71_IN_SEL_CFG,
    #[doc = "0x250 - GPIO_FUNC72_IN_SEL_CFG"]
    pub func72_in_sel_cfg: FUNC72_IN_SEL_CFG,
    #[doc = "0x254 - GPIO_FUNC73_IN_SEL_CFG"]
    pub func73_in_sel_cfg: FUNC73_IN_SEL_CFG,
    #[doc = "0x258 - GPIO_FUNC74_IN_SEL_CFG"]
    pub func74_in_sel_cfg: FUNC74_IN_SEL_CFG,
    #[doc = "0x25c - GPIO_FUNC75_IN_SEL_CFG"]
    pub func75_in_sel_cfg: FUNC75_IN_SEL_CFG,
    #[doc = "0x260 - GPIO_FUNC76_IN_SEL_CFG"]
    pub func76_in_sel_cfg: FUNC76_IN_SEL_CFG,
    #[doc = "0x264 - GPIO_FUNC77_IN_SEL_CFG"]
    pub func77_in_sel_cfg: FUNC77_IN_SEL_CFG,
    #[doc = "0x268 - GPIO_FUNC78_IN_SEL_CFG"]
    pub func78_in_sel_cfg: FUNC78_IN_SEL_CFG,
    #[doc = "0x26c - GPIO_FUNC79_IN_SEL_CFG"]
    pub func79_in_sel_cfg: FUNC79_IN_SEL_CFG,
    #[doc = "0x270 - GPIO_FUNC80_IN_SEL_CFG"]
    pub func80_in_sel_cfg: FUNC80_IN_SEL_CFG,
    #[doc = "0x274 - GPIO_FUNC81_IN_SEL_CFG"]
    pub func81_in_sel_cfg: FUNC81_IN_SEL_CFG,
    #[doc = "0x278 - GPIO_FUNC82_IN_SEL_CFG"]
    pub func82_in_sel_cfg: FUNC82_IN_SEL_CFG,
    #[doc = "0x27c - GPIO_FUNC83_IN_SEL_CFG"]
    pub func83_in_sel_cfg: FUNC83_IN_SEL_CFG,
    #[doc = "0x280 - GPIO_FUNC84_IN_SEL_CFG"]
    pub func84_in_sel_cfg: FUNC84_IN_SEL_CFG,
    #[doc = "0x284 - GPIO_FUNC85_IN_SEL_CFG"]
    pub func85_in_sel_cfg: FUNC85_IN_SEL_CFG,
    #[doc = "0x288 - GPIO_FUNC86_IN_SEL_CFG"]
    pub func86_in_sel_cfg: FUNC86_IN_SEL_CFG,
    #[doc = "0x28c - GPIO_FUNC87_IN_SEL_CFG"]
    pub func87_in_sel_cfg: FUNC87_IN_SEL_CFG,
    #[doc = "0x290 - GPIO_FUNC88_IN_SEL_CFG"]
    pub func88_in_sel_cfg: FUNC88_IN_SEL_CFG,
    #[doc = "0x294 - GPIO_FUNC89_IN_SEL_CFG"]
    pub func89_in_sel_cfg: FUNC89_IN_SEL_CFG,
    #[doc = "0x298 - GPIO_FUNC90_IN_SEL_CFG"]
    pub func90_in_sel_cfg: FUNC90_IN_SEL_CFG,
    #[doc = "0x29c - GPIO_FUNC91_IN_SEL_CFG"]
    pub func91_in_sel_cfg: FUNC91_IN_SEL_CFG,
    #[doc = "0x2a0 - GPIO_FUNC92_IN_SEL_CFG"]
    pub func92_in_sel_cfg: FUNC92_IN_SEL_CFG,
    #[doc = "0x2a4 - GPIO_FUNC93_IN_SEL_CFG"]
    pub func93_in_sel_cfg: FUNC93_IN_SEL_CFG,
    #[doc = "0x2a8 - GPIO_FUNC94_IN_SEL_CFG"]
    pub func94_in_sel_cfg: FUNC94_IN_SEL_CFG,
    #[doc = "0x2ac - GPIO_FUNC95_IN_SEL_CFG"]
    pub func95_in_sel_cfg: FUNC95_IN_SEL_CFG,
    #[doc = "0x2b0 - GPIO_FUNC96_IN_SEL_CFG"]
    pub func96_in_sel_cfg: FUNC96_IN_SEL_CFG,
    #[doc = "0x2b4 - GPIO_FUNC97_IN_SEL_CFG"]
    pub func97_in_sel_cfg: FUNC97_IN_SEL_CFG,
    #[doc = "0x2b8 - GPIO_FUNC98_IN_SEL_CFG"]
    pub func98_in_sel_cfg: FUNC98_IN_SEL_CFG,
    #[doc = "0x2bc - GPIO_FUNC99_IN_SEL_CFG"]
    pub func99_in_sel_cfg: FUNC99_IN_SEL_CFG,
    #[doc = "0x2c0 - GPIO_FUNC100_IN_SEL_CFG"]
    pub func100_in_sel_cfg: FUNC100_IN_SEL_CFG,
    #[doc = "0x2c4 - GPIO_FUNC101_IN_SEL_CFG"]
    pub func101_in_sel_cfg: FUNC101_IN_SEL_CFG,
    #[doc = "0x2c8 - GPIO_FUNC102_IN_SEL_CFG"]
    pub func102_in_sel_cfg: FUNC102_IN_SEL_CFG,
    #[doc = "0x2cc - GPIO_FUNC103_IN_SEL_CFG"]
    pub func103_in_sel_cfg: FUNC103_IN_SEL_CFG,
    #[doc = "0x2d0 - GPIO_FUNC104_IN_SEL_CFG"]
    pub func104_in_sel_cfg: FUNC104_IN_SEL_CFG,
    #[doc = "0x2d4 - GPIO_FUNC105_IN_SEL_CFG"]
    pub func105_in_sel_cfg: FUNC105_IN_SEL_CFG,
    #[doc = "0x2d8 - GPIO_FUNC106_IN_SEL_CFG"]
    pub func106_in_sel_cfg: FUNC106_IN_SEL_CFG,
    #[doc = "0x2dc - GPIO_FUNC107_IN_SEL_CFG"]
    pub func107_in_sel_cfg: FUNC107_IN_SEL_CFG,
    #[doc = "0x2e0 - GPIO_FUNC108_IN_SEL_CFG"]
    pub func108_in_sel_cfg: FUNC108_IN_SEL_CFG,
    #[doc = "0x2e4 - GPIO_FUNC109_IN_SEL_CFG"]
    pub func109_in_sel_cfg: FUNC109_IN_SEL_CFG,
    #[doc = "0x2e8 - GPIO_FUNC110_IN_SEL_CFG"]
    pub func110_in_sel_cfg: FUNC110_IN_SEL_CFG,
    #[doc = "0x2ec - GPIO_FUNC111_IN_SEL_CFG"]
    pub func111_in_sel_cfg: FUNC111_IN_SEL_CFG,
    #[doc = "0x2f0 - GPIO_FUNC112_IN_SEL_CFG"]
    pub func112_in_sel_cfg: FUNC112_IN_SEL_CFG,
    #[doc = "0x2f4 - GPIO_FUNC113_IN_SEL_CFG"]
    pub func113_in_sel_cfg: FUNC113_IN_SEL_CFG,
    #[doc = "0x2f8 - GPIO_FUNC114_IN_SEL_CFG"]
    pub func114_in_sel_cfg: FUNC114_IN_SEL_CFG,
    #[doc = "0x2fc - GPIO_FUNC115_IN_SEL_CFG"]
    pub func115_in_sel_cfg: FUNC115_IN_SEL_CFG,
    #[doc = "0x300 - GPIO_FUNC116_IN_SEL_CFG"]
    pub func116_in_sel_cfg: FUNC116_IN_SEL_CFG,
    #[doc = "0x304 - GPIO_FUNC117_IN_SEL_CFG"]
    pub func117_in_sel_cfg: FUNC117_IN_SEL_CFG,
    #[doc = "0x308 - GPIO_FUNC118_IN_SEL_CFG"]
    pub func118_in_sel_cfg: FUNC118_IN_SEL_CFG,
    #[doc = "0x30c - GPIO_FUNC119_IN_SEL_CFG"]
    pub func119_in_sel_cfg: FUNC119_IN_SEL_CFG,
    #[doc = "0x310 - GPIO_FUNC120_IN_SEL_CFG"]
    pub func120_in_sel_cfg: FUNC120_IN_SEL_CFG,
    #[doc = "0x314 - GPIO_FUNC121_IN_SEL_CFG"]
    pub func121_in_sel_cfg: FUNC121_IN_SEL_CFG,
    #[doc = "0x318 - GPIO_FUNC122_IN_SEL_CFG"]
    pub func122_in_sel_cfg: FUNC122_IN_SEL_CFG,
    #[doc = "0x31c - GPIO_FUNC123_IN_SEL_CFG"]
    pub func123_in_sel_cfg: FUNC123_IN_SEL_CFG,
    #[doc = "0x320 - GPIO_FUNC124_IN_SEL_CFG"]
    pub func124_in_sel_cfg: FUNC124_IN_SEL_CFG,
    #[doc = "0x324 - GPIO_FUNC125_IN_SEL_CFG"]
    pub func125_in_sel_cfg: FUNC125_IN_SEL_CFG,
    #[doc = "0x328 - GPIO_FUNC126_IN_SEL_CFG"]
    pub func126_in_sel_cfg: FUNC126_IN_SEL_CFG,
    #[doc = "0x32c - GPIO_FUNC127_IN_SEL_CFG"]
    pub func127_in_sel_cfg: FUNC127_IN_SEL_CFG,
    #[doc = "0x330 - GPIO_FUNC128_IN_SEL_CFG"]
    pub func128_in_sel_cfg: FUNC128_IN_SEL_CFG,
    #[doc = "0x334 - GPIO_FUNC129_IN_SEL_CFG"]
    pub func129_in_sel_cfg: FUNC129_IN_SEL_CFG,
    #[doc = "0x338 - GPIO_FUNC130_IN_SEL_CFG"]
    pub func130_in_sel_cfg: FUNC130_IN_SEL_CFG,
    #[doc = "0x33c - GPIO_FUNC131_IN_SEL_CFG"]
    pub func131_in_sel_cfg: FUNC131_IN_SEL_CFG,
    #[doc = "0x340 - GPIO_FUNC132_IN_SEL_CFG"]
    pub func132_in_sel_cfg: FUNC132_IN_SEL_CFG,
    #[doc = "0x344 - GPIO_FUNC133_IN_SEL_CFG"]
    pub func133_in_sel_cfg: FUNC133_IN_SEL_CFG,
    #[doc = "0x348 - GPIO_FUNC134_IN_SEL_CFG"]
    pub func134_in_sel_cfg: FUNC134_IN_SEL_CFG,
    #[doc = "0x34c - GPIO_FUNC135_IN_SEL_CFG"]
    pub func135_in_sel_cfg: FUNC135_IN_SEL_CFG,
    #[doc = "0x350 - GPIO_FUNC136_IN_SEL_CFG"]
    pub func136_in_sel_cfg: FUNC136_IN_SEL_CFG,
    #[doc = "0x354 - GPIO_FUNC137_IN_SEL_CFG"]
    pub func137_in_sel_cfg: FUNC137_IN_SEL_CFG,
    #[doc = "0x358 - GPIO_FUNC138_IN_SEL_CFG"]
    pub func138_in_sel_cfg: FUNC138_IN_SEL_CFG,
    #[doc = "0x35c - GPIO_FUNC139_IN_SEL_CFG"]
    pub func139_in_sel_cfg: FUNC139_IN_SEL_CFG,
    #[doc = "0x360 - GPIO_FUNC140_IN_SEL_CFG"]
    pub func140_in_sel_cfg: FUNC140_IN_SEL_CFG,
    #[doc = "0x364 - GPIO_FUNC141_IN_SEL_CFG"]
    pub func141_in_sel_cfg: FUNC141_IN_SEL_CFG,
    #[doc = "0x368 - GPIO_FUNC142_IN_SEL_CFG"]
    pub func142_in_sel_cfg: FUNC142_IN_SEL_CFG,
    #[doc = "0x36c - GPIO_FUNC143_IN_SEL_CFG"]
    pub func143_in_sel_cfg: FUNC143_IN_SEL_CFG,
    #[doc = "0x370 - GPIO_FUNC144_IN_SEL_CFG"]
    pub func144_in_sel_cfg: FUNC144_IN_SEL_CFG,
    #[doc = "0x374 - GPIO_FUNC145_IN_SEL_CFG"]
    pub func145_in_sel_cfg: FUNC145_IN_SEL_CFG,
    #[doc = "0x378 - GPIO_FUNC146_IN_SEL_CFG"]
    pub func146_in_sel_cfg: FUNC146_IN_SEL_CFG,
    #[doc = "0x37c - GPIO_FUNC147_IN_SEL_CFG"]
    pub func147_in_sel_cfg: FUNC147_IN_SEL_CFG,
    #[doc = "0x380 - GPIO_FUNC148_IN_SEL_CFG"]
    pub func148_in_sel_cfg: FUNC148_IN_SEL_CFG,
    #[doc = "0x384 - GPIO_FUNC149_IN_SEL_CFG"]
    pub func149_in_sel_cfg: FUNC149_IN_SEL_CFG,
    #[doc = "0x388 - GPIO_FUNC150_IN_SEL_CFG"]
    pub func150_in_sel_cfg: FUNC150_IN_SEL_CFG,
    #[doc = "0x38c - GPIO_FUNC151_IN_SEL_CFG"]
    pub func151_in_sel_cfg: FUNC151_IN_SEL_CFG,
    #[doc = "0x390 - GPIO_FUNC152_IN_SEL_CFG"]
    pub func152_in_sel_cfg: FUNC152_IN_SEL_CFG,
    #[doc = "0x394 - GPIO_FUNC153_IN_SEL_CFG"]
    pub func153_in_sel_cfg: FUNC153_IN_SEL_CFG,
    #[doc = "0x398 - GPIO_FUNC154_IN_SEL_CFG"]
    pub func154_in_sel_cfg: FUNC154_IN_SEL_CFG,
    #[doc = "0x39c - GPIO_FUNC155_IN_SEL_CFG"]
    pub func155_in_sel_cfg: FUNC155_IN_SEL_CFG,
    #[doc = "0x3a0 - GPIO_FUNC156_IN_SEL_CFG"]
    pub func156_in_sel_cfg: FUNC156_IN_SEL_CFG,
    #[doc = "0x3a4 - GPIO_FUNC157_IN_SEL_CFG"]
    pub func157_in_sel_cfg: FUNC157_IN_SEL_CFG,
    #[doc = "0x3a8 - GPIO_FUNC158_IN_SEL_CFG"]
    pub func158_in_sel_cfg: FUNC158_IN_SEL_CFG,
    #[doc = "0x3ac - GPIO_FUNC159_IN_SEL_CFG"]
    pub func159_in_sel_cfg: FUNC159_IN_SEL_CFG,
    #[doc = "0x3b0 - GPIO_FUNC160_IN_SEL_CFG"]
    pub func160_in_sel_cfg: FUNC160_IN_SEL_CFG,
    #[doc = "0x3b4 - GPIO_FUNC161_IN_SEL_CFG"]
    pub func161_in_sel_cfg: FUNC161_IN_SEL_CFG,
    #[doc = "0x3b8 - GPIO_FUNC162_IN_SEL_CFG"]
    pub func162_in_sel_cfg: FUNC162_IN_SEL_CFG,
    #[doc = "0x3bc - GPIO_FUNC163_IN_SEL_CFG"]
    pub func163_in_sel_cfg: FUNC163_IN_SEL_CFG,
    #[doc = "0x3c0 - GPIO_FUNC164_IN_SEL_CFG"]
    pub func164_in_sel_cfg: FUNC164_IN_SEL_CFG,
    #[doc = "0x3c4 - GPIO_FUNC165_IN_SEL_CFG"]
    pub func165_in_sel_cfg: FUNC165_IN_SEL_CFG,
    #[doc = "0x3c8 - GPIO_FUNC166_IN_SEL_CFG"]
    pub func166_in_sel_cfg: FUNC166_IN_SEL_CFG,
    #[doc = "0x3cc - GPIO_FUNC167_IN_SEL_CFG"]
    pub func167_in_sel_cfg: FUNC167_IN_SEL_CFG,
    #[doc = "0x3d0 - GPIO_FUNC168_IN_SEL_CFG"]
    pub func168_in_sel_cfg: FUNC168_IN_SEL_CFG,
    #[doc = "0x3d4 - GPIO_FUNC169_IN_SEL_CFG"]
    pub func169_in_sel_cfg: FUNC169_IN_SEL_CFG,
    #[doc = "0x3d8 - GPIO_FUNC170_IN_SEL_CFG"]
    pub func170_in_sel_cfg: FUNC170_IN_SEL_CFG,
    #[doc = "0x3dc - GPIO_FUNC171_IN_SEL_CFG"]
    pub func171_in_sel_cfg: FUNC171_IN_SEL_CFG,
    #[doc = "0x3e0 - GPIO_FUNC172_IN_SEL_CFG"]
    pub func172_in_sel_cfg: FUNC172_IN_SEL_CFG,
    #[doc = "0x3e4 - GPIO_FUNC173_IN_SEL_CFG"]
    pub func173_in_sel_cfg: FUNC173_IN_SEL_CFG,
    #[doc = "0x3e8 - GPIO_FUNC174_IN_SEL_CFG"]
    pub func174_in_sel_cfg: FUNC174_IN_SEL_CFG,
    #[doc = "0x3ec - GPIO_FUNC175_IN_SEL_CFG"]
    pub func175_in_sel_cfg: FUNC175_IN_SEL_CFG,
    #[doc = "0x3f0 - GPIO_FUNC176_IN_SEL_CFG"]
    pub func176_in_sel_cfg: FUNC176_IN_SEL_CFG,
    #[doc = "0x3f4 - GPIO_FUNC177_IN_SEL_CFG"]
    pub func177_in_sel_cfg: FUNC177_IN_SEL_CFG,
    #[doc = "0x3f8 - GPIO_FUNC178_IN_SEL_CFG"]
    pub func178_in_sel_cfg: FUNC178_IN_SEL_CFG,
    #[doc = "0x3fc - GPIO_FUNC179_IN_SEL_CFG"]
    pub func179_in_sel_cfg: FUNC179_IN_SEL_CFG,
    #[doc = "0x400 - GPIO_FUNC180_IN_SEL_CFG"]
    pub func180_in_sel_cfg: FUNC180_IN_SEL_CFG,
    #[doc = "0x404 - GPIO_FUNC181_IN_SEL_CFG"]
    pub func181_in_sel_cfg: FUNC181_IN_SEL_CFG,
    #[doc = "0x408 - GPIO_FUNC182_IN_SEL_CFG"]
    pub func182_in_sel_cfg: FUNC182_IN_SEL_CFG,
    #[doc = "0x40c - GPIO_FUNC183_IN_SEL_CFG"]
    pub func183_in_sel_cfg: FUNC183_IN_SEL_CFG,
    #[doc = "0x410 - GPIO_FUNC184_IN_SEL_CFG"]
    pub func184_in_sel_cfg: FUNC184_IN_SEL_CFG,
    #[doc = "0x414 - GPIO_FUNC185_IN_SEL_CFG"]
    pub func185_in_sel_cfg: FUNC185_IN_SEL_CFG,
    #[doc = "0x418 - GPIO_FUNC186_IN_SEL_CFG"]
    pub func186_in_sel_cfg: FUNC186_IN_SEL_CFG,
    #[doc = "0x41c - GPIO_FUNC187_IN_SEL_CFG"]
    pub func187_in_sel_cfg: FUNC187_IN_SEL_CFG,
    #[doc = "0x420 - GPIO_FUNC188_IN_SEL_CFG"]
    pub func188_in_sel_cfg: FUNC188_IN_SEL_CFG,
    #[doc = "0x424 - GPIO_FUNC189_IN_SEL_CFG"]
    pub func189_in_sel_cfg: FUNC189_IN_SEL_CFG,
    #[doc = "0x428 - GPIO_FUNC190_IN_SEL_CFG"]
    pub func190_in_sel_cfg: FUNC190_IN_SEL_CFG,
    #[doc = "0x42c - GPIO_FUNC191_IN_SEL_CFG"]
    pub func191_in_sel_cfg: FUNC191_IN_SEL_CFG,
    #[doc = "0x430 - GPIO_FUNC192_IN_SEL_CFG"]
    pub func192_in_sel_cfg: FUNC192_IN_SEL_CFG,
    #[doc = "0x434 - GPIO_FUNC193_IN_SEL_CFG"]
    pub func193_in_sel_cfg: FUNC193_IN_SEL_CFG,
    #[doc = "0x438 - GPIO_FUNC194_IN_SEL_CFG"]
    pub func194_in_sel_cfg: FUNC194_IN_SEL_CFG,
    #[doc = "0x43c - GPIO_FUNC195_IN_SEL_CFG"]
    pub func195_in_sel_cfg: FUNC195_IN_SEL_CFG,
    #[doc = "0x440 - GPIO_FUNC196_IN_SEL_CFG"]
    pub func196_in_sel_cfg: FUNC196_IN_SEL_CFG,
    #[doc = "0x444 - GPIO_FUNC197_IN_SEL_CFG"]
    pub func197_in_sel_cfg: FUNC197_IN_SEL_CFG,
    #[doc = "0x448 - GPIO_FUNC198_IN_SEL_CFG"]
    pub func198_in_sel_cfg: FUNC198_IN_SEL_CFG,
    #[doc = "0x44c - GPIO_FUNC199_IN_SEL_CFG"]
    pub func199_in_sel_cfg: FUNC199_IN_SEL_CFG,
    #[doc = "0x450 - GPIO_FUNC200_IN_SEL_CFG"]
    pub func200_in_sel_cfg: FUNC200_IN_SEL_CFG,
    #[doc = "0x454 - GPIO_FUNC201_IN_SEL_CFG"]
    pub func201_in_sel_cfg: FUNC201_IN_SEL_CFG,
    #[doc = "0x458 - GPIO_FUNC202_IN_SEL_CFG"]
    pub func202_in_sel_cfg: FUNC202_IN_SEL_CFG,
    #[doc = "0x45c - GPIO_FUNC203_IN_SEL_CFG"]
    pub func203_in_sel_cfg: FUNC203_IN_SEL_CFG,
    #[doc = "0x460 - GPIO_FUNC204_IN_SEL_CFG"]
    pub func204_in_sel_cfg: FUNC204_IN_SEL_CFG,
    #[doc = "0x464 - GPIO_FUNC205_IN_SEL_CFG"]
    pub func205_in_sel_cfg: FUNC205_IN_SEL_CFG,
    #[doc = "0x468 - GPIO_FUNC206_IN_SEL_CFG"]
    pub func206_in_sel_cfg: FUNC206_IN_SEL_CFG,
    #[doc = "0x46c - GPIO_FUNC207_IN_SEL_CFG"]
    pub func207_in_sel_cfg: FUNC207_IN_SEL_CFG,
    #[doc = "0x470 - GPIO_FUNC208_IN_SEL_CFG"]
    pub func208_in_sel_cfg: FUNC208_IN_SEL_CFG,
    #[doc = "0x474 - GPIO_FUNC209_IN_SEL_CFG"]
    pub func209_in_sel_cfg: FUNC209_IN_SEL_CFG,
    #[doc = "0x478 - GPIO_FUNC210_IN_SEL_CFG"]
    pub func210_in_sel_cfg: FUNC210_IN_SEL_CFG,
    #[doc = "0x47c - GPIO_FUNC211_IN_SEL_CFG"]
    pub func211_in_sel_cfg: FUNC211_IN_SEL_CFG,
    #[doc = "0x480 - GPIO_FUNC212_IN_SEL_CFG"]
    pub func212_in_sel_cfg: FUNC212_IN_SEL_CFG,
    #[doc = "0x484 - GPIO_FUNC213_IN_SEL_CFG"]
    pub func213_in_sel_cfg: FUNC213_IN_SEL_CFG,
    #[doc = "0x488 - GPIO_FUNC214_IN_SEL_CFG"]
    pub func214_in_sel_cfg: FUNC214_IN_SEL_CFG,
    #[doc = "0x48c - GPIO_FUNC215_IN_SEL_CFG"]
    pub func215_in_sel_cfg: FUNC215_IN_SEL_CFG,
    #[doc = "0x490 - GPIO_FUNC216_IN_SEL_CFG"]
    pub func216_in_sel_cfg: FUNC216_IN_SEL_CFG,
    #[doc = "0x494 - GPIO_FUNC217_IN_SEL_CFG"]
    pub func217_in_sel_cfg: FUNC217_IN_SEL_CFG,
    #[doc = "0x498 - GPIO_FUNC218_IN_SEL_CFG"]
    pub func218_in_sel_cfg: FUNC218_IN_SEL_CFG,
    #[doc = "0x49c - GPIO_FUNC219_IN_SEL_CFG"]
    pub func219_in_sel_cfg: FUNC219_IN_SEL_CFG,
    #[doc = "0x4a0 - GPIO_FUNC220_IN_SEL_CFG"]
    pub func220_in_sel_cfg: FUNC220_IN_SEL_CFG,
    #[doc = "0x4a4 - GPIO_FUNC221_IN_SEL_CFG"]
    pub func221_in_sel_cfg: FUNC221_IN_SEL_CFG,
    #[doc = "0x4a8 - GPIO_FUNC222_IN_SEL_CFG"]
    pub func222_in_sel_cfg: FUNC222_IN_SEL_CFG,
    #[doc = "0x4ac - GPIO_FUNC223_IN_SEL_CFG"]
    pub func223_in_sel_cfg: FUNC223_IN_SEL_CFG,
    #[doc = "0x4b0 - GPIO_FUNC224_IN_SEL_CFG"]
    pub func224_in_sel_cfg: FUNC224_IN_SEL_CFG,
    #[doc = "0x4b4 - GPIO_FUNC225_IN_SEL_CFG"]
    pub func225_in_sel_cfg: FUNC225_IN_SEL_CFG,
    #[doc = "0x4b8 - GPIO_FUNC226_IN_SEL_CFG"]
    pub func226_in_sel_cfg: FUNC226_IN_SEL_CFG,
    #[doc = "0x4bc - GPIO_FUNC227_IN_SEL_CFG"]
    pub func227_in_sel_cfg: FUNC227_IN_SEL_CFG,
    #[doc = "0x4c0 - GPIO_FUNC228_IN_SEL_CFG"]
    pub func228_in_sel_cfg: FUNC228_IN_SEL_CFG,
    #[doc = "0x4c4 - GPIO_FUNC229_IN_SEL_CFG"]
    pub func229_in_sel_cfg: FUNC229_IN_SEL_CFG,
    #[doc = "0x4c8 - GPIO_FUNC230_IN_SEL_CFG"]
    pub func230_in_sel_cfg: FUNC230_IN_SEL_CFG,
    #[doc = "0x4cc - GPIO_FUNC231_IN_SEL_CFG"]
    pub func231_in_sel_cfg: FUNC231_IN_SEL_CFG,
    #[doc = "0x4d0 - GPIO_FUNC232_IN_SEL_CFG"]
    pub func232_in_sel_cfg: FUNC232_IN_SEL_CFG,
    #[doc = "0x4d4 - GPIO_FUNC233_IN_SEL_CFG"]
    pub func233_in_sel_cfg: FUNC233_IN_SEL_CFG,
    #[doc = "0x4d8 - GPIO_FUNC234_IN_SEL_CFG"]
    pub func234_in_sel_cfg: FUNC234_IN_SEL_CFG,
    #[doc = "0x4dc - GPIO_FUNC235_IN_SEL_CFG"]
    pub func235_in_sel_cfg: FUNC235_IN_SEL_CFG,
    #[doc = "0x4e0 - GPIO_FUNC236_IN_SEL_CFG"]
    pub func236_in_sel_cfg: FUNC236_IN_SEL_CFG,
    #[doc = "0x4e4 - GPIO_FUNC237_IN_SEL_CFG"]
    pub func237_in_sel_cfg: FUNC237_IN_SEL_CFG,
    #[doc = "0x4e8 - GPIO_FUNC238_IN_SEL_CFG"]
    pub func238_in_sel_cfg: FUNC238_IN_SEL_CFG,
    #[doc = "0x4ec - GPIO_FUNC239_IN_SEL_CFG"]
    pub func239_in_sel_cfg: FUNC239_IN_SEL_CFG,
    #[doc = "0x4f0 - GPIO_FUNC240_IN_SEL_CFG"]
    pub func240_in_sel_cfg: FUNC240_IN_SEL_CFG,
    #[doc = "0x4f4 - GPIO_FUNC241_IN_SEL_CFG"]
    pub func241_in_sel_cfg: FUNC241_IN_SEL_CFG,
    #[doc = "0x4f8 - GPIO_FUNC242_IN_SEL_CFG"]
    pub func242_in_sel_cfg: FUNC242_IN_SEL_CFG,
    #[doc = "0x4fc - GPIO_FUNC243_IN_SEL_CFG"]
    pub func243_in_sel_cfg: FUNC243_IN_SEL_CFG,
    #[doc = "0x500 - GPIO_FUNC244_IN_SEL_CFG"]
    pub func244_in_sel_cfg: FUNC244_IN_SEL_CFG,
    #[doc = "0x504 - GPIO_FUNC245_IN_SEL_CFG"]
    pub func245_in_sel_cfg: FUNC245_IN_SEL_CFG,
    #[doc = "0x508 - GPIO_FUNC246_IN_SEL_CFG"]
    pub func246_in_sel_cfg: FUNC246_IN_SEL_CFG,
    #[doc = "0x50c - GPIO_FUNC247_IN_SEL_CFG"]
    pub func247_in_sel_cfg: FUNC247_IN_SEL_CFG,
    #[doc = "0x510 - GPIO_FUNC248_IN_SEL_CFG"]
    pub func248_in_sel_cfg: FUNC248_IN_SEL_CFG,
    #[doc = "0x514 - GPIO_FUNC249_IN_SEL_CFG"]
    pub func249_in_sel_cfg: FUNC249_IN_SEL_CFG,
    #[doc = "0x518 - GPIO_FUNC250_IN_SEL_CFG"]
    pub func250_in_sel_cfg: FUNC250_IN_SEL_CFG,
    #[doc = "0x51c - GPIO_FUNC251_IN_SEL_CFG"]
    pub func251_in_sel_cfg: FUNC251_IN_SEL_CFG,
    #[doc = "0x520 - GPIO_FUNC252_IN_SEL_CFG"]
    pub func252_in_sel_cfg: FUNC252_IN_SEL_CFG,
    #[doc = "0x524 - GPIO_FUNC253_IN_SEL_CFG"]
    pub func253_in_sel_cfg: FUNC253_IN_SEL_CFG,
    #[doc = "0x528 - GPIO_FUNC254_IN_SEL_CFG"]
    pub func254_in_sel_cfg: FUNC254_IN_SEL_CFG,
    #[doc = "0x52c - GPIO_FUNC255_IN_SEL_CFG"]
    pub func255_in_sel_cfg: FUNC255_IN_SEL_CFG,
    #[doc = "0x530 - GPIO_FUNC0_OUT_SEL_CFG"]
    pub func0_out_sel_cfg: FUNC0_OUT_SEL_CFG,
    #[doc = "0x534 - GPIO_FUNC1_OUT_SEL_CFG"]
    pub func1_out_sel_cfg: FUNC1_OUT_SEL_CFG,
    #[doc = "0x538 - GPIO_FUNC2_OUT_SEL_CFG"]
    pub func2_out_sel_cfg: FUNC2_OUT_SEL_CFG,
    #[doc = "0x53c - GPIO_FUNC3_OUT_SEL_CFG"]
    pub func3_out_sel_cfg: FUNC3_OUT_SEL_CFG,
    #[doc = "0x540 - GPIO_FUNC4_OUT_SEL_CFG"]
    pub func4_out_sel_cfg: FUNC4_OUT_SEL_CFG,
    #[doc = "0x544 - GPIO_FUNC5_OUT_SEL_CFG"]
    pub func5_out_sel_cfg: FUNC5_OUT_SEL_CFG,
    #[doc = "0x548 - GPIO_FUNC6_OUT_SEL_CFG"]
    pub func6_out_sel_cfg: FUNC6_OUT_SEL_CFG,
    #[doc = "0x54c - GPIO_FUNC7_OUT_SEL_CFG"]
    pub func7_out_sel_cfg: FUNC7_OUT_SEL_CFG,
    #[doc = "0x550 - GPIO_FUNC8_OUT_SEL_CFG"]
    pub func8_out_sel_cfg: FUNC8_OUT_SEL_CFG,
    #[doc = "0x554 - GPIO_FUNC9_OUT_SEL_CFG"]
    pub func9_out_sel_cfg: FUNC9_OUT_SEL_CFG,
    #[doc = "0x558 - GPIO_FUNC10_OUT_SEL_CFG"]
    pub func10_out_sel_cfg: FUNC10_OUT_SEL_CFG,
    #[doc = "0x55c - GPIO_FUNC11_OUT_SEL_CFG"]
    pub func11_out_sel_cfg: FUNC11_OUT_SEL_CFG,
    #[doc = "0x560 - GPIO_FUNC12_OUT_SEL_CFG"]
    pub func12_out_sel_cfg: FUNC12_OUT_SEL_CFG,
    #[doc = "0x564 - GPIO_FUNC13_OUT_SEL_CFG"]
    pub func13_out_sel_cfg: FUNC13_OUT_SEL_CFG,
    #[doc = "0x568 - GPIO_FUNC14_OUT_SEL_CFG"]
    pub func14_out_sel_cfg: FUNC14_OUT_SEL_CFG,
    #[doc = "0x56c - GPIO_FUNC15_OUT_SEL_CFG"]
    pub func15_out_sel_cfg: FUNC15_OUT_SEL_CFG,
    #[doc = "0x570 - GPIO_FUNC16_OUT_SEL_CFG"]
    pub func16_out_sel_cfg: FUNC16_OUT_SEL_CFG,
    #[doc = "0x574 - GPIO_FUNC17_OUT_SEL_CFG"]
    pub func17_out_sel_cfg: FUNC17_OUT_SEL_CFG,
    #[doc = "0x578 - GPIO_FUNC18_OUT_SEL_CFG"]
    pub func18_out_sel_cfg: FUNC18_OUT_SEL_CFG,
    #[doc = "0x57c - GPIO_FUNC19_OUT_SEL_CFG"]
    pub func19_out_sel_cfg: FUNC19_OUT_SEL_CFG,
    #[doc = "0x580 - GPIO_FUNC20_OUT_SEL_CFG"]
    pub func20_out_sel_cfg: FUNC20_OUT_SEL_CFG,
    #[doc = "0x584 - GPIO_FUNC21_OUT_SEL_CFG"]
    pub func21_out_sel_cfg: FUNC21_OUT_SEL_CFG,
    #[doc = "0x588 - GPIO_FUNC22_OUT_SEL_CFG"]
    pub func22_out_sel_cfg: FUNC22_OUT_SEL_CFG,
    #[doc = "0x58c - GPIO_FUNC23_OUT_SEL_CFG"]
    pub func23_out_sel_cfg: FUNC23_OUT_SEL_CFG,
    #[doc = "0x590 - GPIO_FUNC24_OUT_SEL_CFG"]
    pub func24_out_sel_cfg: FUNC24_OUT_SEL_CFG,
    #[doc = "0x594 - GPIO_FUNC25_OUT_SEL_CFG"]
    pub func25_out_sel_cfg: FUNC25_OUT_SEL_CFG,
    #[doc = "0x598 - GPIO_FUNC26_OUT_SEL_CFG"]
    pub func26_out_sel_cfg: FUNC26_OUT_SEL_CFG,
    #[doc = "0x59c - GPIO_FUNC27_OUT_SEL_CFG"]
    pub func27_out_sel_cfg: FUNC27_OUT_SEL_CFG,
    #[doc = "0x5a0 - GPIO_FUNC28_OUT_SEL_CFG"]
    pub func28_out_sel_cfg: FUNC28_OUT_SEL_CFG,
    #[doc = "0x5a4 - GPIO_FUNC29_OUT_SEL_CFG"]
    pub func29_out_sel_cfg: FUNC29_OUT_SEL_CFG,
    #[doc = "0x5a8 - GPIO_FUNC30_OUT_SEL_CFG"]
    pub func30_out_sel_cfg: FUNC30_OUT_SEL_CFG,
    #[doc = "0x5ac - GPIO_FUNC31_OUT_SEL_CFG"]
    pub func31_out_sel_cfg: FUNC31_OUT_SEL_CFG,
    #[doc = "0x5b0 - GPIO_FUNC32_OUT_SEL_CFG"]
    pub func32_out_sel_cfg: FUNC32_OUT_SEL_CFG,
    #[doc = "0x5b4 - GPIO_FUNC33_OUT_SEL_CFG"]
    pub func33_out_sel_cfg: FUNC33_OUT_SEL_CFG,
    #[doc = "0x5b8 - GPIO_FUNC34_OUT_SEL_CFG"]
    pub func34_out_sel_cfg: FUNC34_OUT_SEL_CFG,
    #[doc = "0x5bc - GPIO_FUNC35_OUT_SEL_CFG"]
    pub func35_out_sel_cfg: FUNC35_OUT_SEL_CFG,
    #[doc = "0x5c0 - GPIO_FUNC36_OUT_SEL_CFG"]
    pub func36_out_sel_cfg: FUNC36_OUT_SEL_CFG,
    #[doc = "0x5c4 - GPIO_FUNC37_OUT_SEL_CFG"]
    pub func37_out_sel_cfg: FUNC37_OUT_SEL_CFG,
    #[doc = "0x5c8 - GPIO_FUNC38_OUT_SEL_CFG"]
    pub func38_out_sel_cfg: FUNC38_OUT_SEL_CFG,
    #[doc = "0x5cc - GPIO_FUNC39_OUT_SEL_CFG"]
    pub func39_out_sel_cfg: FUNC39_OUT_SEL_CFG,
}
#[doc = "GPIO_BT_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bt_select](bt_select) module"]
pub type BT_SELECT = crate::Reg<u32, _BT_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_SELECT;
#[doc = "`read()` method returns [bt_select::R](bt_select::R) reader structure"]
impl crate::Readable for BT_SELECT {}
#[doc = "`write(|w| ..)` method takes [bt_select::W](bt_select::W) writer structure"]
impl crate::Writable for BT_SELECT {}
#[doc = "GPIO_BT_SELECT"]
pub mod bt_select;
#[doc = "GPIO_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "GPIO_OUT"]
pub mod out;
#[doc = "GPIO_OUT_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_w1ts](out_w1ts) module"]
pub type OUT_W1TS = crate::Reg<u32, _OUT_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_W1TS;
#[doc = "`read()` method returns [out_w1ts::R](out_w1ts::R) reader structure"]
impl crate::Readable for OUT_W1TS {}
#[doc = "`write(|w| ..)` method takes [out_w1ts::W](out_w1ts::W) writer structure"]
impl crate::Writable for OUT_W1TS {}
#[doc = "GPIO_OUT_W1TS"]
pub mod out_w1ts;
#[doc = "GPIO_OUT_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_w1tc](out_w1tc) module"]
pub type OUT_W1TC = crate::Reg<u32, _OUT_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_W1TC;
#[doc = "`read()` method returns [out_w1tc::R](out_w1tc::R) reader structure"]
impl crate::Readable for OUT_W1TC {}
#[doc = "`write(|w| ..)` method takes [out_w1tc::W](out_w1tc::W) writer structure"]
impl crate::Writable for OUT_W1TC {}
#[doc = "GPIO_OUT_W1TC"]
pub mod out_w1tc;
#[doc = "GPIO_OUT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out1](out1) module"]
pub type OUT1 = crate::Reg<u32, _OUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT1;
#[doc = "`read()` method returns [out1::R](out1::R) reader structure"]
impl crate::Readable for OUT1 {}
#[doc = "`write(|w| ..)` method takes [out1::W](out1::W) writer structure"]
impl crate::Writable for OUT1 {}
#[doc = "GPIO_OUT1"]
pub mod out1;
#[doc = "GPIO_OUT1_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out1_w1ts](out1_w1ts) module"]
pub type OUT1_W1TS = crate::Reg<u32, _OUT1_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT1_W1TS;
#[doc = "`read()` method returns [out1_w1ts::R](out1_w1ts::R) reader structure"]
impl crate::Readable for OUT1_W1TS {}
#[doc = "`write(|w| ..)` method takes [out1_w1ts::W](out1_w1ts::W) writer structure"]
impl crate::Writable for OUT1_W1TS {}
#[doc = "GPIO_OUT1_W1TS"]
pub mod out1_w1ts;
#[doc = "GPIO_OUT1_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out1_w1tc](out1_w1tc) module"]
pub type OUT1_W1TC = crate::Reg<u32, _OUT1_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT1_W1TC;
#[doc = "`read()` method returns [out1_w1tc::R](out1_w1tc::R) reader structure"]
impl crate::Readable for OUT1_W1TC {}
#[doc = "`write(|w| ..)` method takes [out1_w1tc::W](out1_w1tc::W) writer structure"]
impl crate::Writable for OUT1_W1TC {}
#[doc = "GPIO_OUT1_W1TC"]
pub mod out1_w1tc;
#[doc = "GPIO_SDIO_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_select](sdio_select) module"]
pub type SDIO_SELECT = crate::Reg<u32, _SDIO_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_SELECT;
#[doc = "`read()` method returns [sdio_select::R](sdio_select::R) reader structure"]
impl crate::Readable for SDIO_SELECT {}
#[doc = "`write(|w| ..)` method takes [sdio_select::W](sdio_select::W) writer structure"]
impl crate::Writable for SDIO_SELECT {}
#[doc = "GPIO_SDIO_SELECT"]
pub mod sdio_select;
#[doc = "GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "GPIO_ENABLE"]
pub mod enable;
#[doc = "GPIO_ENABLE_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable_w1ts](enable_w1ts) module"]
pub type ENABLE_W1TS = crate::Reg<u32, _ENABLE_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_W1TS;
#[doc = "`read()` method returns [enable_w1ts::R](enable_w1ts::R) reader structure"]
impl crate::Readable for ENABLE_W1TS {}
#[doc = "`write(|w| ..)` method takes [enable_w1ts::W](enable_w1ts::W) writer structure"]
impl crate::Writable for ENABLE_W1TS {}
#[doc = "GPIO_ENABLE_W1TS"]
pub mod enable_w1ts;
#[doc = "GPIO_ENABLE_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable_w1tc](enable_w1tc) module"]
pub type ENABLE_W1TC = crate::Reg<u32, _ENABLE_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_W1TC;
#[doc = "`read()` method returns [enable_w1tc::R](enable_w1tc::R) reader structure"]
impl crate::Readable for ENABLE_W1TC {}
#[doc = "`write(|w| ..)` method takes [enable_w1tc::W](enable_w1tc::W) writer structure"]
impl crate::Writable for ENABLE_W1TC {}
#[doc = "GPIO_ENABLE_W1TC"]
pub mod enable_w1tc;
#[doc = "GPIO_ENABLE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable1](enable1) module"]
pub type ENABLE1 = crate::Reg<u32, _ENABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE1;
#[doc = "`read()` method returns [enable1::R](enable1::R) reader structure"]
impl crate::Readable for ENABLE1 {}
#[doc = "`write(|w| ..)` method takes [enable1::W](enable1::W) writer structure"]
impl crate::Writable for ENABLE1 {}
#[doc = "GPIO_ENABLE1"]
pub mod enable1;
#[doc = "GPIO_ENABLE1_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable1_w1ts](enable1_w1ts) module"]
pub type ENABLE1_W1TS = crate::Reg<u32, _ENABLE1_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE1_W1TS;
#[doc = "`read()` method returns [enable1_w1ts::R](enable1_w1ts::R) reader structure"]
impl crate::Readable for ENABLE1_W1TS {}
#[doc = "`write(|w| ..)` method takes [enable1_w1ts::W](enable1_w1ts::W) writer structure"]
impl crate::Writable for ENABLE1_W1TS {}
#[doc = "GPIO_ENABLE1_W1TS"]
pub mod enable1_w1ts;
#[doc = "GPIO_ENABLE1_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable1_w1tc](enable1_w1tc) module"]
pub type ENABLE1_W1TC = crate::Reg<u32, _ENABLE1_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE1_W1TC;
#[doc = "`read()` method returns [enable1_w1tc::R](enable1_w1tc::R) reader structure"]
impl crate::Readable for ENABLE1_W1TC {}
#[doc = "`write(|w| ..)` method takes [enable1_w1tc::W](enable1_w1tc::W) writer structure"]
impl crate::Writable for ENABLE1_W1TC {}
#[doc = "GPIO_ENABLE1_W1TC"]
pub mod enable1_w1tc;
#[doc = "GPIO_STRAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [strap](strap) module"]
pub type STRAP = crate::Reg<u32, _STRAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STRAP;
#[doc = "`read()` method returns [strap::R](strap::R) reader structure"]
impl crate::Readable for STRAP {}
#[doc = "`write(|w| ..)` method takes [strap::W](strap::W) writer structure"]
impl crate::Writable for STRAP {}
#[doc = "GPIO_STRAP"]
pub mod strap;
#[doc = "GPIO_IN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "`write(|w| ..)` method takes [in_::W](in_::W) writer structure"]
impl crate::Writable for IN {}
#[doc = "GPIO_IN"]
pub mod in_;
#[doc = "GPIO_IN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in1](in1) module"]
pub type IN1 = crate::Reg<u32, _IN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN1;
#[doc = "`read()` method returns [in1::R](in1::R) reader structure"]
impl crate::Readable for IN1 {}
#[doc = "`write(|w| ..)` method takes [in1::W](in1::W) writer structure"]
impl crate::Writable for IN1 {}
#[doc = "GPIO_IN1"]
pub mod in1;
#[doc = "GPIO_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "GPIO_STATUS"]
pub mod status;
#[doc = "GPIO_STATUS_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status_w1ts](status_w1ts) module"]
pub type STATUS_W1TS = crate::Reg<u32, _STATUS_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_W1TS;
#[doc = "`read()` method returns [status_w1ts::R](status_w1ts::R) reader structure"]
impl crate::Readable for STATUS_W1TS {}
#[doc = "`write(|w| ..)` method takes [status_w1ts::W](status_w1ts::W) writer structure"]
impl crate::Writable for STATUS_W1TS {}
#[doc = "GPIO_STATUS_W1TS"]
pub mod status_w1ts;
#[doc = "GPIO_STATUS_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status_w1tc](status_w1tc) module"]
pub type STATUS_W1TC = crate::Reg<u32, _STATUS_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_W1TC;
#[doc = "`read()` method returns [status_w1tc::R](status_w1tc::R) reader structure"]
impl crate::Readable for STATUS_W1TC {}
#[doc = "`write(|w| ..)` method takes [status_w1tc::W](status_w1tc::W) writer structure"]
impl crate::Writable for STATUS_W1TC {}
#[doc = "GPIO_STATUS_W1TC"]
pub mod status_w1tc;
#[doc = "GPIO_STATUS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status1](status1) module"]
pub type STATUS1 = crate::Reg<u32, _STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS1;
#[doc = "`read()` method returns [status1::R](status1::R) reader structure"]
impl crate::Readable for STATUS1 {}
#[doc = "`write(|w| ..)` method takes [status1::W](status1::W) writer structure"]
impl crate::Writable for STATUS1 {}
#[doc = "GPIO_STATUS1"]
pub mod status1;
#[doc = "GPIO_STATUS1_W1TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status1_w1ts](status1_w1ts) module"]
pub type STATUS1_W1TS = crate::Reg<u32, _STATUS1_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS1_W1TS;
#[doc = "`read()` method returns [status1_w1ts::R](status1_w1ts::R) reader structure"]
impl crate::Readable for STATUS1_W1TS {}
#[doc = "`write(|w| ..)` method takes [status1_w1ts::W](status1_w1ts::W) writer structure"]
impl crate::Writable for STATUS1_W1TS {}
#[doc = "GPIO_STATUS1_W1TS"]
pub mod status1_w1ts;
#[doc = "GPIO_STATUS1_W1TC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status1_w1tc](status1_w1tc) module"]
pub type STATUS1_W1TC = crate::Reg<u32, _STATUS1_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS1_W1TC;
#[doc = "`read()` method returns [status1_w1tc::R](status1_w1tc::R) reader structure"]
impl crate::Readable for STATUS1_W1TC {}
#[doc = "`write(|w| ..)` method takes [status1_w1tc::W](status1_w1tc::W) writer structure"]
impl crate::Writable for STATUS1_W1TC {}
#[doc = "GPIO_STATUS1_W1TC"]
pub mod status1_w1tc;
#[doc = "GPIO_ACPU_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acpu_int](acpu_int) module"]
pub type ACPU_INT = crate::Reg<u32, _ACPU_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPU_INT;
#[doc = "`read()` method returns [acpu_int::R](acpu_int::R) reader structure"]
impl crate::Readable for ACPU_INT {}
#[doc = "`write(|w| ..)` method takes [acpu_int::W](acpu_int::W) writer structure"]
impl crate::Writable for ACPU_INT {}
#[doc = "GPIO_ACPU_INT"]
pub mod acpu_int;
#[doc = "GPIO_ACPU_NMI_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acpu_nmi_int](acpu_nmi_int) module"]
pub type ACPU_NMI_INT = crate::Reg<u32, _ACPU_NMI_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPU_NMI_INT;
#[doc = "`read()` method returns [acpu_nmi_int::R](acpu_nmi_int::R) reader structure"]
impl crate::Readable for ACPU_NMI_INT {}
#[doc = "`write(|w| ..)` method takes [acpu_nmi_int::W](acpu_nmi_int::W) writer structure"]
impl crate::Writable for ACPU_NMI_INT {}
#[doc = "GPIO_ACPU_NMI_INT"]
pub mod acpu_nmi_int;
#[doc = "GPIO_PCPU_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcpu_int](pcpu_int) module"]
pub type PCPU_INT = crate::Reg<u32, _PCPU_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_INT;
#[doc = "`read()` method returns [pcpu_int::R](pcpu_int::R) reader structure"]
impl crate::Readable for PCPU_INT {}
#[doc = "`write(|w| ..)` method takes [pcpu_int::W](pcpu_int::W) writer structure"]
impl crate::Writable for PCPU_INT {}
#[doc = "GPIO_PCPU_INT"]
pub mod pcpu_int;
#[doc = "GPIO_PCPU_NMI_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcpu_nmi_int](pcpu_nmi_int) module"]
pub type PCPU_NMI_INT = crate::Reg<u32, _PCPU_NMI_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_NMI_INT;
#[doc = "`read()` method returns [pcpu_nmi_int::R](pcpu_nmi_int::R) reader structure"]
impl crate::Readable for PCPU_NMI_INT {}
#[doc = "`write(|w| ..)` method takes [pcpu_nmi_int::W](pcpu_nmi_int::W) writer structure"]
impl crate::Writable for PCPU_NMI_INT {}
#[doc = "GPIO_PCPU_NMI_INT"]
pub mod pcpu_nmi_int;
#[doc = "GPIO_CPUSDIO_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpusdio_int](cpusdio_int) module"]
pub type CPUSDIO_INT = crate::Reg<u32, _CPUSDIO_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSDIO_INT;
#[doc = "`read()` method returns [cpusdio_int::R](cpusdio_int::R) reader structure"]
impl crate::Readable for CPUSDIO_INT {}
#[doc = "`write(|w| ..)` method takes [cpusdio_int::W](cpusdio_int::W) writer structure"]
impl crate::Writable for CPUSDIO_INT {}
#[doc = "GPIO_CPUSDIO_INT"]
pub mod cpusdio_int;
#[doc = "GPIO_ACPU_INT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acpu_int1](acpu_int1) module"]
pub type ACPU_INT1 = crate::Reg<u32, _ACPU_INT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPU_INT1;
#[doc = "`read()` method returns [acpu_int1::R](acpu_int1::R) reader structure"]
impl crate::Readable for ACPU_INT1 {}
#[doc = "`write(|w| ..)` method takes [acpu_int1::W](acpu_int1::W) writer structure"]
impl crate::Writable for ACPU_INT1 {}
#[doc = "GPIO_ACPU_INT1"]
pub mod acpu_int1;
#[doc = "GPIO_ACPU_NMI_INT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acpu_nmi_int1](acpu_nmi_int1) module"]
pub type ACPU_NMI_INT1 = crate::Reg<u32, _ACPU_NMI_INT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPU_NMI_INT1;
#[doc = "`read()` method returns [acpu_nmi_int1::R](acpu_nmi_int1::R) reader structure"]
impl crate::Readable for ACPU_NMI_INT1 {}
#[doc = "`write(|w| ..)` method takes [acpu_nmi_int1::W](acpu_nmi_int1::W) writer structure"]
impl crate::Writable for ACPU_NMI_INT1 {}
#[doc = "GPIO_ACPU_NMI_INT1"]
pub mod acpu_nmi_int1;
#[doc = "GPIO_PCPU_INT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcpu_int1](pcpu_int1) module"]
pub type PCPU_INT1 = crate::Reg<u32, _PCPU_INT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_INT1;
#[doc = "`read()` method returns [pcpu_int1::R](pcpu_int1::R) reader structure"]
impl crate::Readable for PCPU_INT1 {}
#[doc = "`write(|w| ..)` method takes [pcpu_int1::W](pcpu_int1::W) writer structure"]
impl crate::Writable for PCPU_INT1 {}
#[doc = "GPIO_PCPU_INT1"]
pub mod pcpu_int1;
#[doc = "GPIO_PCPU_NMI_INT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcpu_nmi_int1](pcpu_nmi_int1) module"]
pub type PCPU_NMI_INT1 = crate::Reg<u32, _PCPU_NMI_INT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_NMI_INT1;
#[doc = "`read()` method returns [pcpu_nmi_int1::R](pcpu_nmi_int1::R) reader structure"]
impl crate::Readable for PCPU_NMI_INT1 {}
#[doc = "`write(|w| ..)` method takes [pcpu_nmi_int1::W](pcpu_nmi_int1::W) writer structure"]
impl crate::Writable for PCPU_NMI_INT1 {}
#[doc = "GPIO_PCPU_NMI_INT1"]
pub mod pcpu_nmi_int1;
#[doc = "GPIO_CPUSDIO_INT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpusdio_int1](cpusdio_int1) module"]
pub type CPUSDIO_INT1 = crate::Reg<u32, _CPUSDIO_INT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSDIO_INT1;
#[doc = "`read()` method returns [cpusdio_int1::R](cpusdio_int1::R) reader structure"]
impl crate::Readable for CPUSDIO_INT1 {}
#[doc = "`write(|w| ..)` method takes [cpusdio_int1::W](cpusdio_int1::W) writer structure"]
impl crate::Writable for CPUSDIO_INT1 {}
#[doc = "GPIO_CPUSDIO_INT1"]
pub mod cpusdio_int1;
#[doc = "GPIO_PIN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin0](pin0) module"]
pub type PIN0 = crate::Reg<u32, _PIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN0;
#[doc = "`read()` method returns [pin0::R](pin0::R) reader structure"]
impl crate::Readable for PIN0 {}
#[doc = "`write(|w| ..)` method takes [pin0::W](pin0::W) writer structure"]
impl crate::Writable for PIN0 {}
#[doc = "GPIO_PIN0"]
pub mod pin0;
#[doc = "GPIO_PIN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin1](pin1) module"]
pub type PIN1 = crate::Reg<u32, _PIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN1;
#[doc = "`read()` method returns [pin1::R](pin1::R) reader structure"]
impl crate::Readable for PIN1 {}
#[doc = "`write(|w| ..)` method takes [pin1::W](pin1::W) writer structure"]
impl crate::Writable for PIN1 {}
#[doc = "GPIO_PIN1"]
pub mod pin1;
#[doc = "GPIO_PIN2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin2](pin2) module"]
pub type PIN2 = crate::Reg<u32, _PIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN2;
#[doc = "`read()` method returns [pin2::R](pin2::R) reader structure"]
impl crate::Readable for PIN2 {}
#[doc = "`write(|w| ..)` method takes [pin2::W](pin2::W) writer structure"]
impl crate::Writable for PIN2 {}
#[doc = "GPIO_PIN2"]
pub mod pin2;
#[doc = "GPIO_PIN3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin3](pin3) module"]
pub type PIN3 = crate::Reg<u32, _PIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN3;
#[doc = "`read()` method returns [pin3::R](pin3::R) reader structure"]
impl crate::Readable for PIN3 {}
#[doc = "`write(|w| ..)` method takes [pin3::W](pin3::W) writer structure"]
impl crate::Writable for PIN3 {}
#[doc = "GPIO_PIN3"]
pub mod pin3;
#[doc = "GPIO_PIN4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin4](pin4) module"]
pub type PIN4 = crate::Reg<u32, _PIN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN4;
#[doc = "`read()` method returns [pin4::R](pin4::R) reader structure"]
impl crate::Readable for PIN4 {}
#[doc = "`write(|w| ..)` method takes [pin4::W](pin4::W) writer structure"]
impl crate::Writable for PIN4 {}
#[doc = "GPIO_PIN4"]
pub mod pin4;
#[doc = "GPIO_PIN5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin5](pin5) module"]
pub type PIN5 = crate::Reg<u32, _PIN5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN5;
#[doc = "`read()` method returns [pin5::R](pin5::R) reader structure"]
impl crate::Readable for PIN5 {}
#[doc = "`write(|w| ..)` method takes [pin5::W](pin5::W) writer structure"]
impl crate::Writable for PIN5 {}
#[doc = "GPIO_PIN5"]
pub mod pin5;
#[doc = "GPIO_PIN6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin6](pin6) module"]
pub type PIN6 = crate::Reg<u32, _PIN6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN6;
#[doc = "`read()` method returns [pin6::R](pin6::R) reader structure"]
impl crate::Readable for PIN6 {}
#[doc = "`write(|w| ..)` method takes [pin6::W](pin6::W) writer structure"]
impl crate::Writable for PIN6 {}
#[doc = "GPIO_PIN6"]
pub mod pin6;
#[doc = "GPIO_PIN7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin7](pin7) module"]
pub type PIN7 = crate::Reg<u32, _PIN7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN7;
#[doc = "`read()` method returns [pin7::R](pin7::R) reader structure"]
impl crate::Readable for PIN7 {}
#[doc = "`write(|w| ..)` method takes [pin7::W](pin7::W) writer structure"]
impl crate::Writable for PIN7 {}
#[doc = "GPIO_PIN7"]
pub mod pin7;
#[doc = "GPIO_PIN8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin8](pin8) module"]
pub type PIN8 = crate::Reg<u32, _PIN8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN8;
#[doc = "`read()` method returns [pin8::R](pin8::R) reader structure"]
impl crate::Readable for PIN8 {}
#[doc = "`write(|w| ..)` method takes [pin8::W](pin8::W) writer structure"]
impl crate::Writable for PIN8 {}
#[doc = "GPIO_PIN8"]
pub mod pin8;
#[doc = "GPIO_PIN9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin9](pin9) module"]
pub type PIN9 = crate::Reg<u32, _PIN9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN9;
#[doc = "`read()` method returns [pin9::R](pin9::R) reader structure"]
impl crate::Readable for PIN9 {}
#[doc = "`write(|w| ..)` method takes [pin9::W](pin9::W) writer structure"]
impl crate::Writable for PIN9 {}
#[doc = "GPIO_PIN9"]
pub mod pin9;
#[doc = "GPIO_PIN10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin10](pin10) module"]
pub type PIN10 = crate::Reg<u32, _PIN10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN10;
#[doc = "`read()` method returns [pin10::R](pin10::R) reader structure"]
impl crate::Readable for PIN10 {}
#[doc = "`write(|w| ..)` method takes [pin10::W](pin10::W) writer structure"]
impl crate::Writable for PIN10 {}
#[doc = "GPIO_PIN10"]
pub mod pin10;
#[doc = "GPIO_PIN11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin11](pin11) module"]
pub type PIN11 = crate::Reg<u32, _PIN11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN11;
#[doc = "`read()` method returns [pin11::R](pin11::R) reader structure"]
impl crate::Readable for PIN11 {}
#[doc = "`write(|w| ..)` method takes [pin11::W](pin11::W) writer structure"]
impl crate::Writable for PIN11 {}
#[doc = "GPIO_PIN11"]
pub mod pin11;
#[doc = "GPIO_PIN12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin12](pin12) module"]
pub type PIN12 = crate::Reg<u32, _PIN12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN12;
#[doc = "`read()` method returns [pin12::R](pin12::R) reader structure"]
impl crate::Readable for PIN12 {}
#[doc = "`write(|w| ..)` method takes [pin12::W](pin12::W) writer structure"]
impl crate::Writable for PIN12 {}
#[doc = "GPIO_PIN12"]
pub mod pin12;
#[doc = "GPIO_PIN13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin13](pin13) module"]
pub type PIN13 = crate::Reg<u32, _PIN13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN13;
#[doc = "`read()` method returns [pin13::R](pin13::R) reader structure"]
impl crate::Readable for PIN13 {}
#[doc = "`write(|w| ..)` method takes [pin13::W](pin13::W) writer structure"]
impl crate::Writable for PIN13 {}
#[doc = "GPIO_PIN13"]
pub mod pin13;
#[doc = "GPIO_PIN14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin14](pin14) module"]
pub type PIN14 = crate::Reg<u32, _PIN14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN14;
#[doc = "`read()` method returns [pin14::R](pin14::R) reader structure"]
impl crate::Readable for PIN14 {}
#[doc = "`write(|w| ..)` method takes [pin14::W](pin14::W) writer structure"]
impl crate::Writable for PIN14 {}
#[doc = "GPIO_PIN14"]
pub mod pin14;
#[doc = "GPIO_PIN15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin15](pin15) module"]
pub type PIN15 = crate::Reg<u32, _PIN15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN15;
#[doc = "`read()` method returns [pin15::R](pin15::R) reader structure"]
impl crate::Readable for PIN15 {}
#[doc = "`write(|w| ..)` method takes [pin15::W](pin15::W) writer structure"]
impl crate::Writable for PIN15 {}
#[doc = "GPIO_PIN15"]
pub mod pin15;
#[doc = "GPIO_PIN16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin16](pin16) module"]
pub type PIN16 = crate::Reg<u32, _PIN16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN16;
#[doc = "`read()` method returns [pin16::R](pin16::R) reader structure"]
impl crate::Readable for PIN16 {}
#[doc = "`write(|w| ..)` method takes [pin16::W](pin16::W) writer structure"]
impl crate::Writable for PIN16 {}
#[doc = "GPIO_PIN16"]
pub mod pin16;
#[doc = "GPIO_PIN17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin17](pin17) module"]
pub type PIN17 = crate::Reg<u32, _PIN17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN17;
#[doc = "`read()` method returns [pin17::R](pin17::R) reader structure"]
impl crate::Readable for PIN17 {}
#[doc = "`write(|w| ..)` method takes [pin17::W](pin17::W) writer structure"]
impl crate::Writable for PIN17 {}
#[doc = "GPIO_PIN17"]
pub mod pin17;
#[doc = "GPIO_PIN18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin18](pin18) module"]
pub type PIN18 = crate::Reg<u32, _PIN18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN18;
#[doc = "`read()` method returns [pin18::R](pin18::R) reader structure"]
impl crate::Readable for PIN18 {}
#[doc = "`write(|w| ..)` method takes [pin18::W](pin18::W) writer structure"]
impl crate::Writable for PIN18 {}
#[doc = "GPIO_PIN18"]
pub mod pin18;
#[doc = "GPIO_PIN19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin19](pin19) module"]
pub type PIN19 = crate::Reg<u32, _PIN19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN19;
#[doc = "`read()` method returns [pin19::R](pin19::R) reader structure"]
impl crate::Readable for PIN19 {}
#[doc = "`write(|w| ..)` method takes [pin19::W](pin19::W) writer structure"]
impl crate::Writable for PIN19 {}
#[doc = "GPIO_PIN19"]
pub mod pin19;
#[doc = "GPIO_PIN20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin20](pin20) module"]
pub type PIN20 = crate::Reg<u32, _PIN20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN20;
#[doc = "`read()` method returns [pin20::R](pin20::R) reader structure"]
impl crate::Readable for PIN20 {}
#[doc = "`write(|w| ..)` method takes [pin20::W](pin20::W) writer structure"]
impl crate::Writable for PIN20 {}
#[doc = "GPIO_PIN20"]
pub mod pin20;
#[doc = "GPIO_PIN21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin21](pin21) module"]
pub type PIN21 = crate::Reg<u32, _PIN21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN21;
#[doc = "`read()` method returns [pin21::R](pin21::R) reader structure"]
impl crate::Readable for PIN21 {}
#[doc = "`write(|w| ..)` method takes [pin21::W](pin21::W) writer structure"]
impl crate::Writable for PIN21 {}
#[doc = "GPIO_PIN21"]
pub mod pin21;
#[doc = "GPIO_PIN22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin22](pin22) module"]
pub type PIN22 = crate::Reg<u32, _PIN22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN22;
#[doc = "`read()` method returns [pin22::R](pin22::R) reader structure"]
impl crate::Readable for PIN22 {}
#[doc = "`write(|w| ..)` method takes [pin22::W](pin22::W) writer structure"]
impl crate::Writable for PIN22 {}
#[doc = "GPIO_PIN22"]
pub mod pin22;
#[doc = "GPIO_PIN23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin23](pin23) module"]
pub type PIN23 = crate::Reg<u32, _PIN23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN23;
#[doc = "`read()` method returns [pin23::R](pin23::R) reader structure"]
impl crate::Readable for PIN23 {}
#[doc = "`write(|w| ..)` method takes [pin23::W](pin23::W) writer structure"]
impl crate::Writable for PIN23 {}
#[doc = "GPIO_PIN23"]
pub mod pin23;
#[doc = "GPIO_PIN24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin24](pin24) module"]
pub type PIN24 = crate::Reg<u32, _PIN24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN24;
#[doc = "`read()` method returns [pin24::R](pin24::R) reader structure"]
impl crate::Readable for PIN24 {}
#[doc = "`write(|w| ..)` method takes [pin24::W](pin24::W) writer structure"]
impl crate::Writable for PIN24 {}
#[doc = "GPIO_PIN24"]
pub mod pin24;
#[doc = "GPIO_PIN25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin25](pin25) module"]
pub type PIN25 = crate::Reg<u32, _PIN25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN25;
#[doc = "`read()` method returns [pin25::R](pin25::R) reader structure"]
impl crate::Readable for PIN25 {}
#[doc = "`write(|w| ..)` method takes [pin25::W](pin25::W) writer structure"]
impl crate::Writable for PIN25 {}
#[doc = "GPIO_PIN25"]
pub mod pin25;
#[doc = "GPIO_PIN26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin26](pin26) module"]
pub type PIN26 = crate::Reg<u32, _PIN26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN26;
#[doc = "`read()` method returns [pin26::R](pin26::R) reader structure"]
impl crate::Readable for PIN26 {}
#[doc = "`write(|w| ..)` method takes [pin26::W](pin26::W) writer structure"]
impl crate::Writable for PIN26 {}
#[doc = "GPIO_PIN26"]
pub mod pin26;
#[doc = "GPIO_PIN27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin27](pin27) module"]
pub type PIN27 = crate::Reg<u32, _PIN27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN27;
#[doc = "`read()` method returns [pin27::R](pin27::R) reader structure"]
impl crate::Readable for PIN27 {}
#[doc = "`write(|w| ..)` method takes [pin27::W](pin27::W) writer structure"]
impl crate::Writable for PIN27 {}
#[doc = "GPIO_PIN27"]
pub mod pin27;
#[doc = "GPIO_PIN28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin28](pin28) module"]
pub type PIN28 = crate::Reg<u32, _PIN28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN28;
#[doc = "`read()` method returns [pin28::R](pin28::R) reader structure"]
impl crate::Readable for PIN28 {}
#[doc = "`write(|w| ..)` method takes [pin28::W](pin28::W) writer structure"]
impl crate::Writable for PIN28 {}
#[doc = "GPIO_PIN28"]
pub mod pin28;
#[doc = "GPIO_PIN29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin29](pin29) module"]
pub type PIN29 = crate::Reg<u32, _PIN29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN29;
#[doc = "`read()` method returns [pin29::R](pin29::R) reader structure"]
impl crate::Readable for PIN29 {}
#[doc = "`write(|w| ..)` method takes [pin29::W](pin29::W) writer structure"]
impl crate::Writable for PIN29 {}
#[doc = "GPIO_PIN29"]
pub mod pin29;
#[doc = "GPIO_PIN30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin30](pin30) module"]
pub type PIN30 = crate::Reg<u32, _PIN30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN30;
#[doc = "`read()` method returns [pin30::R](pin30::R) reader structure"]
impl crate::Readable for PIN30 {}
#[doc = "`write(|w| ..)` method takes [pin30::W](pin30::W) writer structure"]
impl crate::Writable for PIN30 {}
#[doc = "GPIO_PIN30"]
pub mod pin30;
#[doc = "GPIO_PIN31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin31](pin31) module"]
pub type PIN31 = crate::Reg<u32, _PIN31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN31;
#[doc = "`read()` method returns [pin31::R](pin31::R) reader structure"]
impl crate::Readable for PIN31 {}
#[doc = "`write(|w| ..)` method takes [pin31::W](pin31::W) writer structure"]
impl crate::Writable for PIN31 {}
#[doc = "GPIO_PIN31"]
pub mod pin31;
#[doc = "GPIO_PIN32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin32](pin32) module"]
pub type PIN32 = crate::Reg<u32, _PIN32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN32;
#[doc = "`read()` method returns [pin32::R](pin32::R) reader structure"]
impl crate::Readable for PIN32 {}
#[doc = "`write(|w| ..)` method takes [pin32::W](pin32::W) writer structure"]
impl crate::Writable for PIN32 {}
#[doc = "GPIO_PIN32"]
pub mod pin32;
#[doc = "GPIO_PIN33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin33](pin33) module"]
pub type PIN33 = crate::Reg<u32, _PIN33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN33;
#[doc = "`read()` method returns [pin33::R](pin33::R) reader structure"]
impl crate::Readable for PIN33 {}
#[doc = "`write(|w| ..)` method takes [pin33::W](pin33::W) writer structure"]
impl crate::Writable for PIN33 {}
#[doc = "GPIO_PIN33"]
pub mod pin33;
#[doc = "GPIO_PIN34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin34](pin34) module"]
pub type PIN34 = crate::Reg<u32, _PIN34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN34;
#[doc = "`read()` method returns [pin34::R](pin34::R) reader structure"]
impl crate::Readable for PIN34 {}
#[doc = "`write(|w| ..)` method takes [pin34::W](pin34::W) writer structure"]
impl crate::Writable for PIN34 {}
#[doc = "GPIO_PIN34"]
pub mod pin34;
#[doc = "GPIO_PIN35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin35](pin35) module"]
pub type PIN35 = crate::Reg<u32, _PIN35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN35;
#[doc = "`read()` method returns [pin35::R](pin35::R) reader structure"]
impl crate::Readable for PIN35 {}
#[doc = "`write(|w| ..)` method takes [pin35::W](pin35::W) writer structure"]
impl crate::Writable for PIN35 {}
#[doc = "GPIO_PIN35"]
pub mod pin35;
#[doc = "GPIO_PIN36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin36](pin36) module"]
pub type PIN36 = crate::Reg<u32, _PIN36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN36;
#[doc = "`read()` method returns [pin36::R](pin36::R) reader structure"]
impl crate::Readable for PIN36 {}
#[doc = "`write(|w| ..)` method takes [pin36::W](pin36::W) writer structure"]
impl crate::Writable for PIN36 {}
#[doc = "GPIO_PIN36"]
pub mod pin36;
#[doc = "GPIO_PIN37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin37](pin37) module"]
pub type PIN37 = crate::Reg<u32, _PIN37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN37;
#[doc = "`read()` method returns [pin37::R](pin37::R) reader structure"]
impl crate::Readable for PIN37 {}
#[doc = "`write(|w| ..)` method takes [pin37::W](pin37::W) writer structure"]
impl crate::Writable for PIN37 {}
#[doc = "GPIO_PIN37"]
pub mod pin37;
#[doc = "GPIO_PIN38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin38](pin38) module"]
pub type PIN38 = crate::Reg<u32, _PIN38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN38;
#[doc = "`read()` method returns [pin38::R](pin38::R) reader structure"]
impl crate::Readable for PIN38 {}
#[doc = "`write(|w| ..)` method takes [pin38::W](pin38::W) writer structure"]
impl crate::Writable for PIN38 {}
#[doc = "GPIO_PIN38"]
pub mod pin38;
#[doc = "GPIO_PIN39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pin39](pin39) module"]
pub type PIN39 = crate::Reg<u32, _PIN39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN39;
#[doc = "`read()` method returns [pin39::R](pin39::R) reader structure"]
impl crate::Readable for PIN39 {}
#[doc = "`write(|w| ..)` method takes [pin39::W](pin39::W) writer structure"]
impl crate::Writable for PIN39 {}
#[doc = "GPIO_PIN39"]
pub mod pin39;
#[doc = "GPIO_cali_conf\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cali_conf](cali_conf) module"]
pub type CALI_CONF = crate::Reg<u32, _CALI_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALI_CONF;
#[doc = "`read()` method returns [cali_conf::R](cali_conf::R) reader structure"]
impl crate::Readable for CALI_CONF {}
#[doc = "`write(|w| ..)` method takes [cali_conf::W](cali_conf::W) writer structure"]
impl crate::Writable for CALI_CONF {}
#[doc = "GPIO_cali_conf"]
pub mod cali_conf;
#[doc = "GPIO_cali_data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cali_data](cali_data) module"]
pub type CALI_DATA = crate::Reg<u32, _CALI_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALI_DATA;
#[doc = "`read()` method returns [cali_data::R](cali_data::R) reader structure"]
impl crate::Readable for CALI_DATA {}
#[doc = "`write(|w| ..)` method takes [cali_data::W](cali_data::W) writer structure"]
impl crate::Writable for CALI_DATA {}
#[doc = "GPIO_cali_data"]
pub mod cali_data;
#[doc = "GPIO_FUNC0_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func0_in_sel_cfg](func0_in_sel_cfg) module"]
pub type FUNC0_IN_SEL_CFG = crate::Reg<u32, _FUNC0_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC0_IN_SEL_CFG;
#[doc = "`read()` method returns [func0_in_sel_cfg::R](func0_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC0_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func0_in_sel_cfg::W](func0_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC0_IN_SEL_CFG {}
#[doc = "GPIO_FUNC0_IN_SEL_CFG"]
pub mod func0_in_sel_cfg;
#[doc = "GPIO_FUNC1_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func1_in_sel_cfg](func1_in_sel_cfg) module"]
pub type FUNC1_IN_SEL_CFG = crate::Reg<u32, _FUNC1_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC1_IN_SEL_CFG;
#[doc = "`read()` method returns [func1_in_sel_cfg::R](func1_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC1_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func1_in_sel_cfg::W](func1_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC1_IN_SEL_CFG {}
#[doc = "GPIO_FUNC1_IN_SEL_CFG"]
pub mod func1_in_sel_cfg;
#[doc = "GPIO_FUNC2_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func2_in_sel_cfg](func2_in_sel_cfg) module"]
pub type FUNC2_IN_SEL_CFG = crate::Reg<u32, _FUNC2_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC2_IN_SEL_CFG;
#[doc = "`read()` method returns [func2_in_sel_cfg::R](func2_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC2_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func2_in_sel_cfg::W](func2_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC2_IN_SEL_CFG {}
#[doc = "GPIO_FUNC2_IN_SEL_CFG"]
pub mod func2_in_sel_cfg;
#[doc = "GPIO_FUNC3_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func3_in_sel_cfg](func3_in_sel_cfg) module"]
pub type FUNC3_IN_SEL_CFG = crate::Reg<u32, _FUNC3_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC3_IN_SEL_CFG;
#[doc = "`read()` method returns [func3_in_sel_cfg::R](func3_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC3_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func3_in_sel_cfg::W](func3_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC3_IN_SEL_CFG {}
#[doc = "GPIO_FUNC3_IN_SEL_CFG"]
pub mod func3_in_sel_cfg;
#[doc = "GPIO_FUNC4_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func4_in_sel_cfg](func4_in_sel_cfg) module"]
pub type FUNC4_IN_SEL_CFG = crate::Reg<u32, _FUNC4_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC4_IN_SEL_CFG;
#[doc = "`read()` method returns [func4_in_sel_cfg::R](func4_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC4_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func4_in_sel_cfg::W](func4_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC4_IN_SEL_CFG {}
#[doc = "GPIO_FUNC4_IN_SEL_CFG"]
pub mod func4_in_sel_cfg;
#[doc = "GPIO_FUNC5_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func5_in_sel_cfg](func5_in_sel_cfg) module"]
pub type FUNC5_IN_SEL_CFG = crate::Reg<u32, _FUNC5_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC5_IN_SEL_CFG;
#[doc = "`read()` method returns [func5_in_sel_cfg::R](func5_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC5_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func5_in_sel_cfg::W](func5_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC5_IN_SEL_CFG {}
#[doc = "GPIO_FUNC5_IN_SEL_CFG"]
pub mod func5_in_sel_cfg;
#[doc = "GPIO_FUNC6_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func6_in_sel_cfg](func6_in_sel_cfg) module"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<u32, _FUNC6_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC6_IN_SEL_CFG;
#[doc = "`read()` method returns [func6_in_sel_cfg::R](func6_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC6_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func6_in_sel_cfg::W](func6_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC6_IN_SEL_CFG {}
#[doc = "GPIO_FUNC6_IN_SEL_CFG"]
pub mod func6_in_sel_cfg;
#[doc = "GPIO_FUNC7_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func7_in_sel_cfg](func7_in_sel_cfg) module"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<u32, _FUNC7_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC7_IN_SEL_CFG;
#[doc = "`read()` method returns [func7_in_sel_cfg::R](func7_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC7_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func7_in_sel_cfg::W](func7_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC7_IN_SEL_CFG {}
#[doc = "GPIO_FUNC7_IN_SEL_CFG"]
pub mod func7_in_sel_cfg;
#[doc = "GPIO_FUNC8_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func8_in_sel_cfg](func8_in_sel_cfg) module"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<u32, _FUNC8_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC8_IN_SEL_CFG;
#[doc = "`read()` method returns [func8_in_sel_cfg::R](func8_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC8_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func8_in_sel_cfg::W](func8_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC8_IN_SEL_CFG {}
#[doc = "GPIO_FUNC8_IN_SEL_CFG"]
pub mod func8_in_sel_cfg;
#[doc = "GPIO_FUNC9_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func9_in_sel_cfg](func9_in_sel_cfg) module"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<u32, _FUNC9_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC9_IN_SEL_CFG;
#[doc = "`read()` method returns [func9_in_sel_cfg::R](func9_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC9_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func9_in_sel_cfg::W](func9_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC9_IN_SEL_CFG {}
#[doc = "GPIO_FUNC9_IN_SEL_CFG"]
pub mod func9_in_sel_cfg;
#[doc = "GPIO_FUNC10_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func10_in_sel_cfg](func10_in_sel_cfg) module"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<u32, _FUNC10_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC10_IN_SEL_CFG;
#[doc = "`read()` method returns [func10_in_sel_cfg::R](func10_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC10_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func10_in_sel_cfg::W](func10_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC10_IN_SEL_CFG {}
#[doc = "GPIO_FUNC10_IN_SEL_CFG"]
pub mod func10_in_sel_cfg;
#[doc = "GPIO_FUNC11_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func11_in_sel_cfg](func11_in_sel_cfg) module"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<u32, _FUNC11_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC11_IN_SEL_CFG;
#[doc = "`read()` method returns [func11_in_sel_cfg::R](func11_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC11_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func11_in_sel_cfg::W](func11_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC11_IN_SEL_CFG {}
#[doc = "GPIO_FUNC11_IN_SEL_CFG"]
pub mod func11_in_sel_cfg;
#[doc = "GPIO_FUNC12_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func12_in_sel_cfg](func12_in_sel_cfg) module"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<u32, _FUNC12_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC12_IN_SEL_CFG;
#[doc = "`read()` method returns [func12_in_sel_cfg::R](func12_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC12_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func12_in_sel_cfg::W](func12_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC12_IN_SEL_CFG {}
#[doc = "GPIO_FUNC12_IN_SEL_CFG"]
pub mod func12_in_sel_cfg;
#[doc = "GPIO_FUNC13_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func13_in_sel_cfg](func13_in_sel_cfg) module"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<u32, _FUNC13_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC13_IN_SEL_CFG;
#[doc = "`read()` method returns [func13_in_sel_cfg::R](func13_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC13_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func13_in_sel_cfg::W](func13_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC13_IN_SEL_CFG {}
#[doc = "GPIO_FUNC13_IN_SEL_CFG"]
pub mod func13_in_sel_cfg;
#[doc = "GPIO_FUNC14_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func14_in_sel_cfg](func14_in_sel_cfg) module"]
pub type FUNC14_IN_SEL_CFG = crate::Reg<u32, _FUNC14_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC14_IN_SEL_CFG;
#[doc = "`read()` method returns [func14_in_sel_cfg::R](func14_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC14_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func14_in_sel_cfg::W](func14_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC14_IN_SEL_CFG {}
#[doc = "GPIO_FUNC14_IN_SEL_CFG"]
pub mod func14_in_sel_cfg;
#[doc = "GPIO_FUNC15_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func15_in_sel_cfg](func15_in_sel_cfg) module"]
pub type FUNC15_IN_SEL_CFG = crate::Reg<u32, _FUNC15_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC15_IN_SEL_CFG;
#[doc = "`read()` method returns [func15_in_sel_cfg::R](func15_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC15_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func15_in_sel_cfg::W](func15_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC15_IN_SEL_CFG {}
#[doc = "GPIO_FUNC15_IN_SEL_CFG"]
pub mod func15_in_sel_cfg;
#[doc = "GPIO_FUNC16_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func16_in_sel_cfg](func16_in_sel_cfg) module"]
pub type FUNC16_IN_SEL_CFG = crate::Reg<u32, _FUNC16_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC16_IN_SEL_CFG;
#[doc = "`read()` method returns [func16_in_sel_cfg::R](func16_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC16_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func16_in_sel_cfg::W](func16_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC16_IN_SEL_CFG {}
#[doc = "GPIO_FUNC16_IN_SEL_CFG"]
pub mod func16_in_sel_cfg;
#[doc = "GPIO_FUNC17_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func17_in_sel_cfg](func17_in_sel_cfg) module"]
pub type FUNC17_IN_SEL_CFG = crate::Reg<u32, _FUNC17_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC17_IN_SEL_CFG;
#[doc = "`read()` method returns [func17_in_sel_cfg::R](func17_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC17_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func17_in_sel_cfg::W](func17_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC17_IN_SEL_CFG {}
#[doc = "GPIO_FUNC17_IN_SEL_CFG"]
pub mod func17_in_sel_cfg;
#[doc = "GPIO_FUNC18_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func18_in_sel_cfg](func18_in_sel_cfg) module"]
pub type FUNC18_IN_SEL_CFG = crate::Reg<u32, _FUNC18_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC18_IN_SEL_CFG;
#[doc = "`read()` method returns [func18_in_sel_cfg::R](func18_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC18_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func18_in_sel_cfg::W](func18_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC18_IN_SEL_CFG {}
#[doc = "GPIO_FUNC18_IN_SEL_CFG"]
pub mod func18_in_sel_cfg;
#[doc = "GPIO_FUNC19_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func19_in_sel_cfg](func19_in_sel_cfg) module"]
pub type FUNC19_IN_SEL_CFG = crate::Reg<u32, _FUNC19_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC19_IN_SEL_CFG;
#[doc = "`read()` method returns [func19_in_sel_cfg::R](func19_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC19_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func19_in_sel_cfg::W](func19_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC19_IN_SEL_CFG {}
#[doc = "GPIO_FUNC19_IN_SEL_CFG"]
pub mod func19_in_sel_cfg;
#[doc = "GPIO_FUNC20_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func20_in_sel_cfg](func20_in_sel_cfg) module"]
pub type FUNC20_IN_SEL_CFG = crate::Reg<u32, _FUNC20_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC20_IN_SEL_CFG;
#[doc = "`read()` method returns [func20_in_sel_cfg::R](func20_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC20_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func20_in_sel_cfg::W](func20_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC20_IN_SEL_CFG {}
#[doc = "GPIO_FUNC20_IN_SEL_CFG"]
pub mod func20_in_sel_cfg;
#[doc = "GPIO_FUNC21_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func21_in_sel_cfg](func21_in_sel_cfg) module"]
pub type FUNC21_IN_SEL_CFG = crate::Reg<u32, _FUNC21_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC21_IN_SEL_CFG;
#[doc = "`read()` method returns [func21_in_sel_cfg::R](func21_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC21_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func21_in_sel_cfg::W](func21_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC21_IN_SEL_CFG {}
#[doc = "GPIO_FUNC21_IN_SEL_CFG"]
pub mod func21_in_sel_cfg;
#[doc = "GPIO_FUNC22_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func22_in_sel_cfg](func22_in_sel_cfg) module"]
pub type FUNC22_IN_SEL_CFG = crate::Reg<u32, _FUNC22_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC22_IN_SEL_CFG;
#[doc = "`read()` method returns [func22_in_sel_cfg::R](func22_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC22_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func22_in_sel_cfg::W](func22_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC22_IN_SEL_CFG {}
#[doc = "GPIO_FUNC22_IN_SEL_CFG"]
pub mod func22_in_sel_cfg;
#[doc = "GPIO_FUNC23_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func23_in_sel_cfg](func23_in_sel_cfg) module"]
pub type FUNC23_IN_SEL_CFG = crate::Reg<u32, _FUNC23_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC23_IN_SEL_CFG;
#[doc = "`read()` method returns [func23_in_sel_cfg::R](func23_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC23_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func23_in_sel_cfg::W](func23_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC23_IN_SEL_CFG {}
#[doc = "GPIO_FUNC23_IN_SEL_CFG"]
pub mod func23_in_sel_cfg;
#[doc = "GPIO_FUNC24_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func24_in_sel_cfg](func24_in_sel_cfg) module"]
pub type FUNC24_IN_SEL_CFG = crate::Reg<u32, _FUNC24_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC24_IN_SEL_CFG;
#[doc = "`read()` method returns [func24_in_sel_cfg::R](func24_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC24_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func24_in_sel_cfg::W](func24_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC24_IN_SEL_CFG {}
#[doc = "GPIO_FUNC24_IN_SEL_CFG"]
pub mod func24_in_sel_cfg;
#[doc = "GPIO_FUNC25_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func25_in_sel_cfg](func25_in_sel_cfg) module"]
pub type FUNC25_IN_SEL_CFG = crate::Reg<u32, _FUNC25_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC25_IN_SEL_CFG;
#[doc = "`read()` method returns [func25_in_sel_cfg::R](func25_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC25_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func25_in_sel_cfg::W](func25_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC25_IN_SEL_CFG {}
#[doc = "GPIO_FUNC25_IN_SEL_CFG"]
pub mod func25_in_sel_cfg;
#[doc = "GPIO_FUNC26_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func26_in_sel_cfg](func26_in_sel_cfg) module"]
pub type FUNC26_IN_SEL_CFG = crate::Reg<u32, _FUNC26_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC26_IN_SEL_CFG;
#[doc = "`read()` method returns [func26_in_sel_cfg::R](func26_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC26_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func26_in_sel_cfg::W](func26_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC26_IN_SEL_CFG {}
#[doc = "GPIO_FUNC26_IN_SEL_CFG"]
pub mod func26_in_sel_cfg;
#[doc = "GPIO_FUNC27_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func27_in_sel_cfg](func27_in_sel_cfg) module"]
pub type FUNC27_IN_SEL_CFG = crate::Reg<u32, _FUNC27_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC27_IN_SEL_CFG;
#[doc = "`read()` method returns [func27_in_sel_cfg::R](func27_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC27_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func27_in_sel_cfg::W](func27_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC27_IN_SEL_CFG {}
#[doc = "GPIO_FUNC27_IN_SEL_CFG"]
pub mod func27_in_sel_cfg;
#[doc = "GPIO_FUNC28_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func28_in_sel_cfg](func28_in_sel_cfg) module"]
pub type FUNC28_IN_SEL_CFG = crate::Reg<u32, _FUNC28_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC28_IN_SEL_CFG;
#[doc = "`read()` method returns [func28_in_sel_cfg::R](func28_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC28_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func28_in_sel_cfg::W](func28_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC28_IN_SEL_CFG {}
#[doc = "GPIO_FUNC28_IN_SEL_CFG"]
pub mod func28_in_sel_cfg;
#[doc = "GPIO_FUNC29_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func29_in_sel_cfg](func29_in_sel_cfg) module"]
pub type FUNC29_IN_SEL_CFG = crate::Reg<u32, _FUNC29_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC29_IN_SEL_CFG;
#[doc = "`read()` method returns [func29_in_sel_cfg::R](func29_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC29_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func29_in_sel_cfg::W](func29_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC29_IN_SEL_CFG {}
#[doc = "GPIO_FUNC29_IN_SEL_CFG"]
pub mod func29_in_sel_cfg;
#[doc = "GPIO_FUNC30_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func30_in_sel_cfg](func30_in_sel_cfg) module"]
pub type FUNC30_IN_SEL_CFG = crate::Reg<u32, _FUNC30_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC30_IN_SEL_CFG;
#[doc = "`read()` method returns [func30_in_sel_cfg::R](func30_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC30_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func30_in_sel_cfg::W](func30_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC30_IN_SEL_CFG {}
#[doc = "GPIO_FUNC30_IN_SEL_CFG"]
pub mod func30_in_sel_cfg;
#[doc = "GPIO_FUNC31_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func31_in_sel_cfg](func31_in_sel_cfg) module"]
pub type FUNC31_IN_SEL_CFG = crate::Reg<u32, _FUNC31_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC31_IN_SEL_CFG;
#[doc = "`read()` method returns [func31_in_sel_cfg::R](func31_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC31_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func31_in_sel_cfg::W](func31_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC31_IN_SEL_CFG {}
#[doc = "GPIO_FUNC31_IN_SEL_CFG"]
pub mod func31_in_sel_cfg;
#[doc = "GPIO_FUNC32_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func32_in_sel_cfg](func32_in_sel_cfg) module"]
pub type FUNC32_IN_SEL_CFG = crate::Reg<u32, _FUNC32_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC32_IN_SEL_CFG;
#[doc = "`read()` method returns [func32_in_sel_cfg::R](func32_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC32_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func32_in_sel_cfg::W](func32_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC32_IN_SEL_CFG {}
#[doc = "GPIO_FUNC32_IN_SEL_CFG"]
pub mod func32_in_sel_cfg;
#[doc = "GPIO_FUNC33_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func33_in_sel_cfg](func33_in_sel_cfg) module"]
pub type FUNC33_IN_SEL_CFG = crate::Reg<u32, _FUNC33_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC33_IN_SEL_CFG;
#[doc = "`read()` method returns [func33_in_sel_cfg::R](func33_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC33_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func33_in_sel_cfg::W](func33_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC33_IN_SEL_CFG {}
#[doc = "GPIO_FUNC33_IN_SEL_CFG"]
pub mod func33_in_sel_cfg;
#[doc = "GPIO_FUNC34_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func34_in_sel_cfg](func34_in_sel_cfg) module"]
pub type FUNC34_IN_SEL_CFG = crate::Reg<u32, _FUNC34_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC34_IN_SEL_CFG;
#[doc = "`read()` method returns [func34_in_sel_cfg::R](func34_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC34_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func34_in_sel_cfg::W](func34_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC34_IN_SEL_CFG {}
#[doc = "GPIO_FUNC34_IN_SEL_CFG"]
pub mod func34_in_sel_cfg;
#[doc = "GPIO_FUNC35_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func35_in_sel_cfg](func35_in_sel_cfg) module"]
pub type FUNC35_IN_SEL_CFG = crate::Reg<u32, _FUNC35_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC35_IN_SEL_CFG;
#[doc = "`read()` method returns [func35_in_sel_cfg::R](func35_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC35_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func35_in_sel_cfg::W](func35_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC35_IN_SEL_CFG {}
#[doc = "GPIO_FUNC35_IN_SEL_CFG"]
pub mod func35_in_sel_cfg;
#[doc = "GPIO_FUNC36_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func36_in_sel_cfg](func36_in_sel_cfg) module"]
pub type FUNC36_IN_SEL_CFG = crate::Reg<u32, _FUNC36_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC36_IN_SEL_CFG;
#[doc = "`read()` method returns [func36_in_sel_cfg::R](func36_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC36_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func36_in_sel_cfg::W](func36_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC36_IN_SEL_CFG {}
#[doc = "GPIO_FUNC36_IN_SEL_CFG"]
pub mod func36_in_sel_cfg;
#[doc = "GPIO_FUNC37_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func37_in_sel_cfg](func37_in_sel_cfg) module"]
pub type FUNC37_IN_SEL_CFG = crate::Reg<u32, _FUNC37_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC37_IN_SEL_CFG;
#[doc = "`read()` method returns [func37_in_sel_cfg::R](func37_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC37_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func37_in_sel_cfg::W](func37_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC37_IN_SEL_CFG {}
#[doc = "GPIO_FUNC37_IN_SEL_CFG"]
pub mod func37_in_sel_cfg;
#[doc = "GPIO_FUNC38_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func38_in_sel_cfg](func38_in_sel_cfg) module"]
pub type FUNC38_IN_SEL_CFG = crate::Reg<u32, _FUNC38_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC38_IN_SEL_CFG;
#[doc = "`read()` method returns [func38_in_sel_cfg::R](func38_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC38_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func38_in_sel_cfg::W](func38_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC38_IN_SEL_CFG {}
#[doc = "GPIO_FUNC38_IN_SEL_CFG"]
pub mod func38_in_sel_cfg;
#[doc = "GPIO_FUNC39_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func39_in_sel_cfg](func39_in_sel_cfg) module"]
pub type FUNC39_IN_SEL_CFG = crate::Reg<u32, _FUNC39_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC39_IN_SEL_CFG;
#[doc = "`read()` method returns [func39_in_sel_cfg::R](func39_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC39_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func39_in_sel_cfg::W](func39_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC39_IN_SEL_CFG {}
#[doc = "GPIO_FUNC39_IN_SEL_CFG"]
pub mod func39_in_sel_cfg;
#[doc = "GPIO_FUNC40_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func40_in_sel_cfg](func40_in_sel_cfg) module"]
pub type FUNC40_IN_SEL_CFG = crate::Reg<u32, _FUNC40_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC40_IN_SEL_CFG;
#[doc = "`read()` method returns [func40_in_sel_cfg::R](func40_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC40_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func40_in_sel_cfg::W](func40_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC40_IN_SEL_CFG {}
#[doc = "GPIO_FUNC40_IN_SEL_CFG"]
pub mod func40_in_sel_cfg;
#[doc = "GPIO_FUNC41_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func41_in_sel_cfg](func41_in_sel_cfg) module"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<u32, _FUNC41_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC41_IN_SEL_CFG;
#[doc = "`read()` method returns [func41_in_sel_cfg::R](func41_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC41_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func41_in_sel_cfg::W](func41_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC41_IN_SEL_CFG {}
#[doc = "GPIO_FUNC41_IN_SEL_CFG"]
pub mod func41_in_sel_cfg;
#[doc = "GPIO_FUNC42_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func42_in_sel_cfg](func42_in_sel_cfg) module"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<u32, _FUNC42_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC42_IN_SEL_CFG;
#[doc = "`read()` method returns [func42_in_sel_cfg::R](func42_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC42_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func42_in_sel_cfg::W](func42_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC42_IN_SEL_CFG {}
#[doc = "GPIO_FUNC42_IN_SEL_CFG"]
pub mod func42_in_sel_cfg;
#[doc = "GPIO_FUNC43_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func43_in_sel_cfg](func43_in_sel_cfg) module"]
pub type FUNC43_IN_SEL_CFG = crate::Reg<u32, _FUNC43_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC43_IN_SEL_CFG;
#[doc = "`read()` method returns [func43_in_sel_cfg::R](func43_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC43_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func43_in_sel_cfg::W](func43_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC43_IN_SEL_CFG {}
#[doc = "GPIO_FUNC43_IN_SEL_CFG"]
pub mod func43_in_sel_cfg;
#[doc = "GPIO_FUNC44_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func44_in_sel_cfg](func44_in_sel_cfg) module"]
pub type FUNC44_IN_SEL_CFG = crate::Reg<u32, _FUNC44_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC44_IN_SEL_CFG;
#[doc = "`read()` method returns [func44_in_sel_cfg::R](func44_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC44_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func44_in_sel_cfg::W](func44_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC44_IN_SEL_CFG {}
#[doc = "GPIO_FUNC44_IN_SEL_CFG"]
pub mod func44_in_sel_cfg;
#[doc = "GPIO_FUNC45_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func45_in_sel_cfg](func45_in_sel_cfg) module"]
pub type FUNC45_IN_SEL_CFG = crate::Reg<u32, _FUNC45_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC45_IN_SEL_CFG;
#[doc = "`read()` method returns [func45_in_sel_cfg::R](func45_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC45_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func45_in_sel_cfg::W](func45_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC45_IN_SEL_CFG {}
#[doc = "GPIO_FUNC45_IN_SEL_CFG"]
pub mod func45_in_sel_cfg;
#[doc = "GPIO_FUNC46_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func46_in_sel_cfg](func46_in_sel_cfg) module"]
pub type FUNC46_IN_SEL_CFG = crate::Reg<u32, _FUNC46_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC46_IN_SEL_CFG;
#[doc = "`read()` method returns [func46_in_sel_cfg::R](func46_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC46_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func46_in_sel_cfg::W](func46_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC46_IN_SEL_CFG {}
#[doc = "GPIO_FUNC46_IN_SEL_CFG"]
pub mod func46_in_sel_cfg;
#[doc = "GPIO_FUNC47_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func47_in_sel_cfg](func47_in_sel_cfg) module"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<u32, _FUNC47_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC47_IN_SEL_CFG;
#[doc = "`read()` method returns [func47_in_sel_cfg::R](func47_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC47_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func47_in_sel_cfg::W](func47_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC47_IN_SEL_CFG {}
#[doc = "GPIO_FUNC47_IN_SEL_CFG"]
pub mod func47_in_sel_cfg;
#[doc = "GPIO_FUNC48_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func48_in_sel_cfg](func48_in_sel_cfg) module"]
pub type FUNC48_IN_SEL_CFG = crate::Reg<u32, _FUNC48_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC48_IN_SEL_CFG;
#[doc = "`read()` method returns [func48_in_sel_cfg::R](func48_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC48_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func48_in_sel_cfg::W](func48_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC48_IN_SEL_CFG {}
#[doc = "GPIO_FUNC48_IN_SEL_CFG"]
pub mod func48_in_sel_cfg;
#[doc = "GPIO_FUNC49_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func49_in_sel_cfg](func49_in_sel_cfg) module"]
pub type FUNC49_IN_SEL_CFG = crate::Reg<u32, _FUNC49_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC49_IN_SEL_CFG;
#[doc = "`read()` method returns [func49_in_sel_cfg::R](func49_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC49_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func49_in_sel_cfg::W](func49_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC49_IN_SEL_CFG {}
#[doc = "GPIO_FUNC49_IN_SEL_CFG"]
pub mod func49_in_sel_cfg;
#[doc = "GPIO_FUNC50_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func50_in_sel_cfg](func50_in_sel_cfg) module"]
pub type FUNC50_IN_SEL_CFG = crate::Reg<u32, _FUNC50_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC50_IN_SEL_CFG;
#[doc = "`read()` method returns [func50_in_sel_cfg::R](func50_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC50_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func50_in_sel_cfg::W](func50_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC50_IN_SEL_CFG {}
#[doc = "GPIO_FUNC50_IN_SEL_CFG"]
pub mod func50_in_sel_cfg;
#[doc = "GPIO_FUNC51_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func51_in_sel_cfg](func51_in_sel_cfg) module"]
pub type FUNC51_IN_SEL_CFG = crate::Reg<u32, _FUNC51_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC51_IN_SEL_CFG;
#[doc = "`read()` method returns [func51_in_sel_cfg::R](func51_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC51_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func51_in_sel_cfg::W](func51_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC51_IN_SEL_CFG {}
#[doc = "GPIO_FUNC51_IN_SEL_CFG"]
pub mod func51_in_sel_cfg;
#[doc = "GPIO_FUNC52_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func52_in_sel_cfg](func52_in_sel_cfg) module"]
pub type FUNC52_IN_SEL_CFG = crate::Reg<u32, _FUNC52_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC52_IN_SEL_CFG;
#[doc = "`read()` method returns [func52_in_sel_cfg::R](func52_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC52_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func52_in_sel_cfg::W](func52_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC52_IN_SEL_CFG {}
#[doc = "GPIO_FUNC52_IN_SEL_CFG"]
pub mod func52_in_sel_cfg;
#[doc = "GPIO_FUNC53_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func53_in_sel_cfg](func53_in_sel_cfg) module"]
pub type FUNC53_IN_SEL_CFG = crate::Reg<u32, _FUNC53_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC53_IN_SEL_CFG;
#[doc = "`read()` method returns [func53_in_sel_cfg::R](func53_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC53_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func53_in_sel_cfg::W](func53_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC53_IN_SEL_CFG {}
#[doc = "GPIO_FUNC53_IN_SEL_CFG"]
pub mod func53_in_sel_cfg;
#[doc = "GPIO_FUNC54_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func54_in_sel_cfg](func54_in_sel_cfg) module"]
pub type FUNC54_IN_SEL_CFG = crate::Reg<u32, _FUNC54_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC54_IN_SEL_CFG;
#[doc = "`read()` method returns [func54_in_sel_cfg::R](func54_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC54_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func54_in_sel_cfg::W](func54_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC54_IN_SEL_CFG {}
#[doc = "GPIO_FUNC54_IN_SEL_CFG"]
pub mod func54_in_sel_cfg;
#[doc = "GPIO_FUNC55_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func55_in_sel_cfg](func55_in_sel_cfg) module"]
pub type FUNC55_IN_SEL_CFG = crate::Reg<u32, _FUNC55_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC55_IN_SEL_CFG;
#[doc = "`read()` method returns [func55_in_sel_cfg::R](func55_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC55_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func55_in_sel_cfg::W](func55_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC55_IN_SEL_CFG {}
#[doc = "GPIO_FUNC55_IN_SEL_CFG"]
pub mod func55_in_sel_cfg;
#[doc = "GPIO_FUNC56_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func56_in_sel_cfg](func56_in_sel_cfg) module"]
pub type FUNC56_IN_SEL_CFG = crate::Reg<u32, _FUNC56_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC56_IN_SEL_CFG;
#[doc = "`read()` method returns [func56_in_sel_cfg::R](func56_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC56_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func56_in_sel_cfg::W](func56_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC56_IN_SEL_CFG {}
#[doc = "GPIO_FUNC56_IN_SEL_CFG"]
pub mod func56_in_sel_cfg;
#[doc = "GPIO_FUNC57_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func57_in_sel_cfg](func57_in_sel_cfg) module"]
pub type FUNC57_IN_SEL_CFG = crate::Reg<u32, _FUNC57_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC57_IN_SEL_CFG;
#[doc = "`read()` method returns [func57_in_sel_cfg::R](func57_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC57_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func57_in_sel_cfg::W](func57_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC57_IN_SEL_CFG {}
#[doc = "GPIO_FUNC57_IN_SEL_CFG"]
pub mod func57_in_sel_cfg;
#[doc = "GPIO_FUNC58_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func58_in_sel_cfg](func58_in_sel_cfg) module"]
pub type FUNC58_IN_SEL_CFG = crate::Reg<u32, _FUNC58_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC58_IN_SEL_CFG;
#[doc = "`read()` method returns [func58_in_sel_cfg::R](func58_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC58_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func58_in_sel_cfg::W](func58_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC58_IN_SEL_CFG {}
#[doc = "GPIO_FUNC58_IN_SEL_CFG"]
pub mod func58_in_sel_cfg;
#[doc = "GPIO_FUNC59_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func59_in_sel_cfg](func59_in_sel_cfg) module"]
pub type FUNC59_IN_SEL_CFG = crate::Reg<u32, _FUNC59_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC59_IN_SEL_CFG;
#[doc = "`read()` method returns [func59_in_sel_cfg::R](func59_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC59_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func59_in_sel_cfg::W](func59_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC59_IN_SEL_CFG {}
#[doc = "GPIO_FUNC59_IN_SEL_CFG"]
pub mod func59_in_sel_cfg;
#[doc = "GPIO_FUNC60_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func60_in_sel_cfg](func60_in_sel_cfg) module"]
pub type FUNC60_IN_SEL_CFG = crate::Reg<u32, _FUNC60_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC60_IN_SEL_CFG;
#[doc = "`read()` method returns [func60_in_sel_cfg::R](func60_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC60_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func60_in_sel_cfg::W](func60_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC60_IN_SEL_CFG {}
#[doc = "GPIO_FUNC60_IN_SEL_CFG"]
pub mod func60_in_sel_cfg;
#[doc = "GPIO_FUNC61_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func61_in_sel_cfg](func61_in_sel_cfg) module"]
pub type FUNC61_IN_SEL_CFG = crate::Reg<u32, _FUNC61_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC61_IN_SEL_CFG;
#[doc = "`read()` method returns [func61_in_sel_cfg::R](func61_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC61_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func61_in_sel_cfg::W](func61_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC61_IN_SEL_CFG {}
#[doc = "GPIO_FUNC61_IN_SEL_CFG"]
pub mod func61_in_sel_cfg;
#[doc = "GPIO_FUNC62_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func62_in_sel_cfg](func62_in_sel_cfg) module"]
pub type FUNC62_IN_SEL_CFG = crate::Reg<u32, _FUNC62_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC62_IN_SEL_CFG;
#[doc = "`read()` method returns [func62_in_sel_cfg::R](func62_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC62_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func62_in_sel_cfg::W](func62_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC62_IN_SEL_CFG {}
#[doc = "GPIO_FUNC62_IN_SEL_CFG"]
pub mod func62_in_sel_cfg;
#[doc = "GPIO_FUNC63_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func63_in_sel_cfg](func63_in_sel_cfg) module"]
pub type FUNC63_IN_SEL_CFG = crate::Reg<u32, _FUNC63_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC63_IN_SEL_CFG;
#[doc = "`read()` method returns [func63_in_sel_cfg::R](func63_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC63_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func63_in_sel_cfg::W](func63_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC63_IN_SEL_CFG {}
#[doc = "GPIO_FUNC63_IN_SEL_CFG"]
pub mod func63_in_sel_cfg;
#[doc = "GPIO_FUNC64_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func64_in_sel_cfg](func64_in_sel_cfg) module"]
pub type FUNC64_IN_SEL_CFG = crate::Reg<u32, _FUNC64_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC64_IN_SEL_CFG;
#[doc = "`read()` method returns [func64_in_sel_cfg::R](func64_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC64_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func64_in_sel_cfg::W](func64_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC64_IN_SEL_CFG {}
#[doc = "GPIO_FUNC64_IN_SEL_CFG"]
pub mod func64_in_sel_cfg;
#[doc = "GPIO_FUNC65_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func65_in_sel_cfg](func65_in_sel_cfg) module"]
pub type FUNC65_IN_SEL_CFG = crate::Reg<u32, _FUNC65_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC65_IN_SEL_CFG;
#[doc = "`read()` method returns [func65_in_sel_cfg::R](func65_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC65_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func65_in_sel_cfg::W](func65_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC65_IN_SEL_CFG {}
#[doc = "GPIO_FUNC65_IN_SEL_CFG"]
pub mod func65_in_sel_cfg;
#[doc = "GPIO_FUNC66_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func66_in_sel_cfg](func66_in_sel_cfg) module"]
pub type FUNC66_IN_SEL_CFG = crate::Reg<u32, _FUNC66_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC66_IN_SEL_CFG;
#[doc = "`read()` method returns [func66_in_sel_cfg::R](func66_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC66_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func66_in_sel_cfg::W](func66_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC66_IN_SEL_CFG {}
#[doc = "GPIO_FUNC66_IN_SEL_CFG"]
pub mod func66_in_sel_cfg;
#[doc = "GPIO_FUNC67_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func67_in_sel_cfg](func67_in_sel_cfg) module"]
pub type FUNC67_IN_SEL_CFG = crate::Reg<u32, _FUNC67_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC67_IN_SEL_CFG;
#[doc = "`read()` method returns [func67_in_sel_cfg::R](func67_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC67_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func67_in_sel_cfg::W](func67_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC67_IN_SEL_CFG {}
#[doc = "GPIO_FUNC67_IN_SEL_CFG"]
pub mod func67_in_sel_cfg;
#[doc = "GPIO_FUNC68_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func68_in_sel_cfg](func68_in_sel_cfg) module"]
pub type FUNC68_IN_SEL_CFG = crate::Reg<u32, _FUNC68_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC68_IN_SEL_CFG;
#[doc = "`read()` method returns [func68_in_sel_cfg::R](func68_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC68_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func68_in_sel_cfg::W](func68_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC68_IN_SEL_CFG {}
#[doc = "GPIO_FUNC68_IN_SEL_CFG"]
pub mod func68_in_sel_cfg;
#[doc = "GPIO_FUNC69_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func69_in_sel_cfg](func69_in_sel_cfg) module"]
pub type FUNC69_IN_SEL_CFG = crate::Reg<u32, _FUNC69_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC69_IN_SEL_CFG;
#[doc = "`read()` method returns [func69_in_sel_cfg::R](func69_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC69_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func69_in_sel_cfg::W](func69_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC69_IN_SEL_CFG {}
#[doc = "GPIO_FUNC69_IN_SEL_CFG"]
pub mod func69_in_sel_cfg;
#[doc = "GPIO_FUNC70_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func70_in_sel_cfg](func70_in_sel_cfg) module"]
pub type FUNC70_IN_SEL_CFG = crate::Reg<u32, _FUNC70_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC70_IN_SEL_CFG;
#[doc = "`read()` method returns [func70_in_sel_cfg::R](func70_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC70_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func70_in_sel_cfg::W](func70_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC70_IN_SEL_CFG {}
#[doc = "GPIO_FUNC70_IN_SEL_CFG"]
pub mod func70_in_sel_cfg;
#[doc = "GPIO_FUNC71_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func71_in_sel_cfg](func71_in_sel_cfg) module"]
pub type FUNC71_IN_SEL_CFG = crate::Reg<u32, _FUNC71_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC71_IN_SEL_CFG;
#[doc = "`read()` method returns [func71_in_sel_cfg::R](func71_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC71_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func71_in_sel_cfg::W](func71_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC71_IN_SEL_CFG {}
#[doc = "GPIO_FUNC71_IN_SEL_CFG"]
pub mod func71_in_sel_cfg;
#[doc = "GPIO_FUNC72_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func72_in_sel_cfg](func72_in_sel_cfg) module"]
pub type FUNC72_IN_SEL_CFG = crate::Reg<u32, _FUNC72_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC72_IN_SEL_CFG;
#[doc = "`read()` method returns [func72_in_sel_cfg::R](func72_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC72_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func72_in_sel_cfg::W](func72_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC72_IN_SEL_CFG {}
#[doc = "GPIO_FUNC72_IN_SEL_CFG"]
pub mod func72_in_sel_cfg;
#[doc = "GPIO_FUNC73_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func73_in_sel_cfg](func73_in_sel_cfg) module"]
pub type FUNC73_IN_SEL_CFG = crate::Reg<u32, _FUNC73_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC73_IN_SEL_CFG;
#[doc = "`read()` method returns [func73_in_sel_cfg::R](func73_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC73_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func73_in_sel_cfg::W](func73_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC73_IN_SEL_CFG {}
#[doc = "GPIO_FUNC73_IN_SEL_CFG"]
pub mod func73_in_sel_cfg;
#[doc = "GPIO_FUNC74_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func74_in_sel_cfg](func74_in_sel_cfg) module"]
pub type FUNC74_IN_SEL_CFG = crate::Reg<u32, _FUNC74_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC74_IN_SEL_CFG;
#[doc = "`read()` method returns [func74_in_sel_cfg::R](func74_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC74_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func74_in_sel_cfg::W](func74_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC74_IN_SEL_CFG {}
#[doc = "GPIO_FUNC74_IN_SEL_CFG"]
pub mod func74_in_sel_cfg;
#[doc = "GPIO_FUNC75_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func75_in_sel_cfg](func75_in_sel_cfg) module"]
pub type FUNC75_IN_SEL_CFG = crate::Reg<u32, _FUNC75_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC75_IN_SEL_CFG;
#[doc = "`read()` method returns [func75_in_sel_cfg::R](func75_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC75_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func75_in_sel_cfg::W](func75_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC75_IN_SEL_CFG {}
#[doc = "GPIO_FUNC75_IN_SEL_CFG"]
pub mod func75_in_sel_cfg;
#[doc = "GPIO_FUNC76_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func76_in_sel_cfg](func76_in_sel_cfg) module"]
pub type FUNC76_IN_SEL_CFG = crate::Reg<u32, _FUNC76_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC76_IN_SEL_CFG;
#[doc = "`read()` method returns [func76_in_sel_cfg::R](func76_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC76_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func76_in_sel_cfg::W](func76_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC76_IN_SEL_CFG {}
#[doc = "GPIO_FUNC76_IN_SEL_CFG"]
pub mod func76_in_sel_cfg;
#[doc = "GPIO_FUNC77_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func77_in_sel_cfg](func77_in_sel_cfg) module"]
pub type FUNC77_IN_SEL_CFG = crate::Reg<u32, _FUNC77_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC77_IN_SEL_CFG;
#[doc = "`read()` method returns [func77_in_sel_cfg::R](func77_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC77_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func77_in_sel_cfg::W](func77_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC77_IN_SEL_CFG {}
#[doc = "GPIO_FUNC77_IN_SEL_CFG"]
pub mod func77_in_sel_cfg;
#[doc = "GPIO_FUNC78_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func78_in_sel_cfg](func78_in_sel_cfg) module"]
pub type FUNC78_IN_SEL_CFG = crate::Reg<u32, _FUNC78_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC78_IN_SEL_CFG;
#[doc = "`read()` method returns [func78_in_sel_cfg::R](func78_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC78_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func78_in_sel_cfg::W](func78_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC78_IN_SEL_CFG {}
#[doc = "GPIO_FUNC78_IN_SEL_CFG"]
pub mod func78_in_sel_cfg;
#[doc = "GPIO_FUNC79_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func79_in_sel_cfg](func79_in_sel_cfg) module"]
pub type FUNC79_IN_SEL_CFG = crate::Reg<u32, _FUNC79_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC79_IN_SEL_CFG;
#[doc = "`read()` method returns [func79_in_sel_cfg::R](func79_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC79_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func79_in_sel_cfg::W](func79_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC79_IN_SEL_CFG {}
#[doc = "GPIO_FUNC79_IN_SEL_CFG"]
pub mod func79_in_sel_cfg;
#[doc = "GPIO_FUNC80_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func80_in_sel_cfg](func80_in_sel_cfg) module"]
pub type FUNC80_IN_SEL_CFG = crate::Reg<u32, _FUNC80_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC80_IN_SEL_CFG;
#[doc = "`read()` method returns [func80_in_sel_cfg::R](func80_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC80_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func80_in_sel_cfg::W](func80_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC80_IN_SEL_CFG {}
#[doc = "GPIO_FUNC80_IN_SEL_CFG"]
pub mod func80_in_sel_cfg;
#[doc = "GPIO_FUNC81_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func81_in_sel_cfg](func81_in_sel_cfg) module"]
pub type FUNC81_IN_SEL_CFG = crate::Reg<u32, _FUNC81_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC81_IN_SEL_CFG;
#[doc = "`read()` method returns [func81_in_sel_cfg::R](func81_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC81_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func81_in_sel_cfg::W](func81_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC81_IN_SEL_CFG {}
#[doc = "GPIO_FUNC81_IN_SEL_CFG"]
pub mod func81_in_sel_cfg;
#[doc = "GPIO_FUNC82_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func82_in_sel_cfg](func82_in_sel_cfg) module"]
pub type FUNC82_IN_SEL_CFG = crate::Reg<u32, _FUNC82_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC82_IN_SEL_CFG;
#[doc = "`read()` method returns [func82_in_sel_cfg::R](func82_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC82_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func82_in_sel_cfg::W](func82_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC82_IN_SEL_CFG {}
#[doc = "GPIO_FUNC82_IN_SEL_CFG"]
pub mod func82_in_sel_cfg;
#[doc = "GPIO_FUNC83_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func83_in_sel_cfg](func83_in_sel_cfg) module"]
pub type FUNC83_IN_SEL_CFG = crate::Reg<u32, _FUNC83_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC83_IN_SEL_CFG;
#[doc = "`read()` method returns [func83_in_sel_cfg::R](func83_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC83_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func83_in_sel_cfg::W](func83_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC83_IN_SEL_CFG {}
#[doc = "GPIO_FUNC83_IN_SEL_CFG"]
pub mod func83_in_sel_cfg;
#[doc = "GPIO_FUNC84_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func84_in_sel_cfg](func84_in_sel_cfg) module"]
pub type FUNC84_IN_SEL_CFG = crate::Reg<u32, _FUNC84_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC84_IN_SEL_CFG;
#[doc = "`read()` method returns [func84_in_sel_cfg::R](func84_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC84_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func84_in_sel_cfg::W](func84_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC84_IN_SEL_CFG {}
#[doc = "GPIO_FUNC84_IN_SEL_CFG"]
pub mod func84_in_sel_cfg;
#[doc = "GPIO_FUNC85_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func85_in_sel_cfg](func85_in_sel_cfg) module"]
pub type FUNC85_IN_SEL_CFG = crate::Reg<u32, _FUNC85_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC85_IN_SEL_CFG;
#[doc = "`read()` method returns [func85_in_sel_cfg::R](func85_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC85_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func85_in_sel_cfg::W](func85_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC85_IN_SEL_CFG {}
#[doc = "GPIO_FUNC85_IN_SEL_CFG"]
pub mod func85_in_sel_cfg;
#[doc = "GPIO_FUNC86_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func86_in_sel_cfg](func86_in_sel_cfg) module"]
pub type FUNC86_IN_SEL_CFG = crate::Reg<u32, _FUNC86_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC86_IN_SEL_CFG;
#[doc = "`read()` method returns [func86_in_sel_cfg::R](func86_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC86_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func86_in_sel_cfg::W](func86_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC86_IN_SEL_CFG {}
#[doc = "GPIO_FUNC86_IN_SEL_CFG"]
pub mod func86_in_sel_cfg;
#[doc = "GPIO_FUNC87_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func87_in_sel_cfg](func87_in_sel_cfg) module"]
pub type FUNC87_IN_SEL_CFG = crate::Reg<u32, _FUNC87_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC87_IN_SEL_CFG;
#[doc = "`read()` method returns [func87_in_sel_cfg::R](func87_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC87_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func87_in_sel_cfg::W](func87_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC87_IN_SEL_CFG {}
#[doc = "GPIO_FUNC87_IN_SEL_CFG"]
pub mod func87_in_sel_cfg;
#[doc = "GPIO_FUNC88_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func88_in_sel_cfg](func88_in_sel_cfg) module"]
pub type FUNC88_IN_SEL_CFG = crate::Reg<u32, _FUNC88_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC88_IN_SEL_CFG;
#[doc = "`read()` method returns [func88_in_sel_cfg::R](func88_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC88_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func88_in_sel_cfg::W](func88_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC88_IN_SEL_CFG {}
#[doc = "GPIO_FUNC88_IN_SEL_CFG"]
pub mod func88_in_sel_cfg;
#[doc = "GPIO_FUNC89_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func89_in_sel_cfg](func89_in_sel_cfg) module"]
pub type FUNC89_IN_SEL_CFG = crate::Reg<u32, _FUNC89_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC89_IN_SEL_CFG;
#[doc = "`read()` method returns [func89_in_sel_cfg::R](func89_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC89_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func89_in_sel_cfg::W](func89_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC89_IN_SEL_CFG {}
#[doc = "GPIO_FUNC89_IN_SEL_CFG"]
pub mod func89_in_sel_cfg;
#[doc = "GPIO_FUNC90_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func90_in_sel_cfg](func90_in_sel_cfg) module"]
pub type FUNC90_IN_SEL_CFG = crate::Reg<u32, _FUNC90_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC90_IN_SEL_CFG;
#[doc = "`read()` method returns [func90_in_sel_cfg::R](func90_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC90_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func90_in_sel_cfg::W](func90_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC90_IN_SEL_CFG {}
#[doc = "GPIO_FUNC90_IN_SEL_CFG"]
pub mod func90_in_sel_cfg;
#[doc = "GPIO_FUNC91_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func91_in_sel_cfg](func91_in_sel_cfg) module"]
pub type FUNC91_IN_SEL_CFG = crate::Reg<u32, _FUNC91_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC91_IN_SEL_CFG;
#[doc = "`read()` method returns [func91_in_sel_cfg::R](func91_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC91_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func91_in_sel_cfg::W](func91_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC91_IN_SEL_CFG {}
#[doc = "GPIO_FUNC91_IN_SEL_CFG"]
pub mod func91_in_sel_cfg;
#[doc = "GPIO_FUNC92_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func92_in_sel_cfg](func92_in_sel_cfg) module"]
pub type FUNC92_IN_SEL_CFG = crate::Reg<u32, _FUNC92_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC92_IN_SEL_CFG;
#[doc = "`read()` method returns [func92_in_sel_cfg::R](func92_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC92_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func92_in_sel_cfg::W](func92_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC92_IN_SEL_CFG {}
#[doc = "GPIO_FUNC92_IN_SEL_CFG"]
pub mod func92_in_sel_cfg;
#[doc = "GPIO_FUNC93_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func93_in_sel_cfg](func93_in_sel_cfg) module"]
pub type FUNC93_IN_SEL_CFG = crate::Reg<u32, _FUNC93_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC93_IN_SEL_CFG;
#[doc = "`read()` method returns [func93_in_sel_cfg::R](func93_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC93_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func93_in_sel_cfg::W](func93_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC93_IN_SEL_CFG {}
#[doc = "GPIO_FUNC93_IN_SEL_CFG"]
pub mod func93_in_sel_cfg;
#[doc = "GPIO_FUNC94_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func94_in_sel_cfg](func94_in_sel_cfg) module"]
pub type FUNC94_IN_SEL_CFG = crate::Reg<u32, _FUNC94_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC94_IN_SEL_CFG;
#[doc = "`read()` method returns [func94_in_sel_cfg::R](func94_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC94_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func94_in_sel_cfg::W](func94_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC94_IN_SEL_CFG {}
#[doc = "GPIO_FUNC94_IN_SEL_CFG"]
pub mod func94_in_sel_cfg;
#[doc = "GPIO_FUNC95_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func95_in_sel_cfg](func95_in_sel_cfg) module"]
pub type FUNC95_IN_SEL_CFG = crate::Reg<u32, _FUNC95_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC95_IN_SEL_CFG;
#[doc = "`read()` method returns [func95_in_sel_cfg::R](func95_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC95_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func95_in_sel_cfg::W](func95_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC95_IN_SEL_CFG {}
#[doc = "GPIO_FUNC95_IN_SEL_CFG"]
pub mod func95_in_sel_cfg;
#[doc = "GPIO_FUNC96_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func96_in_sel_cfg](func96_in_sel_cfg) module"]
pub type FUNC96_IN_SEL_CFG = crate::Reg<u32, _FUNC96_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC96_IN_SEL_CFG;
#[doc = "`read()` method returns [func96_in_sel_cfg::R](func96_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC96_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func96_in_sel_cfg::W](func96_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC96_IN_SEL_CFG {}
#[doc = "GPIO_FUNC96_IN_SEL_CFG"]
pub mod func96_in_sel_cfg;
#[doc = "GPIO_FUNC97_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func97_in_sel_cfg](func97_in_sel_cfg) module"]
pub type FUNC97_IN_SEL_CFG = crate::Reg<u32, _FUNC97_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC97_IN_SEL_CFG;
#[doc = "`read()` method returns [func97_in_sel_cfg::R](func97_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC97_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func97_in_sel_cfg::W](func97_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC97_IN_SEL_CFG {}
#[doc = "GPIO_FUNC97_IN_SEL_CFG"]
pub mod func97_in_sel_cfg;
#[doc = "GPIO_FUNC98_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func98_in_sel_cfg](func98_in_sel_cfg) module"]
pub type FUNC98_IN_SEL_CFG = crate::Reg<u32, _FUNC98_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC98_IN_SEL_CFG;
#[doc = "`read()` method returns [func98_in_sel_cfg::R](func98_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC98_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func98_in_sel_cfg::W](func98_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC98_IN_SEL_CFG {}
#[doc = "GPIO_FUNC98_IN_SEL_CFG"]
pub mod func98_in_sel_cfg;
#[doc = "GPIO_FUNC99_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func99_in_sel_cfg](func99_in_sel_cfg) module"]
pub type FUNC99_IN_SEL_CFG = crate::Reg<u32, _FUNC99_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC99_IN_SEL_CFG;
#[doc = "`read()` method returns [func99_in_sel_cfg::R](func99_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC99_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func99_in_sel_cfg::W](func99_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC99_IN_SEL_CFG {}
#[doc = "GPIO_FUNC99_IN_SEL_CFG"]
pub mod func99_in_sel_cfg;
#[doc = "GPIO_FUNC100_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func100_in_sel_cfg](func100_in_sel_cfg) module"]
pub type FUNC100_IN_SEL_CFG = crate::Reg<u32, _FUNC100_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC100_IN_SEL_CFG;
#[doc = "`read()` method returns [func100_in_sel_cfg::R](func100_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC100_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func100_in_sel_cfg::W](func100_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC100_IN_SEL_CFG {}
#[doc = "GPIO_FUNC100_IN_SEL_CFG"]
pub mod func100_in_sel_cfg;
#[doc = "GPIO_FUNC101_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func101_in_sel_cfg](func101_in_sel_cfg) module"]
pub type FUNC101_IN_SEL_CFG = crate::Reg<u32, _FUNC101_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC101_IN_SEL_CFG;
#[doc = "`read()` method returns [func101_in_sel_cfg::R](func101_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC101_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func101_in_sel_cfg::W](func101_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC101_IN_SEL_CFG {}
#[doc = "GPIO_FUNC101_IN_SEL_CFG"]
pub mod func101_in_sel_cfg;
#[doc = "GPIO_FUNC102_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func102_in_sel_cfg](func102_in_sel_cfg) module"]
pub type FUNC102_IN_SEL_CFG = crate::Reg<u32, _FUNC102_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC102_IN_SEL_CFG;
#[doc = "`read()` method returns [func102_in_sel_cfg::R](func102_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC102_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func102_in_sel_cfg::W](func102_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC102_IN_SEL_CFG {}
#[doc = "GPIO_FUNC102_IN_SEL_CFG"]
pub mod func102_in_sel_cfg;
#[doc = "GPIO_FUNC103_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func103_in_sel_cfg](func103_in_sel_cfg) module"]
pub type FUNC103_IN_SEL_CFG = crate::Reg<u32, _FUNC103_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC103_IN_SEL_CFG;
#[doc = "`read()` method returns [func103_in_sel_cfg::R](func103_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC103_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func103_in_sel_cfg::W](func103_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC103_IN_SEL_CFG {}
#[doc = "GPIO_FUNC103_IN_SEL_CFG"]
pub mod func103_in_sel_cfg;
#[doc = "GPIO_FUNC104_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func104_in_sel_cfg](func104_in_sel_cfg) module"]
pub type FUNC104_IN_SEL_CFG = crate::Reg<u32, _FUNC104_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC104_IN_SEL_CFG;
#[doc = "`read()` method returns [func104_in_sel_cfg::R](func104_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC104_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func104_in_sel_cfg::W](func104_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC104_IN_SEL_CFG {}
#[doc = "GPIO_FUNC104_IN_SEL_CFG"]
pub mod func104_in_sel_cfg;
#[doc = "GPIO_FUNC105_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func105_in_sel_cfg](func105_in_sel_cfg) module"]
pub type FUNC105_IN_SEL_CFG = crate::Reg<u32, _FUNC105_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC105_IN_SEL_CFG;
#[doc = "`read()` method returns [func105_in_sel_cfg::R](func105_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC105_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func105_in_sel_cfg::W](func105_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC105_IN_SEL_CFG {}
#[doc = "GPIO_FUNC105_IN_SEL_CFG"]
pub mod func105_in_sel_cfg;
#[doc = "GPIO_FUNC106_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func106_in_sel_cfg](func106_in_sel_cfg) module"]
pub type FUNC106_IN_SEL_CFG = crate::Reg<u32, _FUNC106_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC106_IN_SEL_CFG;
#[doc = "`read()` method returns [func106_in_sel_cfg::R](func106_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC106_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func106_in_sel_cfg::W](func106_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC106_IN_SEL_CFG {}
#[doc = "GPIO_FUNC106_IN_SEL_CFG"]
pub mod func106_in_sel_cfg;
#[doc = "GPIO_FUNC107_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func107_in_sel_cfg](func107_in_sel_cfg) module"]
pub type FUNC107_IN_SEL_CFG = crate::Reg<u32, _FUNC107_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC107_IN_SEL_CFG;
#[doc = "`read()` method returns [func107_in_sel_cfg::R](func107_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC107_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func107_in_sel_cfg::W](func107_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC107_IN_SEL_CFG {}
#[doc = "GPIO_FUNC107_IN_SEL_CFG"]
pub mod func107_in_sel_cfg;
#[doc = "GPIO_FUNC108_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func108_in_sel_cfg](func108_in_sel_cfg) module"]
pub type FUNC108_IN_SEL_CFG = crate::Reg<u32, _FUNC108_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC108_IN_SEL_CFG;
#[doc = "`read()` method returns [func108_in_sel_cfg::R](func108_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC108_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func108_in_sel_cfg::W](func108_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC108_IN_SEL_CFG {}
#[doc = "GPIO_FUNC108_IN_SEL_CFG"]
pub mod func108_in_sel_cfg;
#[doc = "GPIO_FUNC109_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func109_in_sel_cfg](func109_in_sel_cfg) module"]
pub type FUNC109_IN_SEL_CFG = crate::Reg<u32, _FUNC109_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC109_IN_SEL_CFG;
#[doc = "`read()` method returns [func109_in_sel_cfg::R](func109_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC109_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func109_in_sel_cfg::W](func109_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC109_IN_SEL_CFG {}
#[doc = "GPIO_FUNC109_IN_SEL_CFG"]
pub mod func109_in_sel_cfg;
#[doc = "GPIO_FUNC110_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func110_in_sel_cfg](func110_in_sel_cfg) module"]
pub type FUNC110_IN_SEL_CFG = crate::Reg<u32, _FUNC110_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC110_IN_SEL_CFG;
#[doc = "`read()` method returns [func110_in_sel_cfg::R](func110_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC110_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func110_in_sel_cfg::W](func110_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC110_IN_SEL_CFG {}
#[doc = "GPIO_FUNC110_IN_SEL_CFG"]
pub mod func110_in_sel_cfg;
#[doc = "GPIO_FUNC111_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func111_in_sel_cfg](func111_in_sel_cfg) module"]
pub type FUNC111_IN_SEL_CFG = crate::Reg<u32, _FUNC111_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC111_IN_SEL_CFG;
#[doc = "`read()` method returns [func111_in_sel_cfg::R](func111_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC111_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func111_in_sel_cfg::W](func111_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC111_IN_SEL_CFG {}
#[doc = "GPIO_FUNC111_IN_SEL_CFG"]
pub mod func111_in_sel_cfg;
#[doc = "GPIO_FUNC112_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func112_in_sel_cfg](func112_in_sel_cfg) module"]
pub type FUNC112_IN_SEL_CFG = crate::Reg<u32, _FUNC112_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC112_IN_SEL_CFG;
#[doc = "`read()` method returns [func112_in_sel_cfg::R](func112_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC112_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func112_in_sel_cfg::W](func112_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC112_IN_SEL_CFG {}
#[doc = "GPIO_FUNC112_IN_SEL_CFG"]
pub mod func112_in_sel_cfg;
#[doc = "GPIO_FUNC113_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func113_in_sel_cfg](func113_in_sel_cfg) module"]
pub type FUNC113_IN_SEL_CFG = crate::Reg<u32, _FUNC113_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC113_IN_SEL_CFG;
#[doc = "`read()` method returns [func113_in_sel_cfg::R](func113_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC113_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func113_in_sel_cfg::W](func113_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC113_IN_SEL_CFG {}
#[doc = "GPIO_FUNC113_IN_SEL_CFG"]
pub mod func113_in_sel_cfg;
#[doc = "GPIO_FUNC114_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func114_in_sel_cfg](func114_in_sel_cfg) module"]
pub type FUNC114_IN_SEL_CFG = crate::Reg<u32, _FUNC114_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC114_IN_SEL_CFG;
#[doc = "`read()` method returns [func114_in_sel_cfg::R](func114_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC114_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func114_in_sel_cfg::W](func114_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC114_IN_SEL_CFG {}
#[doc = "GPIO_FUNC114_IN_SEL_CFG"]
pub mod func114_in_sel_cfg;
#[doc = "GPIO_FUNC115_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func115_in_sel_cfg](func115_in_sel_cfg) module"]
pub type FUNC115_IN_SEL_CFG = crate::Reg<u32, _FUNC115_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC115_IN_SEL_CFG;
#[doc = "`read()` method returns [func115_in_sel_cfg::R](func115_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC115_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func115_in_sel_cfg::W](func115_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC115_IN_SEL_CFG {}
#[doc = "GPIO_FUNC115_IN_SEL_CFG"]
pub mod func115_in_sel_cfg;
#[doc = "GPIO_FUNC116_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func116_in_sel_cfg](func116_in_sel_cfg) module"]
pub type FUNC116_IN_SEL_CFG = crate::Reg<u32, _FUNC116_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC116_IN_SEL_CFG;
#[doc = "`read()` method returns [func116_in_sel_cfg::R](func116_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC116_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func116_in_sel_cfg::W](func116_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC116_IN_SEL_CFG {}
#[doc = "GPIO_FUNC116_IN_SEL_CFG"]
pub mod func116_in_sel_cfg;
#[doc = "GPIO_FUNC117_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func117_in_sel_cfg](func117_in_sel_cfg) module"]
pub type FUNC117_IN_SEL_CFG = crate::Reg<u32, _FUNC117_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC117_IN_SEL_CFG;
#[doc = "`read()` method returns [func117_in_sel_cfg::R](func117_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC117_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func117_in_sel_cfg::W](func117_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC117_IN_SEL_CFG {}
#[doc = "GPIO_FUNC117_IN_SEL_CFG"]
pub mod func117_in_sel_cfg;
#[doc = "GPIO_FUNC118_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func118_in_sel_cfg](func118_in_sel_cfg) module"]
pub type FUNC118_IN_SEL_CFG = crate::Reg<u32, _FUNC118_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC118_IN_SEL_CFG;
#[doc = "`read()` method returns [func118_in_sel_cfg::R](func118_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC118_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func118_in_sel_cfg::W](func118_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC118_IN_SEL_CFG {}
#[doc = "GPIO_FUNC118_IN_SEL_CFG"]
pub mod func118_in_sel_cfg;
#[doc = "GPIO_FUNC119_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func119_in_sel_cfg](func119_in_sel_cfg) module"]
pub type FUNC119_IN_SEL_CFG = crate::Reg<u32, _FUNC119_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC119_IN_SEL_CFG;
#[doc = "`read()` method returns [func119_in_sel_cfg::R](func119_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC119_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func119_in_sel_cfg::W](func119_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC119_IN_SEL_CFG {}
#[doc = "GPIO_FUNC119_IN_SEL_CFG"]
pub mod func119_in_sel_cfg;
#[doc = "GPIO_FUNC120_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func120_in_sel_cfg](func120_in_sel_cfg) module"]
pub type FUNC120_IN_SEL_CFG = crate::Reg<u32, _FUNC120_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC120_IN_SEL_CFG;
#[doc = "`read()` method returns [func120_in_sel_cfg::R](func120_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC120_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func120_in_sel_cfg::W](func120_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC120_IN_SEL_CFG {}
#[doc = "GPIO_FUNC120_IN_SEL_CFG"]
pub mod func120_in_sel_cfg;
#[doc = "GPIO_FUNC121_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func121_in_sel_cfg](func121_in_sel_cfg) module"]
pub type FUNC121_IN_SEL_CFG = crate::Reg<u32, _FUNC121_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC121_IN_SEL_CFG;
#[doc = "`read()` method returns [func121_in_sel_cfg::R](func121_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC121_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func121_in_sel_cfg::W](func121_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC121_IN_SEL_CFG {}
#[doc = "GPIO_FUNC121_IN_SEL_CFG"]
pub mod func121_in_sel_cfg;
#[doc = "GPIO_FUNC122_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func122_in_sel_cfg](func122_in_sel_cfg) module"]
pub type FUNC122_IN_SEL_CFG = crate::Reg<u32, _FUNC122_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC122_IN_SEL_CFG;
#[doc = "`read()` method returns [func122_in_sel_cfg::R](func122_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC122_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func122_in_sel_cfg::W](func122_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC122_IN_SEL_CFG {}
#[doc = "GPIO_FUNC122_IN_SEL_CFG"]
pub mod func122_in_sel_cfg;
#[doc = "GPIO_FUNC123_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func123_in_sel_cfg](func123_in_sel_cfg) module"]
pub type FUNC123_IN_SEL_CFG = crate::Reg<u32, _FUNC123_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC123_IN_SEL_CFG;
#[doc = "`read()` method returns [func123_in_sel_cfg::R](func123_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC123_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func123_in_sel_cfg::W](func123_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC123_IN_SEL_CFG {}
#[doc = "GPIO_FUNC123_IN_SEL_CFG"]
pub mod func123_in_sel_cfg;
#[doc = "GPIO_FUNC124_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func124_in_sel_cfg](func124_in_sel_cfg) module"]
pub type FUNC124_IN_SEL_CFG = crate::Reg<u32, _FUNC124_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC124_IN_SEL_CFG;
#[doc = "`read()` method returns [func124_in_sel_cfg::R](func124_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC124_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func124_in_sel_cfg::W](func124_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC124_IN_SEL_CFG {}
#[doc = "GPIO_FUNC124_IN_SEL_CFG"]
pub mod func124_in_sel_cfg;
#[doc = "GPIO_FUNC125_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func125_in_sel_cfg](func125_in_sel_cfg) module"]
pub type FUNC125_IN_SEL_CFG = crate::Reg<u32, _FUNC125_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC125_IN_SEL_CFG;
#[doc = "`read()` method returns [func125_in_sel_cfg::R](func125_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC125_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func125_in_sel_cfg::W](func125_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC125_IN_SEL_CFG {}
#[doc = "GPIO_FUNC125_IN_SEL_CFG"]
pub mod func125_in_sel_cfg;
#[doc = "GPIO_FUNC126_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func126_in_sel_cfg](func126_in_sel_cfg) module"]
pub type FUNC126_IN_SEL_CFG = crate::Reg<u32, _FUNC126_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC126_IN_SEL_CFG;
#[doc = "`read()` method returns [func126_in_sel_cfg::R](func126_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC126_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func126_in_sel_cfg::W](func126_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC126_IN_SEL_CFG {}
#[doc = "GPIO_FUNC126_IN_SEL_CFG"]
pub mod func126_in_sel_cfg;
#[doc = "GPIO_FUNC127_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func127_in_sel_cfg](func127_in_sel_cfg) module"]
pub type FUNC127_IN_SEL_CFG = crate::Reg<u32, _FUNC127_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC127_IN_SEL_CFG;
#[doc = "`read()` method returns [func127_in_sel_cfg::R](func127_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC127_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func127_in_sel_cfg::W](func127_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC127_IN_SEL_CFG {}
#[doc = "GPIO_FUNC127_IN_SEL_CFG"]
pub mod func127_in_sel_cfg;
#[doc = "GPIO_FUNC128_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func128_in_sel_cfg](func128_in_sel_cfg) module"]
pub type FUNC128_IN_SEL_CFG = crate::Reg<u32, _FUNC128_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC128_IN_SEL_CFG;
#[doc = "`read()` method returns [func128_in_sel_cfg::R](func128_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC128_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func128_in_sel_cfg::W](func128_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC128_IN_SEL_CFG {}
#[doc = "GPIO_FUNC128_IN_SEL_CFG"]
pub mod func128_in_sel_cfg;
#[doc = "GPIO_FUNC129_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func129_in_sel_cfg](func129_in_sel_cfg) module"]
pub type FUNC129_IN_SEL_CFG = crate::Reg<u32, _FUNC129_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC129_IN_SEL_CFG;
#[doc = "`read()` method returns [func129_in_sel_cfg::R](func129_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC129_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func129_in_sel_cfg::W](func129_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC129_IN_SEL_CFG {}
#[doc = "GPIO_FUNC129_IN_SEL_CFG"]
pub mod func129_in_sel_cfg;
#[doc = "GPIO_FUNC130_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func130_in_sel_cfg](func130_in_sel_cfg) module"]
pub type FUNC130_IN_SEL_CFG = crate::Reg<u32, _FUNC130_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC130_IN_SEL_CFG;
#[doc = "`read()` method returns [func130_in_sel_cfg::R](func130_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC130_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func130_in_sel_cfg::W](func130_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC130_IN_SEL_CFG {}
#[doc = "GPIO_FUNC130_IN_SEL_CFG"]
pub mod func130_in_sel_cfg;
#[doc = "GPIO_FUNC131_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func131_in_sel_cfg](func131_in_sel_cfg) module"]
pub type FUNC131_IN_SEL_CFG = crate::Reg<u32, _FUNC131_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC131_IN_SEL_CFG;
#[doc = "`read()` method returns [func131_in_sel_cfg::R](func131_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC131_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func131_in_sel_cfg::W](func131_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC131_IN_SEL_CFG {}
#[doc = "GPIO_FUNC131_IN_SEL_CFG"]
pub mod func131_in_sel_cfg;
#[doc = "GPIO_FUNC132_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func132_in_sel_cfg](func132_in_sel_cfg) module"]
pub type FUNC132_IN_SEL_CFG = crate::Reg<u32, _FUNC132_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC132_IN_SEL_CFG;
#[doc = "`read()` method returns [func132_in_sel_cfg::R](func132_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC132_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func132_in_sel_cfg::W](func132_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC132_IN_SEL_CFG {}
#[doc = "GPIO_FUNC132_IN_SEL_CFG"]
pub mod func132_in_sel_cfg;
#[doc = "GPIO_FUNC133_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func133_in_sel_cfg](func133_in_sel_cfg) module"]
pub type FUNC133_IN_SEL_CFG = crate::Reg<u32, _FUNC133_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC133_IN_SEL_CFG;
#[doc = "`read()` method returns [func133_in_sel_cfg::R](func133_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC133_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func133_in_sel_cfg::W](func133_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC133_IN_SEL_CFG {}
#[doc = "GPIO_FUNC133_IN_SEL_CFG"]
pub mod func133_in_sel_cfg;
#[doc = "GPIO_FUNC134_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func134_in_sel_cfg](func134_in_sel_cfg) module"]
pub type FUNC134_IN_SEL_CFG = crate::Reg<u32, _FUNC134_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC134_IN_SEL_CFG;
#[doc = "`read()` method returns [func134_in_sel_cfg::R](func134_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC134_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func134_in_sel_cfg::W](func134_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC134_IN_SEL_CFG {}
#[doc = "GPIO_FUNC134_IN_SEL_CFG"]
pub mod func134_in_sel_cfg;
#[doc = "GPIO_FUNC135_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func135_in_sel_cfg](func135_in_sel_cfg) module"]
pub type FUNC135_IN_SEL_CFG = crate::Reg<u32, _FUNC135_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC135_IN_SEL_CFG;
#[doc = "`read()` method returns [func135_in_sel_cfg::R](func135_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC135_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func135_in_sel_cfg::W](func135_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC135_IN_SEL_CFG {}
#[doc = "GPIO_FUNC135_IN_SEL_CFG"]
pub mod func135_in_sel_cfg;
#[doc = "GPIO_FUNC136_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func136_in_sel_cfg](func136_in_sel_cfg) module"]
pub type FUNC136_IN_SEL_CFG = crate::Reg<u32, _FUNC136_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC136_IN_SEL_CFG;
#[doc = "`read()` method returns [func136_in_sel_cfg::R](func136_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC136_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func136_in_sel_cfg::W](func136_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC136_IN_SEL_CFG {}
#[doc = "GPIO_FUNC136_IN_SEL_CFG"]
pub mod func136_in_sel_cfg;
#[doc = "GPIO_FUNC137_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func137_in_sel_cfg](func137_in_sel_cfg) module"]
pub type FUNC137_IN_SEL_CFG = crate::Reg<u32, _FUNC137_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC137_IN_SEL_CFG;
#[doc = "`read()` method returns [func137_in_sel_cfg::R](func137_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC137_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func137_in_sel_cfg::W](func137_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC137_IN_SEL_CFG {}
#[doc = "GPIO_FUNC137_IN_SEL_CFG"]
pub mod func137_in_sel_cfg;
#[doc = "GPIO_FUNC138_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func138_in_sel_cfg](func138_in_sel_cfg) module"]
pub type FUNC138_IN_SEL_CFG = crate::Reg<u32, _FUNC138_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC138_IN_SEL_CFG;
#[doc = "`read()` method returns [func138_in_sel_cfg::R](func138_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC138_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func138_in_sel_cfg::W](func138_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC138_IN_SEL_CFG {}
#[doc = "GPIO_FUNC138_IN_SEL_CFG"]
pub mod func138_in_sel_cfg;
#[doc = "GPIO_FUNC139_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func139_in_sel_cfg](func139_in_sel_cfg) module"]
pub type FUNC139_IN_SEL_CFG = crate::Reg<u32, _FUNC139_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC139_IN_SEL_CFG;
#[doc = "`read()` method returns [func139_in_sel_cfg::R](func139_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC139_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func139_in_sel_cfg::W](func139_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC139_IN_SEL_CFG {}
#[doc = "GPIO_FUNC139_IN_SEL_CFG"]
pub mod func139_in_sel_cfg;
#[doc = "GPIO_FUNC140_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func140_in_sel_cfg](func140_in_sel_cfg) module"]
pub type FUNC140_IN_SEL_CFG = crate::Reg<u32, _FUNC140_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC140_IN_SEL_CFG;
#[doc = "`read()` method returns [func140_in_sel_cfg::R](func140_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC140_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func140_in_sel_cfg::W](func140_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC140_IN_SEL_CFG {}
#[doc = "GPIO_FUNC140_IN_SEL_CFG"]
pub mod func140_in_sel_cfg;
#[doc = "GPIO_FUNC141_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func141_in_sel_cfg](func141_in_sel_cfg) module"]
pub type FUNC141_IN_SEL_CFG = crate::Reg<u32, _FUNC141_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC141_IN_SEL_CFG;
#[doc = "`read()` method returns [func141_in_sel_cfg::R](func141_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC141_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func141_in_sel_cfg::W](func141_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC141_IN_SEL_CFG {}
#[doc = "GPIO_FUNC141_IN_SEL_CFG"]
pub mod func141_in_sel_cfg;
#[doc = "GPIO_FUNC142_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func142_in_sel_cfg](func142_in_sel_cfg) module"]
pub type FUNC142_IN_SEL_CFG = crate::Reg<u32, _FUNC142_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC142_IN_SEL_CFG;
#[doc = "`read()` method returns [func142_in_sel_cfg::R](func142_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC142_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func142_in_sel_cfg::W](func142_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC142_IN_SEL_CFG {}
#[doc = "GPIO_FUNC142_IN_SEL_CFG"]
pub mod func142_in_sel_cfg;
#[doc = "GPIO_FUNC143_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func143_in_sel_cfg](func143_in_sel_cfg) module"]
pub type FUNC143_IN_SEL_CFG = crate::Reg<u32, _FUNC143_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC143_IN_SEL_CFG;
#[doc = "`read()` method returns [func143_in_sel_cfg::R](func143_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC143_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func143_in_sel_cfg::W](func143_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC143_IN_SEL_CFG {}
#[doc = "GPIO_FUNC143_IN_SEL_CFG"]
pub mod func143_in_sel_cfg;
#[doc = "GPIO_FUNC144_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func144_in_sel_cfg](func144_in_sel_cfg) module"]
pub type FUNC144_IN_SEL_CFG = crate::Reg<u32, _FUNC144_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC144_IN_SEL_CFG;
#[doc = "`read()` method returns [func144_in_sel_cfg::R](func144_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC144_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func144_in_sel_cfg::W](func144_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC144_IN_SEL_CFG {}
#[doc = "GPIO_FUNC144_IN_SEL_CFG"]
pub mod func144_in_sel_cfg;
#[doc = "GPIO_FUNC145_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func145_in_sel_cfg](func145_in_sel_cfg) module"]
pub type FUNC145_IN_SEL_CFG = crate::Reg<u32, _FUNC145_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC145_IN_SEL_CFG;
#[doc = "`read()` method returns [func145_in_sel_cfg::R](func145_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC145_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func145_in_sel_cfg::W](func145_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC145_IN_SEL_CFG {}
#[doc = "GPIO_FUNC145_IN_SEL_CFG"]
pub mod func145_in_sel_cfg;
#[doc = "GPIO_FUNC146_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func146_in_sel_cfg](func146_in_sel_cfg) module"]
pub type FUNC146_IN_SEL_CFG = crate::Reg<u32, _FUNC146_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC146_IN_SEL_CFG;
#[doc = "`read()` method returns [func146_in_sel_cfg::R](func146_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC146_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func146_in_sel_cfg::W](func146_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC146_IN_SEL_CFG {}
#[doc = "GPIO_FUNC146_IN_SEL_CFG"]
pub mod func146_in_sel_cfg;
#[doc = "GPIO_FUNC147_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func147_in_sel_cfg](func147_in_sel_cfg) module"]
pub type FUNC147_IN_SEL_CFG = crate::Reg<u32, _FUNC147_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC147_IN_SEL_CFG;
#[doc = "`read()` method returns [func147_in_sel_cfg::R](func147_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC147_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func147_in_sel_cfg::W](func147_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC147_IN_SEL_CFG {}
#[doc = "GPIO_FUNC147_IN_SEL_CFG"]
pub mod func147_in_sel_cfg;
#[doc = "GPIO_FUNC148_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func148_in_sel_cfg](func148_in_sel_cfg) module"]
pub type FUNC148_IN_SEL_CFG = crate::Reg<u32, _FUNC148_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC148_IN_SEL_CFG;
#[doc = "`read()` method returns [func148_in_sel_cfg::R](func148_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC148_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func148_in_sel_cfg::W](func148_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC148_IN_SEL_CFG {}
#[doc = "GPIO_FUNC148_IN_SEL_CFG"]
pub mod func148_in_sel_cfg;
#[doc = "GPIO_FUNC149_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func149_in_sel_cfg](func149_in_sel_cfg) module"]
pub type FUNC149_IN_SEL_CFG = crate::Reg<u32, _FUNC149_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC149_IN_SEL_CFG;
#[doc = "`read()` method returns [func149_in_sel_cfg::R](func149_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC149_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func149_in_sel_cfg::W](func149_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC149_IN_SEL_CFG {}
#[doc = "GPIO_FUNC149_IN_SEL_CFG"]
pub mod func149_in_sel_cfg;
#[doc = "GPIO_FUNC150_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func150_in_sel_cfg](func150_in_sel_cfg) module"]
pub type FUNC150_IN_SEL_CFG = crate::Reg<u32, _FUNC150_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC150_IN_SEL_CFG;
#[doc = "`read()` method returns [func150_in_sel_cfg::R](func150_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC150_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func150_in_sel_cfg::W](func150_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC150_IN_SEL_CFG {}
#[doc = "GPIO_FUNC150_IN_SEL_CFG"]
pub mod func150_in_sel_cfg;
#[doc = "GPIO_FUNC151_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func151_in_sel_cfg](func151_in_sel_cfg) module"]
pub type FUNC151_IN_SEL_CFG = crate::Reg<u32, _FUNC151_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC151_IN_SEL_CFG;
#[doc = "`read()` method returns [func151_in_sel_cfg::R](func151_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC151_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func151_in_sel_cfg::W](func151_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC151_IN_SEL_CFG {}
#[doc = "GPIO_FUNC151_IN_SEL_CFG"]
pub mod func151_in_sel_cfg;
#[doc = "GPIO_FUNC152_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func152_in_sel_cfg](func152_in_sel_cfg) module"]
pub type FUNC152_IN_SEL_CFG = crate::Reg<u32, _FUNC152_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC152_IN_SEL_CFG;
#[doc = "`read()` method returns [func152_in_sel_cfg::R](func152_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC152_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func152_in_sel_cfg::W](func152_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC152_IN_SEL_CFG {}
#[doc = "GPIO_FUNC152_IN_SEL_CFG"]
pub mod func152_in_sel_cfg;
#[doc = "GPIO_FUNC153_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func153_in_sel_cfg](func153_in_sel_cfg) module"]
pub type FUNC153_IN_SEL_CFG = crate::Reg<u32, _FUNC153_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC153_IN_SEL_CFG;
#[doc = "`read()` method returns [func153_in_sel_cfg::R](func153_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC153_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func153_in_sel_cfg::W](func153_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC153_IN_SEL_CFG {}
#[doc = "GPIO_FUNC153_IN_SEL_CFG"]
pub mod func153_in_sel_cfg;
#[doc = "GPIO_FUNC154_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func154_in_sel_cfg](func154_in_sel_cfg) module"]
pub type FUNC154_IN_SEL_CFG = crate::Reg<u32, _FUNC154_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC154_IN_SEL_CFG;
#[doc = "`read()` method returns [func154_in_sel_cfg::R](func154_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC154_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func154_in_sel_cfg::W](func154_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC154_IN_SEL_CFG {}
#[doc = "GPIO_FUNC154_IN_SEL_CFG"]
pub mod func154_in_sel_cfg;
#[doc = "GPIO_FUNC155_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func155_in_sel_cfg](func155_in_sel_cfg) module"]
pub type FUNC155_IN_SEL_CFG = crate::Reg<u32, _FUNC155_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC155_IN_SEL_CFG;
#[doc = "`read()` method returns [func155_in_sel_cfg::R](func155_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC155_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func155_in_sel_cfg::W](func155_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC155_IN_SEL_CFG {}
#[doc = "GPIO_FUNC155_IN_SEL_CFG"]
pub mod func155_in_sel_cfg;
#[doc = "GPIO_FUNC156_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func156_in_sel_cfg](func156_in_sel_cfg) module"]
pub type FUNC156_IN_SEL_CFG = crate::Reg<u32, _FUNC156_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC156_IN_SEL_CFG;
#[doc = "`read()` method returns [func156_in_sel_cfg::R](func156_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC156_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func156_in_sel_cfg::W](func156_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC156_IN_SEL_CFG {}
#[doc = "GPIO_FUNC156_IN_SEL_CFG"]
pub mod func156_in_sel_cfg;
#[doc = "GPIO_FUNC157_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func157_in_sel_cfg](func157_in_sel_cfg) module"]
pub type FUNC157_IN_SEL_CFG = crate::Reg<u32, _FUNC157_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC157_IN_SEL_CFG;
#[doc = "`read()` method returns [func157_in_sel_cfg::R](func157_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC157_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func157_in_sel_cfg::W](func157_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC157_IN_SEL_CFG {}
#[doc = "GPIO_FUNC157_IN_SEL_CFG"]
pub mod func157_in_sel_cfg;
#[doc = "GPIO_FUNC158_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func158_in_sel_cfg](func158_in_sel_cfg) module"]
pub type FUNC158_IN_SEL_CFG = crate::Reg<u32, _FUNC158_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC158_IN_SEL_CFG;
#[doc = "`read()` method returns [func158_in_sel_cfg::R](func158_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC158_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func158_in_sel_cfg::W](func158_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC158_IN_SEL_CFG {}
#[doc = "GPIO_FUNC158_IN_SEL_CFG"]
pub mod func158_in_sel_cfg;
#[doc = "GPIO_FUNC159_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func159_in_sel_cfg](func159_in_sel_cfg) module"]
pub type FUNC159_IN_SEL_CFG = crate::Reg<u32, _FUNC159_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC159_IN_SEL_CFG;
#[doc = "`read()` method returns [func159_in_sel_cfg::R](func159_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC159_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func159_in_sel_cfg::W](func159_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC159_IN_SEL_CFG {}
#[doc = "GPIO_FUNC159_IN_SEL_CFG"]
pub mod func159_in_sel_cfg;
#[doc = "GPIO_FUNC160_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func160_in_sel_cfg](func160_in_sel_cfg) module"]
pub type FUNC160_IN_SEL_CFG = crate::Reg<u32, _FUNC160_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC160_IN_SEL_CFG;
#[doc = "`read()` method returns [func160_in_sel_cfg::R](func160_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC160_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func160_in_sel_cfg::W](func160_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC160_IN_SEL_CFG {}
#[doc = "GPIO_FUNC160_IN_SEL_CFG"]
pub mod func160_in_sel_cfg;
#[doc = "GPIO_FUNC161_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func161_in_sel_cfg](func161_in_sel_cfg) module"]
pub type FUNC161_IN_SEL_CFG = crate::Reg<u32, _FUNC161_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC161_IN_SEL_CFG;
#[doc = "`read()` method returns [func161_in_sel_cfg::R](func161_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC161_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func161_in_sel_cfg::W](func161_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC161_IN_SEL_CFG {}
#[doc = "GPIO_FUNC161_IN_SEL_CFG"]
pub mod func161_in_sel_cfg;
#[doc = "GPIO_FUNC162_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func162_in_sel_cfg](func162_in_sel_cfg) module"]
pub type FUNC162_IN_SEL_CFG = crate::Reg<u32, _FUNC162_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC162_IN_SEL_CFG;
#[doc = "`read()` method returns [func162_in_sel_cfg::R](func162_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC162_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func162_in_sel_cfg::W](func162_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC162_IN_SEL_CFG {}
#[doc = "GPIO_FUNC162_IN_SEL_CFG"]
pub mod func162_in_sel_cfg;
#[doc = "GPIO_FUNC163_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func163_in_sel_cfg](func163_in_sel_cfg) module"]
pub type FUNC163_IN_SEL_CFG = crate::Reg<u32, _FUNC163_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC163_IN_SEL_CFG;
#[doc = "`read()` method returns [func163_in_sel_cfg::R](func163_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC163_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func163_in_sel_cfg::W](func163_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC163_IN_SEL_CFG {}
#[doc = "GPIO_FUNC163_IN_SEL_CFG"]
pub mod func163_in_sel_cfg;
#[doc = "GPIO_FUNC164_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func164_in_sel_cfg](func164_in_sel_cfg) module"]
pub type FUNC164_IN_SEL_CFG = crate::Reg<u32, _FUNC164_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC164_IN_SEL_CFG;
#[doc = "`read()` method returns [func164_in_sel_cfg::R](func164_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC164_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func164_in_sel_cfg::W](func164_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC164_IN_SEL_CFG {}
#[doc = "GPIO_FUNC164_IN_SEL_CFG"]
pub mod func164_in_sel_cfg;
#[doc = "GPIO_FUNC165_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func165_in_sel_cfg](func165_in_sel_cfg) module"]
pub type FUNC165_IN_SEL_CFG = crate::Reg<u32, _FUNC165_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC165_IN_SEL_CFG;
#[doc = "`read()` method returns [func165_in_sel_cfg::R](func165_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC165_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func165_in_sel_cfg::W](func165_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC165_IN_SEL_CFG {}
#[doc = "GPIO_FUNC165_IN_SEL_CFG"]
pub mod func165_in_sel_cfg;
#[doc = "GPIO_FUNC166_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func166_in_sel_cfg](func166_in_sel_cfg) module"]
pub type FUNC166_IN_SEL_CFG = crate::Reg<u32, _FUNC166_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC166_IN_SEL_CFG;
#[doc = "`read()` method returns [func166_in_sel_cfg::R](func166_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC166_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func166_in_sel_cfg::W](func166_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC166_IN_SEL_CFG {}
#[doc = "GPIO_FUNC166_IN_SEL_CFG"]
pub mod func166_in_sel_cfg;
#[doc = "GPIO_FUNC167_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func167_in_sel_cfg](func167_in_sel_cfg) module"]
pub type FUNC167_IN_SEL_CFG = crate::Reg<u32, _FUNC167_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC167_IN_SEL_CFG;
#[doc = "`read()` method returns [func167_in_sel_cfg::R](func167_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC167_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func167_in_sel_cfg::W](func167_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC167_IN_SEL_CFG {}
#[doc = "GPIO_FUNC167_IN_SEL_CFG"]
pub mod func167_in_sel_cfg;
#[doc = "GPIO_FUNC168_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func168_in_sel_cfg](func168_in_sel_cfg) module"]
pub type FUNC168_IN_SEL_CFG = crate::Reg<u32, _FUNC168_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC168_IN_SEL_CFG;
#[doc = "`read()` method returns [func168_in_sel_cfg::R](func168_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC168_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func168_in_sel_cfg::W](func168_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC168_IN_SEL_CFG {}
#[doc = "GPIO_FUNC168_IN_SEL_CFG"]
pub mod func168_in_sel_cfg;
#[doc = "GPIO_FUNC169_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func169_in_sel_cfg](func169_in_sel_cfg) module"]
pub type FUNC169_IN_SEL_CFG = crate::Reg<u32, _FUNC169_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC169_IN_SEL_CFG;
#[doc = "`read()` method returns [func169_in_sel_cfg::R](func169_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC169_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func169_in_sel_cfg::W](func169_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC169_IN_SEL_CFG {}
#[doc = "GPIO_FUNC169_IN_SEL_CFG"]
pub mod func169_in_sel_cfg;
#[doc = "GPIO_FUNC170_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func170_in_sel_cfg](func170_in_sel_cfg) module"]
pub type FUNC170_IN_SEL_CFG = crate::Reg<u32, _FUNC170_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC170_IN_SEL_CFG;
#[doc = "`read()` method returns [func170_in_sel_cfg::R](func170_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC170_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func170_in_sel_cfg::W](func170_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC170_IN_SEL_CFG {}
#[doc = "GPIO_FUNC170_IN_SEL_CFG"]
pub mod func170_in_sel_cfg;
#[doc = "GPIO_FUNC171_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func171_in_sel_cfg](func171_in_sel_cfg) module"]
pub type FUNC171_IN_SEL_CFG = crate::Reg<u32, _FUNC171_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC171_IN_SEL_CFG;
#[doc = "`read()` method returns [func171_in_sel_cfg::R](func171_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC171_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func171_in_sel_cfg::W](func171_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC171_IN_SEL_CFG {}
#[doc = "GPIO_FUNC171_IN_SEL_CFG"]
pub mod func171_in_sel_cfg;
#[doc = "GPIO_FUNC172_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func172_in_sel_cfg](func172_in_sel_cfg) module"]
pub type FUNC172_IN_SEL_CFG = crate::Reg<u32, _FUNC172_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC172_IN_SEL_CFG;
#[doc = "`read()` method returns [func172_in_sel_cfg::R](func172_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC172_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func172_in_sel_cfg::W](func172_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC172_IN_SEL_CFG {}
#[doc = "GPIO_FUNC172_IN_SEL_CFG"]
pub mod func172_in_sel_cfg;
#[doc = "GPIO_FUNC173_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func173_in_sel_cfg](func173_in_sel_cfg) module"]
pub type FUNC173_IN_SEL_CFG = crate::Reg<u32, _FUNC173_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC173_IN_SEL_CFG;
#[doc = "`read()` method returns [func173_in_sel_cfg::R](func173_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC173_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func173_in_sel_cfg::W](func173_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC173_IN_SEL_CFG {}
#[doc = "GPIO_FUNC173_IN_SEL_CFG"]
pub mod func173_in_sel_cfg;
#[doc = "GPIO_FUNC174_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func174_in_sel_cfg](func174_in_sel_cfg) module"]
pub type FUNC174_IN_SEL_CFG = crate::Reg<u32, _FUNC174_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC174_IN_SEL_CFG;
#[doc = "`read()` method returns [func174_in_sel_cfg::R](func174_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC174_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func174_in_sel_cfg::W](func174_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC174_IN_SEL_CFG {}
#[doc = "GPIO_FUNC174_IN_SEL_CFG"]
pub mod func174_in_sel_cfg;
#[doc = "GPIO_FUNC175_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func175_in_sel_cfg](func175_in_sel_cfg) module"]
pub type FUNC175_IN_SEL_CFG = crate::Reg<u32, _FUNC175_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC175_IN_SEL_CFG;
#[doc = "`read()` method returns [func175_in_sel_cfg::R](func175_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC175_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func175_in_sel_cfg::W](func175_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC175_IN_SEL_CFG {}
#[doc = "GPIO_FUNC175_IN_SEL_CFG"]
pub mod func175_in_sel_cfg;
#[doc = "GPIO_FUNC176_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func176_in_sel_cfg](func176_in_sel_cfg) module"]
pub type FUNC176_IN_SEL_CFG = crate::Reg<u32, _FUNC176_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC176_IN_SEL_CFG;
#[doc = "`read()` method returns [func176_in_sel_cfg::R](func176_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC176_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func176_in_sel_cfg::W](func176_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC176_IN_SEL_CFG {}
#[doc = "GPIO_FUNC176_IN_SEL_CFG"]
pub mod func176_in_sel_cfg;
#[doc = "GPIO_FUNC177_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func177_in_sel_cfg](func177_in_sel_cfg) module"]
pub type FUNC177_IN_SEL_CFG = crate::Reg<u32, _FUNC177_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC177_IN_SEL_CFG;
#[doc = "`read()` method returns [func177_in_sel_cfg::R](func177_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC177_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func177_in_sel_cfg::W](func177_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC177_IN_SEL_CFG {}
#[doc = "GPIO_FUNC177_IN_SEL_CFG"]
pub mod func177_in_sel_cfg;
#[doc = "GPIO_FUNC178_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func178_in_sel_cfg](func178_in_sel_cfg) module"]
pub type FUNC178_IN_SEL_CFG = crate::Reg<u32, _FUNC178_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC178_IN_SEL_CFG;
#[doc = "`read()` method returns [func178_in_sel_cfg::R](func178_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC178_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func178_in_sel_cfg::W](func178_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC178_IN_SEL_CFG {}
#[doc = "GPIO_FUNC178_IN_SEL_CFG"]
pub mod func178_in_sel_cfg;
#[doc = "GPIO_FUNC179_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func179_in_sel_cfg](func179_in_sel_cfg) module"]
pub type FUNC179_IN_SEL_CFG = crate::Reg<u32, _FUNC179_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC179_IN_SEL_CFG;
#[doc = "`read()` method returns [func179_in_sel_cfg::R](func179_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC179_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func179_in_sel_cfg::W](func179_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC179_IN_SEL_CFG {}
#[doc = "GPIO_FUNC179_IN_SEL_CFG"]
pub mod func179_in_sel_cfg;
#[doc = "GPIO_FUNC180_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func180_in_sel_cfg](func180_in_sel_cfg) module"]
pub type FUNC180_IN_SEL_CFG = crate::Reg<u32, _FUNC180_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC180_IN_SEL_CFG;
#[doc = "`read()` method returns [func180_in_sel_cfg::R](func180_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC180_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func180_in_sel_cfg::W](func180_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC180_IN_SEL_CFG {}
#[doc = "GPIO_FUNC180_IN_SEL_CFG"]
pub mod func180_in_sel_cfg;
#[doc = "GPIO_FUNC181_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func181_in_sel_cfg](func181_in_sel_cfg) module"]
pub type FUNC181_IN_SEL_CFG = crate::Reg<u32, _FUNC181_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC181_IN_SEL_CFG;
#[doc = "`read()` method returns [func181_in_sel_cfg::R](func181_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC181_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func181_in_sel_cfg::W](func181_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC181_IN_SEL_CFG {}
#[doc = "GPIO_FUNC181_IN_SEL_CFG"]
pub mod func181_in_sel_cfg;
#[doc = "GPIO_FUNC182_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func182_in_sel_cfg](func182_in_sel_cfg) module"]
pub type FUNC182_IN_SEL_CFG = crate::Reg<u32, _FUNC182_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC182_IN_SEL_CFG;
#[doc = "`read()` method returns [func182_in_sel_cfg::R](func182_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC182_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func182_in_sel_cfg::W](func182_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC182_IN_SEL_CFG {}
#[doc = "GPIO_FUNC182_IN_SEL_CFG"]
pub mod func182_in_sel_cfg;
#[doc = "GPIO_FUNC183_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func183_in_sel_cfg](func183_in_sel_cfg) module"]
pub type FUNC183_IN_SEL_CFG = crate::Reg<u32, _FUNC183_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC183_IN_SEL_CFG;
#[doc = "`read()` method returns [func183_in_sel_cfg::R](func183_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC183_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func183_in_sel_cfg::W](func183_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC183_IN_SEL_CFG {}
#[doc = "GPIO_FUNC183_IN_SEL_CFG"]
pub mod func183_in_sel_cfg;
#[doc = "GPIO_FUNC184_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func184_in_sel_cfg](func184_in_sel_cfg) module"]
pub type FUNC184_IN_SEL_CFG = crate::Reg<u32, _FUNC184_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC184_IN_SEL_CFG;
#[doc = "`read()` method returns [func184_in_sel_cfg::R](func184_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC184_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func184_in_sel_cfg::W](func184_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC184_IN_SEL_CFG {}
#[doc = "GPIO_FUNC184_IN_SEL_CFG"]
pub mod func184_in_sel_cfg;
#[doc = "GPIO_FUNC185_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func185_in_sel_cfg](func185_in_sel_cfg) module"]
pub type FUNC185_IN_SEL_CFG = crate::Reg<u32, _FUNC185_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC185_IN_SEL_CFG;
#[doc = "`read()` method returns [func185_in_sel_cfg::R](func185_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC185_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func185_in_sel_cfg::W](func185_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC185_IN_SEL_CFG {}
#[doc = "GPIO_FUNC185_IN_SEL_CFG"]
pub mod func185_in_sel_cfg;
#[doc = "GPIO_FUNC186_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func186_in_sel_cfg](func186_in_sel_cfg) module"]
pub type FUNC186_IN_SEL_CFG = crate::Reg<u32, _FUNC186_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC186_IN_SEL_CFG;
#[doc = "`read()` method returns [func186_in_sel_cfg::R](func186_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC186_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func186_in_sel_cfg::W](func186_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC186_IN_SEL_CFG {}
#[doc = "GPIO_FUNC186_IN_SEL_CFG"]
pub mod func186_in_sel_cfg;
#[doc = "GPIO_FUNC187_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func187_in_sel_cfg](func187_in_sel_cfg) module"]
pub type FUNC187_IN_SEL_CFG = crate::Reg<u32, _FUNC187_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC187_IN_SEL_CFG;
#[doc = "`read()` method returns [func187_in_sel_cfg::R](func187_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC187_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func187_in_sel_cfg::W](func187_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC187_IN_SEL_CFG {}
#[doc = "GPIO_FUNC187_IN_SEL_CFG"]
pub mod func187_in_sel_cfg;
#[doc = "GPIO_FUNC188_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func188_in_sel_cfg](func188_in_sel_cfg) module"]
pub type FUNC188_IN_SEL_CFG = crate::Reg<u32, _FUNC188_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC188_IN_SEL_CFG;
#[doc = "`read()` method returns [func188_in_sel_cfg::R](func188_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC188_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func188_in_sel_cfg::W](func188_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC188_IN_SEL_CFG {}
#[doc = "GPIO_FUNC188_IN_SEL_CFG"]
pub mod func188_in_sel_cfg;
#[doc = "GPIO_FUNC189_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func189_in_sel_cfg](func189_in_sel_cfg) module"]
pub type FUNC189_IN_SEL_CFG = crate::Reg<u32, _FUNC189_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC189_IN_SEL_CFG;
#[doc = "`read()` method returns [func189_in_sel_cfg::R](func189_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC189_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func189_in_sel_cfg::W](func189_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC189_IN_SEL_CFG {}
#[doc = "GPIO_FUNC189_IN_SEL_CFG"]
pub mod func189_in_sel_cfg;
#[doc = "GPIO_FUNC190_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func190_in_sel_cfg](func190_in_sel_cfg) module"]
pub type FUNC190_IN_SEL_CFG = crate::Reg<u32, _FUNC190_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC190_IN_SEL_CFG;
#[doc = "`read()` method returns [func190_in_sel_cfg::R](func190_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC190_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func190_in_sel_cfg::W](func190_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC190_IN_SEL_CFG {}
#[doc = "GPIO_FUNC190_IN_SEL_CFG"]
pub mod func190_in_sel_cfg;
#[doc = "GPIO_FUNC191_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func191_in_sel_cfg](func191_in_sel_cfg) module"]
pub type FUNC191_IN_SEL_CFG = crate::Reg<u32, _FUNC191_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC191_IN_SEL_CFG;
#[doc = "`read()` method returns [func191_in_sel_cfg::R](func191_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC191_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func191_in_sel_cfg::W](func191_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC191_IN_SEL_CFG {}
#[doc = "GPIO_FUNC191_IN_SEL_CFG"]
pub mod func191_in_sel_cfg;
#[doc = "GPIO_FUNC192_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func192_in_sel_cfg](func192_in_sel_cfg) module"]
pub type FUNC192_IN_SEL_CFG = crate::Reg<u32, _FUNC192_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC192_IN_SEL_CFG;
#[doc = "`read()` method returns [func192_in_sel_cfg::R](func192_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC192_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func192_in_sel_cfg::W](func192_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC192_IN_SEL_CFG {}
#[doc = "GPIO_FUNC192_IN_SEL_CFG"]
pub mod func192_in_sel_cfg;
#[doc = "GPIO_FUNC193_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func193_in_sel_cfg](func193_in_sel_cfg) module"]
pub type FUNC193_IN_SEL_CFG = crate::Reg<u32, _FUNC193_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC193_IN_SEL_CFG;
#[doc = "`read()` method returns [func193_in_sel_cfg::R](func193_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC193_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func193_in_sel_cfg::W](func193_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC193_IN_SEL_CFG {}
#[doc = "GPIO_FUNC193_IN_SEL_CFG"]
pub mod func193_in_sel_cfg;
#[doc = "GPIO_FUNC194_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func194_in_sel_cfg](func194_in_sel_cfg) module"]
pub type FUNC194_IN_SEL_CFG = crate::Reg<u32, _FUNC194_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC194_IN_SEL_CFG;
#[doc = "`read()` method returns [func194_in_sel_cfg::R](func194_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC194_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func194_in_sel_cfg::W](func194_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC194_IN_SEL_CFG {}
#[doc = "GPIO_FUNC194_IN_SEL_CFG"]
pub mod func194_in_sel_cfg;
#[doc = "GPIO_FUNC195_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func195_in_sel_cfg](func195_in_sel_cfg) module"]
pub type FUNC195_IN_SEL_CFG = crate::Reg<u32, _FUNC195_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC195_IN_SEL_CFG;
#[doc = "`read()` method returns [func195_in_sel_cfg::R](func195_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC195_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func195_in_sel_cfg::W](func195_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC195_IN_SEL_CFG {}
#[doc = "GPIO_FUNC195_IN_SEL_CFG"]
pub mod func195_in_sel_cfg;
#[doc = "GPIO_FUNC196_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func196_in_sel_cfg](func196_in_sel_cfg) module"]
pub type FUNC196_IN_SEL_CFG = crate::Reg<u32, _FUNC196_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC196_IN_SEL_CFG;
#[doc = "`read()` method returns [func196_in_sel_cfg::R](func196_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC196_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func196_in_sel_cfg::W](func196_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC196_IN_SEL_CFG {}
#[doc = "GPIO_FUNC196_IN_SEL_CFG"]
pub mod func196_in_sel_cfg;
#[doc = "GPIO_FUNC197_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func197_in_sel_cfg](func197_in_sel_cfg) module"]
pub type FUNC197_IN_SEL_CFG = crate::Reg<u32, _FUNC197_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC197_IN_SEL_CFG;
#[doc = "`read()` method returns [func197_in_sel_cfg::R](func197_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC197_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func197_in_sel_cfg::W](func197_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC197_IN_SEL_CFG {}
#[doc = "GPIO_FUNC197_IN_SEL_CFG"]
pub mod func197_in_sel_cfg;
#[doc = "GPIO_FUNC198_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func198_in_sel_cfg](func198_in_sel_cfg) module"]
pub type FUNC198_IN_SEL_CFG = crate::Reg<u32, _FUNC198_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC198_IN_SEL_CFG;
#[doc = "`read()` method returns [func198_in_sel_cfg::R](func198_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC198_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func198_in_sel_cfg::W](func198_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC198_IN_SEL_CFG {}
#[doc = "GPIO_FUNC198_IN_SEL_CFG"]
pub mod func198_in_sel_cfg;
#[doc = "GPIO_FUNC199_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func199_in_sel_cfg](func199_in_sel_cfg) module"]
pub type FUNC199_IN_SEL_CFG = crate::Reg<u32, _FUNC199_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC199_IN_SEL_CFG;
#[doc = "`read()` method returns [func199_in_sel_cfg::R](func199_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC199_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func199_in_sel_cfg::W](func199_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC199_IN_SEL_CFG {}
#[doc = "GPIO_FUNC199_IN_SEL_CFG"]
pub mod func199_in_sel_cfg;
#[doc = "GPIO_FUNC200_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func200_in_sel_cfg](func200_in_sel_cfg) module"]
pub type FUNC200_IN_SEL_CFG = crate::Reg<u32, _FUNC200_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC200_IN_SEL_CFG;
#[doc = "`read()` method returns [func200_in_sel_cfg::R](func200_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC200_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func200_in_sel_cfg::W](func200_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC200_IN_SEL_CFG {}
#[doc = "GPIO_FUNC200_IN_SEL_CFG"]
pub mod func200_in_sel_cfg;
#[doc = "GPIO_FUNC201_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func201_in_sel_cfg](func201_in_sel_cfg) module"]
pub type FUNC201_IN_SEL_CFG = crate::Reg<u32, _FUNC201_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC201_IN_SEL_CFG;
#[doc = "`read()` method returns [func201_in_sel_cfg::R](func201_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC201_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func201_in_sel_cfg::W](func201_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC201_IN_SEL_CFG {}
#[doc = "GPIO_FUNC201_IN_SEL_CFG"]
pub mod func201_in_sel_cfg;
#[doc = "GPIO_FUNC202_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func202_in_sel_cfg](func202_in_sel_cfg) module"]
pub type FUNC202_IN_SEL_CFG = crate::Reg<u32, _FUNC202_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC202_IN_SEL_CFG;
#[doc = "`read()` method returns [func202_in_sel_cfg::R](func202_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC202_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func202_in_sel_cfg::W](func202_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC202_IN_SEL_CFG {}
#[doc = "GPIO_FUNC202_IN_SEL_CFG"]
pub mod func202_in_sel_cfg;
#[doc = "GPIO_FUNC203_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func203_in_sel_cfg](func203_in_sel_cfg) module"]
pub type FUNC203_IN_SEL_CFG = crate::Reg<u32, _FUNC203_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC203_IN_SEL_CFG;
#[doc = "`read()` method returns [func203_in_sel_cfg::R](func203_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC203_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func203_in_sel_cfg::W](func203_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC203_IN_SEL_CFG {}
#[doc = "GPIO_FUNC203_IN_SEL_CFG"]
pub mod func203_in_sel_cfg;
#[doc = "GPIO_FUNC204_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func204_in_sel_cfg](func204_in_sel_cfg) module"]
pub type FUNC204_IN_SEL_CFG = crate::Reg<u32, _FUNC204_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC204_IN_SEL_CFG;
#[doc = "`read()` method returns [func204_in_sel_cfg::R](func204_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC204_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func204_in_sel_cfg::W](func204_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC204_IN_SEL_CFG {}
#[doc = "GPIO_FUNC204_IN_SEL_CFG"]
pub mod func204_in_sel_cfg;
#[doc = "GPIO_FUNC205_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func205_in_sel_cfg](func205_in_sel_cfg) module"]
pub type FUNC205_IN_SEL_CFG = crate::Reg<u32, _FUNC205_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC205_IN_SEL_CFG;
#[doc = "`read()` method returns [func205_in_sel_cfg::R](func205_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC205_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func205_in_sel_cfg::W](func205_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC205_IN_SEL_CFG {}
#[doc = "GPIO_FUNC205_IN_SEL_CFG"]
pub mod func205_in_sel_cfg;
#[doc = "GPIO_FUNC206_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func206_in_sel_cfg](func206_in_sel_cfg) module"]
pub type FUNC206_IN_SEL_CFG = crate::Reg<u32, _FUNC206_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC206_IN_SEL_CFG;
#[doc = "`read()` method returns [func206_in_sel_cfg::R](func206_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC206_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func206_in_sel_cfg::W](func206_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC206_IN_SEL_CFG {}
#[doc = "GPIO_FUNC206_IN_SEL_CFG"]
pub mod func206_in_sel_cfg;
#[doc = "GPIO_FUNC207_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func207_in_sel_cfg](func207_in_sel_cfg) module"]
pub type FUNC207_IN_SEL_CFG = crate::Reg<u32, _FUNC207_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC207_IN_SEL_CFG;
#[doc = "`read()` method returns [func207_in_sel_cfg::R](func207_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC207_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func207_in_sel_cfg::W](func207_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC207_IN_SEL_CFG {}
#[doc = "GPIO_FUNC207_IN_SEL_CFG"]
pub mod func207_in_sel_cfg;
#[doc = "GPIO_FUNC208_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func208_in_sel_cfg](func208_in_sel_cfg) module"]
pub type FUNC208_IN_SEL_CFG = crate::Reg<u32, _FUNC208_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC208_IN_SEL_CFG;
#[doc = "`read()` method returns [func208_in_sel_cfg::R](func208_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC208_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func208_in_sel_cfg::W](func208_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC208_IN_SEL_CFG {}
#[doc = "GPIO_FUNC208_IN_SEL_CFG"]
pub mod func208_in_sel_cfg;
#[doc = "GPIO_FUNC209_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func209_in_sel_cfg](func209_in_sel_cfg) module"]
pub type FUNC209_IN_SEL_CFG = crate::Reg<u32, _FUNC209_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC209_IN_SEL_CFG;
#[doc = "`read()` method returns [func209_in_sel_cfg::R](func209_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC209_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func209_in_sel_cfg::W](func209_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC209_IN_SEL_CFG {}
#[doc = "GPIO_FUNC209_IN_SEL_CFG"]
pub mod func209_in_sel_cfg;
#[doc = "GPIO_FUNC210_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func210_in_sel_cfg](func210_in_sel_cfg) module"]
pub type FUNC210_IN_SEL_CFG = crate::Reg<u32, _FUNC210_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC210_IN_SEL_CFG;
#[doc = "`read()` method returns [func210_in_sel_cfg::R](func210_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC210_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func210_in_sel_cfg::W](func210_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC210_IN_SEL_CFG {}
#[doc = "GPIO_FUNC210_IN_SEL_CFG"]
pub mod func210_in_sel_cfg;
#[doc = "GPIO_FUNC211_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func211_in_sel_cfg](func211_in_sel_cfg) module"]
pub type FUNC211_IN_SEL_CFG = crate::Reg<u32, _FUNC211_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC211_IN_SEL_CFG;
#[doc = "`read()` method returns [func211_in_sel_cfg::R](func211_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC211_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func211_in_sel_cfg::W](func211_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC211_IN_SEL_CFG {}
#[doc = "GPIO_FUNC211_IN_SEL_CFG"]
pub mod func211_in_sel_cfg;
#[doc = "GPIO_FUNC212_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func212_in_sel_cfg](func212_in_sel_cfg) module"]
pub type FUNC212_IN_SEL_CFG = crate::Reg<u32, _FUNC212_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC212_IN_SEL_CFG;
#[doc = "`read()` method returns [func212_in_sel_cfg::R](func212_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC212_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func212_in_sel_cfg::W](func212_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC212_IN_SEL_CFG {}
#[doc = "GPIO_FUNC212_IN_SEL_CFG"]
pub mod func212_in_sel_cfg;
#[doc = "GPIO_FUNC213_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func213_in_sel_cfg](func213_in_sel_cfg) module"]
pub type FUNC213_IN_SEL_CFG = crate::Reg<u32, _FUNC213_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC213_IN_SEL_CFG;
#[doc = "`read()` method returns [func213_in_sel_cfg::R](func213_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC213_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func213_in_sel_cfg::W](func213_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC213_IN_SEL_CFG {}
#[doc = "GPIO_FUNC213_IN_SEL_CFG"]
pub mod func213_in_sel_cfg;
#[doc = "GPIO_FUNC214_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func214_in_sel_cfg](func214_in_sel_cfg) module"]
pub type FUNC214_IN_SEL_CFG = crate::Reg<u32, _FUNC214_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC214_IN_SEL_CFG;
#[doc = "`read()` method returns [func214_in_sel_cfg::R](func214_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC214_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func214_in_sel_cfg::W](func214_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC214_IN_SEL_CFG {}
#[doc = "GPIO_FUNC214_IN_SEL_CFG"]
pub mod func214_in_sel_cfg;
#[doc = "GPIO_FUNC215_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func215_in_sel_cfg](func215_in_sel_cfg) module"]
pub type FUNC215_IN_SEL_CFG = crate::Reg<u32, _FUNC215_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC215_IN_SEL_CFG;
#[doc = "`read()` method returns [func215_in_sel_cfg::R](func215_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC215_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func215_in_sel_cfg::W](func215_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC215_IN_SEL_CFG {}
#[doc = "GPIO_FUNC215_IN_SEL_CFG"]
pub mod func215_in_sel_cfg;
#[doc = "GPIO_FUNC216_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func216_in_sel_cfg](func216_in_sel_cfg) module"]
pub type FUNC216_IN_SEL_CFG = crate::Reg<u32, _FUNC216_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC216_IN_SEL_CFG;
#[doc = "`read()` method returns [func216_in_sel_cfg::R](func216_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC216_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func216_in_sel_cfg::W](func216_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC216_IN_SEL_CFG {}
#[doc = "GPIO_FUNC216_IN_SEL_CFG"]
pub mod func216_in_sel_cfg;
#[doc = "GPIO_FUNC217_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func217_in_sel_cfg](func217_in_sel_cfg) module"]
pub type FUNC217_IN_SEL_CFG = crate::Reg<u32, _FUNC217_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC217_IN_SEL_CFG;
#[doc = "`read()` method returns [func217_in_sel_cfg::R](func217_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC217_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func217_in_sel_cfg::W](func217_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC217_IN_SEL_CFG {}
#[doc = "GPIO_FUNC217_IN_SEL_CFG"]
pub mod func217_in_sel_cfg;
#[doc = "GPIO_FUNC218_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func218_in_sel_cfg](func218_in_sel_cfg) module"]
pub type FUNC218_IN_SEL_CFG = crate::Reg<u32, _FUNC218_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC218_IN_SEL_CFG;
#[doc = "`read()` method returns [func218_in_sel_cfg::R](func218_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC218_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func218_in_sel_cfg::W](func218_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC218_IN_SEL_CFG {}
#[doc = "GPIO_FUNC218_IN_SEL_CFG"]
pub mod func218_in_sel_cfg;
#[doc = "GPIO_FUNC219_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func219_in_sel_cfg](func219_in_sel_cfg) module"]
pub type FUNC219_IN_SEL_CFG = crate::Reg<u32, _FUNC219_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC219_IN_SEL_CFG;
#[doc = "`read()` method returns [func219_in_sel_cfg::R](func219_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC219_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func219_in_sel_cfg::W](func219_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC219_IN_SEL_CFG {}
#[doc = "GPIO_FUNC219_IN_SEL_CFG"]
pub mod func219_in_sel_cfg;
#[doc = "GPIO_FUNC220_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func220_in_sel_cfg](func220_in_sel_cfg) module"]
pub type FUNC220_IN_SEL_CFG = crate::Reg<u32, _FUNC220_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC220_IN_SEL_CFG;
#[doc = "`read()` method returns [func220_in_sel_cfg::R](func220_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC220_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func220_in_sel_cfg::W](func220_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC220_IN_SEL_CFG {}
#[doc = "GPIO_FUNC220_IN_SEL_CFG"]
pub mod func220_in_sel_cfg;
#[doc = "GPIO_FUNC221_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func221_in_sel_cfg](func221_in_sel_cfg) module"]
pub type FUNC221_IN_SEL_CFG = crate::Reg<u32, _FUNC221_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC221_IN_SEL_CFG;
#[doc = "`read()` method returns [func221_in_sel_cfg::R](func221_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC221_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func221_in_sel_cfg::W](func221_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC221_IN_SEL_CFG {}
#[doc = "GPIO_FUNC221_IN_SEL_CFG"]
pub mod func221_in_sel_cfg;
#[doc = "GPIO_FUNC222_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func222_in_sel_cfg](func222_in_sel_cfg) module"]
pub type FUNC222_IN_SEL_CFG = crate::Reg<u32, _FUNC222_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC222_IN_SEL_CFG;
#[doc = "`read()` method returns [func222_in_sel_cfg::R](func222_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC222_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func222_in_sel_cfg::W](func222_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC222_IN_SEL_CFG {}
#[doc = "GPIO_FUNC222_IN_SEL_CFG"]
pub mod func222_in_sel_cfg;
#[doc = "GPIO_FUNC223_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func223_in_sel_cfg](func223_in_sel_cfg) module"]
pub type FUNC223_IN_SEL_CFG = crate::Reg<u32, _FUNC223_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC223_IN_SEL_CFG;
#[doc = "`read()` method returns [func223_in_sel_cfg::R](func223_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC223_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func223_in_sel_cfg::W](func223_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC223_IN_SEL_CFG {}
#[doc = "GPIO_FUNC223_IN_SEL_CFG"]
pub mod func223_in_sel_cfg;
#[doc = "GPIO_FUNC224_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func224_in_sel_cfg](func224_in_sel_cfg) module"]
pub type FUNC224_IN_SEL_CFG = crate::Reg<u32, _FUNC224_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC224_IN_SEL_CFG;
#[doc = "`read()` method returns [func224_in_sel_cfg::R](func224_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC224_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func224_in_sel_cfg::W](func224_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC224_IN_SEL_CFG {}
#[doc = "GPIO_FUNC224_IN_SEL_CFG"]
pub mod func224_in_sel_cfg;
#[doc = "GPIO_FUNC225_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func225_in_sel_cfg](func225_in_sel_cfg) module"]
pub type FUNC225_IN_SEL_CFG = crate::Reg<u32, _FUNC225_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC225_IN_SEL_CFG;
#[doc = "`read()` method returns [func225_in_sel_cfg::R](func225_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC225_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func225_in_sel_cfg::W](func225_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC225_IN_SEL_CFG {}
#[doc = "GPIO_FUNC225_IN_SEL_CFG"]
pub mod func225_in_sel_cfg;
#[doc = "GPIO_FUNC226_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func226_in_sel_cfg](func226_in_sel_cfg) module"]
pub type FUNC226_IN_SEL_CFG = crate::Reg<u32, _FUNC226_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC226_IN_SEL_CFG;
#[doc = "`read()` method returns [func226_in_sel_cfg::R](func226_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC226_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func226_in_sel_cfg::W](func226_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC226_IN_SEL_CFG {}
#[doc = "GPIO_FUNC226_IN_SEL_CFG"]
pub mod func226_in_sel_cfg;
#[doc = "GPIO_FUNC227_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func227_in_sel_cfg](func227_in_sel_cfg) module"]
pub type FUNC227_IN_SEL_CFG = crate::Reg<u32, _FUNC227_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC227_IN_SEL_CFG;
#[doc = "`read()` method returns [func227_in_sel_cfg::R](func227_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC227_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func227_in_sel_cfg::W](func227_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC227_IN_SEL_CFG {}
#[doc = "GPIO_FUNC227_IN_SEL_CFG"]
pub mod func227_in_sel_cfg;
#[doc = "GPIO_FUNC228_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func228_in_sel_cfg](func228_in_sel_cfg) module"]
pub type FUNC228_IN_SEL_CFG = crate::Reg<u32, _FUNC228_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC228_IN_SEL_CFG;
#[doc = "`read()` method returns [func228_in_sel_cfg::R](func228_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC228_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func228_in_sel_cfg::W](func228_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC228_IN_SEL_CFG {}
#[doc = "GPIO_FUNC228_IN_SEL_CFG"]
pub mod func228_in_sel_cfg;
#[doc = "GPIO_FUNC229_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func229_in_sel_cfg](func229_in_sel_cfg) module"]
pub type FUNC229_IN_SEL_CFG = crate::Reg<u32, _FUNC229_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC229_IN_SEL_CFG;
#[doc = "`read()` method returns [func229_in_sel_cfg::R](func229_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC229_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func229_in_sel_cfg::W](func229_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC229_IN_SEL_CFG {}
#[doc = "GPIO_FUNC229_IN_SEL_CFG"]
pub mod func229_in_sel_cfg;
#[doc = "GPIO_FUNC230_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func230_in_sel_cfg](func230_in_sel_cfg) module"]
pub type FUNC230_IN_SEL_CFG = crate::Reg<u32, _FUNC230_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC230_IN_SEL_CFG;
#[doc = "`read()` method returns [func230_in_sel_cfg::R](func230_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC230_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func230_in_sel_cfg::W](func230_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC230_IN_SEL_CFG {}
#[doc = "GPIO_FUNC230_IN_SEL_CFG"]
pub mod func230_in_sel_cfg;
#[doc = "GPIO_FUNC231_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func231_in_sel_cfg](func231_in_sel_cfg) module"]
pub type FUNC231_IN_SEL_CFG = crate::Reg<u32, _FUNC231_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC231_IN_SEL_CFG;
#[doc = "`read()` method returns [func231_in_sel_cfg::R](func231_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC231_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func231_in_sel_cfg::W](func231_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC231_IN_SEL_CFG {}
#[doc = "GPIO_FUNC231_IN_SEL_CFG"]
pub mod func231_in_sel_cfg;
#[doc = "GPIO_FUNC232_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func232_in_sel_cfg](func232_in_sel_cfg) module"]
pub type FUNC232_IN_SEL_CFG = crate::Reg<u32, _FUNC232_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC232_IN_SEL_CFG;
#[doc = "`read()` method returns [func232_in_sel_cfg::R](func232_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC232_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func232_in_sel_cfg::W](func232_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC232_IN_SEL_CFG {}
#[doc = "GPIO_FUNC232_IN_SEL_CFG"]
pub mod func232_in_sel_cfg;
#[doc = "GPIO_FUNC233_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func233_in_sel_cfg](func233_in_sel_cfg) module"]
pub type FUNC233_IN_SEL_CFG = crate::Reg<u32, _FUNC233_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC233_IN_SEL_CFG;
#[doc = "`read()` method returns [func233_in_sel_cfg::R](func233_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC233_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func233_in_sel_cfg::W](func233_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC233_IN_SEL_CFG {}
#[doc = "GPIO_FUNC233_IN_SEL_CFG"]
pub mod func233_in_sel_cfg;
#[doc = "GPIO_FUNC234_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func234_in_sel_cfg](func234_in_sel_cfg) module"]
pub type FUNC234_IN_SEL_CFG = crate::Reg<u32, _FUNC234_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC234_IN_SEL_CFG;
#[doc = "`read()` method returns [func234_in_sel_cfg::R](func234_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC234_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func234_in_sel_cfg::W](func234_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC234_IN_SEL_CFG {}
#[doc = "GPIO_FUNC234_IN_SEL_CFG"]
pub mod func234_in_sel_cfg;
#[doc = "GPIO_FUNC235_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func235_in_sel_cfg](func235_in_sel_cfg) module"]
pub type FUNC235_IN_SEL_CFG = crate::Reg<u32, _FUNC235_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC235_IN_SEL_CFG;
#[doc = "`read()` method returns [func235_in_sel_cfg::R](func235_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC235_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func235_in_sel_cfg::W](func235_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC235_IN_SEL_CFG {}
#[doc = "GPIO_FUNC235_IN_SEL_CFG"]
pub mod func235_in_sel_cfg;
#[doc = "GPIO_FUNC236_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func236_in_sel_cfg](func236_in_sel_cfg) module"]
pub type FUNC236_IN_SEL_CFG = crate::Reg<u32, _FUNC236_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC236_IN_SEL_CFG;
#[doc = "`read()` method returns [func236_in_sel_cfg::R](func236_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC236_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func236_in_sel_cfg::W](func236_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC236_IN_SEL_CFG {}
#[doc = "GPIO_FUNC236_IN_SEL_CFG"]
pub mod func236_in_sel_cfg;
#[doc = "GPIO_FUNC237_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func237_in_sel_cfg](func237_in_sel_cfg) module"]
pub type FUNC237_IN_SEL_CFG = crate::Reg<u32, _FUNC237_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC237_IN_SEL_CFG;
#[doc = "`read()` method returns [func237_in_sel_cfg::R](func237_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC237_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func237_in_sel_cfg::W](func237_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC237_IN_SEL_CFG {}
#[doc = "GPIO_FUNC237_IN_SEL_CFG"]
pub mod func237_in_sel_cfg;
#[doc = "GPIO_FUNC238_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func238_in_sel_cfg](func238_in_sel_cfg) module"]
pub type FUNC238_IN_SEL_CFG = crate::Reg<u32, _FUNC238_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC238_IN_SEL_CFG;
#[doc = "`read()` method returns [func238_in_sel_cfg::R](func238_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC238_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func238_in_sel_cfg::W](func238_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC238_IN_SEL_CFG {}
#[doc = "GPIO_FUNC238_IN_SEL_CFG"]
pub mod func238_in_sel_cfg;
#[doc = "GPIO_FUNC239_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func239_in_sel_cfg](func239_in_sel_cfg) module"]
pub type FUNC239_IN_SEL_CFG = crate::Reg<u32, _FUNC239_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC239_IN_SEL_CFG;
#[doc = "`read()` method returns [func239_in_sel_cfg::R](func239_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC239_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func239_in_sel_cfg::W](func239_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC239_IN_SEL_CFG {}
#[doc = "GPIO_FUNC239_IN_SEL_CFG"]
pub mod func239_in_sel_cfg;
#[doc = "GPIO_FUNC240_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func240_in_sel_cfg](func240_in_sel_cfg) module"]
pub type FUNC240_IN_SEL_CFG = crate::Reg<u32, _FUNC240_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC240_IN_SEL_CFG;
#[doc = "`read()` method returns [func240_in_sel_cfg::R](func240_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC240_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func240_in_sel_cfg::W](func240_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC240_IN_SEL_CFG {}
#[doc = "GPIO_FUNC240_IN_SEL_CFG"]
pub mod func240_in_sel_cfg;
#[doc = "GPIO_FUNC241_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func241_in_sel_cfg](func241_in_sel_cfg) module"]
pub type FUNC241_IN_SEL_CFG = crate::Reg<u32, _FUNC241_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC241_IN_SEL_CFG;
#[doc = "`read()` method returns [func241_in_sel_cfg::R](func241_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC241_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func241_in_sel_cfg::W](func241_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC241_IN_SEL_CFG {}
#[doc = "GPIO_FUNC241_IN_SEL_CFG"]
pub mod func241_in_sel_cfg;
#[doc = "GPIO_FUNC242_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func242_in_sel_cfg](func242_in_sel_cfg) module"]
pub type FUNC242_IN_SEL_CFG = crate::Reg<u32, _FUNC242_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC242_IN_SEL_CFG;
#[doc = "`read()` method returns [func242_in_sel_cfg::R](func242_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC242_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func242_in_sel_cfg::W](func242_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC242_IN_SEL_CFG {}
#[doc = "GPIO_FUNC242_IN_SEL_CFG"]
pub mod func242_in_sel_cfg;
#[doc = "GPIO_FUNC243_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func243_in_sel_cfg](func243_in_sel_cfg) module"]
pub type FUNC243_IN_SEL_CFG = crate::Reg<u32, _FUNC243_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC243_IN_SEL_CFG;
#[doc = "`read()` method returns [func243_in_sel_cfg::R](func243_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC243_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func243_in_sel_cfg::W](func243_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC243_IN_SEL_CFG {}
#[doc = "GPIO_FUNC243_IN_SEL_CFG"]
pub mod func243_in_sel_cfg;
#[doc = "GPIO_FUNC244_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func244_in_sel_cfg](func244_in_sel_cfg) module"]
pub type FUNC244_IN_SEL_CFG = crate::Reg<u32, _FUNC244_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC244_IN_SEL_CFG;
#[doc = "`read()` method returns [func244_in_sel_cfg::R](func244_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC244_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func244_in_sel_cfg::W](func244_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC244_IN_SEL_CFG {}
#[doc = "GPIO_FUNC244_IN_SEL_CFG"]
pub mod func244_in_sel_cfg;
#[doc = "GPIO_FUNC245_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func245_in_sel_cfg](func245_in_sel_cfg) module"]
pub type FUNC245_IN_SEL_CFG = crate::Reg<u32, _FUNC245_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC245_IN_SEL_CFG;
#[doc = "`read()` method returns [func245_in_sel_cfg::R](func245_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC245_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func245_in_sel_cfg::W](func245_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC245_IN_SEL_CFG {}
#[doc = "GPIO_FUNC245_IN_SEL_CFG"]
pub mod func245_in_sel_cfg;
#[doc = "GPIO_FUNC246_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func246_in_sel_cfg](func246_in_sel_cfg) module"]
pub type FUNC246_IN_SEL_CFG = crate::Reg<u32, _FUNC246_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC246_IN_SEL_CFG;
#[doc = "`read()` method returns [func246_in_sel_cfg::R](func246_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC246_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func246_in_sel_cfg::W](func246_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC246_IN_SEL_CFG {}
#[doc = "GPIO_FUNC246_IN_SEL_CFG"]
pub mod func246_in_sel_cfg;
#[doc = "GPIO_FUNC247_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func247_in_sel_cfg](func247_in_sel_cfg) module"]
pub type FUNC247_IN_SEL_CFG = crate::Reg<u32, _FUNC247_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC247_IN_SEL_CFG;
#[doc = "`read()` method returns [func247_in_sel_cfg::R](func247_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC247_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func247_in_sel_cfg::W](func247_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC247_IN_SEL_CFG {}
#[doc = "GPIO_FUNC247_IN_SEL_CFG"]
pub mod func247_in_sel_cfg;
#[doc = "GPIO_FUNC248_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func248_in_sel_cfg](func248_in_sel_cfg) module"]
pub type FUNC248_IN_SEL_CFG = crate::Reg<u32, _FUNC248_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC248_IN_SEL_CFG;
#[doc = "`read()` method returns [func248_in_sel_cfg::R](func248_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC248_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func248_in_sel_cfg::W](func248_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC248_IN_SEL_CFG {}
#[doc = "GPIO_FUNC248_IN_SEL_CFG"]
pub mod func248_in_sel_cfg;
#[doc = "GPIO_FUNC249_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func249_in_sel_cfg](func249_in_sel_cfg) module"]
pub type FUNC249_IN_SEL_CFG = crate::Reg<u32, _FUNC249_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC249_IN_SEL_CFG;
#[doc = "`read()` method returns [func249_in_sel_cfg::R](func249_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC249_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func249_in_sel_cfg::W](func249_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC249_IN_SEL_CFG {}
#[doc = "GPIO_FUNC249_IN_SEL_CFG"]
pub mod func249_in_sel_cfg;
#[doc = "GPIO_FUNC250_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func250_in_sel_cfg](func250_in_sel_cfg) module"]
pub type FUNC250_IN_SEL_CFG = crate::Reg<u32, _FUNC250_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC250_IN_SEL_CFG;
#[doc = "`read()` method returns [func250_in_sel_cfg::R](func250_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC250_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func250_in_sel_cfg::W](func250_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC250_IN_SEL_CFG {}
#[doc = "GPIO_FUNC250_IN_SEL_CFG"]
pub mod func250_in_sel_cfg;
#[doc = "GPIO_FUNC251_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func251_in_sel_cfg](func251_in_sel_cfg) module"]
pub type FUNC251_IN_SEL_CFG = crate::Reg<u32, _FUNC251_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC251_IN_SEL_CFG;
#[doc = "`read()` method returns [func251_in_sel_cfg::R](func251_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC251_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func251_in_sel_cfg::W](func251_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC251_IN_SEL_CFG {}
#[doc = "GPIO_FUNC251_IN_SEL_CFG"]
pub mod func251_in_sel_cfg;
#[doc = "GPIO_FUNC252_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func252_in_sel_cfg](func252_in_sel_cfg) module"]
pub type FUNC252_IN_SEL_CFG = crate::Reg<u32, _FUNC252_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC252_IN_SEL_CFG;
#[doc = "`read()` method returns [func252_in_sel_cfg::R](func252_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC252_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func252_in_sel_cfg::W](func252_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC252_IN_SEL_CFG {}
#[doc = "GPIO_FUNC252_IN_SEL_CFG"]
pub mod func252_in_sel_cfg;
#[doc = "GPIO_FUNC253_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func253_in_sel_cfg](func253_in_sel_cfg) module"]
pub type FUNC253_IN_SEL_CFG = crate::Reg<u32, _FUNC253_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC253_IN_SEL_CFG;
#[doc = "`read()` method returns [func253_in_sel_cfg::R](func253_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC253_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func253_in_sel_cfg::W](func253_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC253_IN_SEL_CFG {}
#[doc = "GPIO_FUNC253_IN_SEL_CFG"]
pub mod func253_in_sel_cfg;
#[doc = "GPIO_FUNC254_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func254_in_sel_cfg](func254_in_sel_cfg) module"]
pub type FUNC254_IN_SEL_CFG = crate::Reg<u32, _FUNC254_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC254_IN_SEL_CFG;
#[doc = "`read()` method returns [func254_in_sel_cfg::R](func254_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC254_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func254_in_sel_cfg::W](func254_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC254_IN_SEL_CFG {}
#[doc = "GPIO_FUNC254_IN_SEL_CFG"]
pub mod func254_in_sel_cfg;
#[doc = "GPIO_FUNC255_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func255_in_sel_cfg](func255_in_sel_cfg) module"]
pub type FUNC255_IN_SEL_CFG = crate::Reg<u32, _FUNC255_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC255_IN_SEL_CFG;
#[doc = "`read()` method returns [func255_in_sel_cfg::R](func255_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC255_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func255_in_sel_cfg::W](func255_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC255_IN_SEL_CFG {}
#[doc = "GPIO_FUNC255_IN_SEL_CFG"]
pub mod func255_in_sel_cfg;
#[doc = "GPIO_FUNC0_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func0_out_sel_cfg](func0_out_sel_cfg) module"]
pub type FUNC0_OUT_SEL_CFG = crate::Reg<u32, _FUNC0_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC0_OUT_SEL_CFG;
#[doc = "`read()` method returns [func0_out_sel_cfg::R](func0_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC0_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func0_out_sel_cfg::W](func0_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC0_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC0_OUT_SEL_CFG"]
pub mod func0_out_sel_cfg;
#[doc = "GPIO_FUNC1_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func1_out_sel_cfg](func1_out_sel_cfg) module"]
pub type FUNC1_OUT_SEL_CFG = crate::Reg<u32, _FUNC1_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC1_OUT_SEL_CFG;
#[doc = "`read()` method returns [func1_out_sel_cfg::R](func1_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC1_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func1_out_sel_cfg::W](func1_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC1_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC1_OUT_SEL_CFG"]
pub mod func1_out_sel_cfg;
#[doc = "GPIO_FUNC2_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func2_out_sel_cfg](func2_out_sel_cfg) module"]
pub type FUNC2_OUT_SEL_CFG = crate::Reg<u32, _FUNC2_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC2_OUT_SEL_CFG;
#[doc = "`read()` method returns [func2_out_sel_cfg::R](func2_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC2_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func2_out_sel_cfg::W](func2_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC2_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC2_OUT_SEL_CFG"]
pub mod func2_out_sel_cfg;
#[doc = "GPIO_FUNC3_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func3_out_sel_cfg](func3_out_sel_cfg) module"]
pub type FUNC3_OUT_SEL_CFG = crate::Reg<u32, _FUNC3_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC3_OUT_SEL_CFG;
#[doc = "`read()` method returns [func3_out_sel_cfg::R](func3_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC3_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func3_out_sel_cfg::W](func3_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC3_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC3_OUT_SEL_CFG"]
pub mod func3_out_sel_cfg;
#[doc = "GPIO_FUNC4_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func4_out_sel_cfg](func4_out_sel_cfg) module"]
pub type FUNC4_OUT_SEL_CFG = crate::Reg<u32, _FUNC4_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC4_OUT_SEL_CFG;
#[doc = "`read()` method returns [func4_out_sel_cfg::R](func4_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC4_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func4_out_sel_cfg::W](func4_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC4_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC4_OUT_SEL_CFG"]
pub mod func4_out_sel_cfg;
#[doc = "GPIO_FUNC5_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func5_out_sel_cfg](func5_out_sel_cfg) module"]
pub type FUNC5_OUT_SEL_CFG = crate::Reg<u32, _FUNC5_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC5_OUT_SEL_CFG;
#[doc = "`read()` method returns [func5_out_sel_cfg::R](func5_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC5_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func5_out_sel_cfg::W](func5_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC5_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC5_OUT_SEL_CFG"]
pub mod func5_out_sel_cfg;
#[doc = "GPIO_FUNC6_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func6_out_sel_cfg](func6_out_sel_cfg) module"]
pub type FUNC6_OUT_SEL_CFG = crate::Reg<u32, _FUNC6_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC6_OUT_SEL_CFG;
#[doc = "`read()` method returns [func6_out_sel_cfg::R](func6_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC6_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func6_out_sel_cfg::W](func6_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC6_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC6_OUT_SEL_CFG"]
pub mod func6_out_sel_cfg;
#[doc = "GPIO_FUNC7_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func7_out_sel_cfg](func7_out_sel_cfg) module"]
pub type FUNC7_OUT_SEL_CFG = crate::Reg<u32, _FUNC7_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC7_OUT_SEL_CFG;
#[doc = "`read()` method returns [func7_out_sel_cfg::R](func7_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC7_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func7_out_sel_cfg::W](func7_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC7_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC7_OUT_SEL_CFG"]
pub mod func7_out_sel_cfg;
#[doc = "GPIO_FUNC8_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func8_out_sel_cfg](func8_out_sel_cfg) module"]
pub type FUNC8_OUT_SEL_CFG = crate::Reg<u32, _FUNC8_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC8_OUT_SEL_CFG;
#[doc = "`read()` method returns [func8_out_sel_cfg::R](func8_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC8_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func8_out_sel_cfg::W](func8_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC8_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC8_OUT_SEL_CFG"]
pub mod func8_out_sel_cfg;
#[doc = "GPIO_FUNC9_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func9_out_sel_cfg](func9_out_sel_cfg) module"]
pub type FUNC9_OUT_SEL_CFG = crate::Reg<u32, _FUNC9_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC9_OUT_SEL_CFG;
#[doc = "`read()` method returns [func9_out_sel_cfg::R](func9_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC9_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func9_out_sel_cfg::W](func9_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC9_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC9_OUT_SEL_CFG"]
pub mod func9_out_sel_cfg;
#[doc = "GPIO_FUNC10_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func10_out_sel_cfg](func10_out_sel_cfg) module"]
pub type FUNC10_OUT_SEL_CFG = crate::Reg<u32, _FUNC10_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC10_OUT_SEL_CFG;
#[doc = "`read()` method returns [func10_out_sel_cfg::R](func10_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC10_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func10_out_sel_cfg::W](func10_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC10_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC10_OUT_SEL_CFG"]
pub mod func10_out_sel_cfg;
#[doc = "GPIO_FUNC11_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func11_out_sel_cfg](func11_out_sel_cfg) module"]
pub type FUNC11_OUT_SEL_CFG = crate::Reg<u32, _FUNC11_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC11_OUT_SEL_CFG;
#[doc = "`read()` method returns [func11_out_sel_cfg::R](func11_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC11_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func11_out_sel_cfg::W](func11_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC11_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC11_OUT_SEL_CFG"]
pub mod func11_out_sel_cfg;
#[doc = "GPIO_FUNC12_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func12_out_sel_cfg](func12_out_sel_cfg) module"]
pub type FUNC12_OUT_SEL_CFG = crate::Reg<u32, _FUNC12_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC12_OUT_SEL_CFG;
#[doc = "`read()` method returns [func12_out_sel_cfg::R](func12_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC12_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func12_out_sel_cfg::W](func12_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC12_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC12_OUT_SEL_CFG"]
pub mod func12_out_sel_cfg;
#[doc = "GPIO_FUNC13_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func13_out_sel_cfg](func13_out_sel_cfg) module"]
pub type FUNC13_OUT_SEL_CFG = crate::Reg<u32, _FUNC13_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC13_OUT_SEL_CFG;
#[doc = "`read()` method returns [func13_out_sel_cfg::R](func13_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC13_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func13_out_sel_cfg::W](func13_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC13_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC13_OUT_SEL_CFG"]
pub mod func13_out_sel_cfg;
#[doc = "GPIO_FUNC14_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func14_out_sel_cfg](func14_out_sel_cfg) module"]
pub type FUNC14_OUT_SEL_CFG = crate::Reg<u32, _FUNC14_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC14_OUT_SEL_CFG;
#[doc = "`read()` method returns [func14_out_sel_cfg::R](func14_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC14_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func14_out_sel_cfg::W](func14_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC14_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC14_OUT_SEL_CFG"]
pub mod func14_out_sel_cfg;
#[doc = "GPIO_FUNC15_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func15_out_sel_cfg](func15_out_sel_cfg) module"]
pub type FUNC15_OUT_SEL_CFG = crate::Reg<u32, _FUNC15_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC15_OUT_SEL_CFG;
#[doc = "`read()` method returns [func15_out_sel_cfg::R](func15_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC15_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func15_out_sel_cfg::W](func15_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC15_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC15_OUT_SEL_CFG"]
pub mod func15_out_sel_cfg;
#[doc = "GPIO_FUNC16_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func16_out_sel_cfg](func16_out_sel_cfg) module"]
pub type FUNC16_OUT_SEL_CFG = crate::Reg<u32, _FUNC16_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC16_OUT_SEL_CFG;
#[doc = "`read()` method returns [func16_out_sel_cfg::R](func16_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC16_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func16_out_sel_cfg::W](func16_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC16_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC16_OUT_SEL_CFG"]
pub mod func16_out_sel_cfg;
#[doc = "GPIO_FUNC17_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func17_out_sel_cfg](func17_out_sel_cfg) module"]
pub type FUNC17_OUT_SEL_CFG = crate::Reg<u32, _FUNC17_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC17_OUT_SEL_CFG;
#[doc = "`read()` method returns [func17_out_sel_cfg::R](func17_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC17_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func17_out_sel_cfg::W](func17_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC17_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC17_OUT_SEL_CFG"]
pub mod func17_out_sel_cfg;
#[doc = "GPIO_FUNC18_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func18_out_sel_cfg](func18_out_sel_cfg) module"]
pub type FUNC18_OUT_SEL_CFG = crate::Reg<u32, _FUNC18_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC18_OUT_SEL_CFG;
#[doc = "`read()` method returns [func18_out_sel_cfg::R](func18_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC18_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func18_out_sel_cfg::W](func18_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC18_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC18_OUT_SEL_CFG"]
pub mod func18_out_sel_cfg;
#[doc = "GPIO_FUNC19_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func19_out_sel_cfg](func19_out_sel_cfg) module"]
pub type FUNC19_OUT_SEL_CFG = crate::Reg<u32, _FUNC19_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC19_OUT_SEL_CFG;
#[doc = "`read()` method returns [func19_out_sel_cfg::R](func19_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC19_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func19_out_sel_cfg::W](func19_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC19_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC19_OUT_SEL_CFG"]
pub mod func19_out_sel_cfg;
#[doc = "GPIO_FUNC20_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func20_out_sel_cfg](func20_out_sel_cfg) module"]
pub type FUNC20_OUT_SEL_CFG = crate::Reg<u32, _FUNC20_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC20_OUT_SEL_CFG;
#[doc = "`read()` method returns [func20_out_sel_cfg::R](func20_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC20_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func20_out_sel_cfg::W](func20_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC20_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC20_OUT_SEL_CFG"]
pub mod func20_out_sel_cfg;
#[doc = "GPIO_FUNC21_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func21_out_sel_cfg](func21_out_sel_cfg) module"]
pub type FUNC21_OUT_SEL_CFG = crate::Reg<u32, _FUNC21_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC21_OUT_SEL_CFG;
#[doc = "`read()` method returns [func21_out_sel_cfg::R](func21_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC21_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func21_out_sel_cfg::W](func21_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC21_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC21_OUT_SEL_CFG"]
pub mod func21_out_sel_cfg;
#[doc = "GPIO_FUNC22_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func22_out_sel_cfg](func22_out_sel_cfg) module"]
pub type FUNC22_OUT_SEL_CFG = crate::Reg<u32, _FUNC22_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC22_OUT_SEL_CFG;
#[doc = "`read()` method returns [func22_out_sel_cfg::R](func22_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC22_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func22_out_sel_cfg::W](func22_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC22_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC22_OUT_SEL_CFG"]
pub mod func22_out_sel_cfg;
#[doc = "GPIO_FUNC23_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func23_out_sel_cfg](func23_out_sel_cfg) module"]
pub type FUNC23_OUT_SEL_CFG = crate::Reg<u32, _FUNC23_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC23_OUT_SEL_CFG;
#[doc = "`read()` method returns [func23_out_sel_cfg::R](func23_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC23_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func23_out_sel_cfg::W](func23_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC23_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC23_OUT_SEL_CFG"]
pub mod func23_out_sel_cfg;
#[doc = "GPIO_FUNC24_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func24_out_sel_cfg](func24_out_sel_cfg) module"]
pub type FUNC24_OUT_SEL_CFG = crate::Reg<u32, _FUNC24_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC24_OUT_SEL_CFG;
#[doc = "`read()` method returns [func24_out_sel_cfg::R](func24_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC24_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func24_out_sel_cfg::W](func24_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC24_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC24_OUT_SEL_CFG"]
pub mod func24_out_sel_cfg;
#[doc = "GPIO_FUNC25_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func25_out_sel_cfg](func25_out_sel_cfg) module"]
pub type FUNC25_OUT_SEL_CFG = crate::Reg<u32, _FUNC25_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC25_OUT_SEL_CFG;
#[doc = "`read()` method returns [func25_out_sel_cfg::R](func25_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC25_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func25_out_sel_cfg::W](func25_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC25_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC25_OUT_SEL_CFG"]
pub mod func25_out_sel_cfg;
#[doc = "GPIO_FUNC26_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func26_out_sel_cfg](func26_out_sel_cfg) module"]
pub type FUNC26_OUT_SEL_CFG = crate::Reg<u32, _FUNC26_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC26_OUT_SEL_CFG;
#[doc = "`read()` method returns [func26_out_sel_cfg::R](func26_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC26_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func26_out_sel_cfg::W](func26_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC26_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC26_OUT_SEL_CFG"]
pub mod func26_out_sel_cfg;
#[doc = "GPIO_FUNC27_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func27_out_sel_cfg](func27_out_sel_cfg) module"]
pub type FUNC27_OUT_SEL_CFG = crate::Reg<u32, _FUNC27_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC27_OUT_SEL_CFG;
#[doc = "`read()` method returns [func27_out_sel_cfg::R](func27_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC27_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func27_out_sel_cfg::W](func27_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC27_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC27_OUT_SEL_CFG"]
pub mod func27_out_sel_cfg;
#[doc = "GPIO_FUNC28_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func28_out_sel_cfg](func28_out_sel_cfg) module"]
pub type FUNC28_OUT_SEL_CFG = crate::Reg<u32, _FUNC28_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC28_OUT_SEL_CFG;
#[doc = "`read()` method returns [func28_out_sel_cfg::R](func28_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC28_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func28_out_sel_cfg::W](func28_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC28_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC28_OUT_SEL_CFG"]
pub mod func28_out_sel_cfg;
#[doc = "GPIO_FUNC29_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func29_out_sel_cfg](func29_out_sel_cfg) module"]
pub type FUNC29_OUT_SEL_CFG = crate::Reg<u32, _FUNC29_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC29_OUT_SEL_CFG;
#[doc = "`read()` method returns [func29_out_sel_cfg::R](func29_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC29_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func29_out_sel_cfg::W](func29_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC29_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC29_OUT_SEL_CFG"]
pub mod func29_out_sel_cfg;
#[doc = "GPIO_FUNC30_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func30_out_sel_cfg](func30_out_sel_cfg) module"]
pub type FUNC30_OUT_SEL_CFG = crate::Reg<u32, _FUNC30_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC30_OUT_SEL_CFG;
#[doc = "`read()` method returns [func30_out_sel_cfg::R](func30_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC30_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func30_out_sel_cfg::W](func30_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC30_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC30_OUT_SEL_CFG"]
pub mod func30_out_sel_cfg;
#[doc = "GPIO_FUNC31_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func31_out_sel_cfg](func31_out_sel_cfg) module"]
pub type FUNC31_OUT_SEL_CFG = crate::Reg<u32, _FUNC31_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC31_OUT_SEL_CFG;
#[doc = "`read()` method returns [func31_out_sel_cfg::R](func31_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC31_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func31_out_sel_cfg::W](func31_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC31_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC31_OUT_SEL_CFG"]
pub mod func31_out_sel_cfg;
#[doc = "GPIO_FUNC32_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func32_out_sel_cfg](func32_out_sel_cfg) module"]
pub type FUNC32_OUT_SEL_CFG = crate::Reg<u32, _FUNC32_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC32_OUT_SEL_CFG;
#[doc = "`read()` method returns [func32_out_sel_cfg::R](func32_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC32_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func32_out_sel_cfg::W](func32_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC32_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC32_OUT_SEL_CFG"]
pub mod func32_out_sel_cfg;
#[doc = "GPIO_FUNC33_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func33_out_sel_cfg](func33_out_sel_cfg) module"]
pub type FUNC33_OUT_SEL_CFG = crate::Reg<u32, _FUNC33_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC33_OUT_SEL_CFG;
#[doc = "`read()` method returns [func33_out_sel_cfg::R](func33_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC33_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func33_out_sel_cfg::W](func33_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC33_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC33_OUT_SEL_CFG"]
pub mod func33_out_sel_cfg;
#[doc = "GPIO_FUNC34_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func34_out_sel_cfg](func34_out_sel_cfg) module"]
pub type FUNC34_OUT_SEL_CFG = crate::Reg<u32, _FUNC34_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC34_OUT_SEL_CFG;
#[doc = "`read()` method returns [func34_out_sel_cfg::R](func34_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC34_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func34_out_sel_cfg::W](func34_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC34_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC34_OUT_SEL_CFG"]
pub mod func34_out_sel_cfg;
#[doc = "GPIO_FUNC35_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func35_out_sel_cfg](func35_out_sel_cfg) module"]
pub type FUNC35_OUT_SEL_CFG = crate::Reg<u32, _FUNC35_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC35_OUT_SEL_CFG;
#[doc = "`read()` method returns [func35_out_sel_cfg::R](func35_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC35_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func35_out_sel_cfg::W](func35_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC35_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC35_OUT_SEL_CFG"]
pub mod func35_out_sel_cfg;
#[doc = "GPIO_FUNC36_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func36_out_sel_cfg](func36_out_sel_cfg) module"]
pub type FUNC36_OUT_SEL_CFG = crate::Reg<u32, _FUNC36_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC36_OUT_SEL_CFG;
#[doc = "`read()` method returns [func36_out_sel_cfg::R](func36_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC36_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func36_out_sel_cfg::W](func36_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC36_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC36_OUT_SEL_CFG"]
pub mod func36_out_sel_cfg;
#[doc = "GPIO_FUNC37_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func37_out_sel_cfg](func37_out_sel_cfg) module"]
pub type FUNC37_OUT_SEL_CFG = crate::Reg<u32, _FUNC37_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC37_OUT_SEL_CFG;
#[doc = "`read()` method returns [func37_out_sel_cfg::R](func37_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC37_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func37_out_sel_cfg::W](func37_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC37_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC37_OUT_SEL_CFG"]
pub mod func37_out_sel_cfg;
#[doc = "GPIO_FUNC38_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func38_out_sel_cfg](func38_out_sel_cfg) module"]
pub type FUNC38_OUT_SEL_CFG = crate::Reg<u32, _FUNC38_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC38_OUT_SEL_CFG;
#[doc = "`read()` method returns [func38_out_sel_cfg::R](func38_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC38_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func38_out_sel_cfg::W](func38_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC38_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC38_OUT_SEL_CFG"]
pub mod func38_out_sel_cfg;
#[doc = "GPIO_FUNC39_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [func39_out_sel_cfg](func39_out_sel_cfg) module"]
pub type FUNC39_OUT_SEL_CFG = crate::Reg<u32, _FUNC39_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC39_OUT_SEL_CFG;
#[doc = "`read()` method returns [func39_out_sel_cfg::R](func39_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC39_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func39_out_sel_cfg::W](func39_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC39_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC39_OUT_SEL_CFG"]
pub mod func39_out_sel_cfg;
