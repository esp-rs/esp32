#[doc = "Reader of register ANA_CONF"]
pub type R = crate::R<u32, super::ANA_CONF>;
#[doc = "Writer for register ANA_CONF"]
pub type W = crate::W<u32, super::ANA_CONF>;
#[doc = "Register ANA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLL_I2C_PU`"]
pub type PLL_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_I2C_PU`"]
pub struct PLL_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_I2C_PU_W<'a> {
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
#[doc = "Reader of field `CKGEN_I2C_PU`"]
pub type CKGEN_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKGEN_I2C_PU`"]
pub struct CKGEN_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGEN_I2C_PU_W<'a> {
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
#[doc = "Reader of field `RFRX_PBUS_PU`"]
pub type RFRX_PBUS_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFRX_PBUS_PU`"]
pub struct RFRX_PBUS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRX_PBUS_PU_W<'a> {
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
#[doc = "Reader of field `TXRF_I2C_PU`"]
pub type TXRF_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRF_I2C_PU`"]
pub struct TXRF_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRF_I2C_PU_W<'a> {
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
#[doc = "Reader of field `PVTMON_PU`"]
pub type PVTMON_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVTMON_PU`"]
pub struct PVTMON_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PVTMON_PU_W<'a> {
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
#[doc = "Reader of field `BBPLL_CAL_SLP_START`"]
pub type BBPLL_CAL_SLP_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_CAL_SLP_START`"]
pub struct BBPLL_CAL_SLP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_CAL_SLP_START_W<'a> {
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
#[doc = "Reader of field `PLLA_FORCE_PU`"]
pub type PLLA_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLA_FORCE_PU`"]
pub struct PLLA_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `PLLA_FORCE_PD`"]
pub type PLLA_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLA_FORCE_PD`"]
pub struct PLLA_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PD_W<'a> {
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
impl R {
    #[doc = "Bit 31 - 1: PLL_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: PVTMON power up otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1: PLL_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W {
        PLL_I2C_PU_W { w: self }
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W {
        CKGEN_I2C_PU_W { w: self }
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W {
        RFRX_PBUS_PU_W { w: self }
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W {
        TXRF_I2C_PU_W { w: self }
    }
    #[doc = "Bit 26 - 1: PVTMON power up otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W {
        PVTMON_PU_W { w: self }
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W {
        BBPLL_CAL_SLP_START_W { w: self }
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W {
        PLLA_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W {
        PLLA_FORCE_PD_W { w: self }
    }
}
