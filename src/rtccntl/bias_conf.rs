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
#[doc = "Reader of field `RTC_CNTL_RST_BIAS_I2C`"]
pub type RTC_CNTL_RST_BIAS_I2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_RST_BIAS_I2C`"]
pub struct RTC_CNTL_RST_BIAS_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RST_BIAS_I2C_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DEC_HEARTBEAT_WIDTH`"]
pub type RTC_CNTL_DEC_HEARTBEAT_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DEC_HEARTBEAT_WIDTH`"]
pub struct RTC_CNTL_DEC_HEARTBEAT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEC_HEARTBEAT_WIDTH_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INC_HEARTBEAT_PERIOD`"]
pub type RTC_CNTL_INC_HEARTBEAT_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INC_HEARTBEAT_PERIOD`"]
pub struct RTC_CNTL_INC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DEC_HEARTBEAT_PERIOD`"]
pub type RTC_CNTL_DEC_HEARTBEAT_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DEC_HEARTBEAT_PERIOD`"]
pub struct RTC_CNTL_DEC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_INC_HEARTBEAT_REFRESH`"]
pub type RTC_CNTL_INC_HEARTBEAT_REFRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_INC_HEARTBEAT_REFRESH`"]
pub struct RTC_CNTL_INC_HEARTBEAT_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_INC_HEARTBEAT_REFRESH_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ENB_SCK_XTAL`"]
pub type RTC_CNTL_ENB_SCK_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ENB_SCK_XTAL`"]
pub struct RTC_CNTL_ENB_SCK_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ENB_SCK_XTAL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DBG_ATTEN`"]
pub type RTC_CNTL_DBG_ATTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DBG_ATTEN`"]
pub struct RTC_CNTL_DBG_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBG_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DBOOST_FORCE_PU`"]
pub type RTC_CNTL_DBOOST_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DBOOST_FORCE_PU`"]
pub struct RTC_CNTL_DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBOOST_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DBOOST_FORCE_PD`"]
pub type RTC_CNTL_DBOOST_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DBOOST_FORCE_PD`"]
pub struct RTC_CNTL_DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBOOST_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DBIAS_WAK`"]
pub type RTC_CNTL_DBIAS_WAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DBIAS_WAK`"]
pub struct RTC_CNTL_DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DBIAS_SLP`"]
pub type RTC_CNTL_DBIAS_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DBIAS_SLP`"]
pub struct RTC_CNTL_DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SCK_DCAP`"]
pub type RTC_CNTL_SCK_DCAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SCK_DCAP`"]
pub struct RTC_CNTL_SCK_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SCK_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DIG_DBIAS_WAK`"]
pub type RTC_CNTL_DIG_DBIAS_WAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_DBIAS_WAK`"]
pub struct RTC_CNTL_DIG_DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DIG_DBIAS_SLP`"]
pub type RTC_CNTL_DIG_DBIAS_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_DBIAS_SLP`"]
pub struct RTC_CNTL_DIG_DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SCK_DCAP_FORCE`"]
pub type RTC_CNTL_SCK_DCAP_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SCK_DCAP_FORCE`"]
pub struct RTC_CNTL_SCK_DCAP_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SCK_DCAP_FORCE_W<'a> {
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
    pub fn rtc_cntl_rst_bias_i2c(&self) -> RTC_CNTL_RST_BIAS_I2C_R {
        RTC_CNTL_RST_BIAS_I2C_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn rtc_cntl_dec_heartbeat_width(&self) -> RTC_CNTL_DEC_HEARTBEAT_WIDTH_R {
        RTC_CNTL_DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn rtc_cntl_inc_heartbeat_period(&self) -> RTC_CNTL_INC_HEARTBEAT_PERIOD_R {
        RTC_CNTL_INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn rtc_cntl_dec_heartbeat_period(&self) -> RTC_CNTL_DEC_HEARTBEAT_PERIOD_R {
        RTC_CNTL_DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn rtc_cntl_inc_heartbeat_refresh(&self) -> RTC_CNTL_INC_HEARTBEAT_REFRESH_R {
        RTC_CNTL_INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn rtc_cntl_enb_sck_xtal(&self) -> RTC_CNTL_ENB_SCK_XTAL_R {
        RTC_CNTL_ENB_SCK_XTAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten(&self) -> RTC_CNTL_DBG_ATTEN_R {
        RTC_CNTL_DBG_ATTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn rtc_cntl_force_pu(&self) -> RTC_CNTL_FORCE_PU_R {
        RTC_CNTL_FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_cntl_force_pd(&self) -> RTC_CNTL_FORCE_PD_R {
        RTC_CNTL_FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn rtc_cntl_dboost_force_pu(&self) -> RTC_CNTL_DBOOST_FORCE_PU_R {
        RTC_CNTL_DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn rtc_cntl_dboost_force_pd(&self) -> RTC_CNTL_DBOOST_FORCE_PD_R {
        RTC_CNTL_DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn rtc_cntl_dbias_wak(&self) -> RTC_CNTL_DBIAS_WAK_R {
        RTC_CNTL_DBIAS_WAK_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dbias_slp(&self) -> RTC_CNTL_DBIAS_SLP_R {
        RTC_CNTL_DBIAS_SLP_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn rtc_cntl_sck_dcap(&self) -> RTC_CNTL_SCK_DCAP_R {
        RTC_CNTL_SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn rtc_cntl_dig_dbias_wak(&self) -> RTC_CNTL_DIG_DBIAS_WAK_R {
        RTC_CNTL_DIG_DBIAS_WAK_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dig_dbias_slp(&self) -> RTC_CNTL_DIG_DBIAS_SLP_R {
        RTC_CNTL_DIG_DBIAS_SLP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_sck_dcap_force(&self) -> RTC_CNTL_SCK_DCAP_FORCE_R {
        RTC_CNTL_SCK_DCAP_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - RST_BIAS_I2C"]
    #[inline(always)]
    pub fn rtc_cntl_rst_bias_i2c(&mut self) -> RTC_CNTL_RST_BIAS_I2C_W {
        RTC_CNTL_RST_BIAS_I2C_W { w: self }
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn rtc_cntl_dec_heartbeat_width(&mut self) -> RTC_CNTL_DEC_HEARTBEAT_WIDTH_W {
        RTC_CNTL_DEC_HEARTBEAT_WIDTH_W { w: self }
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn rtc_cntl_inc_heartbeat_period(&mut self) -> RTC_CNTL_INC_HEARTBEAT_PERIOD_W {
        RTC_CNTL_INC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn rtc_cntl_dec_heartbeat_period(&mut self) -> RTC_CNTL_DEC_HEARTBEAT_PERIOD_W {
        RTC_CNTL_DEC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn rtc_cntl_inc_heartbeat_refresh(&mut self) -> RTC_CNTL_INC_HEARTBEAT_REFRESH_W {
        RTC_CNTL_INC_HEARTBEAT_REFRESH_W { w: self }
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn rtc_cntl_enb_sck_xtal(&mut self) -> RTC_CNTL_ENB_SCK_XTAL_W {
        RTC_CNTL_ENB_SCK_XTAL_W { w: self }
    }
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten(&mut self) -> RTC_CNTL_DBG_ATTEN_W {
        RTC_CNTL_DBG_ATTEN_W { w: self }
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn rtc_cntl_force_pu(&mut self) -> RTC_CNTL_FORCE_PU_W {
        RTC_CNTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_cntl_force_pd(&mut self) -> RTC_CNTL_FORCE_PD_W {
        RTC_CNTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn rtc_cntl_dboost_force_pu(&mut self) -> RTC_CNTL_DBOOST_FORCE_PU_W {
        RTC_CNTL_DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn rtc_cntl_dboost_force_pd(&mut self) -> RTC_CNTL_DBOOST_FORCE_PD_W {
        RTC_CNTL_DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn rtc_cntl_dbias_wak(&mut self) -> RTC_CNTL_DBIAS_WAK_W {
        RTC_CNTL_DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dbias_slp(&mut self) -> RTC_CNTL_DBIAS_SLP_W {
        RTC_CNTL_DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn rtc_cntl_sck_dcap(&mut self) -> RTC_CNTL_SCK_DCAP_W {
        RTC_CNTL_SCK_DCAP_W { w: self }
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn rtc_cntl_dig_dbias_wak(&mut self) -> RTC_CNTL_DIG_DBIAS_WAK_W {
        RTC_CNTL_DIG_DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_dig_dbias_slp(&mut self) -> RTC_CNTL_DIG_DBIAS_SLP_W {
        RTC_CNTL_DIG_DBIAS_SLP_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rtc_cntl_sck_dcap_force(&mut self) -> RTC_CNTL_SCK_DCAP_FORCE_W {
        RTC_CNTL_SCK_DCAP_FORCE_W { w: self }
    }
}
