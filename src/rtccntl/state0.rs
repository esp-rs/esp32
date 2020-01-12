#[doc = "Reader of register STATE0"]
pub type R = crate::R<u32, super::STATE0>;
#[doc = "Writer for register STATE0"]
pub type W = crate::W<u32, super::STATE0>;
#[doc = "Register STATE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STATE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLEEP_EN`"]
pub type SLEEP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEP_EN`"]
pub struct SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SLP_REJECT`"]
pub type SLP_REJECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_REJECT`"]
pub struct SLP_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_W<'a> {
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
#[doc = "Reader of field `SLP_WAKEUP`"]
pub type SLP_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_WAKEUP`"]
pub struct SLP_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SDIO_ACTIVE_IND`"]
pub type SDIO_ACTIVE_IND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_ACTIVE_IND`"]
pub struct SDIO_ACTIVE_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ACTIVE_IND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ULP_CP_SLP_TIMER_EN`"]
pub type ULP_CP_SLP_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULP_CP_SLP_TIMER_EN`"]
pub struct ULP_CP_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_SLP_TIMER_EN_W<'a> {
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
#[doc = "Reader of field `TOUCH_SLP_TIMER_EN`"]
pub type TOUCH_SLP_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_SLP_TIMER_EN`"]
pub struct TOUCH_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `APB2RTC_BRIDGE_SEL`"]
pub type APB2RTC_BRIDGE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB2RTC_BRIDGE_SEL`"]
pub struct APB2RTC_BRIDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2RTC_BRIDGE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ULP_CP_WAKEUP_FORCE_EN`"]
pub type ULP_CP_WAKEUP_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULP_CP_WAKEUP_FORCE_EN`"]
pub struct ULP_CP_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_WAKEUP_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_WAKEUP_FORCE_EN`"]
pub type TOUCH_WAKEUP_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_WAKEUP_FORCE_EN`"]
pub struct TOUCH_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_WAKEUP_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&self) -> TOUCH_SLP_TIMER_EN_R {
        TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn ulp_cp_wakeup_force_en(&self) -> ULP_CP_WAKEUP_FORCE_EN_R {
        ULP_CP_WAKEUP_FORCE_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn touch_wakeup_force_en(&self) -> TOUCH_WAKEUP_FORCE_EN_R {
        TOUCH_WAKEUP_FORCE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W {
        SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W {
        SLP_REJECT_W { w: self }
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W {
        SLP_WAKEUP_W { w: self }
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&mut self) -> SDIO_ACTIVE_IND_W {
        SDIO_ACTIVE_IND_W { w: self }
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W {
        ULP_CP_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&mut self) -> TOUCH_SLP_TIMER_EN_W {
        TOUCH_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W {
        APB2RTC_BRIDGE_SEL_W { w: self }
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn ulp_cp_wakeup_force_en(&mut self) -> ULP_CP_WAKEUP_FORCE_EN_W {
        ULP_CP_WAKEUP_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn touch_wakeup_force_en(&mut self) -> TOUCH_WAKEUP_FORCE_EN_W {
        TOUCH_WAKEUP_FORCE_EN_W { w: self }
    }
}
