#[doc = "Reader of register SLP_TIMER1"]
pub type R = crate::R<u32, super::SLP_TIMER1>;
#[doc = "Writer for register SLP_TIMER1"]
pub type W = crate::W<u32, super::SLP_TIMER1>;
#[doc = "Register SLP_TIMER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLP_TIMER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAIN_TIMER_ALARM_EN`"]
pub type MAIN_TIMER_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_TIMER_ALARM_EN`"]
pub struct MAIN_TIMER_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_ALARM_EN_W<'a> {
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
#[doc = "Reader of field `SLP_VAL_HI`"]
pub type SLP_VAL_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLP_VAL_HI`"]
pub struct SLP_VAL_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_VAL_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn main_timer_alarm_en(&self) -> MAIN_TIMER_ALARM_EN_R {
        MAIN_TIMER_ALARM_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W {
        MAIN_TIMER_ALARM_EN_W { w: self }
    }
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W {
        SLP_VAL_HI_W { w: self }
    }
}
