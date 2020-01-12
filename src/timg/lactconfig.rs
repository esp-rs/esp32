#[doc = "Reader of register LACTCONFIG"]
pub type R = crate::R<u32, super::LACTCONFIG>;
#[doc = "Writer for register LACTCONFIG"]
pub type W = crate::W<u32, super::LACTCONFIG>;
#[doc = "Register LACTCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_EN`"]
pub type LACT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_EN`"]
pub struct LACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_EN_W<'a> {
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
#[doc = "Reader of field `LACT_INCREASE`"]
pub type LACT_INCREASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_INCREASE`"]
pub struct LACT_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_INCREASE_W<'a> {
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
#[doc = "Reader of field `LACT_AUTORELOAD`"]
pub type LACT_AUTORELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_AUTORELOAD`"]
pub struct LACT_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_AUTORELOAD_W<'a> {
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
#[doc = "Reader of field `LACT_DIVIDER`"]
pub type LACT_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LACT_DIVIDER`"]
pub struct LACT_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | (((value as u32) & 0xffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `LACT_EDGE_INT_EN`"]
pub type LACT_EDGE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_EDGE_INT_EN`"]
pub struct LACT_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_EDGE_INT_EN_W<'a> {
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
#[doc = "Reader of field `LACT_LEVEL_INT_EN`"]
pub type LACT_LEVEL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_LEVEL_INT_EN`"]
pub struct LACT_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_LEVEL_INT_EN_W<'a> {
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
#[doc = "Reader of field `LACT_ALARM_EN`"]
pub type LACT_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_ALARM_EN`"]
pub struct LACT_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_ALARM_EN_W<'a> {
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
#[doc = "Reader of field `LACT_LAC_EN`"]
pub type LACT_LAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_LAC_EN`"]
pub struct LACT_LAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_LAC_EN_W<'a> {
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
#[doc = "Reader of field `LACT_CPST_EN`"]
pub type LACT_CPST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_CPST_EN`"]
pub struct LACT_CPST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_CPST_EN_W<'a> {
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
#[doc = "Reader of field `LACT_RTC_ONLY`"]
pub type LACT_RTC_ONLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_RTC_ONLY`"]
pub struct LACT_RTC_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_RTC_ONLY_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&self) -> LACT_EN_R {
        LACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&self) -> LACT_INCREASE_R {
        LACT_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&self) -> LACT_AUTORELOAD_R {
        LACT_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&self) -> LACT_DIVIDER_R {
        LACT_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&self) -> LACT_EDGE_INT_EN_R {
        LACT_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&self) -> LACT_LEVEL_INT_EN_R {
        LACT_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&self) -> LACT_ALARM_EN_R {
        LACT_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&self) -> LACT_LAC_EN_R {
        LACT_LAC_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&self) -> LACT_CPST_EN_R {
        LACT_CPST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&self) -> LACT_RTC_ONLY_R {
        LACT_RTC_ONLY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&mut self) -> LACT_EN_W {
        LACT_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&mut self) -> LACT_INCREASE_W {
        LACT_INCREASE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&mut self) -> LACT_AUTORELOAD_W {
        LACT_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&mut self) -> LACT_DIVIDER_W {
        LACT_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&mut self) -> LACT_EDGE_INT_EN_W {
        LACT_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&mut self) -> LACT_LEVEL_INT_EN_W {
        LACT_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&mut self) -> LACT_ALARM_EN_W {
        LACT_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&mut self) -> LACT_LAC_EN_W {
        LACT_LAC_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&mut self) -> LACT_CPST_EN_W {
        LACT_CPST_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&mut self) -> LACT_RTC_ONLY_W {
        LACT_RTC_ONLY_W { w: self }
    }
}
