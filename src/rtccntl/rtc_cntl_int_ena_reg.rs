#[doc = "Reader of register RTC_CNTL_INT_ENA_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_INT_ENA_REG>;
#[doc = "Writer for register RTC_CNTL_INT_ENA_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_INT_ENA_REG>;
#[doc = "Register RTC_CNTL_INT_ENA_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_INT_ENA_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_MAIN_TIMER_INT_ENA`"]
pub type RTC_CNTL_MAIN_TIMER_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_MAIN_TIMER_INT_ENA`"]
pub struct RTC_CNTL_MAIN_TIMER_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_MAIN_TIMER_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BROWN_OUT_INT_ENA`"]
pub type RTC_CNTL_BROWN_OUT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BROWN_OUT_INT_ENA`"]
pub struct RTC_CNTL_BROWN_OUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BROWN_OUT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_TOUCH_INT_ENA`"]
pub type RTC_CNTL_TOUCH_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TOUCH_INT_ENA`"]
pub struct RTC_CNTL_TOUCH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TOUCH_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ULP_CP_INT_ENA`"]
pub type RTC_CNTL_ULP_CP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ULP_CP_INT_ENA`"]
pub struct RTC_CNTL_ULP_CP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ULP_CP_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_TIME_VALID_INT_ENA`"]
pub type RTC_CNTL_TIME_VALID_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIME_VALID_INT_ENA`"]
pub struct RTC_CNTL_TIME_VALID_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIME_VALID_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_INT_ENA`"]
pub type RTC_CNTL_WDT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_INT_ENA`"]
pub struct RTC_CNTL_WDT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SDIO_IDLE_INT_ENA`"]
pub type RTC_CNTL_SDIO_IDLE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_IDLE_INT_ENA`"]
pub struct RTC_CNTL_SDIO_IDLE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_IDLE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_REJECT_INT_ENA`"]
pub type RTC_CNTL_SLP_REJECT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_REJECT_INT_ENA`"]
pub struct RTC_CNTL_SLP_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_REJECT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_WAKEUP_INT_ENA`"]
pub type RTC_CNTL_SLP_WAKEUP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_WAKEUP_INT_ENA`"]
pub struct RTC_CNTL_SLP_WAKEUP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_WAKEUP_INT_ENA_W<'a> {
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
    #[doc = "Bit 8 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_int_ena(&self) -> RTC_CNTL_MAIN_TIMER_INT_ENA_R {
        RTC_CNTL_MAIN_TIMER_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_brown_out_int_ena(&self) -> RTC_CNTL_BROWN_OUT_INT_ENA_R {
        RTC_CNTL_BROWN_OUT_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - enable touch interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_touch_int_ena(&self) -> RTC_CNTL_TOUCH_INT_ENA_R {
        RTC_CNTL_TOUCH_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_int_ena(&self) -> RTC_CNTL_ULP_CP_INT_ENA_R {
        RTC_CNTL_ULP_CP_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable RTC time valid interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_time_valid_int_ena(&self) -> RTC_CNTL_TIME_VALID_INT_ENA_R {
        RTC_CNTL_TIME_VALID_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_int_ena(&self) -> RTC_CNTL_WDT_INT_ENA_R {
        RTC_CNTL_WDT_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_idle_int_ena(&self) -> RTC_CNTL_SDIO_IDLE_INT_ENA_R {
        RTC_CNTL_SDIO_IDLE_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject_int_ena(&self) -> RTC_CNTL_SLP_REJECT_INT_ENA_R {
        RTC_CNTL_SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup_int_ena(&self) -> RTC_CNTL_SLP_WAKEUP_INT_ENA_R {
        RTC_CNTL_SLP_WAKEUP_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_int_ena(&mut self) -> RTC_CNTL_MAIN_TIMER_INT_ENA_W {
        RTC_CNTL_MAIN_TIMER_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_brown_out_int_ena(&mut self) -> RTC_CNTL_BROWN_OUT_INT_ENA_W {
        RTC_CNTL_BROWN_OUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - enable touch interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_touch_int_ena(&mut self) -> RTC_CNTL_TOUCH_INT_ENA_W {
        RTC_CNTL_TOUCH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_int_ena(&mut self) -> RTC_CNTL_ULP_CP_INT_ENA_W {
        RTC_CNTL_ULP_CP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - enable RTC time valid interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_time_valid_int_ena(&mut self) -> RTC_CNTL_TIME_VALID_INT_ENA_W {
        RTC_CNTL_TIME_VALID_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_int_ena(&mut self) -> RTC_CNTL_WDT_INT_ENA_W {
        RTC_CNTL_WDT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_idle_int_ena(&mut self) -> RTC_CNTL_SDIO_IDLE_INT_ENA_W {
        RTC_CNTL_SDIO_IDLE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject_int_ena(&mut self) -> RTC_CNTL_SLP_REJECT_INT_ENA_W {
        RTC_CNTL_SLP_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup_int_ena(&mut self) -> RTC_CNTL_SLP_WAKEUP_INT_ENA_W {
        RTC_CNTL_SLP_WAKEUP_INT_ENA_W { w: self }
    }
}