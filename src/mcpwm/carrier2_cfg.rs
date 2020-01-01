#[doc = "Reader of register CARRIER2_CFG"]
pub type R = crate::R<u32, super::CARRIER2_CFG>;
#[doc = "Writer for register CARRIER2_CFG"]
pub type W = crate::W<u32, super::CARRIER2_CFG>;
#[doc = "Register CARRIER2_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CARRIER2_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CARRIER2_IN_INVERT`"]
pub type MCPWM_CARRIER2_IN_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_IN_INVERT`"]
pub struct MCPWM_CARRIER2_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_IN_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CARRIER2_OUT_INVERT`"]
pub type MCPWM_CARRIER2_OUT_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_OUT_INVERT`"]
pub struct MCPWM_CARRIER2_OUT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_OUT_INVERT_W<'a> {
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
#[doc = "Reader of field `MCPWM_CARRIER2_OSHWTH`"]
pub type MCPWM_CARRIER2_OSHWTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_OSHWTH`"]
pub struct MCPWM_CARRIER2_OSHWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_OSHWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CARRIER2_DUTY`"]
pub type MCPWM_CARRIER2_DUTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_DUTY`"]
pub struct MCPWM_CARRIER2_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CARRIER2_PRESCALE`"]
pub type MCPWM_CARRIER2_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_PRESCALE`"]
pub struct MCPWM_CARRIER2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CARRIER2_EN`"]
pub type MCPWM_CARRIER2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CARRIER2_EN`"]
pub struct MCPWM_CARRIER2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CARRIER2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - When set invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn mcpwm_carrier2_in_invert(&self) -> MCPWM_CARRIER2_IN_INVERT_R {
        MCPWM_CARRIER2_IN_INVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When set invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn mcpwm_carrier2_out_invert(&self) -> MCPWM_CARRIER2_OUT_INVERT_R {
        MCPWM_CARRIER2_OUT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn mcpwm_carrier2_oshwth(&self) -> MCPWM_CARRIER2_OSHWTH_R {
        MCPWM_CARRIER2_OSHWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Carrier duty selection. Duty = PWM_CARRIER2_DUTY / 8"]
    #[inline(always)]
    pub fn mcpwm_carrier2_duty(&self) -> MCPWM_CARRIER2_DUTY_R {
        MCPWM_CARRIER2_DUTY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 1:4 - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_carrier2_prescale(&self) -> MCPWM_CARRIER2_PRESCALE_R {
        MCPWM_CARRIER2_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - When set carrier2 function is enabled. When cleared carrier2 is bypassed"]
    #[inline(always)]
    pub fn mcpwm_carrier2_en(&self) -> MCPWM_CARRIER2_EN_R {
        MCPWM_CARRIER2_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - When set invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn mcpwm_carrier2_in_invert(&mut self) -> MCPWM_CARRIER2_IN_INVERT_W {
        MCPWM_CARRIER2_IN_INVERT_W { w: self }
    }
    #[doc = "Bit 12 - When set invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn mcpwm_carrier2_out_invert(&mut self) -> MCPWM_CARRIER2_OUT_INVERT_W {
        MCPWM_CARRIER2_OUT_INVERT_W { w: self }
    }
    #[doc = "Bits 8:11 - Width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn mcpwm_carrier2_oshwth(&mut self) -> MCPWM_CARRIER2_OSHWTH_W {
        MCPWM_CARRIER2_OSHWTH_W { w: self }
    }
    #[doc = "Bits 5:7 - Carrier duty selection. Duty = PWM_CARRIER2_DUTY / 8"]
    #[inline(always)]
    pub fn mcpwm_carrier2_duty(&mut self) -> MCPWM_CARRIER2_DUTY_W {
        MCPWM_CARRIER2_DUTY_W { w: self }
    }
    #[doc = "Bits 1:4 - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn mcpwm_carrier2_prescale(&mut self) -> MCPWM_CARRIER2_PRESCALE_W {
        MCPWM_CARRIER2_PRESCALE_W { w: self }
    }
    #[doc = "Bit 0 - When set carrier2 function is enabled. When cleared carrier2 is bypassed"]
    #[inline(always)]
    pub fn mcpwm_carrier2_en(&mut self) -> MCPWM_CARRIER2_EN_W {
        MCPWM_CARRIER2_EN_W { w: self }
    }
}
