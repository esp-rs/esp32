#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Writer for register INT_ST"]
pub type W = crate::W<u32, super::INT_ST>;
#[doc = "Register INT_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAIN_TIMER_INT_ST`"]
pub type MAIN_TIMER_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_TIMER_INT_ST`"]
pub struct MAIN_TIMER_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BROWN_OUT_INT_ST`"]
pub type BROWN_OUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_INT_ST`"]
pub struct BROWN_OUT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_INT_ST`"]
pub type TOUCH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_INT_ST`"]
pub struct TOUCH_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SAR_INT_ST`"]
pub type SAR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR_INT_ST`"]
pub struct SAR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIME_VALID_INT_ST`"]
pub type TIME_VALID_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_VALID_INT_ST`"]
pub struct TIME_VALID_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_VALID_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WDT_INT_ST`"]
pub type WDT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_INT_ST`"]
pub struct WDT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SDIO_IDLE_INT_ST`"]
pub type SDIO_IDLE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_IDLE_INT_ST`"]
pub struct SDIO_IDLE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IDLE_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SLP_REJECT_INT_ST`"]
pub type SLP_REJECT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_REJECT_INT_ST`"]
pub struct SLP_REJECT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SLP_WAKEUP_INT_ST`"]
pub type SLP_WAKEUP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_WAKEUP_INT_ST`"]
pub struct SLP_WAKEUP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_ST_W<'a> {
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
    #[doc = "Bit 8 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_st(&self) -> MAIN_TIMER_INT_ST_R {
        MAIN_TIMER_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - touch interrupt state"]
    #[inline(always)]
    pub fn touch_int_st(&self) -> TOUCH_INT_ST_R {
        TOUCH_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn sar_int_st(&self) -> SAR_INT_ST_R {
        SAR_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC time valid interrupt state"]
    #[inline(always)]
    pub fn time_valid_int_st(&self) -> TIME_VALID_INT_ST_R {
        TIME_VALID_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_st(&self) -> SDIO_IDLE_INT_ST_R {
        SDIO_IDLE_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_st(&self) -> SLP_REJECT_INT_ST_R {
        SLP_REJECT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&self) -> SLP_WAKEUP_INT_ST_R {
        SLP_WAKEUP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_st(&mut self) -> MAIN_TIMER_INT_ST_W {
        MAIN_TIMER_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_st(&mut self) -> BROWN_OUT_INT_ST_W {
        BROWN_OUT_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - touch interrupt state"]
    #[inline(always)]
    pub fn touch_int_st(&mut self) -> TOUCH_INT_ST_W {
        TOUCH_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn sar_int_st(&mut self) -> SAR_INT_ST_W {
        SAR_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - RTC time valid interrupt state"]
    #[inline(always)]
    pub fn time_valid_int_st(&mut self) -> TIME_VALID_INT_ST_W {
        TIME_VALID_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_st(&mut self) -> WDT_INT_ST_W {
        WDT_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_st(&mut self) -> SDIO_IDLE_INT_ST_W {
        SDIO_IDLE_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_st(&mut self) -> SLP_REJECT_INT_ST_W {
        SLP_REJECT_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&mut self) -> SLP_WAKEUP_INT_ST_W {
        SLP_WAKEUP_INT_ST_W { w: self }
    }
}
