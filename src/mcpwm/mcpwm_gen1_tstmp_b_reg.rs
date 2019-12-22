#[doc = "Reader of register MCPWM_GEN1_TSTMP_B_REG"]
pub type R = crate::R<u32, super::MCPWM_GEN1_TSTMP_B_REG>;
#[doc = "Writer for register MCPWM_GEN1_TSTMP_B_REG"]
pub type W = crate::W<u32, super::MCPWM_GEN1_TSTMP_B_REG>;
#[doc = "Register MCPWM_GEN1_TSTMP_B_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_GEN1_TSTMP_B_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_GEN1_B`"]
pub type MCPWM_GEN1_B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_GEN1_B`"]
pub struct MCPWM_GEN1_B_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN1_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen1_b(&self) -> MCPWM_GEN1_B_R {
        MCPWM_GEN1_B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp B's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen1_b(&mut self) -> MCPWM_GEN1_B_W {
        MCPWM_GEN1_B_W { w: self }
    }
}
