#[doc = "Reader of register PWC"]
pub type R = crate::R<u32, super::PWC>;
#[doc = "Writer for register PWC"]
pub type W = crate::W<u32, super::PWC>;
#[doc = "Register PWC `reset()`'s with value 0"]
impl crate::ResetValue for super::PWC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_PD_EN`"]
pub type RTC_CNTL_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PD_EN`"]
pub struct RTC_CNTL_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FORCE_PU`"]
pub type RTC_CNTL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FORCE_PU`"]
pub struct RTC_CNTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_FORCE_PD`"]
pub type RTC_CNTL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FORCE_PD`"]
pub struct RTC_CNTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_PD_EN`"]
pub type RTC_CNTL_SLOWMEM_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_PD_EN`"]
pub struct RTC_CNTL_SLOWMEM_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_PU`"]
pub type RTC_CNTL_SLOWMEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_PU`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_PD`"]
pub type RTC_CNTL_SLOWMEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_PD`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_FASTMEM_PD_EN`"]
pub type RTC_CNTL_FASTMEM_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_PD_EN`"]
pub struct RTC_CNTL_FASTMEM_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_PU`"]
pub type RTC_CNTL_FASTMEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_PU`"]
pub struct RTC_CNTL_FASTMEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_PD`"]
pub type RTC_CNTL_FASTMEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_PD`"]
pub struct RTC_CNTL_FASTMEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_LPU`"]
pub type RTC_CNTL_SLOWMEM_FORCE_LPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_LPU`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_LPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_LPD`"]
pub type RTC_CNTL_SLOWMEM_FORCE_LPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_LPD`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_LPD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FOLW_CPU`"]
pub type RTC_CNTL_SLOWMEM_FOLW_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FOLW_CPU`"]
pub struct RTC_CNTL_SLOWMEM_FOLW_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FOLW_CPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_LPU`"]
pub type RTC_CNTL_FASTMEM_FORCE_LPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_LPU`"]
pub struct RTC_CNTL_FASTMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_LPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_LPD`"]
pub type RTC_CNTL_FASTMEM_FORCE_LPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_LPD`"]
pub struct RTC_CNTL_FASTMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_LPD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FOLW_CPU`"]
pub type RTC_CNTL_FASTMEM_FOLW_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FOLW_CPU`"]
pub struct RTC_CNTL_FASTMEM_FOLW_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FOLW_CPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FORCE_NOISO`"]
pub type RTC_CNTL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FORCE_NOISO`"]
pub struct RTC_CNTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FORCE_ISO`"]
pub type RTC_CNTL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FORCE_ISO`"]
pub struct RTC_CNTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_ISO`"]
pub type RTC_CNTL_SLOWMEM_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_ISO`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLOWMEM_FORCE_NOISO`"]
pub type RTC_CNTL_SLOWMEM_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLOWMEM_FORCE_NOISO`"]
pub struct RTC_CNTL_SLOWMEM_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLOWMEM_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_ISO`"]
pub type RTC_CNTL_FASTMEM_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_ISO`"]
pub struct RTC_CNTL_FASTMEM_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_NOISO`"]
pub type RTC_CNTL_FASTMEM_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_NOISO`"]
pub struct RTC_CNTL_FASTMEM_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_NOISO_W<'a> {
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
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_pd_en(&self) -> RTC_CNTL_PD_EN_R {
        RTC_CNTL_PD_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn rtc_cntl_force_pu(&self) -> RTC_CNTL_FORCE_PU_R {
        RTC_CNTL_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn rtc_cntl_force_pd(&self) -> RTC_CNTL_FORCE_PD_R {
        RTC_CNTL_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_pd_en(&self) -> RTC_CNTL_SLOWMEM_PD_EN_R {
        RTC_CNTL_SLOWMEM_PD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_pu(&self) -> RTC_CNTL_SLOWMEM_FORCE_PU_R {
        RTC_CNTL_SLOWMEM_FORCE_PU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_pd(&self) -> RTC_CNTL_SLOWMEM_FORCE_PD_R {
        RTC_CNTL_SLOWMEM_FORCE_PD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_pd_en(&self) -> RTC_CNTL_FASTMEM_PD_EN_R {
        RTC_CNTL_FASTMEM_PD_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_pu(&self) -> RTC_CNTL_FASTMEM_FORCE_PU_R {
        RTC_CNTL_FASTMEM_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_pd(&self) -> RTC_CNTL_FASTMEM_FORCE_PD_R {
        RTC_CNTL_FASTMEM_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_lpu(&self) -> RTC_CNTL_SLOWMEM_FORCE_LPU_R {
        RTC_CNTL_SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_lpd(&self) -> RTC_CNTL_SLOWMEM_FORCE_LPD_R {
        RTC_CNTL_SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_folw_cpu(&self) -> RTC_CNTL_SLOWMEM_FOLW_CPU_R {
        RTC_CNTL_SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpu(&self) -> RTC_CNTL_FASTMEM_FORCE_LPU_R {
        RTC_CNTL_FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpd(&self) -> RTC_CNTL_FASTMEM_FORCE_LPD_R {
        RTC_CNTL_FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_folw_cpu(&self) -> RTC_CNTL_FASTMEM_FOLW_CPU_R {
        RTC_CNTL_FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_force_noiso(&self) -> RTC_CNTL_FORCE_NOISO_R {
        RTC_CNTL_FORCE_NOISO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_force_iso(&self) -> RTC_CNTL_FORCE_ISO_R {
        RTC_CNTL_FORCE_ISO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_iso(&self) -> RTC_CNTL_SLOWMEM_FORCE_ISO_R {
        RTC_CNTL_SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_noiso(&self) -> RTC_CNTL_SLOWMEM_FORCE_NOISO_R {
        RTC_CNTL_SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_iso(&self) -> RTC_CNTL_FASTMEM_FORCE_ISO_R {
        RTC_CNTL_FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_noiso(&self) -> RTC_CNTL_FASTMEM_FORCE_NOISO_R {
        RTC_CNTL_FASTMEM_FORCE_NOISO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_pd_en(&mut self) -> RTC_CNTL_PD_EN_W {
        RTC_CNTL_PD_EN_W { w: self }
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn rtc_cntl_force_pu(&mut self) -> RTC_CNTL_FORCE_PU_W {
        RTC_CNTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn rtc_cntl_force_pd(&mut self) -> RTC_CNTL_FORCE_PD_W {
        RTC_CNTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_pd_en(&mut self) -> RTC_CNTL_SLOWMEM_PD_EN_W {
        RTC_CNTL_SLOWMEM_PD_EN_W { w: self }
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_pu(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_PU_W {
        RTC_CNTL_SLOWMEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_pd(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_PD_W {
        RTC_CNTL_SLOWMEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_pd_en(&mut self) -> RTC_CNTL_FASTMEM_PD_EN_W {
        RTC_CNTL_FASTMEM_PD_EN_W { w: self }
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_pu(&mut self) -> RTC_CNTL_FASTMEM_FORCE_PU_W {
        RTC_CNTL_FASTMEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_pd(&mut self) -> RTC_CNTL_FASTMEM_FORCE_PD_W {
        RTC_CNTL_FASTMEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_lpu(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_LPU_W {
        RTC_CNTL_SLOWMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_lpd(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_LPD_W {
        RTC_CNTL_SLOWMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_folw_cpu(&mut self) -> RTC_CNTL_SLOWMEM_FOLW_CPU_W {
        RTC_CNTL_SLOWMEM_FOLW_CPU_W { w: self }
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpu(&mut self) -> RTC_CNTL_FASTMEM_FORCE_LPU_W {
        RTC_CNTL_FASTMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpd(&mut self) -> RTC_CNTL_FASTMEM_FORCE_LPD_W {
        RTC_CNTL_FASTMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_folw_cpu(&mut self) -> RTC_CNTL_FASTMEM_FOLW_CPU_W {
        RTC_CNTL_FASTMEM_FOLW_CPU_W { w: self }
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_force_noiso(&mut self) -> RTC_CNTL_FORCE_NOISO_W {
        RTC_CNTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_force_iso(&mut self) -> RTC_CNTL_FORCE_ISO_W {
        RTC_CNTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_iso(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_ISO_W {
        RTC_CNTL_SLOWMEM_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_slowmem_force_noiso(&mut self) -> RTC_CNTL_SLOWMEM_FORCE_NOISO_W {
        RTC_CNTL_SLOWMEM_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_iso(&mut self) -> RTC_CNTL_FASTMEM_FORCE_ISO_W {
        RTC_CNTL_FASTMEM_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_noiso(&mut self) -> RTC_CNTL_FASTMEM_FORCE_NOISO_W {
        RTC_CNTL_FASTMEM_FORCE_NOISO_W { w: self }
    }
}
