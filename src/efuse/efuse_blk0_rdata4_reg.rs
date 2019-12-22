#[doc = "Reader of register EFUSE_BLK0_RDATA4_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_RDATA4_REG>;
#[doc = "Writer for register EFUSE_BLK0_RDATA4_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_RDATA4_REG>;
#[doc = "Register EFUSE_BLK0_RDATA4_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_RDATA4_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_RD_SDIO_FORCE`"]
pub type EFUSE_RD_SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_SDIO_FORCE`"]
pub struct EFUSE_RD_SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SDIO_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_SDIO_TIEH`"]
pub type EFUSE_RD_SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_SDIO_TIEH`"]
pub struct EFUSE_RD_SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SDIO_TIEH_W<'a> {
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
#[doc = "Reader of field `EFUSE_RD_XPD_SDIO_REG`"]
pub type EFUSE_RD_XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_RD_XPD_SDIO_REG`"]
pub struct EFUSE_RD_XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_XPD_SDIO_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_ADC_VREF`"]
pub type EFUSE_RD_ADC_VREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_ADC_VREF`"]
pub struct EFUSE_RD_ADC_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_ADC_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_SDIO_DREFL`"]
pub type EFUSE_RD_SDIO_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_SDIO_DREFL`"]
pub struct EFUSE_RD_SDIO_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SDIO_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_SDIO_DREFM`"]
pub type EFUSE_RD_SDIO_DREFM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_SDIO_DREFM`"]
pub struct EFUSE_RD_SDIO_DREFM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SDIO_DREFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_SDIO_DREFH`"]
pub type EFUSE_RD_SDIO_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_SDIO_DREFH`"]
pub struct EFUSE_RD_SDIO_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_SDIO_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_CK8M_FREQ`"]
pub type EFUSE_RD_CK8M_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_CK8M_FREQ`"]
pub struct EFUSE_RD_CK8M_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_CK8M_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - read for sdio_force"]
    #[inline(always)]
    pub fn efuse_rd_sdio_force(&self) -> EFUSE_RD_SDIO_FORCE_R {
        EFUSE_RD_SDIO_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - read for SDIO_TIEH"]
    #[inline(always)]
    pub fn efuse_rd_sdio_tieh(&self) -> EFUSE_RD_SDIO_TIEH_R {
        EFUSE_RD_SDIO_TIEH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - read for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn efuse_rd_xpd_sdio_reg(&self) -> EFUSE_RD_XPD_SDIO_REG_R {
        EFUSE_RD_XPD_SDIO_REG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn efuse_rd_adc_vref(&self) -> EFUSE_RD_ADC_VREF_R {
        EFUSE_RD_ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefl(&self) -> EFUSE_RD_SDIO_DREFL_R {
        EFUSE_RD_SDIO_DREFL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefm(&self) -> EFUSE_RD_SDIO_DREFM_R {
        EFUSE_RD_SDIO_DREFM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefh(&self) -> EFUSE_RD_SDIO_DREFH_R {
        EFUSE_RD_SDIO_DREFH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn efuse_rd_ck8m_freq(&self) -> EFUSE_RD_CK8M_FREQ_R {
        EFUSE_RD_CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - read for sdio_force"]
    #[inline(always)]
    pub fn efuse_rd_sdio_force(&mut self) -> EFUSE_RD_SDIO_FORCE_W {
        EFUSE_RD_SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 15 - read for SDIO_TIEH"]
    #[inline(always)]
    pub fn efuse_rd_sdio_tieh(&mut self) -> EFUSE_RD_SDIO_TIEH_W {
        EFUSE_RD_SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 14 - read for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn efuse_rd_xpd_sdio_reg(&mut self) -> EFUSE_RD_XPD_SDIO_REG_W {
        EFUSE_RD_XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn efuse_rd_adc_vref(&mut self) -> EFUSE_RD_ADC_VREF_W {
        EFUSE_RD_ADC_VREF_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefl(&mut self) -> EFUSE_RD_SDIO_DREFL_W {
        EFUSE_RD_SDIO_DREFL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefm(&mut self) -> EFUSE_RD_SDIO_DREFM_W {
        EFUSE_RD_SDIO_DREFM_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn efuse_rd_sdio_drefh(&mut self) -> EFUSE_RD_SDIO_DREFH_W {
        EFUSE_RD_SDIO_DREFH_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn efuse_rd_ck8m_freq(&mut self) -> EFUSE_RD_CK8M_FREQ_W {
        EFUSE_RD_CK8M_FREQ_W { w: self }
    }
}
