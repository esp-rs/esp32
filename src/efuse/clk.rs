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
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLK_SEL1`"]
pub type CLK_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_SEL1`"]
pub struct CLK_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLK_SEL0`"]
pub type CLK_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_SEL0`"]
pub struct CLK_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - efuse timing configure"]
    #[inline(always)]
    pub fn clk_sel1(&self) -> CLK_SEL1_R {
        CLK_SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn clk_sel0(&self) -> CLK_SEL0_R {
        CLK_SEL0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - efuse timing configure"]
    #[inline(always)]
    pub fn clk_sel1(&mut self) -> CLK_SEL1_W {
        CLK_SEL1_W { w: self }
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn clk_sel0(&mut self) -> CLK_SEL0_W {
        CLK_SEL0_W { w: self }
    }
}
