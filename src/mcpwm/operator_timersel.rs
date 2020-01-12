#[doc = "Reader of register OPERATOR_TIMERSEL"]
pub type R = crate::R<u32, super::OPERATOR_TIMERSEL>;
#[doc = "Writer for register OPERATOR_TIMERSEL"]
pub type W = crate::W<u32, super::OPERATOR_TIMERSEL>;
#[doc = "Register OPERATOR_TIMERSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::OPERATOR_TIMERSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPERATOR2_TIMERSEL`"]
pub type OPERATOR2_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERATOR2_TIMERSEL`"]
pub struct OPERATOR2_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR2_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OPERATOR1_TIMERSEL`"]
pub type OPERATOR1_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERATOR1_TIMERSEL`"]
pub struct OPERATOR1_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR1_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `OPERATOR0_TIMERSEL`"]
pub type OPERATOR0_TIMERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERATOR0_TIMERSEL`"]
pub struct OPERATOR0_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR0_TIMERSEL_W<'a> {
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
    pub fn operator2_timersel(&self) -> OPERATOR2_TIMERSEL_R {
        OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR1_TIMERSEL_R {
        OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR0_TIMERSEL_R {
        OPERATOR0_TIMERSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn operator2_timersel(&mut self) -> OPERATOR2_TIMERSEL_W {
        OPERATOR2_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&mut self) -> OPERATOR1_TIMERSEL_W {
        OPERATOR1_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0 0: timer0 1: timer1 2: timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&mut self) -> OPERATOR0_TIMERSEL_W {
        OPERATOR0_TIMERSEL_W { w: self }
    }
}
