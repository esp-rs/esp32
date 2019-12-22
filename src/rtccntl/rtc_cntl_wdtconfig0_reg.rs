#[doc = "Reader of register RTC_CNTL_WDTCONFIG0_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_WDTCONFIG0_REG>;
#[doc = "Writer for register RTC_CNTL_WDTCONFIG0_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_WDTCONFIG0_REG>;
#[doc = "Register RTC_CNTL_WDTCONFIG0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_WDTCONFIG0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_EN`"]
pub type RTC_CNTL_WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_EN`"]
pub struct RTC_CNTL_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_STG0`"]
pub type RTC_CNTL_WDT_STG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG0`"]
pub struct RTC_CNTL_WDT_STG0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG1`"]
pub type RTC_CNTL_WDT_STG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG1`"]
pub struct RTC_CNTL_WDT_STG1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG2`"]
pub type RTC_CNTL_WDT_STG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG2`"]
pub struct RTC_CNTL_WDT_STG2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG3`"]
pub type RTC_CNTL_WDT_STG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG3`"]
pub struct RTC_CNTL_WDT_STG3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_EDGE_INT_EN`"]
pub type RTC_CNTL_WDT_EDGE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_EDGE_INT_EN`"]
pub struct RTC_CNTL_WDT_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_EDGE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_LEVEL_INT_EN`"]
pub type RTC_CNTL_WDT_LEVEL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_LEVEL_INT_EN`"]
pub struct RTC_CNTL_WDT_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_LEVEL_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_CPU_RESET_LENGTH`"]
pub type RTC_CNTL_WDT_CPU_RESET_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_CPU_RESET_LENGTH`"]
pub struct RTC_CNTL_WDT_CPU_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_CPU_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_SYS_RESET_LENGTH`"]
pub type RTC_CNTL_WDT_SYS_RESET_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_SYS_RESET_LENGTH`"]
pub struct RTC_CNTL_WDT_SYS_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_SYS_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_FLASHBOOT_MOD_EN`"]
pub type RTC_CNTL_WDT_FLASHBOOT_MOD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_FLASHBOOT_MOD_EN`"]
pub struct RTC_CNTL_WDT_FLASHBOOT_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_FLASHBOOT_MOD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_PROCPU_RESET_EN`"]
pub type RTC_CNTL_WDT_PROCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_PROCPU_RESET_EN`"]
pub struct RTC_CNTL_WDT_PROCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_PROCPU_RESET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_APPCPU_RESET_EN`"]
pub type RTC_CNTL_WDT_APPCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_APPCPU_RESET_EN`"]
pub struct RTC_CNTL_WDT_APPCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_APPCPU_RESET_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_PAUSE_IN_SLP`"]
pub type RTC_CNTL_WDT_PAUSE_IN_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_PAUSE_IN_SLP`"]
pub struct RTC_CNTL_WDT_PAUSE_IN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_PAUSE_IN_SLP_W<'a> {
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
impl R {
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_en(&self) -> RTC_CNTL_WDT_EN_R {
        RTC_CNTL_WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg0(&self) -> RTC_CNTL_WDT_STG0_R {
        RTC_CNTL_WDT_STG0_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg1(&self) -> RTC_CNTL_WDT_STG1_R {
        RTC_CNTL_WDT_STG1_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg2(&self) -> RTC_CNTL_WDT_STG2_R {
        RTC_CNTL_WDT_STG2_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg3(&self) -> RTC_CNTL_WDT_STG3_R {
        RTC_CNTL_WDT_STG3_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_edge_int_en(&self) -> RTC_CNTL_WDT_EDGE_INT_EN_R {
        RTC_CNTL_WDT_EDGE_INT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_level_int_en(&self) -> RTC_CNTL_WDT_LEVEL_INT_EN_R {
        RTC_CNTL_WDT_LEVEL_INT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_cpu_reset_length(&self) -> RTC_CNTL_WDT_CPU_RESET_LENGTH_R {
        RTC_CNTL_WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_sys_reset_length(&self) -> RTC_CNTL_WDT_SYS_RESET_LENGTH_R {
        RTC_CNTL_WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_flashboot_mod_en(&self) -> RTC_CNTL_WDT_FLASHBOOT_MOD_EN_R {
        RTC_CNTL_WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_procpu_reset_en(&self) -> RTC_CNTL_WDT_PROCPU_RESET_EN_R {
        RTC_CNTL_WDT_PROCPU_RESET_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_appcpu_reset_en(&self) -> RTC_CNTL_WDT_APPCPU_RESET_EN_R {
        RTC_CNTL_WDT_APPCPU_RESET_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_pause_in_slp(&self) -> RTC_CNTL_WDT_PAUSE_IN_SLP_R {
        RTC_CNTL_WDT_PAUSE_IN_SLP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_en(&mut self) -> RTC_CNTL_WDT_EN_W {
        RTC_CNTL_WDT_EN_W { w: self }
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg0(&mut self) -> RTC_CNTL_WDT_STG0_W {
        RTC_CNTL_WDT_STG0_W { w: self }
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg1(&mut self) -> RTC_CNTL_WDT_STG1_W {
        RTC_CNTL_WDT_STG1_W { w: self }
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg2(&mut self) -> RTC_CNTL_WDT_STG2_W {
        RTC_CNTL_WDT_STG2_W { w: self }
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg3(&mut self) -> RTC_CNTL_WDT_STG3_W {
        RTC_CNTL_WDT_STG3_W { w: self }
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_edge_int_en(&mut self) -> RTC_CNTL_WDT_EDGE_INT_EN_W {
        RTC_CNTL_WDT_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_level_int_en(&mut self) -> RTC_CNTL_WDT_LEVEL_INT_EN_W {
        RTC_CNTL_WDT_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_cpu_reset_length(&mut self) -> RTC_CNTL_WDT_CPU_RESET_LENGTH_W {
        RTC_CNTL_WDT_CPU_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_sys_reset_length(&mut self) -> RTC_CNTL_WDT_SYS_RESET_LENGTH_W {
        RTC_CNTL_WDT_SYS_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_flashboot_mod_en(&mut self) -> RTC_CNTL_WDT_FLASHBOOT_MOD_EN_W {
        RTC_CNTL_WDT_FLASHBOOT_MOD_EN_W { w: self }
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_procpu_reset_en(&mut self) -> RTC_CNTL_WDT_PROCPU_RESET_EN_W {
        RTC_CNTL_WDT_PROCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_appcpu_reset_en(&mut self) -> RTC_CNTL_WDT_APPCPU_RESET_EN_W {
        RTC_CNTL_WDT_APPCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_pause_in_slp(&mut self) -> RTC_CNTL_WDT_PAUSE_IN_SLP_W {
        RTC_CNTL_WDT_PAUSE_IN_SLP_W { w: self }
    }
}