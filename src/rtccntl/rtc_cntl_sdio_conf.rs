#[doc = "Reader of register RTC_CNTL_SDIO_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_SDIO_CONF>;
#[doc = "Writer for register RTC_CNTL_SDIO_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_SDIO_CONF>;
#[doc = "Register RTC_CNTL_SDIO_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SDIO_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_XPD_SDIO_REG`"]
pub type RTC_CNTL_XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XPD_SDIO_REG`"]
pub struct RTC_CNTL_XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XPD_SDIO_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFH_SDIO`"]
pub type RTC_CNTL_DREFH_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFH_SDIO`"]
pub struct RTC_CNTL_DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFM_SDIO`"]
pub type RTC_CNTL_DREFM_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFM_SDIO`"]
pub struct RTC_CNTL_DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFL_SDIO`"]
pub type RTC_CNTL_DREFL_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFL_SDIO`"]
pub struct RTC_CNTL_DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_REG1P8_READY`"]
pub type RTC_CNTL_REG1P8_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_REG1P8_READY`"]
pub struct RTC_CNTL_REG1P8_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_REG1P8_READY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_TIEH`"]
pub type RTC_CNTL_SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_TIEH`"]
pub struct RTC_CNTL_SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_TIEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_FORCE`"]
pub type RTC_CNTL_SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_FORCE`"]
pub struct RTC_CNTL_SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_PD_EN`"]
pub type RTC_CNTL_SDIO_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_PD_EN`"]
pub struct RTC_CNTL_SDIO_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_sdio_reg(&self) -> RTC_CNTL_XPD_SDIO_REG_R {
        RTC_CNTL_XPD_SDIO_REG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefh_sdio(&self) -> RTC_CNTL_DREFH_SDIO_R {
        RTC_CNTL_DREFH_SDIO_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefm_sdio(&self) -> RTC_CNTL_DREFM_SDIO_R {
        RTC_CNTL_DREFM_SDIO_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefl_sdio(&self) -> RTC_CNTL_DREFL_SDIO_R {
        RTC_CNTL_DREFL_SDIO_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - read only register for REG1P8_READY"]
    #[inline(always)]
    pub fn rtc_cntl_reg1p8_ready(&self) -> RTC_CNTL_REG1P8_READY_R {
        RTC_CNTL_REG1P8_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_tieh(&self) -> RTC_CNTL_SDIO_TIEH_R {
        RTC_CNTL_SDIO_TIEH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_force(&self) -> RTC_CNTL_SDIO_FORCE_R {
        RTC_CNTL_SDIO_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_pd_en(&self) -> RTC_CNTL_SDIO_PD_EN_R {
        RTC_CNTL_SDIO_PD_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_sdio_reg(&mut self) -> RTC_CNTL_XPD_SDIO_REG_W {
        RTC_CNTL_XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefh_sdio(&mut self) -> RTC_CNTL_DREFH_SDIO_W {
        RTC_CNTL_DREFH_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefm_sdio(&mut self) -> RTC_CNTL_DREFM_SDIO_W {
        RTC_CNTL_DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_drefl_sdio(&mut self) -> RTC_CNTL_DREFL_SDIO_W {
        RTC_CNTL_DREFL_SDIO_W { w: self }
    }
    #[doc = "Bit 24 - read only register for REG1P8_READY"]
    #[inline(always)]
    pub fn rtc_cntl_reg1p8_ready(&mut self) -> RTC_CNTL_REG1P8_READY_W {
        RTC_CNTL_REG1P8_READY_W { w: self }
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_tieh(&mut self) -> RTC_CNTL_SDIO_TIEH_W {
        RTC_CNTL_SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_force(&mut self) -> RTC_CNTL_SDIO_FORCE_W {
        RTC_CNTL_SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_pd_en(&mut self) -> RTC_CNTL_SDIO_PD_EN_W {
        RTC_CNTL_SDIO_PD_EN_W { w: self }
    }
}
