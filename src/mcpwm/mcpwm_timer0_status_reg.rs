#[doc = "Reader of register MCPWM_TIMER0_STATUS_REG"]
pub type R = crate::R<u32, super::MCPWM_TIMER0_STATUS_REG>;
#[doc = "Writer for register MCPWM_TIMER0_STATUS_REG"]
pub type W = crate::W<u32, super::MCPWM_TIMER0_STATUS_REG>;
#[doc = "Register MCPWM_TIMER0_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_TIMER0_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_DIRECTION`"]
pub type MCPWM_TIMER0_DIRECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER0_DIRECTION`"]
pub struct MCPWM_TIMER0_DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_DIRECTION_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER0_VALUE`"]
pub type MCPWM_TIMER0_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_TIMER0_VALUE`"]
pub struct MCPWM_TIMER0_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_VALUE_W<'a> {
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
    pub fn mcpwm_timer0_direction(&self) -> MCPWM_TIMER0_DIRECTION_R {
        MCPWM_TIMER0_DIRECTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - Current PWM timer0 counter value"]
    #[inline(always)]
    pub fn mcpwm_timer0_value(&self) -> MCPWM_TIMER0_VALUE_R {
        MCPWM_TIMER0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Current PWM timer0 counter direction 0: increment 1: decrement"]
    #[inline(always)]
    pub fn mcpwm_timer0_direction(&mut self) -> MCPWM_TIMER0_DIRECTION_W {
        MCPWM_TIMER0_DIRECTION_W { w: self }
    }
    #[doc = "Bits 0:15 - Current PWM timer0 counter value"]
    #[inline(always)]
    pub fn mcpwm_timer0_value(&mut self) -> MCPWM_TIMER0_VALUE_W {
        MCPWM_TIMER0_VALUE_W { w: self }
    }
}
