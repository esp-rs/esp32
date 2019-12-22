#[doc = "Reader of register I2S_CLKM_CONF_REG"]
pub type R = crate::R<u32, super::I2S_CLKM_CONF_REG>;
#[doc = "Writer for register I2S_CLKM_CONF_REG"]
pub type W = crate::W<u32, super::I2S_CLKM_CONF_REG>;
#[doc = "Register I2S_CLKM_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CLKM_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_CLKA_ENA`"]
pub type I2S_CLKA_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CLKA_ENA`"]
pub struct I2S_CLKA_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLKA_ENA_W<'a> {
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
#[doc = "Reader of field `I2S_CLK_EN`"]
pub type I2S_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CLK_EN`"]
pub struct I2S_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLK_EN_W<'a> {
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
#[doc = "Reader of field `I2S_CLKM_DIV_A`"]
pub type I2S_CLKM_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CLKM_DIV_A`"]
pub struct I2S_CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `I2S_CLKM_DIV_B`"]
pub type I2S_CLKM_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CLKM_DIV_B`"]
pub struct I2S_CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2S_CLKM_DIV_NUM`"]
pub type I2S_CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CLKM_DIV_NUM`"]
pub struct I2S_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLKM_DIV_NUM_W<'a> {
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
    pub fn i2s_clka_ena(&self) -> I2S_CLKA_ENA_R {
        I2S_CLKA_ENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s_clk_en(&self) -> I2S_CLK_EN_R {
        I2S_CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn i2s_clkm_div_a(&self) -> I2S_CLKM_DIV_A_R {
        I2S_CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn i2s_clkm_div_b(&self) -> I2S_CLKM_DIV_B_R {
        I2S_CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_clkm_div_num(&self) -> I2S_CLKM_DIV_NUM_R {
        I2S_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s_clka_ena(&mut self) -> I2S_CLKA_ENA_W {
        I2S_CLKA_ENA_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s_clk_en(&mut self) -> I2S_CLK_EN_W {
        I2S_CLK_EN_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn i2s_clkm_div_a(&mut self) -> I2S_CLKM_DIV_A_W {
        I2S_CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn i2s_clkm_div_b(&mut self) -> I2S_CLKM_DIV_B_W {
        I2S_CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_clkm_div_num(&mut self) -> I2S_CLKM_DIV_NUM_W {
        I2S_CLKM_DIV_NUM_W { w: self }
    }
}
