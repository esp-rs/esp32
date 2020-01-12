#[doc = "Reader of register CLK_CFG"]
pub type R = crate::R<u32, super::CLK_CFG>;
#[doc = "Writer for register CLK_CFG"]
pub type W = crate::W<u32, super::CLK_CFG>;
#[doc = "Register CLK_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_PRESCALE`"]
pub type CLK_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_PRESCALE`"]
pub struct CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PRESCALE_W<'a> {
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
    pub fn clk_prescale(&self) -> CLK_PRESCALE_R {
        CLK_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period of PWM_clk = 6.25ns * (PWM_CLK_PRESCALE + 1)"]
    #[inline(always)]
    pub fn clk_prescale(&mut self) -> CLK_PRESCALE_W {
        CLK_PRESCALE_W { w: self }
    }
}
