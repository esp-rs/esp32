#[doc = "Reader of register SLP_REJECT_CONF"]
pub type R = crate::R<u32, super::SLP_REJECT_CONF>;
#[doc = "Writer for register SLP_REJECT_CONF"]
pub type W = crate::W<u32, super::SLP_REJECT_CONF>;
#[doc = "Register SLP_REJECT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SLP_REJECT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REJECT_CAUSE`"]
pub type REJECT_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REJECT_CAUSE`"]
pub struct REJECT_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> REJECT_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `DEEP_SLP_REJECT_EN`"]
pub type DEEP_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEEP_SLP_REJECT_EN`"]
pub struct DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLP_REJECT_EN_W<'a> {
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
#[doc = "Reader of field `LIGHT_SLP_REJECT_EN`"]
pub type LIGHT_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIGHT_SLP_REJECT_EN`"]
pub struct LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIGHT_SLP_REJECT_EN_W<'a> {
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
#[doc = "Reader of field `SDIO_REJECT_EN`"]
pub type SDIO_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_REJECT_EN`"]
pub struct SDIO_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_REJECT_EN_W<'a> {
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
#[doc = "Reader of field `GPIO_REJECT_EN`"]
pub type GPIO_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_REJECT_EN`"]
pub struct GPIO_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_REJECT_EN_W<'a> {
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
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn sdio_reject_en(&self) -> SDIO_REJECT_EN_R {
        SDIO_REJECT_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn gpio_reject_en(&self) -> GPIO_REJECT_EN_R {
        GPIO_REJECT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&mut self) -> REJECT_CAUSE_W {
        REJECT_CAUSE_W { w: self }
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W {
        DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W {
        LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn sdio_reject_en(&mut self) -> SDIO_REJECT_EN_W {
        SDIO_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn gpio_reject_en(&mut self) -> GPIO_REJECT_EN_W {
        GPIO_REJECT_EN_W { w: self }
    }
}
