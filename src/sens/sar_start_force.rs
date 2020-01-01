#[doc = "Reader of register SAR_START_FORCE"]
pub type R = crate::R<u32, super::SAR_START_FORCE>;
#[doc = "Writer for register SAR_START_FORCE"]
pub type W = crate::W<u32, super::SAR_START_FORCE>;
#[doc = "Register SAR_START_FORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_START_FORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR2_PWDET_EN`"]
pub type SENS_SAR2_PWDET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR2_PWDET_EN`"]
pub struct SENS_SAR2_PWDET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_PWDET_EN_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_STOP`"]
pub type SENS_SAR1_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_STOP`"]
pub struct SENS_SAR1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_STOP_W<'a> {
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
#[doc = "Reader of field `SENS_SAR2_STOP`"]
pub type SENS_SAR2_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR2_STOP`"]
pub struct SENS_SAR2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_STOP_W<'a> {
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
#[doc = "Reader of field `SENS_PC_INIT`"]
pub type SENS_PC_INIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_PC_INIT`"]
pub struct SENS_PC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_PC_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_SARCLK_EN`"]
pub type SENS_SARCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SARCLK_EN`"]
pub struct SENS_SARCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SARCLK_EN_W<'a> {
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
#[doc = "Reader of field `SENS_ULP_CP_START_TOP`"]
pub type SENS_ULP_CP_START_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_ULP_CP_START_TOP`"]
pub struct SENS_ULP_CP_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_ULP_CP_START_TOP_W<'a> {
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
#[doc = "Reader of field `SENS_ULP_CP_FORCE_START_TOP`"]
pub type SENS_ULP_CP_FORCE_START_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_ULP_CP_FORCE_START_TOP`"]
pub struct SENS_ULP_CP_FORCE_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_ULP_CP_FORCE_START_TOP_W<'a> {
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
#[doc = "Reader of field `SENS_SAR2_PWDET_CCT`"]
pub type SENS_SAR2_PWDET_CCT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR2_PWDET_CCT`"]
pub struct SENS_SAR2_PWDET_CCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_PWDET_CCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR2_EN_TEST`"]
pub type SENS_SAR2_EN_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR2_EN_TEST`"]
pub struct SENS_SAR2_EN_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_EN_TEST_W<'a> {
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
#[doc = "Reader of field `SENS_SAR2_BIT_WIDTH`"]
pub type SENS_SAR2_BIT_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR2_BIT_WIDTH`"]
pub struct SENS_SAR2_BIT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_BIT_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR1_BIT_WIDTH`"]
pub type SENS_SAR1_BIT_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_BIT_WIDTH`"]
pub struct SENS_SAR1_BIT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_BIT_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sens_sar2_pwdet_en(&self) -> SENS_SAR2_PWDET_EN_R {
        SENS_SAR2_PWDET_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sens_sar1_stop(&self) -> SENS_SAR1_STOP_R {
        SENS_SAR1_STOP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sens_sar2_stop(&self) -> SENS_SAR2_STOP_R {
        SENS_SAR2_STOP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_pc_init(&self) -> SENS_PC_INIT_R {
        SENS_PC_INIT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sens_sarclk_en(&self) -> SENS_SARCLK_EN_R {
        SENS_SARCLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn sens_ulp_cp_start_top(&self) -> SENS_ULP_CP_START_TOP_R {
        SENS_ULP_CP_START_TOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn sens_ulp_cp_force_start_top(&self) -> SENS_ULP_CP_FORCE_START_TOP_R {
        SENS_ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sens_sar2_pwdet_cct(&self) -> SENS_SAR2_PWDET_CCT_R {
        SENS_SAR2_PWDET_CCT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sens_sar2_en_test(&self) -> SENS_SAR2_EN_TEST_R {
        SENS_SAR2_EN_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sens_sar2_bit_width(&self) -> SENS_SAR2_BIT_WIDTH_R {
        SENS_SAR2_BIT_WIDTH_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sens_sar1_bit_width(&self) -> SENS_SAR1_BIT_WIDTH_R {
        SENS_SAR1_BIT_WIDTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sens_sar2_pwdet_en(&mut self) -> SENS_SAR2_PWDET_EN_W {
        SENS_SAR2_PWDET_EN_W { w: self }
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sens_sar1_stop(&mut self) -> SENS_SAR1_STOP_W {
        SENS_SAR1_STOP_W { w: self }
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sens_sar2_stop(&mut self) -> SENS_SAR2_STOP_W {
        SENS_SAR2_STOP_W { w: self }
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_pc_init(&mut self) -> SENS_PC_INIT_W {
        SENS_PC_INIT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sens_sarclk_en(&mut self) -> SENS_SARCLK_EN_W {
        SENS_SARCLK_EN_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn sens_ulp_cp_start_top(&mut self) -> SENS_ULP_CP_START_TOP_W {
        SENS_ULP_CP_START_TOP_W { w: self }
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn sens_ulp_cp_force_start_top(&mut self) -> SENS_ULP_CP_FORCE_START_TOP_W {
        SENS_ULP_CP_FORCE_START_TOP_W { w: self }
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sens_sar2_pwdet_cct(&mut self) -> SENS_SAR2_PWDET_CCT_W {
        SENS_SAR2_PWDET_CCT_W { w: self }
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sens_sar2_en_test(&mut self) -> SENS_SAR2_EN_TEST_W {
        SENS_SAR2_EN_TEST_W { w: self }
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sens_sar2_bit_width(&mut self) -> SENS_SAR2_BIT_WIDTH_W {
        SENS_SAR2_BIT_WIDTH_W { w: self }
    }
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sens_sar1_bit_width(&mut self) -> SENS_SAR1_BIT_WIDTH_W {
        SENS_SAR1_BIT_WIDTH_W { w: self }
    }
}
