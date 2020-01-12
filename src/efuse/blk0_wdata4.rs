#[doc = "Reader of register BLK0_WDATA4"]
pub type R = crate::R<u32, super::BLK0_WDATA4>;
#[doc = "Writer for register BLK0_WDATA4"]
pub type W = crate::W<u32, super::BLK0_WDATA4>;
#[doc = "Register BLK0_WDATA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO_FORCE`"]
pub type SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_FORCE`"]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
#[doc = "Reader of field `SDIO_TIEH`"]
pub type SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_TIEH`"]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
#[doc = "Reader of field `XPD_SDIO_REG`"]
pub type XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XPD_SDIO_REG`"]
pub struct XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_REG_W<'a> {
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
#[doc = "Reader of field `ADC_VREF`"]
pub type ADC_VREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_VREF`"]
pub struct ADC_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SDIO_DREFL`"]
pub type SDIO_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DREFL`"]
pub struct SDIO_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SDIO_DREFM`"]
pub type SDIO_DREFM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DREFM`"]
pub struct SDIO_DREFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SDIO_DREFH`"]
pub type SDIO_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DREFH`"]
pub struct SDIO_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CK8M_FREQ`"]
pub type CK8M_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CK8M_FREQ`"]
pub struct CK8M_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&self) -> XPD_SDIO_REG_R {
        XPD_SDIO_REG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn adc_vref(&self) -> ADC_VREF_R {
        ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sdio_drefl(&self) -> SDIO_DREFL_R {
        SDIO_DREFL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sdio_drefm(&self) -> SDIO_DREFM_R {
        SDIO_DREFM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdio_drefh(&self) -> SDIO_DREFH_R {
        SDIO_DREFH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_freq(&self) -> CK8M_FREQ_R {
        CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&mut self) -> XPD_SDIO_REG_W {
        XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn adc_vref(&mut self) -> ADC_VREF_W {
        ADC_VREF_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sdio_drefl(&mut self) -> SDIO_DREFL_W {
        SDIO_DREFL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sdio_drefm(&mut self) -> SDIO_DREFM_W {
        SDIO_DREFM_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdio_drefh(&mut self) -> SDIO_DREFH_W {
        SDIO_DREFH_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_freq(&mut self) -> CK8M_FREQ_W {
        CK8M_FREQ_W { w: self }
    }
}
