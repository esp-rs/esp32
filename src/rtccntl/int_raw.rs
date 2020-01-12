#[doc = "Reader of register INT_RAW"]
pub type R = crate::R<u32, super::INT_RAW>;
#[doc = "Writer for register INT_RAW"]
pub type W = crate::W<u32, super::INT_RAW>;
#[doc = "Register INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAIN_TIMER_INT_RAW`"]
pub type MAIN_TIMER_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_TIMER_INT_RAW`"]
pub struct MAIN_TIMER_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_RAW_W<'a> {
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
#[doc = "Reader of field `BROWN_OUT_INT_RAW`"]
pub type BROWN_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_INT_RAW`"]
pub struct BROWN_OUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TOUCH_INT_RAW`"]
pub type TOUCH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_INT_RAW`"]
pub struct TOUCH_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_INT_RAW_W<'a> {
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
#[doc = "Reader of field `ULP_CP_INT_RAW`"]
pub type ULP_CP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULP_CP_INT_RAW`"]
pub struct ULP_CP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TIME_VALID_INT_RAW`"]
pub type TIME_VALID_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_VALID_INT_RAW`"]
pub struct TIME_VALID_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_VALID_INT_RAW_W<'a> {
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
#[doc = "Reader of field `WDT_INT_RAW`"]
pub type WDT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_INT_RAW`"]
pub struct WDT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SDIO_IDLE_INT_RAW`"]
pub type SDIO_IDLE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_IDLE_INT_RAW`"]
pub struct SDIO_IDLE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IDLE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLP_REJECT_INT_RAW`"]
pub type SLP_REJECT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_REJECT_INT_RAW`"]
pub struct SLP_REJECT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLP_WAKEUP_INT_RAW`"]
pub type SLP_WAKEUP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_WAKEUP_INT_RAW`"]
pub struct SLP_WAKEUP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_RAW_W<'a> {
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
    #[doc = "Bit 8 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn touch_int_raw(&self) -> TOUCH_INT_RAW_R {
        TOUCH_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn ulp_cp_int_raw(&self) -> ULP_CP_INT_RAW_R {
        ULP_CP_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC time valid interrupt raw"]
    #[inline(always)]
    pub fn time_valid_int_raw(&self) -> TIME_VALID_INT_RAW_R {
        TIME_VALID_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&self) -> SDIO_IDLE_INT_RAW_R {
        SDIO_IDLE_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer_int_raw(&mut self) -> MAIN_TIMER_INT_RAW_W {
        MAIN_TIMER_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out_int_raw(&mut self) -> BROWN_OUT_INT_RAW_W {
        BROWN_OUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn touch_int_raw(&mut self) -> TOUCH_INT_RAW_W {
        TOUCH_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn ulp_cp_int_raw(&mut self) -> ULP_CP_INT_RAW_W {
        ULP_CP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - RTC time valid interrupt raw"]
    #[inline(always)]
    pub fn time_valid_int_raw(&mut self) -> TIME_VALID_INT_RAW_W {
        TIME_VALID_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt_int_raw(&mut self) -> WDT_INT_RAW_W {
        WDT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&mut self) -> SDIO_IDLE_INT_RAW_W {
        SDIO_IDLE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&mut self) -> SLP_REJECT_INT_RAW_W {
        SLP_REJECT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&mut self) -> SLP_WAKEUP_INT_RAW_W {
        SLP_WAKEUP_INT_RAW_W { w: self }
    }
}
