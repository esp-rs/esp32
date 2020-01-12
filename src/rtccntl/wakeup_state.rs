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
#[doc = "Reader of field `GPIO_WAKEUP_FILTER`"]
pub type GPIO_WAKEUP_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_WAKEUP_FILTER`"]
pub struct GPIO_WAKEUP_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_FILTER_W<'a> {
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
#[doc = "Reader of field `WAKEUP_ENA`"]
pub type WAKEUP_ENA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WAKEUP_ENA`"]
pub struct WAKEUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `WAKEUP_CAUSE`"]
pub type WAKEUP_CAUSE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WAKEUP_CAUSE`"]
pub struct WAKEUP_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_CAUSE_W<'a> {
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
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W {
        GPIO_WAKEUP_FILTER_W { w: self }
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W {
        WAKEUP_ENA_W { w: self }
    }
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&mut self) -> WAKEUP_CAUSE_W {
        WAKEUP_CAUSE_W { w: self }
    }
}
