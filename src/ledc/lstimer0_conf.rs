#[doc = "Reader of register LSTIMER0_CONF"]
pub type R = crate::R<u32, super::LSTIMER0_CONF>;
#[doc = "Writer for register LSTIMER0_CONF"]
pub type W = crate::W<u32, super::LSTIMER0_CONF>;
#[doc = "Register LSTIMER0_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTIMER0_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSTIMER0_PARA_UP`"]
pub type LSTIMER0_PARA_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER0_PARA_UP`"]
pub struct LSTIMER0_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TICK_SEL_LSTIMER0`"]
pub type TICK_SEL_LSTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_SEL_LSTIMER0`"]
pub struct TICK_SEL_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER0_W<'a> {
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
#[doc = "Reader of field `LSTIMER0_RST`"]
pub type LSTIMER0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER0_RST`"]
pub struct LSTIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_RST_W<'a> {
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
#[doc = "Reader of field `LSTIMER0_PAUSE`"]
pub type LSTIMER0_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER0_PAUSE`"]
pub struct LSTIMER0_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_PAUSE_W<'a> {
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
#[doc = "Reader of field `DIV_NUM_LSTIMER0`"]
pub type DIV_NUM_LSTIMER0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIV_NUM_LSTIMER0`"]
pub struct DIV_NUM_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_LSTIMER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | (((value as u32) & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `LSTIMER0_LIM`"]
pub type LSTIMER0_LIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSTIMER0_LIM`"]
pub struct LSTIMER0_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn lstimer0_para_up(&self) -> LSTIMER0_PARA_UP_R {
        LSTIMER0_PARA_UP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&self) -> TICK_SEL_LSTIMER0_R {
        TICK_SEL_LSTIMER0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer0_rst(&self) -> LSTIMER0_RST_R {
        LSTIMER0_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn lstimer0_pause(&self) -> LSTIMER0_PAUSE_R {
        LSTIMER0_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer0(&self) -> DIV_NUM_LSTIMER0_R {
        DIV_NUM_LSTIMER0_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer0_lim(&self) -> LSTIMER0_LIM_R {
        LSTIMER0_LIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn lstimer0_para_up(&mut self) -> LSTIMER0_PARA_UP_W {
        LSTIMER0_PARA_UP_W { w: self }
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&mut self) -> TICK_SEL_LSTIMER0_W {
        TICK_SEL_LSTIMER0_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer0_rst(&mut self) -> LSTIMER0_RST_W {
        LSTIMER0_RST_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn lstimer0_pause(&mut self) -> LSTIMER0_PAUSE_W {
        LSTIMER0_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer0(&mut self) -> DIV_NUM_LSTIMER0_W {
        DIV_NUM_LSTIMER0_W { w: self }
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer0_lim(&mut self) -> LSTIMER0_LIM_W {
        LSTIMER0_LIM_W { w: self }
    }
}
