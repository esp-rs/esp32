#[doc = "Reader of register CAP_TIMER_PHASE"]
pub type R = crate::R<u32, super::CAP_TIMER_PHASE>;
#[doc = "Writer for register CAP_TIMER_PHASE"]
pub type W = crate::W<u32, super::CAP_TIMER_PHASE>;
#[doc = "Register CAP_TIMER_PHASE `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_TIMER_PHASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP_PHASE`"]
pub type MCPWM_CAP_PHASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCPWM_CAP_PHASE`"]
pub struct MCPWM_CAP_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn mcpwm_cap_phase(&self) -> MCPWM_CAP_PHASE_R {
        MCPWM_CAP_PHASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn mcpwm_cap_phase(&mut self) -> MCPWM_CAP_PHASE_W {
        MCPWM_CAP_PHASE_W { w: self }
    }
}
