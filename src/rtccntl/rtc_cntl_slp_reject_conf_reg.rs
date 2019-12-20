#[doc = "Reader of register RTC_CNTL_SLP_REJECT_CONF_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_SLP_REJECT_CONF_REG>;
#[doc = "Writer for register RTC_CNTL_SLP_REJECT_CONF_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_SLP_REJECT_CONF_REG>;
#[doc = "Register RTC_CNTL_SLP_REJECT_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SLP_REJECT_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_REJECT_CAUSE`"]
pub type RTC_CNTL_REJECT_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_REJECT_CAUSE`"]
pub struct RTC_CNTL_REJECT_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_REJECT_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEEP_SLP_REJECT_EN`"]
pub type RTC_CNTL_DEEP_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DEEP_SLP_REJECT_EN`"]
pub struct RTC_CNTL_DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEEP_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_LIGHT_SLP_REJECT_EN`"]
pub type RTC_CNTL_LIGHT_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LIGHT_SLP_REJECT_EN`"]
pub struct RTC_CNTL_LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LIGHT_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_REJECT_EN`"]
pub type RTC_CNTL_SDIO_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_REJECT_EN`"]
pub struct RTC_CNTL_SDIO_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_REJECT_EN`"]
pub type RTC_CNTL_GPIO_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_REJECT_EN`"]
pub struct RTC_CNTL_GPIO_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_REJECT_EN_W<'a> {
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
impl R {
    #[doc = "Bits 28:31 - sleep reject cause"]
    #[inline(always)]
    pub fn rtc_cntl_reject_cause(&self) -> RTC_CNTL_REJECT_CAUSE_R {
        RTC_CNTL_REJECT_CAUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_deep_slp_reject_en(&self) -> RTC_CNTL_DEEP_SLP_REJECT_EN_R {
        RTC_CNTL_DEEP_SLP_REJECT_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn rtc_cntl_light_slp_reject_en(&self) -> RTC_CNTL_LIGHT_SLP_REJECT_EN_R {
        RTC_CNTL_LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_reject_en(&self) -> RTC_CNTL_SDIO_REJECT_EN_R {
        RTC_CNTL_SDIO_REJECT_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_reject_en(&self) -> RTC_CNTL_GPIO_REJECT_EN_R {
        RTC_CNTL_GPIO_REJECT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - sleep reject cause"]
    #[inline(always)]
    pub fn rtc_cntl_reject_cause(&mut self) -> RTC_CNTL_REJECT_CAUSE_W {
        RTC_CNTL_REJECT_CAUSE_W { w: self }
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_deep_slp_reject_en(&mut self) -> RTC_CNTL_DEEP_SLP_REJECT_EN_W {
        RTC_CNTL_DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn rtc_cntl_light_slp_reject_en(&mut self) -> RTC_CNTL_LIGHT_SLP_REJECT_EN_W {
        RTC_CNTL_LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_reject_en(&mut self) -> RTC_CNTL_SDIO_REJECT_EN_W {
        RTC_CNTL_SDIO_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_reject_en(&mut self) -> RTC_CNTL_GPIO_REJECT_EN_W {
        RTC_CNTL_GPIO_REJECT_EN_W { w: self }
    }
}
