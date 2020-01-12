#[doc = "Reader of register CARRIER0_CFG"]
pub type R = crate::R<u32, super::CARRIER0_CFG>;
#[doc = "Writer for register CARRIER0_CFG"]
pub type W = crate::W<u32, super::CARRIER0_CFG>;
#[doc = "Register CARRIER0_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CARRIER0_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARRIER0_IN_INVERT`"]
pub type CARRIER0_IN_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER0_IN_INVERT`"]
pub struct CARRIER0_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_IN_INVERT_W<'a> {
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
#[doc = "Reader of field `CARRIER0_OUT_INVERT`"]
pub type CARRIER0_OUT_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER0_OUT_INVERT`"]
pub struct CARRIER0_OUT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_OUT_INVERT_W<'a> {
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
#[doc = "Reader of field `CARRIER0_OSHWTH`"]
pub type CARRIER0_OSHWTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARRIER0_OSHWTH`"]
pub struct CARRIER0_OSHWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_OSHWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CARRIER0_DUTY`"]
pub type CARRIER0_DUTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARRIER0_DUTY`"]
pub struct CARRIER0_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `CARRIER0_PRESCALE`"]
pub type CARRIER0_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARRIER0_PRESCALE`"]
pub struct CARRIER0_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `CARRIER0_EN`"]
pub type CARRIER0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER0_EN`"]
pub struct CARRIER0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_EN_W<'a> {
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
    #[doc = "Bit 13 - When set invert the input of PWM0A and PWM0B for this submodule"]
    #[inline(always)]
    pub fn carrier0_in_invert(&self) -> CARRIER0_IN_INVERT_R {
        CARRIER0_IN_INVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When set invert the output of PWM0A and PWM0B for this submodule"]
    #[inline(always)]
    pub fn carrier0_out_invert(&self) -> CARRIER0_OUT_INVERT_R {
        CARRIER0_OUT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn carrier0_oshwth(&self) -> CARRIER0_OSHWTH_R {
        CARRIER0_OSHWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn carrier0_duty(&self) -> CARRIER0_DUTY_R {
        CARRIER0_DUTY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn carrier0_prescale(&self) -> CARRIER0_PRESCALE_R {
        CARRIER0_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - When set carrier0 function is enabled. When cleared carrier0 is bypassed"]
    #[inline(always)]
    pub fn carrier0_en(&self) -> CARRIER0_EN_R {
        CARRIER0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - When set invert the input of PWM0A and PWM0B for this submodule"]
    #[inline(always)]
    pub fn carrier0_in_invert(&mut self) -> CARRIER0_IN_INVERT_W {
        CARRIER0_IN_INVERT_W { w: self }
    }
    #[doc = "Bit 12 - When set invert the output of PWM0A and PWM0B for this submodule"]
    #[inline(always)]
    pub fn carrier0_out_invert(&mut self) -> CARRIER0_OUT_INVERT_W {
        CARRIER0_OUT_INVERT_W { w: self }
    }
    #[doc = "Bits 8:11 - Width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn carrier0_oshwth(&mut self) -> CARRIER0_OSHWTH_W {
        CARRIER0_OSHWTH_W { w: self }
    }
    #[doc = "Bits 5:7 - Carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn carrier0_duty(&mut self) -> CARRIER0_DUTY_W {
        CARRIER0_DUTY_W { w: self }
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn carrier0_prescale(&mut self) -> CARRIER0_PRESCALE_W {
        CARRIER0_PRESCALE_W { w: self }
    }
    #[doc = "Bit 0 - When set carrier0 function is enabled. When cleared carrier0 is bypassed"]
    #[inline(always)]
    pub fn carrier0_en(&mut self) -> CARRIER0_EN_W {
        CARRIER0_EN_W { w: self }
    }
}
