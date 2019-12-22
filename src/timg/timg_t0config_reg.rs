#[doc = "Reader of register TIMG_T0CONFIG_REG"]
pub type R = crate::R<u32, super::TIMG_T0CONFIG_REG>;
#[doc = "Writer for register TIMG_T0CONFIG_REG"]
pub type W = crate::W<u32, super::TIMG_T0CONFIG_REG>;
#[doc = "Register TIMG_T0CONFIG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_T0CONFIG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T0_EN`"]
pub type TIMG_T0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_EN`"]
pub struct TIMG_T0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `TIMG_T0_INCREASE`"]
pub type TIMG_T0_INCREASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_INCREASE`"]
pub struct TIMG_T0_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_INCREASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TIMG_T0_AUTORELOAD`"]
pub type TIMG_T0_AUTORELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_AUTORELOAD`"]
pub struct TIMG_T0_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_AUTORELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIMG_T0_DIVIDER`"]
pub type TIMG_T0_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMG_T0_DIVIDER`"]
pub struct TIMG_T0_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | (((value as u32) & 0xffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMG_T0_EDGE_INT_EN`"]
pub type TIMG_T0_EDGE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_EDGE_INT_EN`"]
pub struct TIMG_T0_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_EDGE_INT_EN_W<'a> {
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
#[doc = "Reader of field `TIMG_T0_LEVEL_INT_EN`"]
pub type TIMG_T0_LEVEL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_LEVEL_INT_EN`"]
pub struct TIMG_T0_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_LEVEL_INT_EN_W<'a> {
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
#[doc = "Reader of field `TIMG_T0_ALARM_EN`"]
pub type TIMG_T0_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_ALARM_EN`"]
pub struct TIMG_T0_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_ALARM_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    pub fn timg_t0_en(&self) -> TIMG_T0_EN_R {
        TIMG_T0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    pub fn timg_t0_increase(&self) -> TIMG_T0_INCREASE_R {
        TIMG_T0_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn timg_t0_autoreload(&self) -> TIMG_T0_AUTORELOAD_R {
        TIMG_T0_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    pub fn timg_t0_divider(&self) -> TIMG_T0_DIVIDER_R {
        TIMG_T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn timg_t0_edge_int_en(&self) -> TIMG_T0_EDGE_INT_EN_R {
        TIMG_T0_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn timg_t0_level_int_en(&self) -> TIMG_T0_LEVEL_INT_EN_R {
        TIMG_T0_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn timg_t0_alarm_en(&self) -> TIMG_T0_ALARM_EN_R {
        TIMG_T0_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    pub fn timg_t0_en(&mut self) -> TIMG_T0_EN_W {
        TIMG_T0_EN_W { w: self }
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    pub fn timg_t0_increase(&mut self) -> TIMG_T0_INCREASE_W {
        TIMG_T0_INCREASE_W { w: self }
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn timg_t0_autoreload(&mut self) -> TIMG_T0_AUTORELOAD_W {
        TIMG_T0_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    pub fn timg_t0_divider(&mut self) -> TIMG_T0_DIVIDER_W {
        TIMG_T0_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn timg_t0_edge_int_en(&mut self) -> TIMG_T0_EDGE_INT_EN_W {
        TIMG_T0_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn timg_t0_level_int_en(&mut self) -> TIMG_T0_LEVEL_INT_EN_W {
        TIMG_T0_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn timg_t0_alarm_en(&mut self) -> TIMG_T0_ALARM_EN_W {
        TIMG_T0_ALARM_EN_W { w: self }
    }
}
