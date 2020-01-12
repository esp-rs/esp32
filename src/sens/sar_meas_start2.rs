#[doc = "Reader of register SAR_MEAS_START2"]
pub type R = crate::R<u32, super::SAR_MEAS_START2>;
#[doc = "Writer for register SAR_MEAS_START2"]
pub type W = crate::W<u32, super::SAR_MEAS_START2>;
#[doc = "Register SAR_MEAS_START2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_START2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_EN_PAD_FORCE`"]
pub type SAR2_EN_PAD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR2_EN_PAD_FORCE`"]
pub struct SAR2_EN_PAD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_PAD_FORCE_W<'a> {
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
#[doc = "Reader of field `SAR2_EN_PAD`"]
pub type SAR2_EN_PAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAR2_EN_PAD`"]
pub struct SAR2_EN_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 19)) | (((value as u32) & 0x0fff) << 19);
        self.w
    }
}
#[doc = "Reader of field `MEAS2_START_FORCE`"]
pub type MEAS2_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEAS2_START_FORCE`"]
pub struct MEAS2_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_START_FORCE_W<'a> {
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
#[doc = "Reader of field `MEAS2_START_SAR`"]
pub type MEAS2_START_SAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEAS2_START_SAR`"]
pub struct MEAS2_START_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_START_SAR_W<'a> {
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
#[doc = "Reader of field `MEAS2_DONE_SAR`"]
pub type MEAS2_DONE_SAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEAS2_DONE_SAR`"]
pub struct MEAS2_DONE_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_DONE_SAR_W<'a> {
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
#[doc = "Reader of field `MEAS2_DATA_SAR`"]
pub type MEAS2_DATA_SAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEAS2_DATA_SAR`"]
pub struct MEAS2_DATA_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_DATA_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> SAR2_EN_PAD_FORCE_R {
        SAR2_EN_PAD_FORCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> SAR2_EN_PAD_R {
        SAR2_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas2_start_force(&self) -> MEAS2_START_FORCE_R {
        MEAS2_START_FORCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> MEAS2_START_SAR_R {
        MEAS2_START_SAR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAR ADC2 conversion done indication"]
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> MEAS2_DONE_SAR_R {
        MEAS2_DONE_SAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - SAR ADC2 data"]
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> MEAS2_DATA_SAR_R {
        MEAS2_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar2_en_pad_force(&mut self) -> SAR2_EN_PAD_FORCE_W {
        SAR2_EN_PAD_FORCE_W { w: self }
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar2_en_pad(&mut self) -> SAR2_EN_PAD_W {
        SAR2_EN_PAD_W { w: self }
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas2_start_force(&mut self) -> MEAS2_START_FORCE_W {
        MEAS2_START_FORCE_W { w: self }
    }
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
    #[inline(always)]
    pub fn meas2_start_sar(&mut self) -> MEAS2_START_SAR_W {
        MEAS2_START_SAR_W { w: self }
    }
    #[doc = "Bit 16 - SAR ADC2 conversion done indication"]
    #[inline(always)]
    pub fn meas2_done_sar(&mut self) -> MEAS2_DONE_SAR_W {
        MEAS2_DONE_SAR_W { w: self }
    }
    #[doc = "Bits 0:15 - SAR ADC2 data"]
    #[inline(always)]
    pub fn meas2_data_sar(&mut self) -> MEAS2_DATA_SAR_W {
        MEAS2_DATA_SAR_W { w: self }
    }
}
