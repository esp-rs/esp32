#[doc = "Reader of register BIAS_CONF"]
pub type R = crate::R<u32, super::BIAS_CONF>;
#[doc = "Writer for register BIAS_CONF"]
pub type W = crate::W<u32, super::BIAS_CONF>;
#[doc = "Register BIAS_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::BIAS_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RST_BIAS_I2C`"]
pub type RST_BIAS_I2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST_BIAS_I2C`"]
pub struct RST_BIAS_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_BIAS_I2C_W<'a> {
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
#[doc = "Reader of field `DEC_HEARTBEAT_WIDTH`"]
pub type DEC_HEARTBEAT_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEC_HEARTBEAT_WIDTH`"]
pub struct DEC_HEARTBEAT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_HEARTBEAT_WIDTH_W<'a> {
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
#[doc = "Reader of field `INC_HEARTBEAT_PERIOD`"]
pub type INC_HEARTBEAT_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INC_HEARTBEAT_PERIOD`"]
pub struct INC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Reader of field `DEC_HEARTBEAT_PERIOD`"]
pub type DEC_HEARTBEAT_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEC_HEARTBEAT_PERIOD`"]
pub struct DEC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Reader of field `INC_HEARTBEAT_REFRESH`"]
pub type INC_HEARTBEAT_REFRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INC_HEARTBEAT_REFRESH`"]
pub struct INC_HEARTBEAT_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_HEARTBEAT_REFRESH_W<'a> {
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
#[doc = "Reader of field `ENB_SCK_XTAL`"]
pub type ENB_SCK_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB_SCK_XTAL`"]
pub struct ENB_SCK_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_SCK_XTAL_W<'a> {
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
#[doc = "Reader of field `DBG_ATTEN`"]
pub type DBG_ATTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBG_ATTEN`"]
pub struct DBG_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DBOOST_FORCE_PU`"]
pub type DBOOST_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBOOST_FORCE_PU`"]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `DBOOST_FORCE_PD`"]
pub type DBOOST_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBOOST_FORCE_PD`"]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `DBIAS_WAK`"]
pub type DBIAS_WAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBIAS_WAK`"]
pub struct DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `DBIAS_SLP`"]
pub type DBIAS_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBIAS_SLP`"]
pub struct DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `SCK_DCAP`"]
pub type SCK_DCAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCK_DCAP`"]
pub struct SCK_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `DIG_DBIAS_WAK`"]
pub type DIG_DBIAS_WAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_DBIAS_WAK`"]
pub struct DIG_DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `DIG_DBIAS_SLP`"]
pub type DIG_DBIAS_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_DBIAS_SLP`"]
pub struct DIG_DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCK_DCAP_FORCE`"]
pub type SCK_DCAP_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCK_DCAP_FORCE`"]
pub struct SCK_DCAP_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_FORCE_W<'a> {
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
    #[doc = "Bit 31 - RST_BIAS_I2C"]
    #[inline(always)]
    pub fn rst_bias_i2c(&self) -> RST_BIAS_I2C_R {
        RST_BIAS_I2C_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&self) -> DEC_HEARTBEAT_WIDTH_R {
        DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&self) -> INC_HEARTBEAT_PERIOD_R {
        INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&self) -> DEC_HEARTBEAT_PERIOD_R {
        DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&self) -> INC_HEARTBEAT_REFRESH_R {
        INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&self) -> ENB_SCK_XTAL_R {
        ENB_SCK_XTAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    pub fn dbg_atten(&self) -> DBG_ATTEN_R {
        DBG_ATTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dbias_wak(&self) -> DBIAS_WAK_R {
        DBIAS_WAK_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn dbias_slp(&self) -> DBIAS_SLP_R {
        DBIAS_SLP_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dig_dbias_wak(&self) -> DIG_DBIAS_WAK_R {
        DIG_DBIAS_WAK_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn dig_dbias_slp(&self) -> DIG_DBIAS_SLP_R {
        DIG_DBIAS_SLP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn sck_dcap_force(&self) -> SCK_DCAP_FORCE_R {
        SCK_DCAP_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - RST_BIAS_I2C"]
    #[inline(always)]
    pub fn rst_bias_i2c(&mut self) -> RST_BIAS_I2C_W {
        RST_BIAS_I2C_W { w: self }
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&mut self) -> DEC_HEARTBEAT_WIDTH_W {
        DEC_HEARTBEAT_WIDTH_W { w: self }
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&mut self) -> INC_HEARTBEAT_PERIOD_W {
        INC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&mut self) -> DEC_HEARTBEAT_PERIOD_W {
        DEC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&mut self) -> INC_HEARTBEAT_REFRESH_W {
        INC_HEARTBEAT_REFRESH_W { w: self }
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&mut self) -> ENB_SCK_XTAL_W {
        ENB_SCK_XTAL_W { w: self }
    }
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    pub fn dbg_atten(&mut self) -> DBG_ATTEN_W {
        DBG_ATTEN_W { w: self }
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn force_pu(&mut self) -> FORCE_PU_W {
        FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn force_pd(&mut self) -> FORCE_PD_W {
        FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dbias_wak(&mut self) -> DBIAS_WAK_W {
        DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn dbias_slp(&mut self) -> DBIAS_SLP_W {
        DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W {
        SCK_DCAP_W { w: self }
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dig_dbias_wak(&mut self) -> DIG_DBIAS_WAK_W {
        DIG_DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn dig_dbias_slp(&mut self) -> DIG_DBIAS_SLP_W {
        DIG_DBIAS_SLP_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn sck_dcap_force(&mut self) -> SCK_DCAP_FORCE_W {
        SCK_DCAP_FORCE_W { w: self }
    }
}
