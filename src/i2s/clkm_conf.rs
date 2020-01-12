#[doc = "Reader of register CLKM_CONF"]
pub type R = crate::R<u32, super::CLKM_CONF>;
#[doc = "Writer for register CLKM_CONF"]
pub type W = crate::W<u32, super::CLKM_CONF>;
#[doc = "Register CLKM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKA_ENA`"]
pub type CLKA_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKA_ENA`"]
pub struct CLKA_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKA_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CLKM_DIV_A`"]
pub type CLKM_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKM_DIV_A`"]
pub struct CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `CLKM_DIV_B`"]
pub type CLKM_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKM_DIV_B`"]
pub struct CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKM_DIV_NUM`"]
pub type CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKM_DIV_NUM`"]
pub struct CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clka_ena(&self) -> CLKA_ENA_R {
        CLKA_ENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn clkm_div_a(&self) -> CLKM_DIV_A_R {
        CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn clkm_div_b(&self) -> CLKM_DIV_B_R {
        CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkm_div_num(&self) -> CLKM_DIV_NUM_R {
        CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clka_ena(&mut self) -> CLKA_ENA_W {
        CLKA_ENA_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn clkm_div_a(&mut self) -> CLKM_DIV_A_W {
        CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn clkm_div_b(&mut self) -> CLKM_DIV_B_W {
        CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkm_div_num(&mut self) -> CLKM_DIV_NUM_W {
        CLKM_DIV_NUM_W { w: self }
    }
}
