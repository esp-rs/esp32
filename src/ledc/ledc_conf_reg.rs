#[doc = "Reader of register LEDC_CONF_REG"]
pub type R = crate::R<u32, super::LEDC_CONF_REG>;
#[doc = "Writer for register LEDC_CONF_REG"]
pub type W = crate::W<u32, super::LEDC_CONF_REG>;
#[doc = "Register LEDC_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_APB_CLK_SEL`"]
pub type LEDC_APB_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_APB_CLK_SEL`"]
pub struct LEDC_APB_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_APB_CLK_SEL_W<'a> {
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
    #[doc = "Bit 0 - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
    #[inline(always)]
    pub fn ledc_apb_clk_sel(&self) -> LEDC_APB_CLK_SEL_R {
        LEDC_APB_CLK_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
    #[inline(always)]
    pub fn ledc_apb_clk_sel(&mut self) -> LEDC_APB_CLK_SEL_W {
        LEDC_APB_CLK_SEL_W { w: self }
    }
}
