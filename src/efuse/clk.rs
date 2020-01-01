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
#[doc = "Reader of field `EFUSE_CLK_EN`"]
pub type EFUSE_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_CLK_EN`"]
pub struct EFUSE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_EN_W<'a> {
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
#[doc = "Reader of field `EFUSE_CLK_SEL1`"]
pub type EFUSE_CLK_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_CLK_SEL1`"]
pub struct EFUSE_CLK_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_CLK_SEL0`"]
pub type EFUSE_CLK_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_CLK_SEL0`"]
pub struct EFUSE_CLK_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_SEL0_W<'a> {
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
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_clk_sel1(&self) -> EFUSE_CLK_SEL1_R {
        EFUSE_CLK_SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_clk_sel0(&self) -> EFUSE_CLK_SEL0_R {
        EFUSE_CLK_SEL0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W {
        EFUSE_CLK_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_clk_sel1(&mut self) -> EFUSE_CLK_SEL1_W {
        EFUSE_CLK_SEL1_W { w: self }
    }
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn efuse_clk_sel0(&mut self) -> EFUSE_CLK_SEL0_W {
        EFUSE_CLK_SEL0_W { w: self }
    }
}
