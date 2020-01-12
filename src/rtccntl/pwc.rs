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
#[doc = "Reader of field `PD_EN`"]
pub type PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_EN`"]
pub struct PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_EN_W<'a> {
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
#[doc = "Reader of field `FORCE_PU`"]
pub type FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_PU`"]
pub struct FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PU_W<'a> {
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
#[doc = "Reader of field `FORCE_PD`"]
pub type FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_PD`"]
pub struct FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_PD_EN`"]
pub type SLOWMEM_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_PD_EN`"]
pub struct SLOWMEM_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_PD_EN_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_PU`"]
pub type SLOWMEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_PU`"]
pub struct SLOWMEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_PD`"]
pub type SLOWMEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_PD`"]
pub struct SLOWMEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `FASTMEM_PD_EN`"]
pub type FASTMEM_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_PD_EN`"]
pub struct FASTMEM_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_PD_EN_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_PU`"]
pub type FASTMEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_PU`"]
pub struct FASTMEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_PD`"]
pub type FASTMEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_PD`"]
pub struct FASTMEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_LPU`"]
pub type SLOWMEM_FORCE_LPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_LPU`"]
pub struct SLOWMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_LPU_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_LPD`"]
pub type SLOWMEM_FORCE_LPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_LPD`"]
pub struct SLOWMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_LPD_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FOLW_CPU`"]
pub type SLOWMEM_FOLW_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FOLW_CPU`"]
pub struct SLOWMEM_FOLW_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FOLW_CPU_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_LPU`"]
pub type FASTMEM_FORCE_LPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_LPU`"]
pub struct FASTMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_LPU_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_LPD`"]
pub type FASTMEM_FORCE_LPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_LPD`"]
pub struct FASTMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_LPD_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FOLW_CPU`"]
pub type FASTMEM_FOLW_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FOLW_CPU`"]
pub struct FASTMEM_FOLW_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FOLW_CPU_W<'a> {
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
#[doc = "Reader of field `FORCE_NOISO`"]
pub type FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_NOISO`"]
pub struct FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `FORCE_ISO`"]
pub type FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_ISO`"]
pub struct FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_ISO`"]
pub type SLOWMEM_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_ISO`"]
pub struct SLOWMEM_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `SLOWMEM_FORCE_NOISO`"]
pub type SLOWMEM_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOWMEM_FORCE_NOISO`"]
pub struct SLOWMEM_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWMEM_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_ISO`"]
pub type FASTMEM_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_ISO`"]
pub struct FASTMEM_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `FASTMEM_FORCE_NOISO`"]
pub type FASTMEM_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTMEM_FORCE_NOISO`"]
pub struct FASTMEM_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_NOISO_W<'a> {
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
    pub fn pd_en(&self) -> PD_EN_R {
        PD_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    pub fn slowmem_pd_en(&self) -> SLOWMEM_PD_EN_R {
        SLOWMEM_PD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    pub fn slowmem_force_pu(&self) -> SLOWMEM_FORCE_PU_R {
        SLOWMEM_FORCE_PU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    pub fn slowmem_force_pd(&self) -> SLOWMEM_FORCE_PD_R {
        SLOWMEM_FORCE_PD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    pub fn fastmem_pd_en(&self) -> FASTMEM_PD_EN_R {
        FASTMEM_PD_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    pub fn fastmem_force_pu(&self) -> FASTMEM_FORCE_PU_R {
        FASTMEM_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    pub fn fastmem_force_pd(&self) -> FASTMEM_FORCE_PD_R {
        FASTMEM_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn slowmem_force_lpu(&self) -> SLOWMEM_FORCE_LPU_R {
        SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn slowmem_force_lpd(&self) -> SLOWMEM_FORCE_LPD_R {
        SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn slowmem_folw_cpu(&self) -> SLOWMEM_FOLW_CPU_R {
        SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn fastmem_folw_cpu(&self) -> FASTMEM_FOLW_CPU_R {
        FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn force_noiso(&self) -> FORCE_NOISO_R {
        FORCE_NOISO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn force_iso(&self) -> FORCE_ISO_R {
        FORCE_ISO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn slowmem_force_iso(&self) -> SLOWMEM_FORCE_ISO_R {
        SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn slowmem_force_noiso(&self) -> SLOWMEM_FORCE_NOISO_R {
        SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn fastmem_force_iso(&self) -> FASTMEM_FORCE_ISO_R {
        FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn fastmem_force_noiso(&self) -> FASTMEM_FORCE_NOISO_R {
        FASTMEM_FORCE_NOISO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn pd_en(&mut self) -> PD_EN_W {
        PD_EN_W { w: self }
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn force_pu(&mut self) -> FORCE_PU_W {
        FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn force_pd(&mut self) -> FORCE_PD_W {
        FORCE_PD_W { w: self }
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    pub fn slowmem_pd_en(&mut self) -> SLOWMEM_PD_EN_W {
        SLOWMEM_PD_EN_W { w: self }
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    pub fn slowmem_force_pu(&mut self) -> SLOWMEM_FORCE_PU_W {
        SLOWMEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    pub fn slowmem_force_pd(&mut self) -> SLOWMEM_FORCE_PD_W {
        SLOWMEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    pub fn fastmem_pd_en(&mut self) -> FASTMEM_PD_EN_W {
        FASTMEM_PD_EN_W { w: self }
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    pub fn fastmem_force_pu(&mut self) -> FASTMEM_FORCE_PU_W {
        FASTMEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    pub fn fastmem_force_pd(&mut self) -> FASTMEM_FORCE_PD_W {
        FASTMEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn slowmem_force_lpu(&mut self) -> SLOWMEM_FORCE_LPU_W {
        SLOWMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn slowmem_force_lpd(&mut self) -> SLOWMEM_FORCE_LPD_W {
        SLOWMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn slowmem_folw_cpu(&mut self) -> SLOWMEM_FOLW_CPU_W {
        SLOWMEM_FOLW_CPU_W { w: self }
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W {
        FASTMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W {
        FASTMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn fastmem_folw_cpu(&mut self) -> FASTMEM_FOLW_CPU_W {
        FASTMEM_FOLW_CPU_W { w: self }
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn force_noiso(&mut self) -> FORCE_NOISO_W {
        FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn force_iso(&mut self) -> FORCE_ISO_W {
        FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn slowmem_force_iso(&mut self) -> SLOWMEM_FORCE_ISO_W {
        SLOWMEM_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn slowmem_force_noiso(&mut self) -> SLOWMEM_FORCE_NOISO_W {
        SLOWMEM_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn fastmem_force_iso(&mut self) -> FASTMEM_FORCE_ISO_W {
        FASTMEM_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn fastmem_force_noiso(&mut self) -> FASTMEM_FORCE_NOISO_W {
        FASTMEM_FORCE_NOISO_W { w: self }
    }
}
