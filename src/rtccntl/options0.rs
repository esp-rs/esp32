#[doc = "Reader of register OPTIONS0"]
pub type R = crate::R<u32, super::OPTIONS0>;
#[doc = "Writer for register OPTIONS0"]
pub type W = crate::W<u32, super::OPTIONS0>;
#[doc = "Register OPTIONS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTIONS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SW_SYS_RST`"]
pub type RTC_CNTL_SW_SYS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SW_SYS_RST`"]
pub struct RTC_CNTL_SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_SYS_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_NORST`"]
pub type RTC_CNTL_DG_WRAP_FORCE_NORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_NORST`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_RST`"]
pub type RTC_CNTL_DG_WRAP_FORCE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_RST`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ANALOG_FORCE_NOISO`"]
pub type RTC_CNTL_ANALOG_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ANALOG_FORCE_NOISO`"]
pub struct RTC_CNTL_ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLL_FORCE_NOISO`"]
pub type RTC_CNTL_PLL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_FORCE_NOISO`"]
pub struct RTC_CNTL_PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_NOISO`"]
pub type RTC_CNTL_XTL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_NOISO`"]
pub struct RTC_CNTL_XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ANALOG_FORCE_ISO`"]
pub type RTC_CNTL_ANALOG_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ANALOG_FORCE_ISO`"]
pub struct RTC_CNTL_ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLL_FORCE_ISO`"]
pub type RTC_CNTL_PLL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_FORCE_ISO`"]
pub struct RTC_CNTL_PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_ISO`"]
pub type RTC_CNTL_XTL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_ISO`"]
pub struct RTC_CNTL_XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_CORE_FORCE_PU`"]
pub type RTC_CNTL_BIAS_CORE_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_CORE_FORCE_PU`"]
pub struct RTC_CNTL_BIAS_CORE_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_CORE_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_CORE_FORCE_PD`"]
pub type RTC_CNTL_BIAS_CORE_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_CORE_FORCE_PD`"]
pub struct RTC_CNTL_BIAS_CORE_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_CORE_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_CORE_FOLW_8M`"]
pub type RTC_CNTL_BIAS_CORE_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_CORE_FOLW_8M`"]
pub struct RTC_CNTL_BIAS_CORE_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_CORE_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_I2C_FORCE_PU`"]
pub type RTC_CNTL_BIAS_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_I2C_FORCE_PU`"]
pub struct RTC_CNTL_BIAS_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_I2C_FORCE_PD`"]
pub type RTC_CNTL_BIAS_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_I2C_FORCE_PD`"]
pub struct RTC_CNTL_BIAS_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_I2C_FOLW_8M`"]
pub type RTC_CNTL_BIAS_I2C_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_I2C_FOLW_8M`"]
pub struct RTC_CNTL_BIAS_I2C_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_I2C_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_FORCE_NOSLEEP`"]
pub type RTC_CNTL_BIAS_FORCE_NOSLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_FORCE_NOSLEEP`"]
pub struct RTC_CNTL_BIAS_FORCE_NOSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_FORCE_NOSLEEP_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_FORCE_SLEEP`"]
pub type RTC_CNTL_BIAS_FORCE_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_FORCE_SLEEP`"]
pub struct RTC_CNTL_BIAS_FORCE_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_FORCE_SLEEP_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BIAS_SLEEP_FOLW_8M`"]
pub type RTC_CNTL_BIAS_SLEEP_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_SLEEP_FOLW_8M`"]
pub struct RTC_CNTL_BIAS_SLEEP_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_SLEEP_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_PU`"]
pub type RTC_CNTL_XTL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_PU`"]
pub struct RTC_CNTL_XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_PD`"]
pub type RTC_CNTL_XTL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_PD`"]
pub struct RTC_CNTL_XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_FORCE_PU`"]
pub type RTC_CNTL_BBPLL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_FORCE_PU`"]
pub struct RTC_CNTL_BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_FORCE_PD`"]
pub type RTC_CNTL_BBPLL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_FORCE_PD`"]
pub struct RTC_CNTL_BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_I2C_FORCE_PU`"]
pub type RTC_CNTL_BBPLL_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_I2C_FORCE_PU`"]
pub struct RTC_CNTL_BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_I2C_FORCE_PD`"]
pub type RTC_CNTL_BBPLL_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_I2C_FORCE_PD`"]
pub struct RTC_CNTL_BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BB_I2C_FORCE_PU`"]
pub type RTC_CNTL_BB_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BB_I2C_FORCE_PU`"]
pub struct RTC_CNTL_BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BB_I2C_FORCE_PD`"]
pub type RTC_CNTL_BB_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BB_I2C_FORCE_PD`"]
pub struct RTC_CNTL_BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BB_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SW_PROCPU_RST`"]
pub type RTC_CNTL_SW_PROCPU_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SW_PROCPU_RST`"]
pub struct RTC_CNTL_SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_PROCPU_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SW_APPCPU_RST`"]
pub type RTC_CNTL_SW_APPCPU_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SW_APPCPU_RST`"]
pub struct RTC_CNTL_SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_APPCPU_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SW_STALL_PROCPU_C0`"]
pub type RTC_CNTL_SW_STALL_PROCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SW_STALL_PROCPU_C0`"]
pub struct RTC_CNTL_SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SW_STALL_APPCPU_C0`"]
pub type RTC_CNTL_SW_STALL_APPCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SW_STALL_APPCPU_C0`"]
pub struct RTC_CNTL_SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_STALL_APPCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_sys_rst(&self) -> RTC_CNTL_SW_SYS_RST_R {
        RTC_CNTL_SW_SYS_RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_norst(&self) -> RTC_CNTL_DG_WRAP_FORCE_NORST_R {
        RTC_CNTL_DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_rst(&self) -> RTC_CNTL_DG_WRAP_FORCE_RST_R {
        RTC_CNTL_DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_noiso(&self) -> RTC_CNTL_ANALOG_FORCE_NOISO_R {
        RTC_CNTL_ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_noiso(&self) -> RTC_CNTL_PLL_FORCE_NOISO_R {
        RTC_CNTL_PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_noiso(&self) -> RTC_CNTL_XTL_FORCE_NOISO_R {
        RTC_CNTL_XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_iso(&self) -> RTC_CNTL_ANALOG_FORCE_ISO_R {
        RTC_CNTL_ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_iso(&self) -> RTC_CNTL_PLL_FORCE_ISO_R {
        RTC_CNTL_PLL_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_iso(&self) -> RTC_CNTL_XTL_FORCE_ISO_R {
        RTC_CNTL_XTL_FORCE_ISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_force_pu(&self) -> RTC_CNTL_BIAS_CORE_FORCE_PU_R {
        RTC_CNTL_BIAS_CORE_FORCE_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_force_pd(&self) -> RTC_CNTL_BIAS_CORE_FORCE_PD_R {
        RTC_CNTL_BIAS_CORE_FORCE_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_folw_8m(&self) -> RTC_CNTL_BIAS_CORE_FOLW_8M_R {
        RTC_CNTL_BIAS_CORE_FOLW_8M_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_force_pu(&self) -> RTC_CNTL_BIAS_I2C_FORCE_PU_R {
        RTC_CNTL_BIAS_I2C_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_force_pd(&self) -> RTC_CNTL_BIAS_I2C_FORCE_PD_R {
        RTC_CNTL_BIAS_I2C_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_folw_8m(&self) -> RTC_CNTL_BIAS_I2C_FOLW_8M_R {
        RTC_CNTL_BIAS_I2C_FOLW_8M_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn rtc_cntl_bias_force_nosleep(&self) -> RTC_CNTL_BIAS_FORCE_NOSLEEP_R {
        RTC_CNTL_BIAS_FORCE_NOSLEEP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn rtc_cntl_bias_force_sleep(&self) -> RTC_CNTL_BIAS_FORCE_SLEEP_R {
        RTC_CNTL_BIAS_FORCE_SLEEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_folw_8m(&self) -> RTC_CNTL_BIAS_SLEEP_FOLW_8M_R {
        RTC_CNTL_BIAS_SLEEP_FOLW_8M_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pu(&self) -> RTC_CNTL_XTL_FORCE_PU_R {
        RTC_CNTL_XTL_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pd(&self) -> RTC_CNTL_XTL_FORCE_PD_R {
        RTC_CNTL_XTL_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pu(&self) -> RTC_CNTL_BBPLL_FORCE_PU_R {
        RTC_CNTL_BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pd(&self) -> RTC_CNTL_BBPLL_FORCE_PD_R {
        RTC_CNTL_BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pu(&self) -> RTC_CNTL_BBPLL_I2C_FORCE_PU_R {
        RTC_CNTL_BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pd(&self) -> RTC_CNTL_BBPLL_I2C_FORCE_PD_R {
        RTC_CNTL_BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pu(&self) -> RTC_CNTL_BB_I2C_FORCE_PU_R {
        RTC_CNTL_BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pd(&self) -> RTC_CNTL_BB_I2C_FORCE_PD_R {
        RTC_CNTL_BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_procpu_rst(&self) -> RTC_CNTL_SW_PROCPU_RST_R {
        RTC_CNTL_SW_PROCPU_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_appcpu_rst(&self) -> RTC_CNTL_SW_APPCPU_RST_R {
        RTC_CNTL_SW_APPCPU_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_procpu_c0(&self) -> RTC_CNTL_SW_STALL_PROCPU_C0_R {
        RTC_CNTL_SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_appcpu_c0(&self) -> RTC_CNTL_SW_STALL_APPCPU_C0_R {
        RTC_CNTL_SW_STALL_APPCPU_C0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_sys_rst(&mut self) -> RTC_CNTL_SW_SYS_RST_W {
        RTC_CNTL_SW_SYS_RST_W { w: self }
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_norst(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_NORST_W {
        RTC_CNTL_DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_rst(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_RST_W {
        RTC_CNTL_DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_noiso(&mut self) -> RTC_CNTL_ANALOG_FORCE_NOISO_W {
        RTC_CNTL_ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_noiso(&mut self) -> RTC_CNTL_PLL_FORCE_NOISO_W {
        RTC_CNTL_PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_noiso(&mut self) -> RTC_CNTL_XTL_FORCE_NOISO_W {
        RTC_CNTL_XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_iso(&mut self) -> RTC_CNTL_ANALOG_FORCE_ISO_W {
        RTC_CNTL_ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_iso(&mut self) -> RTC_CNTL_PLL_FORCE_ISO_W {
        RTC_CNTL_PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_iso(&mut self) -> RTC_CNTL_XTL_FORCE_ISO_W {
        RTC_CNTL_XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_force_pu(&mut self) -> RTC_CNTL_BIAS_CORE_FORCE_PU_W {
        RTC_CNTL_BIAS_CORE_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_force_pd(&mut self) -> RTC_CNTL_BIAS_CORE_FORCE_PD_W {
        RTC_CNTL_BIAS_CORE_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_core_folw_8m(&mut self) -> RTC_CNTL_BIAS_CORE_FOLW_8M_W {
        RTC_CNTL_BIAS_CORE_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_force_pu(&mut self) -> RTC_CNTL_BIAS_I2C_FORCE_PU_W {
        RTC_CNTL_BIAS_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_force_pd(&mut self) -> RTC_CNTL_BIAS_I2C_FORCE_PD_W {
        RTC_CNTL_BIAS_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_i2c_folw_8m(&mut self) -> RTC_CNTL_BIAS_I2C_FOLW_8M_W {
        RTC_CNTL_BIAS_I2C_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn rtc_cntl_bias_force_nosleep(&mut self) -> RTC_CNTL_BIAS_FORCE_NOSLEEP_W {
        RTC_CNTL_BIAS_FORCE_NOSLEEP_W { w: self }
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn rtc_cntl_bias_force_sleep(&mut self) -> RTC_CNTL_BIAS_FORCE_SLEEP_W {
        RTC_CNTL_BIAS_FORCE_SLEEP_W { w: self }
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_folw_8m(&mut self) -> RTC_CNTL_BIAS_SLEEP_FOLW_8M_W {
        RTC_CNTL_BIAS_SLEEP_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pu(&mut self) -> RTC_CNTL_XTL_FORCE_PU_W {
        RTC_CNTL_XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pd(&mut self) -> RTC_CNTL_XTL_FORCE_PD_W {
        RTC_CNTL_XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pu(&mut self) -> RTC_CNTL_BBPLL_FORCE_PU_W {
        RTC_CNTL_BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pd(&mut self) -> RTC_CNTL_BBPLL_FORCE_PD_W {
        RTC_CNTL_BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pu(&mut self) -> RTC_CNTL_BBPLL_I2C_FORCE_PU_W {
        RTC_CNTL_BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pd(&mut self) -> RTC_CNTL_BBPLL_I2C_FORCE_PD_W {
        RTC_CNTL_BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pu(&mut self) -> RTC_CNTL_BB_I2C_FORCE_PU_W {
        RTC_CNTL_BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pd(&mut self) -> RTC_CNTL_BB_I2C_FORCE_PD_W {
        RTC_CNTL_BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_procpu_rst(&mut self) -> RTC_CNTL_SW_PROCPU_RST_W {
        RTC_CNTL_SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn rtc_cntl_sw_appcpu_rst(&mut self) -> RTC_CNTL_SW_APPCPU_RST_W {
        RTC_CNTL_SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_procpu_c0(&mut self) -> RTC_CNTL_SW_STALL_PROCPU_C0_W {
        RTC_CNTL_SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_appcpu_c0(&mut self) -> RTC_CNTL_SW_STALL_APPCPU_C0_W {
        RTC_CNTL_SW_STALL_APPCPU_C0_W { w: self }
    }
}
