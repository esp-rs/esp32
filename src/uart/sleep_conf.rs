#[doc = "Reader of register SLEEP_CONF"]
pub type R = crate::R<u32, super::SLEEP_CONF>;
#[doc = "Writer for register SLEEP_CONF"]
pub type W = crate::W<u32, super::SLEEP_CONF>;
#[doc = "Register SLEEP_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEP_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACTIVE_THRESHOLD`"]
pub type ACTIVE_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ACTIVE_THRESHOLD`"]
pub struct ACTIVE_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
    #[inline(always)]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W {
        ACTIVE_THRESHOLD_W { w: self }
    }
}
