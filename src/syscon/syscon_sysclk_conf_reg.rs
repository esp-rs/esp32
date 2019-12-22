#[doc = "Reader of register SYSCON_SYSCLK_CONF_REG"]
pub type R = crate::R<u32, super::SYSCON_SYSCLK_CONF_REG>;
#[doc = "Writer for register SYSCON_SYSCLK_CONF_REG"]
pub type W = crate::W<u32, super::SYSCON_SYSCLK_CONF_REG>;
#[doc = "Register SYSCON_SYSCLK_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_SYSCLK_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_QUICK_CLK_CHNG`"]
pub type SYSCON_QUICK_CLK_CHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_QUICK_CLK_CHNG`"]
pub struct SYSCON_QUICK_CLK_CHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_QUICK_CLK_CHNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_RST_TICK_CNT`"]
pub type SYSCON_RST_TICK_CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_RST_TICK_CNT`"]
pub struct SYSCON_RST_TICK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_RST_TICK_CNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_CLK_EN`"]
pub type SYSCON_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_EN`"]
pub struct SYSCON_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_CLK_320M_EN`"]
pub type SYSCON_CLK_320M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_320M_EN`"]
pub struct SYSCON_CLK_320M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_320M_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PRE_DIV_CNT`"]
pub type SYSCON_PRE_DIV_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSCON_PRE_DIV_CNT`"]
pub struct SYSCON_PRE_DIV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PRE_DIV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn syscon_quick_clk_chng(&self) -> SYSCON_QUICK_CLK_CHNG_R {
        SYSCON_QUICK_CLK_CHNG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn syscon_rst_tick_cnt(&self) -> SYSCON_RST_TICK_CNT_R {
        SYSCON_RST_TICK_CNT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn syscon_clk_en(&self) -> SYSCON_CLK_EN_R {
        SYSCON_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn syscon_clk_320m_en(&self) -> SYSCON_CLK_320M_EN_R {
        SYSCON_CLK_320M_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn syscon_pre_div_cnt(&self) -> SYSCON_PRE_DIV_CNT_R {
        SYSCON_PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn syscon_quick_clk_chng(&mut self) -> SYSCON_QUICK_CLK_CHNG_W {
        SYSCON_QUICK_CLK_CHNG_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn syscon_rst_tick_cnt(&mut self) -> SYSCON_RST_TICK_CNT_W {
        SYSCON_RST_TICK_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn syscon_clk_en(&mut self) -> SYSCON_CLK_EN_W {
        SYSCON_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn syscon_clk_320m_en(&mut self) -> SYSCON_CLK_320M_EN_W {
        SYSCON_CLK_320M_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn syscon_pre_div_cnt(&mut self) -> SYSCON_PRE_DIV_CNT_W {
        SYSCON_PRE_DIV_CNT_W { w: self }
    }
}
