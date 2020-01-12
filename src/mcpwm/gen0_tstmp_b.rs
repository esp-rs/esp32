#[doc = "Reader of register GEN0_TSTMP_B"]
pub type R = crate::R<u32, super::GEN0_TSTMP_B>;
#[doc = "Writer for register GEN0_TSTMP_B"]
pub type W = crate::W<u32, super::GEN0_TSTMP_B>;
#[doc = "Register GEN0_TSTMP_B `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN0_TSTMP_B {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN0_B`"]
pub type GEN0_B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GEN0_B`"]
pub struct GEN0_B_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN0_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn gen0_b(&self) -> GEN0_B_R {
        GEN0_B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn gen0_b(&mut self) -> GEN0_B_W {
        GEN0_B_W { w: self }
    }
}