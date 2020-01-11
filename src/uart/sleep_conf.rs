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
#[doc = "Reader of field `UART_ACTIVE_THRESHOLD`"]
pub type UART_ACTIVE_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_ACTIVE_THRESHOLD`"]
pub struct UART_ACTIVE_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ACTIVE_THRESHOLD_W<'a> {
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
    pub fn uart_active_threshold(&self) -> UART_ACTIVE_THRESHOLD_R {
        UART_ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
    #[inline(always)]
    pub fn uart_active_threshold(&mut self) -> UART_ACTIVE_THRESHOLD_W {
        UART_ACTIVE_THRESHOLD_W { w: self }
    }
}
