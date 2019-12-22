#[doc = "Reader of register LEDC_HSTIMER3_CONF_REG"]
pub type R = crate::R<u32, super::LEDC_HSTIMER3_CONF_REG>;
#[doc = "Writer for register LEDC_HSTIMER3_CONF_REG"]
pub type W = crate::W<u32, super::LEDC_HSTIMER3_CONF_REG>;
#[doc = "Register LEDC_HSTIMER3_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_HSTIMER3_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_TICK_SEL_HSTIMER3`"]
pub type LEDC_TICK_SEL_HSTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_TICK_SEL_HSTIMER3`"]
pub struct LEDC_TICK_SEL_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TICK_SEL_HSTIMER3_W<'a> {
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
#[doc = "Reader of field `LEDC_HSTIMER3_RST`"]
pub type LEDC_HSTIMER3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER3_RST`"]
pub struct LEDC_HSTIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER3_RST_W<'a> {
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
#[doc = "Reader of field `LEDC_HSTIMER3_PAUSE`"]
pub type LEDC_HSTIMER3_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER3_PAUSE`"]
pub struct LEDC_HSTIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER3_PAUSE_W<'a> {
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
#[doc = "Reader of field `LEDC_DIV_NUM_HSTIMER3`"]
pub type LEDC_DIV_NUM_HSTIMER3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DIV_NUM_HSTIMER3`"]
pub struct LEDC_DIV_NUM_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DIV_NUM_HSTIMER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | (((value as u32) & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `LEDC_HSTIMER3_LIM`"]
pub type LEDC_HSTIMER3_LIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_HSTIMER3_LIM`"]
pub struct LEDC_HSTIMER3_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER3_LIM_W<'a> {
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
    pub fn ledc_tick_sel_hstimer3(&self) -> LEDC_TICK_SEL_HSTIMER3_R {
        LEDC_TICK_SEL_HSTIMER3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn ledc_hstimer3_rst(&self) -> LEDC_HSTIMER3_RST_R {
        LEDC_HSTIMER3_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn ledc_hstimer3_pause(&self) -> LEDC_HSTIMER3_PAUSE_R {
        LEDC_HSTIMER3_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn ledc_div_num_hstimer3(&self) -> LEDC_DIV_NUM_HSTIMER3_R {
        LEDC_DIV_NUM_HSTIMER3_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn ledc_hstimer3_lim(&self) -> LEDC_HSTIMER3_LIM_R {
        LEDC_HSTIMER3_LIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn ledc_tick_sel_hstimer3(&mut self) -> LEDC_TICK_SEL_HSTIMER3_W {
        LEDC_TICK_SEL_HSTIMER3_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn ledc_hstimer3_rst(&mut self) -> LEDC_HSTIMER3_RST_W {
        LEDC_HSTIMER3_RST_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn ledc_hstimer3_pause(&mut self) -> LEDC_HSTIMER3_PAUSE_W {
        LEDC_HSTIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn ledc_div_num_hstimer3(&mut self) -> LEDC_DIV_NUM_HSTIMER3_W {
        LEDC_DIV_NUM_HSTIMER3_W { w: self }
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn ledc_hstimer3_lim(&mut self) -> LEDC_HSTIMER3_LIM_W {
        LEDC_HSTIMER3_LIM_W { w: self }
    }
}
