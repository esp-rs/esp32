#[doc = "Reader of register BLK0_RDATA4"]
pub type R = crate::R<u32, super::BLK0_RDATA4>;
#[doc = "Writer for register BLK0_RDATA4"]
pub type W = crate::W<u32, super::BLK0_RDATA4>;
#[doc = "Register BLK0_RDATA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_RDATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD_SDIO_FORCE`"]
pub type RD_SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_SDIO_FORCE`"]
pub struct RD_SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SDIO_FORCE_W<'a> {
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
#[doc = "Reader of field `RD_SDIO_TIEH`"]
pub type RD_SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_SDIO_TIEH`"]
pub struct RD_SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SDIO_TIEH_W<'a> {
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
#[doc = "Reader of field `RD_XPD_SDIO_REG`"]
pub type RD_XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_XPD_SDIO_REG`"]
pub struct RD_XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_XPD_SDIO_REG_W<'a> {
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
#[doc = "Reader of field `RD_ADC_VREF`"]
pub type RD_ADC_VREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_ADC_VREF`"]
pub struct RD_ADC_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_ADC_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RD_SDIO_DREFL`"]
pub type RD_SDIO_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_SDIO_DREFL`"]
pub struct RD_SDIO_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SDIO_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `RD_SDIO_DREFM`"]
pub type RD_SDIO_DREFM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_SDIO_DREFM`"]
pub struct RD_SDIO_DREFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SDIO_DREFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RD_SDIO_DREFH`"]
pub type RD_SDIO_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_SDIO_DREFH`"]
pub struct RD_SDIO_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SDIO_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RD_CK8M_FREQ`"]
pub type RD_CK8M_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_CK8M_FREQ`"]
pub struct RD_CK8M_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CK8M_FREQ_W<'a> {
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
    pub fn rd_sdio_force(&self) -> RD_SDIO_FORCE_R {
        RD_SDIO_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - read for SDIO_TIEH"]
    #[inline(always)]
    pub fn rd_sdio_tieh(&self) -> RD_SDIO_TIEH_R {
        RD_SDIO_TIEH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - read for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn rd_xpd_sdio_reg(&self) -> RD_XPD_SDIO_REG_R {
        RD_XPD_SDIO_REG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn rd_adc_vref(&self) -> RD_ADC_VREF_R {
        RD_ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn rd_sdio_drefl(&self) -> RD_SDIO_DREFL_R {
        RD_SDIO_DREFL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rd_sdio_drefm(&self) -> RD_SDIO_DREFM_R {
        RD_SDIO_DREFM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rd_sdio_drefh(&self) -> RD_SDIO_DREFH_R {
        RD_SDIO_DREFH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_ck8m_freq(&self) -> RD_CK8M_FREQ_R {
        RD_CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - read for sdio_force"]
    #[inline(always)]
    pub fn rd_sdio_force(&mut self) -> RD_SDIO_FORCE_W {
        RD_SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 15 - read for SDIO_TIEH"]
    #[inline(always)]
    pub fn rd_sdio_tieh(&mut self) -> RD_SDIO_TIEH_W {
        RD_SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 14 - read for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn rd_xpd_sdio_reg(&mut self) -> RD_XPD_SDIO_REG_W {
        RD_XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn rd_adc_vref(&mut self) -> RD_ADC_VREF_W {
        RD_ADC_VREF_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn rd_sdio_drefl(&mut self) -> RD_SDIO_DREFL_W {
        RD_SDIO_DREFL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rd_sdio_drefm(&mut self) -> RD_SDIO_DREFM_W {
        RD_SDIO_DREFM_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rd_sdio_drefh(&mut self) -> RD_SDIO_DREFH_W {
        RD_SDIO_DREFH_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_ck8m_freq(&mut self) -> RD_CK8M_FREQ_W {
        RD_CK8M_FREQ_W { w: self }
    }
}