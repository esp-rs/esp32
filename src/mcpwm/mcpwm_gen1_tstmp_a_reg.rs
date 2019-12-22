#[doc = "Reader of register MCPWM_GEN1_TSTMP_A_REG"]
pub type R = crate::R<u32, super::MCPWM_GEN1_TSTMP_A_REG>;
#[doc = "Writer for register MCPWM_GEN1_TSTMP_A_REG"]
pub type W = crate::W<u32, super::MCPWM_GEN1_TSTMP_A_REG>;
#[doc = "Register MCPWM_GEN1_TSTMP_A_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_GEN1_TSTMP_A_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_GEN1_A`"]
pub type MCPWM_GEN1_A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_GEN1_A`"]
pub struct MCPWM_GEN1_A_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN1_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp A's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen1_a(&self) -> MCPWM_GEN1_A_R {
        MCPWM_GEN1_A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp A's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen1_a(&mut self) -> MCPWM_GEN1_A_W {
        MCPWM_GEN1_A_W { w: self }
    }
}
