#[doc = "Reader of register TIMER0_STATUS"]
pub type R = crate::R<u32, super::TIMER0_STATUS>;
#[doc = "Writer for register TIMER0_STATUS"]
pub type W = crate::W<u32, super::TIMER0_STATUS>;
#[doc = "Register TIMER0_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER0_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER0_DIRECTION`"]
pub type TIMER0_DIRECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_DIRECTION`"]
pub struct TIMER0_DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_DIRECTION_W<'a> {
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
#[doc = "Reader of field `TIMER0_VALUE`"]
pub type TIMER0_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER0_VALUE`"]
pub struct TIMER0_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Current PWM timer0 counter direction 0: increment 1: decrement"]
    #[inline(always)]
    pub fn timer0_direction(&self) -> TIMER0_DIRECTION_R {
        TIMER0_DIRECTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - Current PWM timer0 counter value"]
    #[inline(always)]
    pub fn timer0_value(&self) -> TIMER0_VALUE_R {
        TIMER0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Current PWM timer0 counter direction 0: increment 1: decrement"]
    #[inline(always)]
    pub fn timer0_direction(&mut self) -> TIMER0_DIRECTION_W {
        TIMER0_DIRECTION_W { w: self }
    }
    #[doc = "Bits 0:15 - Current PWM timer0 counter value"]
    #[inline(always)]
    pub fn timer0_value(&mut self) -> TIMER0_VALUE_W {
        TIMER0_VALUE_W { w: self }
    }
}
