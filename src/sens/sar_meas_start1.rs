#[doc = "Reader of register SAR_MEAS_START1"]
pub type R = crate::R<u32, super::SAR_MEAS_START1>;
#[doc = "Writer for register SAR_MEAS_START1"]
pub type W = crate::W<u32, super::SAR_MEAS_START1>;
#[doc = "Register SAR_MEAS_START1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_START1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR1_EN_PAD_FORCE`"]
pub type SENS_SAR1_EN_PAD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_EN_PAD_FORCE`"]
pub struct SENS_SAR1_EN_PAD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_EN_PAD_FORCE_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_EN_PAD`"]
pub type SENS_SAR1_EN_PAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_SAR1_EN_PAD`"]
pub struct SENS_SAR1_EN_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_EN_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 19)) | (((value as u32) & 0x0fff) << 19);
        self.w
    }
}
#[doc = "Reader of field `SENS_MEAS1_START_FORCE`"]
pub type SENS_MEAS1_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_MEAS1_START_FORCE`"]
pub struct SENS_MEAS1_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEAS1_START_FORCE_W<'a> {
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
#[doc = "Reader of field `SENS_MEAS1_START_SAR`"]
pub type SENS_MEAS1_START_SAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_MEAS1_START_SAR`"]
pub struct SENS_MEAS1_START_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEAS1_START_SAR_W<'a> {
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
#[doc = "Reader of field `SENS_MEAS1_DONE_SAR`"]
pub type SENS_MEAS1_DONE_SAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_MEAS1_DONE_SAR`"]
pub struct SENS_MEAS1_DONE_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEAS1_DONE_SAR_W<'a> {
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
#[doc = "Reader of field `SENS_MEAS1_DATA_SAR`"]
pub type SENS_MEAS1_DATA_SAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_MEAS1_DATA_SAR`"]
pub struct SENS_MEAS1_DATA_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEAS1_DATA_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_sar1_en_pad_force(&self) -> SENS_SAR1_EN_PAD_FORCE_R {
        SENS_SAR1_EN_PAD_FORCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
    #[inline(always)]
    pub fn sens_sar1_en_pad(&self) -> SENS_SAR1_EN_PAD_R {
        SENS_SAR1_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_meas1_start_force(&self) -> SENS_MEAS1_START_FORCE_R {
        SENS_MEAS1_START_FORCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
    #[inline(always)]
    pub fn sens_meas1_start_sar(&self) -> SENS_MEAS1_START_SAR_R {
        SENS_MEAS1_START_SAR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAR ADC1 conversion done indication"]
    #[inline(always)]
    pub fn sens_meas1_done_sar(&self) -> SENS_MEAS1_DONE_SAR_R {
        SENS_MEAS1_DONE_SAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - SAR ADC1 data"]
    #[inline(always)]
    pub fn sens_meas1_data_sar(&self) -> SENS_MEAS1_DATA_SAR_R {
        SENS_MEAS1_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_sar1_en_pad_force(&mut self) -> SENS_SAR1_EN_PAD_FORCE_W {
        SENS_SAR1_EN_PAD_FORCE_W { w: self }
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
    #[inline(always)]
    pub fn sens_sar1_en_pad(&mut self) -> SENS_SAR1_EN_PAD_W {
        SENS_SAR1_EN_PAD_W { w: self }
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn sens_meas1_start_force(&mut self) -> SENS_MEAS1_START_FORCE_W {
        SENS_MEAS1_START_FORCE_W { w: self }
    }
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
    #[inline(always)]
    pub fn sens_meas1_start_sar(&mut self) -> SENS_MEAS1_START_SAR_W {
        SENS_MEAS1_START_SAR_W { w: self }
    }
    #[doc = "Bit 16 - SAR ADC1 conversion done indication"]
    #[inline(always)]
    pub fn sens_meas1_done_sar(&mut self) -> SENS_MEAS1_DONE_SAR_W {
        SENS_MEAS1_DONE_SAR_W { w: self }
    }
    #[doc = "Bits 0:15 - SAR ADC1 data"]
    #[inline(always)]
    pub fn sens_meas1_data_sar(&mut self) -> SENS_MEAS1_DATA_SAR_W {
        SENS_MEAS1_DATA_SAR_W { w: self }
    }
}
