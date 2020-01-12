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
#[doc = "Reader of field `SW_SYS_RST`"]
pub type SW_SYS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SYS_RST`"]
pub struct SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SYS_RST_W<'a> {
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
#[doc = "Reader of field `DG_WRAP_FORCE_NORST`"]
pub type DG_WRAP_FORCE_NORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_NORST`"]
pub struct DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Reader of field `DG_WRAP_FORCE_RST`"]
pub type DG_WRAP_FORCE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_RST`"]
pub struct DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Reader of field `ANALOG_FORCE_NOISO`"]
pub type ANALOG_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANALOG_FORCE_NOISO`"]
pub struct ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `PLL_FORCE_NOISO`"]
pub type PLL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_FORCE_NOISO`"]
pub struct PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_NOISO`"]
pub type XTL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_NOISO`"]
pub struct XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `ANALOG_FORCE_ISO`"]
pub type ANALOG_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANALOG_FORCE_ISO`"]
pub struct ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `PLL_FORCE_ISO`"]
pub type PLL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_FORCE_ISO`"]
pub struct PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_ISO`"]
pub type XTL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_ISO`"]
pub struct XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `BIAS_CORE_FORCE_PU`"]
pub type BIAS_CORE_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_CORE_FORCE_PU`"]
pub struct BIAS_CORE_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BIAS_CORE_FORCE_PD`"]
pub type BIAS_CORE_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_CORE_FORCE_PD`"]
pub struct BIAS_CORE_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BIAS_CORE_FOLW_8M`"]
pub type BIAS_CORE_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_CORE_FOLW_8M`"]
pub struct BIAS_CORE_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `BIAS_I2C_FORCE_PU`"]
pub type BIAS_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_I2C_FORCE_PU`"]
pub struct BIAS_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BIAS_I2C_FORCE_PD`"]
pub type BIAS_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_I2C_FORCE_PD`"]
pub struct BIAS_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BIAS_I2C_FOLW_8M`"]
pub type BIAS_I2C_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_I2C_FOLW_8M`"]
pub struct BIAS_I2C_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `BIAS_FORCE_NOSLEEP`"]
pub type BIAS_FORCE_NOSLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_FORCE_NOSLEEP`"]
pub struct BIAS_FORCE_NOSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_FORCE_NOSLEEP_W<'a> {
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
#[doc = "Reader of field `BIAS_FORCE_SLEEP`"]
pub type BIAS_FORCE_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_FORCE_SLEEP`"]
pub struct BIAS_FORCE_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_FORCE_SLEEP_W<'a> {
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
#[doc = "Reader of field `BIAS_SLEEP_FOLW_8M`"]
pub type BIAS_SLEEP_FOLW_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_SLEEP_FOLW_8M`"]
pub struct BIAS_SLEEP_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_FOLW_8M_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_PU`"]
pub type XTL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_PU`"]
pub struct XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_PD`"]
pub type XTL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_PD`"]
pub struct XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BBPLL_FORCE_PU`"]
pub type BBPLL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_FORCE_PU`"]
pub struct BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BBPLL_FORCE_PD`"]
pub type BBPLL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_FORCE_PD`"]
pub struct BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BBPLL_I2C_FORCE_PU`"]
pub type BBPLL_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_I2C_FORCE_PU`"]
pub struct BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BBPLL_I2C_FORCE_PD`"]
pub type BBPLL_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_I2C_FORCE_PD`"]
pub struct BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BB_I2C_FORCE_PU`"]
pub type BB_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_I2C_FORCE_PU`"]
pub struct BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BB_I2C_FORCE_PD`"]
pub type BB_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_I2C_FORCE_PD`"]
pub struct BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SW_PROCPU_RST`"]
pub type SW_PROCPU_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_PROCPU_RST`"]
pub struct SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PROCPU_RST_W<'a> {
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
#[doc = "Reader of field `SW_APPCPU_RST`"]
pub type SW_APPCPU_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_APPCPU_RST`"]
pub struct SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_APPCPU_RST_W<'a> {
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
#[doc = "Reader of field `SW_STALL_PROCPU_C0`"]
pub type SW_STALL_PROCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_PROCPU_C0`"]
pub struct SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SW_STALL_APPCPU_C0`"]
pub type SW_STALL_APPCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_APPCPU_C0`"]
pub struct SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C0_W<'a> {
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
    pub fn sw_sys_rst(&self) -> SW_SYS_RST_R {
        SW_SYS_RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&self) -> PLL_FORCE_NOISO_R {
        PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&self) -> XTL_FORCE_NOISO_R {
        XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&self) -> PLL_FORCE_ISO_R {
        PLL_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&self) -> XTL_FORCE_ISO_R {
        XTL_FORCE_ISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn bias_core_force_pu(&self) -> BIAS_CORE_FORCE_PU_R {
        BIAS_CORE_FORCE_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn bias_core_force_pd(&self) -> BIAS_CORE_FORCE_PD_R {
        BIAS_CORE_FORCE_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn bias_core_folw_8m(&self) -> BIAS_CORE_FOLW_8M_R {
        BIAS_CORE_FOLW_8M_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn bias_i2c_force_pu(&self) -> BIAS_I2C_FORCE_PU_R {
        BIAS_I2C_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn bias_i2c_force_pd(&self) -> BIAS_I2C_FORCE_PD_R {
        BIAS_I2C_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn bias_i2c_folw_8m(&self) -> BIAS_I2C_FOLW_8M_R {
        BIAS_I2C_FOLW_8M_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn bias_force_nosleep(&self) -> BIAS_FORCE_NOSLEEP_R {
        BIAS_FORCE_NOSLEEP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn bias_force_sleep(&self) -> BIAS_FORCE_SLEEP_R {
        BIAS_FORCE_SLEEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn bias_sleep_folw_8m(&self) -> BIAS_SLEEP_FOLW_8M_R {
        BIAS_SLEEP_FOLW_8M_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&self) -> SW_PROCPU_RST_R {
        SW_PROCPU_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn sw_appcpu_rst(&self) -> SW_APPCPU_RST_R {
        SW_APPCPU_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&self) -> SW_STALL_APPCPU_C0_R {
        SW_STALL_APPCPU_C0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W {
        SW_SYS_RST_W { w: self }
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W {
        DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W {
        DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W {
        ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&mut self) -> PLL_FORCE_NOISO_W {
        PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&mut self) -> XTL_FORCE_NOISO_W {
        XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W {
        ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&mut self) -> PLL_FORCE_ISO_W {
        PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&mut self) -> XTL_FORCE_ISO_W {
        XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn bias_core_force_pu(&mut self) -> BIAS_CORE_FORCE_PU_W {
        BIAS_CORE_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn bias_core_force_pd(&mut self) -> BIAS_CORE_FORCE_PD_W {
        BIAS_CORE_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn bias_core_folw_8m(&mut self) -> BIAS_CORE_FOLW_8M_W {
        BIAS_CORE_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn bias_i2c_force_pu(&mut self) -> BIAS_I2C_FORCE_PU_W {
        BIAS_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn bias_i2c_force_pd(&mut self) -> BIAS_I2C_FORCE_PD_W {
        BIAS_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn bias_i2c_folw_8m(&mut self) -> BIAS_I2C_FOLW_8M_W {
        BIAS_I2C_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn bias_force_nosleep(&mut self) -> BIAS_FORCE_NOSLEEP_W {
        BIAS_FORCE_NOSLEEP_W { w: self }
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn bias_force_sleep(&mut self) -> BIAS_FORCE_SLEEP_W {
        BIAS_FORCE_SLEEP_W { w: self }
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn bias_sleep_folw_8m(&mut self) -> BIAS_SLEEP_FOLW_8M_W {
        BIAS_SLEEP_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W {
        XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W {
        XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W {
        BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W {
        BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W {
        BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W {
        BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W {
        BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W {
        BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W {
        SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn sw_appcpu_rst(&mut self) -> SW_APPCPU_RST_W {
        SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W {
        SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&mut self) -> SW_STALL_APPCPU_C0_W {
        SW_STALL_APPCPU_C0_W { w: self }
    }
}
