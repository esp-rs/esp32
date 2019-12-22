#[doc = "Reader of register MCPWM_DT2_FED_CFG_REG"]
pub type R = crate::R<u32, super::MCPWM_DT2_FED_CFG_REG>;
#[doc = "Writer for register MCPWM_DT2_FED_CFG_REG"]
pub type W = crate::W<u32, super::MCPWM_DT2_FED_CFG_REG>;
#[doc = "Register MCPWM_DT2_FED_CFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_DT2_FED_CFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_DT2_FED`"]
pub type MCPWM_DT2_FED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_DT2_FED`"]
pub struct MCPWM_DT2_FED_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT2_FED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow reg for FED"]
    #[inline(always)]
    pub fn mcpwm_dt2_fed(&self) -> MCPWM_DT2_FED_R {
        MCPWM_DT2_FED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow reg for FED"]
    #[inline(always)]
    pub fn mcpwm_dt2_fed(&mut self) -> MCPWM_DT2_FED_W {
        MCPWM_DT2_FED_W { w: self }
    }
}
