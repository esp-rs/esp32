#[doc = "Reader of register CLK"]
pub type R = crate::R<u32, super::CLK>;
#[doc = "Writer for register CLK"]
pub type W = crate::W<u32, super::CLK>;
#[doc = "Register CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CLK_EN`"]
pub type MCPWM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CLK_EN`"]
pub struct MCPWM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force clock on for this reg file"]
    #[inline(always)]
    pub fn mcpwm_clk_en(&self) -> MCPWM_CLK_EN_R {
        MCPWM_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force clock on for this reg file"]
    #[inline(always)]
    pub fn mcpwm_clk_en(&mut self) -> MCPWM_CLK_EN_W {
        MCPWM_CLK_EN_W { w: self }
    }
}
