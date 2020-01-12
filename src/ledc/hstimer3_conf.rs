#[doc = "Reader of register HSTIMER3_CONF"]
pub type R = crate::R<u32, super::HSTIMER3_CONF>;
#[doc = "Writer for register HSTIMER3_CONF"]
pub type W = crate::W<u32, super::HSTIMER3_CONF>;
#[doc = "Register HSTIMER3_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTIMER3_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TICK_SEL_HSTIMER3`"]
pub type TICK_SEL_HSTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_SEL_HSTIMER3`"]
pub struct TICK_SEL_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_HSTIMER3_W<'a> {
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
#[doc = "Reader of field `HSTIMER3_RST`"]
pub type HSTIMER3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER3_RST`"]
pub struct HSTIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_RST_W<'a> {
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
#[doc = "Reader of field `HSTIMER3_PAUSE`"]
pub type HSTIMER3_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER3_PAUSE`"]
pub struct HSTIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_PAUSE_W<'a> {
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
#[doc = "Reader of field `DIV_NUM_HSTIMER3`"]
pub type DIV_NUM_HSTIMER3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIV_NUM_HSTIMER3`"]
pub struct DIV_NUM_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_HSTIMER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | (((value as u32) & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `HSTIMER3_LIM`"]
pub type HSTIMER3_LIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTIMER3_LIM`"]
pub struct HSTIMER3_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer3(&self) -> TICK_SEL_HSTIMER3_R {
        TICK_SEL_HSTIMER3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer3_rst(&self) -> HSTIMER3_RST_R {
        HSTIMER3_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn hstimer3_pause(&self) -> HSTIMER3_PAUSE_R {
        HSTIMER3_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer3(&self) -> DIV_NUM_HSTIMER3_R {
        DIV_NUM_HSTIMER3_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer3_lim(&self) -> HSTIMER3_LIM_R {
        HSTIMER3_LIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer3(&mut self) -> TICK_SEL_HSTIMER3_W {
        TICK_SEL_HSTIMER3_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer3_rst(&mut self) -> HSTIMER3_RST_W {
        HSTIMER3_RST_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn hstimer3_pause(&mut self) -> HSTIMER3_PAUSE_W {
        HSTIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer3(&mut self) -> DIV_NUM_HSTIMER3_W {
        DIV_NUM_HSTIMER3_W { w: self }
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer3_lim(&mut self) -> HSTIMER3_LIM_W {
        HSTIMER3_LIM_W { w: self }
    }
}
