#[doc = "Reader of register SLP_TIMER0"]
pub type R = crate::R<u32, super::SLP_TIMER0>;
#[doc = "Writer for register SLP_TIMER0"]
pub type W = crate::W<u32, super::SLP_TIMER0>;
#[doc = "Register SLP_TIMER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLP_TIMER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLP_VAL_LO`"]
pub type SLP_VAL_LO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLP_VAL_LO`"]
pub struct SLP_VAL_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_VAL_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC sleep timer low 32 bits"]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC sleep timer low 32 bits"]
    #[inline(always)]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W {
        SLP_VAL_LO_W { w: self }
    }
}
