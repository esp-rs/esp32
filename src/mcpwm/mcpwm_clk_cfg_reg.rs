#[doc = "Reader of register MCPWM_CLK_CFG_REG"]
pub type R = crate::R<u32, super::MCPWM_CLK_CFG_REG>;
#[doc = "Writer for register MCPWM_CLK_CFG_REG"]
pub type W = crate::W<u32, super::MCPWM_CLK_CFG_REG>;
#[doc = "Register MCPWM_CLK_CFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_CLK_CFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CLK_PRESCALE`"]
pub type MCPWM_CLK_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_CLK_PRESCALE`"]
pub struct MCPWM_CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CLK_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_clk_prescale(&self) -> MCPWM_CLK_PRESCALE_R {
        MCPWM_CLK_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_clk_prescale(&mut self) -> MCPWM_CLK_PRESCALE_W {
        MCPWM_CLK_PRESCALE_W { w: self }
    }
}
