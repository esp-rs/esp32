#[doc = "Reader of register WAKEUP_STATE"]
pub type R = crate::R<u32, super::WAKEUP_STATE>;
#[doc = "Writer for register WAKEUP_STATE"]
pub type W = crate::W<u32, super::WAKEUP_STATE>;
#[doc = "Register WAKEUP_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_WAKEUP_FILTER`"]
pub type RTC_CNTL_GPIO_WAKEUP_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_WAKEUP_FILTER`"]
pub struct RTC_CNTL_GPIO_WAKEUP_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_WAKEUP_FILTER_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WAKEUP_ENA`"]
pub type RTC_CNTL_WAKEUP_ENA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_WAKEUP_ENA`"]
pub struct RTC_CNTL_WAKEUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WAKEUP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WAKEUP_CAUSE`"]
pub type RTC_CNTL_WAKEUP_CAUSE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_WAKEUP_CAUSE`"]
pub struct RTC_CNTL_WAKEUP_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WAKEUP_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_wakeup_filter(&self) -> RTC_CNTL_GPIO_WAKEUP_FILTER_R {
        RTC_CNTL_GPIO_WAKEUP_FILTER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_ena(&self) -> RTC_CNTL_WAKEUP_ENA_R {
        RTC_CNTL_WAKEUP_ENA_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_cause(&self) -> RTC_CNTL_WAKEUP_CAUSE_R {
        RTC_CNTL_WAKEUP_CAUSE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_wakeup_filter(&mut self) -> RTC_CNTL_GPIO_WAKEUP_FILTER_W {
        RTC_CNTL_GPIO_WAKEUP_FILTER_W { w: self }
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_ena(&mut self) -> RTC_CNTL_WAKEUP_ENA_W {
        RTC_CNTL_WAKEUP_ENA_W { w: self }
    }
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_cause(&mut self) -> RTC_CNTL_WAKEUP_CAUSE_W {
        RTC_CNTL_WAKEUP_CAUSE_W { w: self }
    }
}
