#[doc = "Reader of register GEN2_TSTMP_B"]
pub type R = crate::R<u32, super::GEN2_TSTMP_B>;
#[doc = "Writer for register GEN2_TSTMP_B"]
pub type W = crate::W<u32, super::GEN2_TSTMP_B>;
#[doc = "Register GEN2_TSTMP_B `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN2_TSTMP_B {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN2_B`"]
pub type GEN2_B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GEN2_B`"]
pub struct GEN2_B_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn gen2_b(&self) -> GEN2_B_R {
        GEN2_B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn gen2_b(&mut self) -> GEN2_B_W {
        GEN2_B_W { w: self }
    }
}
