#[doc = "Reader of register GPIO_cali_conf_REG"]
pub type R = crate::R<u32, super::GPIO_CALI_CONF_REG>;
#[doc = "Writer for register GPIO_cali_conf_REG"]
pub type W = crate::W<u32, super::GPIO_CALI_CONF_REG>;
#[doc = "Register GPIO_cali_conf_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CALI_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_CALI_START`"]
pub type GPIO_CALI_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_CALI_START`"]
pub struct GPIO_CALI_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CALI_START_W<'a> {
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
#[doc = "Reader of field `GPIO_CALI_RTC_MAX`"]
pub type GPIO_CALI_RTC_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPIO_CALI_RTC_MAX`"]
pub struct GPIO_CALI_RTC_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CALI_RTC_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_cali_start(&self) -> GPIO_CALI_START_R {
        GPIO_CALI_START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpio_cali_rtc_max(&self) -> GPIO_CALI_RTC_MAX_R {
        GPIO_CALI_RTC_MAX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_cali_start(&mut self) -> GPIO_CALI_START_W {
        GPIO_CALI_START_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpio_cali_rtc_max(&mut self) -> GPIO_CALI_RTC_MAX_W {
        GPIO_CALI_RTC_MAX_W { w: self }
    }
}