#[doc = "Reader of register MCPWM_TIMER0_CFG0_REG"]
pub type R = crate::R<u32, super::MCPWM_TIMER0_CFG0_REG>;
#[doc = "Writer for register MCPWM_TIMER0_CFG0_REG"]
pub type W = crate::W<u32, super::MCPWM_TIMER0_CFG0_REG>;
#[doc = "Register MCPWM_TIMER0_CFG0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_TIMER0_CFG0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_PERIOD_UPMETHOD`"]
pub type MCPWM_TIMER0_PERIOD_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER0_PERIOD_UPMETHOD`"]
pub struct MCPWM_TIMER0_PERIOD_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_PERIOD_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_PERIOD`"]
pub type MCPWM_TIMER0_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_TIMER0_PERIOD`"]
pub struct MCPWM_TIMER0_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_PRESCALE`"]
pub type MCPWM_TIMER0_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER0_PRESCALE`"]
pub struct MCPWM_TIMER0_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25 - Update method for active reg of PWM timer0 period 0: immediate 1: TEZ 2: sync 3: TEZ or sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn mcpwm_timer0_period_upmethod(&self) -> MCPWM_TIMER0_PERIOD_UPMETHOD_R {
        MCPWM_TIMER0_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 8:23 - Period shadow reg of PWM timer0"]
    #[inline(always)]
    pub fn mcpwm_timer0_period(&self) -> MCPWM_TIMER0_PERIOD_R {
        MCPWM_TIMER0_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 0:7 - Period of PT0_clk = Period of PWM_clk * (PWM_TIMER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_timer0_prescale(&self) -> MCPWM_TIMER0_PRESCALE_R {
        MCPWM_TIMER0_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - Update method for active reg of PWM timer0 period 0: immediate 1: TEZ 2: sync 3: TEZ or sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn mcpwm_timer0_period_upmethod(&mut self) -> MCPWM_TIMER0_PERIOD_UPMETHOD_W {
        MCPWM_TIMER0_PERIOD_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 8:23 - Period shadow reg of PWM timer0"]
    #[inline(always)]
    pub fn mcpwm_timer0_period(&mut self) -> MCPWM_TIMER0_PERIOD_W {
        MCPWM_TIMER0_PERIOD_W { w: self }
    }
    #[doc = "Bits 0:7 - Period of PT0_clk = Period of PWM_clk * (PWM_TIMER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_timer0_prescale(&mut self) -> MCPWM_TIMER0_PRESCALE_W {
        MCPWM_TIMER0_PRESCALE_W { w: self }
    }
}
