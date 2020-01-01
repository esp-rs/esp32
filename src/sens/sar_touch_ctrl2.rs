#[doc = "Reader of register SAR_TOUCH_CTRL2"]
pub type R = crate::R<u32, super::SAR_TOUCH_CTRL2>;
#[doc = "Writer for register SAR_TOUCH_CTRL2"]
pub type W = crate::W<u32, super::SAR_TOUCH_CTRL2>;
#[doc = "Register SAR_TOUCH_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_EN_CLR`"]
pub type SENS_TOUCH_MEAS_EN_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_EN_CLR`"]
pub struct SENS_TOUCH_MEAS_EN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_EN_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_SLEEP_CYCLES`"]
pub type SENS_TOUCH_SLEEP_CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_SLEEP_CYCLES`"]
pub struct SENS_TOUCH_SLEEP_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_SLEEP_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | (((value as u32) & 0xffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_START_FORCE`"]
pub type SENS_TOUCH_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_START_FORCE`"]
pub struct SENS_TOUCH_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_START_FORCE_W<'a> {
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
#[doc = "Reader of field `SENS_TOUCH_START_EN`"]
pub type SENS_TOUCH_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_START_EN`"]
pub struct SENS_TOUCH_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_START_EN_W<'a> {
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
#[doc = "Reader of field `SENS_TOUCH_START_FSM_EN`"]
pub type SENS_TOUCH_START_FSM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_START_FSM_EN`"]
pub struct SENS_TOUCH_START_FSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_START_FSM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_DONE`"]
pub type SENS_TOUCH_MEAS_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_DONE`"]
pub struct SENS_TOUCH_MEAS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_EN`"]
pub type SENS_TOUCH_MEAS_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_EN`"]
pub struct SENS_TOUCH_MEAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - to clear reg_touch_meas_en"]
    #[inline(always)]
    pub fn sens_touch_meas_en_clr(&self) -> SENS_TOUCH_MEAS_EN_CLR_R {
        SENS_TOUCH_MEAS_EN_CLR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn sens_touch_sleep_cycles(&self) -> SENS_TOUCH_SLEEP_CYCLES_R {
        SENS_TOUCH_SLEEP_CYCLES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn sens_touch_start_force(&self) -> SENS_TOUCH_START_FORCE_R {
        SENS_TOUCH_START_FORCE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn sens_touch_start_en(&self) -> SENS_TOUCH_START_EN_R {
        SENS_TOUCH_START_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn sens_touch_start_fsm_en(&self) -> SENS_TOUCH_START_FSM_EN_R {
        SENS_TOUCH_START_FSM_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - fsm set 1 to indicate touch touch meas is done"]
    #[inline(always)]
    pub fn sens_touch_meas_done(&self) -> SENS_TOUCH_MEAS_DONE_R {
        SENS_TOUCH_MEAS_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - 10-bit register to indicate which pads are \"touched\""]
    #[inline(always)]
    pub fn sens_touch_meas_en(&self) -> SENS_TOUCH_MEAS_EN_R {
        SENS_TOUCH_MEAS_EN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 30 - to clear reg_touch_meas_en"]
    #[inline(always)]
    pub fn sens_touch_meas_en_clr(&mut self) -> SENS_TOUCH_MEAS_EN_CLR_W {
        SENS_TOUCH_MEAS_EN_CLR_W { w: self }
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn sens_touch_sleep_cycles(&mut self) -> SENS_TOUCH_SLEEP_CYCLES_W {
        SENS_TOUCH_SLEEP_CYCLES_W { w: self }
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn sens_touch_start_force(&mut self) -> SENS_TOUCH_START_FORCE_W {
        SENS_TOUCH_START_FORCE_W { w: self }
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn sens_touch_start_en(&mut self) -> SENS_TOUCH_START_EN_W {
        SENS_TOUCH_START_EN_W { w: self }
    }
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn sens_touch_start_fsm_en(&mut self) -> SENS_TOUCH_START_FSM_EN_W {
        SENS_TOUCH_START_FSM_EN_W { w: self }
    }
    #[doc = "Bit 10 - fsm set 1 to indicate touch touch meas is done"]
    #[inline(always)]
    pub fn sens_touch_meas_done(&mut self) -> SENS_TOUCH_MEAS_DONE_W {
        SENS_TOUCH_MEAS_DONE_W { w: self }
    }
    #[doc = "Bits 0:9 - 10-bit register to indicate which pads are \"touched\""]
    #[inline(always)]
    pub fn sens_touch_meas_en(&mut self) -> SENS_TOUCH_MEAS_EN_W {
        SENS_TOUCH_MEAS_EN_W { w: self }
    }
}
