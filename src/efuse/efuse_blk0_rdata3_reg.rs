#[doc = "Reader of register EFUSE_BLK0_RDATA3_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_RDATA3_REG>;
#[doc = "Writer for register EFUSE_BLK0_RDATA3_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_RDATA3_REG>;
#[doc = "Register EFUSE_BLK0_RDATA3_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_RDATA3_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_REV1`"]
pub type EFUSE_RD_CHIP_VER_REV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_REV1`"]
pub struct EFUSE_RD_CHIP_VER_REV1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_REV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_CPU_FREQ_RATED`"]
pub type EFUSE_RD_CHIP_CPU_FREQ_RATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_CPU_FREQ_RATED`"]
pub struct EFUSE_RD_CHIP_CPU_FREQ_RATED_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_CPU_FREQ_RATED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_CPU_FREQ_LOW`"]
pub type EFUSE_RD_CHIP_CPU_FREQ_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_CPU_FREQ_LOW`"]
pub struct EFUSE_RD_CHIP_CPU_FREQ_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_CPU_FREQ_LOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_PKG`"]
pub type EFUSE_RD_CHIP_VER_PKG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_PKG`"]
pub struct EFUSE_RD_CHIP_VER_PKG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_PKG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_SPI_PAD_CONFIG_HD`"]
pub type EFUSE_RD_SPI_PAD_CONFIG_HD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_SPI_PAD_CONFIG_HD`"]
pub struct EFUSE_RD_SPI_PAD_CONFIG_HD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SPI_PAD_CONFIG_HD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_DIS_CACHE`"]
pub type EFUSE_RD_CHIP_VER_DIS_CACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_DIS_CACHE`"]
pub struct EFUSE_RD_CHIP_VER_DIS_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_DIS_CACHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_32PAD`"]
pub type EFUSE_RD_CHIP_VER_32PAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_32PAD`"]
pub struct EFUSE_RD_CHIP_VER_32PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_32PAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_DIS_BT`"]
pub type EFUSE_RD_CHIP_VER_DIS_BT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_DIS_BT`"]
pub struct EFUSE_RD_CHIP_VER_DIS_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_DIS_BT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CHIP_VER_DIS_APP_CPU`"]
pub type EFUSE_RD_CHIP_VER_DIS_APP_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_CHIP_VER_DIS_APP_CPU`"]
pub struct EFUSE_RD_CHIP_VER_DIS_APP_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CHIP_VER_DIS_APP_CPU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - bit is set to 1 for rev1 silicon"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_rev1(&self) -> EFUSE_RD_CHIP_VER_REV1_R {
        EFUSE_RD_CHIP_VER_REV1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set, the ESP32's maximum CPU frequency has been rated"]
    #[inline(always)]
    pub fn efuse_rd_chip_cpu_freq_rated(&self) -> EFUSE_RD_CHIP_CPU_FREQ_RATED_R {
        EFUSE_RD_CHIP_CPU_FREQ_RATED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If set alongside EFUSE_RD_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
    #[inline(always)]
    pub fn efuse_rd_chip_cpu_freq_low(&self) -> EFUSE_RD_CHIP_CPU_FREQ_LOW_R {
        EFUSE_RD_CHIP_CPU_FREQ_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - chip package"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_pkg(&self) -> EFUSE_RD_CHIP_VER_PKG_R {
        EFUSE_RD_CHIP_VER_PKG_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 4:8 - read for SPI_pad_config_hd"]
    #[inline(always)]
    pub fn efuse_rd_spi_pad_config_hd(&self) -> EFUSE_RD_SPI_PAD_CONFIG_HD_R {
        EFUSE_RD_SPI_PAD_CONFIG_HD_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_cache(&self) -> EFUSE_RD_CHIP_VER_DIS_CACHE_R {
        EFUSE_RD_CHIP_VER_DIS_CACHE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_32pad(&self) -> EFUSE_RD_CHIP_VER_32PAD_R {
        EFUSE_RD_CHIP_VER_32PAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_bt(&self) -> EFUSE_RD_CHIP_VER_DIS_BT_R {
        EFUSE_RD_CHIP_VER_DIS_BT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_app_cpu(&self) -> EFUSE_RD_CHIP_VER_DIS_APP_CPU_R {
        EFUSE_RD_CHIP_VER_DIS_APP_CPU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - bit is set to 1 for rev1 silicon"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_rev1(&mut self) -> EFUSE_RD_CHIP_VER_REV1_W {
        EFUSE_RD_CHIP_VER_REV1_W { w: self }
    }
    #[doc = "Bit 13 - If set, the ESP32's maximum CPU frequency has been rated"]
    #[inline(always)]
    pub fn efuse_rd_chip_cpu_freq_rated(&mut self) -> EFUSE_RD_CHIP_CPU_FREQ_RATED_W {
        EFUSE_RD_CHIP_CPU_FREQ_RATED_W { w: self }
    }
    #[doc = "Bit 12 - If set alongside EFUSE_RD_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
    #[inline(always)]
    pub fn efuse_rd_chip_cpu_freq_low(&mut self) -> EFUSE_RD_CHIP_CPU_FREQ_LOW_W {
        EFUSE_RD_CHIP_CPU_FREQ_LOW_W { w: self }
    }
    #[doc = "Bits 9:11 - chip package"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_pkg(&mut self) -> EFUSE_RD_CHIP_VER_PKG_W {
        EFUSE_RD_CHIP_VER_PKG_W { w: self }
    }
    #[doc = "Bits 4:8 - read for SPI_pad_config_hd"]
    #[inline(always)]
    pub fn efuse_rd_spi_pad_config_hd(&mut self) -> EFUSE_RD_SPI_PAD_CONFIG_HD_W {
        EFUSE_RD_SPI_PAD_CONFIG_HD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_cache(&mut self) -> EFUSE_RD_CHIP_VER_DIS_CACHE_W {
        EFUSE_RD_CHIP_VER_DIS_CACHE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_32pad(&mut self) -> EFUSE_RD_CHIP_VER_32PAD_W {
        EFUSE_RD_CHIP_VER_32PAD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_bt(&mut self) -> EFUSE_RD_CHIP_VER_DIS_BT_W {
        EFUSE_RD_CHIP_VER_DIS_BT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_rd_chip_ver_dis_app_cpu(&mut self) -> EFUSE_RD_CHIP_VER_DIS_APP_CPU_W {
        EFUSE_RD_CHIP_VER_DIS_APP_CPU_W { w: self }
    }
}
