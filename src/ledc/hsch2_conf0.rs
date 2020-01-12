#[doc = "Reader of register HSCH2_CONF0"]
pub type R = crate::R<u32, super::HSCH2_CONF0>;
#[doc = "Writer for register HSCH2_CONF0"]
pub type W = crate::W<u32, super::HSCH2_CONF0>;
#[doc = "Register HSCH2_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH2_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDLE_LV_HSCH2`"]
pub type IDLE_LV_HSCH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE_LV_HSCH2`"]
pub struct IDLE_LV_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_LV_HSCH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SIG_OUT_EN_HSCH2`"]
pub type SIG_OUT_EN_HSCH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG_OUT_EN_HSCH2`"]
pub struct SIG_OUT_EN_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_OUT_EN_HSCH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIMER_SEL_HSCH2`"]
pub type TIMER_SEL_HSCH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_SEL_HSCH2`"]
pub struct TIMER_SEL_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_HSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel2 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch2(&self) -> IDLE_LV_HSCH2_R {
        IDLE_LV_HSCH2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel2"]
    #[inline(always)]
    pub fn sig_out_en_hsch2(&self) -> SIG_OUT_EN_HSCH2_R {
        SIG_OUT_EN_HSCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel2. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch2(&self) -> TIMER_SEL_HSCH2_R {
        TIMER_SEL_HSCH2_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel2 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch2(&mut self) -> IDLE_LV_HSCH2_W {
        IDLE_LV_HSCH2_W { w: self }
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel2"]
    #[inline(always)]
    pub fn sig_out_en_hsch2(&mut self) -> SIG_OUT_EN_HSCH2_W {
        SIG_OUT_EN_HSCH2_W { w: self }
    }
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel2. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch2(&mut self) -> TIMER_SEL_HSCH2_W {
        TIMER_SEL_HSCH2_W { w: self }
    }
}
