#[doc = "Reader of register SAR_TOUCH_CTRL1"]
pub type R = crate::R<u32, super::SAR_TOUCH_CTRL1>;
#[doc = "Writer for register SAR_TOUCH_CTRL1"]
pub type W = crate::W<u32, super::SAR_TOUCH_CTRL1>;
#[doc = "Register SAR_TOUCH_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HALL_PHASE_FORCE`"]
pub type HALL_PHASE_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALL_PHASE_FORCE`"]
pub struct HALL_PHASE_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALL_PHASE_FORCE_W<'a> {
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
#[doc = "Reader of field `XPD_HALL_FORCE`"]
pub type XPD_HALL_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XPD_HALL_FORCE`"]
pub struct XPD_HALL_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_HALL_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_OUT_1EN`"]
pub type TOUCH_OUT_1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_OUT_1EN`"]
pub struct TOUCH_OUT_1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_1EN_W<'a> {
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
#[doc = "Reader of field `TOUCH_OUT_SEL`"]
pub type TOUCH_OUT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_OUT_SEL`"]
pub struct TOUCH_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_SEL_W<'a> {
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
#[doc = "Reader of field `TOUCH_XPD_WAIT`"]
pub type TOUCH_XPD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUCH_XPD_WAIT`"]
pub struct TOUCH_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_MEAS_DELAY`"]
pub type TOUCH_MEAS_DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_MEAS_DELAY`"]
pub struct TOUCH_MEAS_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_DELAY_W<'a> {
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
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&self) -> TOUCH_OUT_1EN_R {
        TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&self) -> TOUCH_MEAS_DELAY_R {
        TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W {
        HALL_PHASE_FORCE_W { w: self }
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W {
        XPD_HALL_FORCE_W { w: self }
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&mut self) -> TOUCH_OUT_1EN_W {
        TOUCH_OUT_1EN_W { w: self }
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W {
        TOUCH_OUT_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W {
        TOUCH_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&mut self) -> TOUCH_MEAS_DELAY_W {
        TOUCH_MEAS_DELAY_W { w: self }
    }
}
