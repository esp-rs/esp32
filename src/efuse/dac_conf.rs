#[doc = "Reader of register DAC_CONF"]
pub type R = crate::R<u32, super::DAC_CONF>;
#[doc = "Writer for register DAC_CONF"]
pub type W = crate::W<u32, super::DAC_CONF>;
#[doc = "Register DAC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_DAC_CLK_PAD_SEL`"]
pub type EFUSE_DAC_CLK_PAD_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DAC_CLK_PAD_SEL`"]
pub struct EFUSE_DAC_CLK_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DAC_CLK_PAD_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_DAC_CLK_DIV`"]
pub type EFUSE_DAC_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_DAC_CLK_DIV`"]
pub struct EFUSE_DAC_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DAC_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dac_clk_pad_sel(&self) -> EFUSE_DAC_CLK_PAD_SEL_R {
        EFUSE_DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_dac_clk_div(&self) -> EFUSE_DAC_CLK_DIV_R {
        EFUSE_DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dac_clk_pad_sel(&mut self) -> EFUSE_DAC_CLK_PAD_SEL_W {
        EFUSE_DAC_CLK_PAD_SEL_W { w: self }
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_dac_clk_div(&mut self) -> EFUSE_DAC_CLK_DIV_W {
        EFUSE_DAC_CLK_DIV_W { w: self }
    }
}
