#[doc = "Reader of register MCPWM_OPERATOR_TIMERSEL_REG"]
pub type R = crate::R<u32, super::MCPWM_OPERATOR_TIMERSEL_REG>;
#[doc = "Writer for register MCPWM_OPERATOR_TIMERSEL_REG"]
pub type W = crate::W<u32, super::MCPWM_OPERATOR_TIMERSEL_REG>;
#[doc = "Register MCPWM_OPERATOR_TIMERSEL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_OPERATOR_TIMERSEL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_OPERATOR2_TIMERSEL`"]
pub type MCPWM_OPERATOR2_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_OPERATOR2_TIMERSEL`"]
pub struct MCPWM_OPERATOR2_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OPERATOR2_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OPERATOR1_TIMERSEL`"]
pub type MCPWM_OPERATOR1_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_OPERATOR1_TIMERSEL`"]
pub struct MCPWM_OPERATOR1_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OPERATOR1_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OPERATOR0_TIMERSEL`"]
pub type MCPWM_OPERATOR0_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_OPERATOR0_TIMERSEL`"]
pub struct MCPWM_OPERATOR0_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OPERATOR0_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator2_timersel(&self) -> MCPWM_OPERATOR2_TIMERSEL_R {
        MCPWM_OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator1_timersel(&self) -> MCPWM_OPERATOR1_TIMERSEL_R {
        MCPWM_OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator0_timersel(&self) -> MCPWM_OPERATOR0_TIMERSEL_R {
        MCPWM_OPERATOR0_TIMERSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator2_timersel(&mut self) -> MCPWM_OPERATOR2_TIMERSEL_W {
        MCPWM_OPERATOR2_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator1_timersel(&mut self) -> MCPWM_OPERATOR1_TIMERSEL_W {
        MCPWM_OPERATOR1_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn mcpwm_operator0_timersel(&mut self) -> MCPWM_OPERATOR0_TIMERSEL_W {
        MCPWM_OPERATOR0_TIMERSEL_W { w: self }
    }
}
