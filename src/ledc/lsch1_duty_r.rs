#[doc = "Reader of register LSCH1_DUTY_R"]
pub type R = crate::R<u32, super::LSCH1_DUTY_R>;
#[doc = "Writer for register LSCH1_DUTY_R"]
pub type W = crate::W<u32, super::LSCH1_DUTY_R>;
#[doc = "Register LSCH1_DUTY_R `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH1_DUTY_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_LSCH1`"]
pub type DUTY_LSCH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_LSCH1`"]
pub struct DUTY_LSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_LSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel1."]
    #[inline(always)]
    pub fn duty_lsch1(&self) -> DUTY_LSCH1_R {
        DUTY_LSCH1_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel1."]
    #[inline(always)]
    pub fn duty_lsch1(&mut self) -> DUTY_LSCH1_W {
        DUTY_LSCH1_W { w: self }
    }
}
