#[doc = "Reader of register SENS_SAR_TOUCH_CTRL1_REG"]
pub type R = crate::R<u32, super::SENS_SAR_TOUCH_CTRL1_REG>;
#[doc = "Writer for register SENS_SAR_TOUCH_CTRL1_REG"]
pub type W = crate::W<u32, super::SENS_SAR_TOUCH_CTRL1_REG>;
#[doc = "Register SENS_SAR_TOUCH_CTRL1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_TOUCH_CTRL1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_HALL_PHASE_FORCE`"]
pub type SENS_HALL_PHASE_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_HALL_PHASE_FORCE`"]
pub struct SENS_HALL_PHASE_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_HALL_PHASE_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SENS_XPD_HALL_FORCE`"]
pub type SENS_XPD_HALL_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_XPD_HALL_FORCE`"]
pub struct SENS_XPD_HALL_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_XPD_HALL_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_OUT_1EN`"]
pub type SENS_TOUCH_OUT_1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_OUT_1EN`"]
pub struct SENS_TOUCH_OUT_1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_OUT_1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_OUT_SEL`"]
pub type SENS_TOUCH_OUT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_OUT_SEL`"]
pub struct SENS_TOUCH_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_OUT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_XPD_WAIT`"]
pub type SENS_TOUCH_XPD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_TOUCH_XPD_WAIT`"]
pub struct SENS_TOUCH_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_DELAY`"]
pub type SENS_TOUCH_MEAS_DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_DELAY`"]
pub struct SENS_TOUCH_MEAS_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_hall_phase_force(&self) -> SENS_HALL_PHASE_FORCE_R {
        SENS_HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_xpd_hall_force(&self) -> SENS_XPD_HALL_FORCE_R {
        SENS_XPD_HALL_FORCE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn sens_touch_out_1en(&self) -> SENS_TOUCH_OUT_1EN_R {
        SENS_TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn sens_touch_out_sel(&self) -> SENS_TOUCH_OUT_SEL_R {
        SENS_TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn sens_touch_xpd_wait(&self) -> SENS_TOUCH_XPD_WAIT_R {
        SENS_TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn sens_touch_meas_delay(&self) -> SENS_TOUCH_MEAS_DELAY_R {
        SENS_TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_hall_phase_force(&mut self) -> SENS_HALL_PHASE_FORCE_W {
        SENS_HALL_PHASE_FORCE_W { w: self }
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_xpd_hall_force(&mut self) -> SENS_XPD_HALL_FORCE_W {
        SENS_XPD_HALL_FORCE_W { w: self }
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn sens_touch_out_1en(&mut self) -> SENS_TOUCH_OUT_1EN_W {
        SENS_TOUCH_OUT_1EN_W { w: self }
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn sens_touch_out_sel(&mut self) -> SENS_TOUCH_OUT_SEL_W {
        SENS_TOUCH_OUT_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn sens_touch_xpd_wait(&mut self) -> SENS_TOUCH_XPD_WAIT_W {
        SENS_TOUCH_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn sens_touch_meas_delay(&mut self) -> SENS_TOUCH_MEAS_DELAY_W {
        SENS_TOUCH_MEAS_DELAY_W { w: self }
    }
}
