#[doc = "Reader of register HSTIMER2_CONF"]
pub type R = crate::R<u32, super::HSTIMER2_CONF>;
#[doc = "Writer for register HSTIMER2_CONF"]
pub type W = crate::W<u32, super::HSTIMER2_CONF>;
#[doc = "Register HSTIMER2_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTIMER2_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TICK_SEL_HSTIMER2`"]
pub type TICK_SEL_HSTIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_SEL_HSTIMER2`"]
pub struct TICK_SEL_HSTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_HSTIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HSTIMER2_RST`"]
pub type HSTIMER2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER2_RST`"]
pub struct HSTIMER2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `HSTIMER2_PAUSE`"]
pub type HSTIMER2_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER2_PAUSE`"]
pub struct HSTIMER2_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DIV_NUM_HSTIMER2`"]
pub type DIV_NUM_HSTIMER2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIV_NUM_HSTIMER2`"]
pub struct DIV_NUM_HSTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_HSTIMER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | (((value as u32) & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `HSTIMER2_LIM`"]
pub type HSTIMER2_LIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTIMER2_LIM`"]
pub struct HSTIMER2_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer2. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer2(&self) -> TICK_SEL_HSTIMER2_R {
        TICK_SEL_HSTIMER2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer2 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer2_rst(&self) -> HSTIMER2_RST_R {
        HSTIMER2_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer2"]
    #[inline(always)]
    pub fn hstimer2_pause(&self) -> HSTIMER2_PAUSE_R {
        HSTIMER2_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer2 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer2(&self) -> DIV_NUM_HSTIMER2_R {
        DIV_NUM_HSTIMER2_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer2. the counter range is \\[0 2**reg_hstimer2_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer2_lim(&self) -> HSTIMER2_LIM_R {
        HSTIMER2_LIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer2. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer2(&mut self) -> TICK_SEL_HSTIMER2_W {
        TICK_SEL_HSTIMER2_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer2 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer2_rst(&mut self) -> HSTIMER2_RST_W {
        HSTIMER2_RST_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer2"]
    #[inline(always)]
    pub fn hstimer2_pause(&mut self) -> HSTIMER2_PAUSE_W {
        HSTIMER2_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer2 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer2(&mut self) -> DIV_NUM_HSTIMER2_W {
        DIV_NUM_HSTIMER2_W { w: self }
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer2. the counter range is \\[0 2**reg_hstimer2_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer2_lim(&mut self) -> HSTIMER2_LIM_W {
        HSTIMER2_LIM_W { w: self }
    }
}
