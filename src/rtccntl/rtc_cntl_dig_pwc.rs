#[doc = "Reader of register RTC_CNTL_DIG_PWC"]
pub type R = crate::R<u32, super::RTC_CNTL_DIG_PWC>;
#[doc = "Writer for register RTC_CNTL_DIG_PWC"]
pub type W = crate::W<u32, super::RTC_CNTL_DIG_PWC>;
#[doc = "Register RTC_CNTL_DIG_PWC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DIG_PWC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_PD_EN`"]
pub type RTC_CNTL_DG_WRAP_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_PD_EN`"]
pub struct RTC_CNTL_DG_WRAP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_PD_EN`"]
pub type RTC_CNTL_WIFI_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_PD_EN`"]
pub struct RTC_CNTL_WIFI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM4_PD_EN`"]
pub type RTC_CNTL_INTER_RAM4_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM4_PD_EN`"]
pub struct RTC_CNTL_INTER_RAM4_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM4_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM3_PD_EN`"]
pub type RTC_CNTL_INTER_RAM3_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM3_PD_EN`"]
pub struct RTC_CNTL_INTER_RAM3_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM3_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM2_PD_EN`"]
pub type RTC_CNTL_INTER_RAM2_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM2_PD_EN`"]
pub struct RTC_CNTL_INTER_RAM2_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM2_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM1_PD_EN`"]
pub type RTC_CNTL_INTER_RAM1_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM1_PD_EN`"]
pub struct RTC_CNTL_INTER_RAM1_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM1_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM0_PD_EN`"]
pub type RTC_CNTL_INTER_RAM0_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM0_PD_EN`"]
pub struct RTC_CNTL_INTER_RAM0_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM0_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ROM0_PD_EN`"]
pub type RTC_CNTL_ROM0_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ROM0_PD_EN`"]
pub struct RTC_CNTL_ROM0_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ROM0_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_PU`"]
pub type RTC_CNTL_DG_WRAP_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_PU`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_PD`"]
pub type RTC_CNTL_DG_WRAP_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_PD`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_PU`"]
pub type RTC_CNTL_WIFI_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_PU`"]
pub struct RTC_CNTL_WIFI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_PD`"]
pub type RTC_CNTL_WIFI_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_PD`"]
pub struct RTC_CNTL_WIFI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM4_FORCE_PU`"]
pub type RTC_CNTL_INTER_RAM4_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM4_FORCE_PU`"]
pub struct RTC_CNTL_INTER_RAM4_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM4_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM4_FORCE_PD`"]
pub type RTC_CNTL_INTER_RAM4_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM4_FORCE_PD`"]
pub struct RTC_CNTL_INTER_RAM4_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM4_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM3_FORCE_PU`"]
pub type RTC_CNTL_INTER_RAM3_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM3_FORCE_PU`"]
pub struct RTC_CNTL_INTER_RAM3_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM3_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM3_FORCE_PD`"]
pub type RTC_CNTL_INTER_RAM3_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM3_FORCE_PD`"]
pub struct RTC_CNTL_INTER_RAM3_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM3_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM2_FORCE_PU`"]
pub type RTC_CNTL_INTER_RAM2_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM2_FORCE_PU`"]
pub struct RTC_CNTL_INTER_RAM2_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM2_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM2_FORCE_PD`"]
pub type RTC_CNTL_INTER_RAM2_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM2_FORCE_PD`"]
pub struct RTC_CNTL_INTER_RAM2_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM2_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM1_FORCE_PU`"]
pub type RTC_CNTL_INTER_RAM1_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM1_FORCE_PU`"]
pub struct RTC_CNTL_INTER_RAM1_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM1_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM1_FORCE_PD`"]
pub type RTC_CNTL_INTER_RAM1_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM1_FORCE_PD`"]
pub struct RTC_CNTL_INTER_RAM1_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM1_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM0_FORCE_PU`"]
pub type RTC_CNTL_INTER_RAM0_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM0_FORCE_PU`"]
pub struct RTC_CNTL_INTER_RAM0_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM0_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INTER_RAM0_FORCE_PD`"]
pub type RTC_CNTL_INTER_RAM0_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INTER_RAM0_FORCE_PD`"]
pub struct RTC_CNTL_INTER_RAM0_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INTER_RAM0_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ROM0_FORCE_PU`"]
pub type RTC_CNTL_ROM0_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ROM0_FORCE_PU`"]
pub struct RTC_CNTL_ROM0_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ROM0_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ROM0_FORCE_PD`"]
pub type RTC_CNTL_ROM0_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ROM0_FORCE_PD`"]
pub struct RTC_CNTL_ROM0_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ROM0_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_LSLP_MEM_FORCE_PU`"]
pub type RTC_CNTL_LSLP_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LSLP_MEM_FORCE_PU`"]
pub struct RTC_CNTL_LSLP_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LSLP_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_LSLP_MEM_FORCE_PD`"]
pub type RTC_CNTL_LSLP_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LSLP_MEM_FORCE_PD`"]
pub struct RTC_CNTL_LSLP_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LSLP_MEM_FORCE_PD_W<'a> {
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
impl R {
    #[doc = "Bit 31 - enable power down digital core in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_pd_en(&self) -> RTC_CNTL_DG_WRAP_PD_EN_R {
        RTC_CNTL_DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_pd_en(&self) -> RTC_CNTL_WIFI_PD_EN_R {
        RTC_CNTL_WIFI_PD_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_pd_en(&self) -> RTC_CNTL_INTER_RAM4_PD_EN_R {
        RTC_CNTL_INTER_RAM4_PD_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_pd_en(&self) -> RTC_CNTL_INTER_RAM3_PD_EN_R {
        RTC_CNTL_INTER_RAM3_PD_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_pd_en(&self) -> RTC_CNTL_INTER_RAM2_PD_EN_R {
        RTC_CNTL_INTER_RAM2_PD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_pd_en(&self) -> RTC_CNTL_INTER_RAM1_PD_EN_R {
        RTC_CNTL_INTER_RAM1_PD_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_pd_en(&self) -> RTC_CNTL_INTER_RAM0_PD_EN_R {
        RTC_CNTL_INTER_RAM0_PD_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_pd_en(&self) -> RTC_CNTL_ROM0_PD_EN_R {
        RTC_CNTL_ROM0_PD_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pu(&self) -> RTC_CNTL_DG_WRAP_FORCE_PU_R {
        RTC_CNTL_DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pd(&self) -> RTC_CNTL_DG_WRAP_FORCE_PD_R {
        RTC_CNTL_DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pu(&self) -> RTC_CNTL_WIFI_FORCE_PU_R {
        RTC_CNTL_WIFI_FORCE_PU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pd(&self) -> RTC_CNTL_WIFI_FORCE_PD_R {
        RTC_CNTL_WIFI_FORCE_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_force_pu(&self) -> RTC_CNTL_INTER_RAM4_FORCE_PU_R {
        RTC_CNTL_INTER_RAM4_FORCE_PU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_force_pd(&self) -> RTC_CNTL_INTER_RAM4_FORCE_PD_R {
        RTC_CNTL_INTER_RAM4_FORCE_PD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_force_pu(&self) -> RTC_CNTL_INTER_RAM3_FORCE_PU_R {
        RTC_CNTL_INTER_RAM3_FORCE_PU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_force_pd(&self) -> RTC_CNTL_INTER_RAM3_FORCE_PD_R {
        RTC_CNTL_INTER_RAM3_FORCE_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_force_pu(&self) -> RTC_CNTL_INTER_RAM2_FORCE_PU_R {
        RTC_CNTL_INTER_RAM2_FORCE_PU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_force_pd(&self) -> RTC_CNTL_INTER_RAM2_FORCE_PD_R {
        RTC_CNTL_INTER_RAM2_FORCE_PD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_force_pu(&self) -> RTC_CNTL_INTER_RAM1_FORCE_PU_R {
        RTC_CNTL_INTER_RAM1_FORCE_PU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_force_pd(&self) -> RTC_CNTL_INTER_RAM1_FORCE_PD_R {
        RTC_CNTL_INTER_RAM1_FORCE_PD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_force_pu(&self) -> RTC_CNTL_INTER_RAM0_FORCE_PU_R {
        RTC_CNTL_INTER_RAM0_FORCE_PU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_force_pd(&self) -> RTC_CNTL_INTER_RAM0_FORCE_PD_R {
        RTC_CNTL_INTER_RAM0_FORCE_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_force_pu(&self) -> RTC_CNTL_ROM0_FORCE_PU_R {
        RTC_CNTL_ROM0_FORCE_PU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_force_pd(&self) -> RTC_CNTL_ROM0_FORCE_PD_R {
        RTC_CNTL_ROM0_FORCE_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pu(&self) -> RTC_CNTL_LSLP_MEM_FORCE_PU_R {
        RTC_CNTL_LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pd(&self) -> RTC_CNTL_LSLP_MEM_FORCE_PD_R {
        RTC_CNTL_LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable power down digital core in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_pd_en(&mut self) -> RTC_CNTL_DG_WRAP_PD_EN_W {
        RTC_CNTL_DG_WRAP_PD_EN_W { w: self }
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_pd_en(&mut self) -> RTC_CNTL_WIFI_PD_EN_W {
        RTC_CNTL_WIFI_PD_EN_W { w: self }
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_pd_en(&mut self) -> RTC_CNTL_INTER_RAM4_PD_EN_W {
        RTC_CNTL_INTER_RAM4_PD_EN_W { w: self }
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_pd_en(&mut self) -> RTC_CNTL_INTER_RAM3_PD_EN_W {
        RTC_CNTL_INTER_RAM3_PD_EN_W { w: self }
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_pd_en(&mut self) -> RTC_CNTL_INTER_RAM2_PD_EN_W {
        RTC_CNTL_INTER_RAM2_PD_EN_W { w: self }
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_pd_en(&mut self) -> RTC_CNTL_INTER_RAM1_PD_EN_W {
        RTC_CNTL_INTER_RAM1_PD_EN_W { w: self }
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_pd_en(&mut self) -> RTC_CNTL_INTER_RAM0_PD_EN_W {
        RTC_CNTL_INTER_RAM0_PD_EN_W { w: self }
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_pd_en(&mut self) -> RTC_CNTL_ROM0_PD_EN_W {
        RTC_CNTL_ROM0_PD_EN_W { w: self }
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pu(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_PU_W {
        RTC_CNTL_DG_WRAP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pd(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_PD_W {
        RTC_CNTL_DG_WRAP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pu(&mut self) -> RTC_CNTL_WIFI_FORCE_PU_W {
        RTC_CNTL_WIFI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pd(&mut self) -> RTC_CNTL_WIFI_FORCE_PD_W {
        RTC_CNTL_WIFI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_force_pu(&mut self) -> RTC_CNTL_INTER_RAM4_FORCE_PU_W {
        RTC_CNTL_INTER_RAM4_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram4_force_pd(&mut self) -> RTC_CNTL_INTER_RAM4_FORCE_PD_W {
        RTC_CNTL_INTER_RAM4_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_force_pu(&mut self) -> RTC_CNTL_INTER_RAM3_FORCE_PU_W {
        RTC_CNTL_INTER_RAM3_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram3_force_pd(&mut self) -> RTC_CNTL_INTER_RAM3_FORCE_PD_W {
        RTC_CNTL_INTER_RAM3_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_force_pu(&mut self) -> RTC_CNTL_INTER_RAM2_FORCE_PU_W {
        RTC_CNTL_INTER_RAM2_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram2_force_pd(&mut self) -> RTC_CNTL_INTER_RAM2_FORCE_PD_W {
        RTC_CNTL_INTER_RAM2_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_force_pu(&mut self) -> RTC_CNTL_INTER_RAM1_FORCE_PU_W {
        RTC_CNTL_INTER_RAM1_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram1_force_pd(&mut self) -> RTC_CNTL_INTER_RAM1_FORCE_PD_W {
        RTC_CNTL_INTER_RAM1_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_force_pu(&mut self) -> RTC_CNTL_INTER_RAM0_FORCE_PU_W {
        RTC_CNTL_INTER_RAM0_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn rtc_cntl_inter_ram0_force_pd(&mut self) -> RTC_CNTL_INTER_RAM0_FORCE_PD_W {
        RTC_CNTL_INTER_RAM0_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_force_pu(&mut self) -> RTC_CNTL_ROM0_FORCE_PU_W {
        RTC_CNTL_ROM0_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rtc_cntl_rom0_force_pd(&mut self) -> RTC_CNTL_ROM0_FORCE_PD_W {
        RTC_CNTL_ROM0_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pu(&mut self) -> RTC_CNTL_LSLP_MEM_FORCE_PU_W {
        RTC_CNTL_LSLP_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pd(&mut self) -> RTC_CNTL_LSLP_MEM_FORCE_PD_W {
        RTC_CNTL_LSLP_MEM_FORCE_PD_W { w: self }
    }
}
