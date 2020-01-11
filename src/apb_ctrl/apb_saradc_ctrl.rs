#[doc = "Reader of register APB_SARADC_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_CTRL>;
#[doc = "Writer for register APB_SARADC_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_CTRL>;
#[doc = "Register APB_SARADC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_DATA_TO_I2S`"]
pub type APB_CTRL_SARADC_DATA_TO_I2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_DATA_TO_I2S`"]
pub struct APB_CTRL_SARADC_DATA_TO_I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_DATA_TO_I2S_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_DATA_SAR_SEL`"]
pub type APB_CTRL_SARADC_DATA_SAR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_DATA_SAR_SEL`"]
pub struct APB_CTRL_SARADC_DATA_SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_DATA_SAR_SEL_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_SAR2_PATT_P_CLEAR`"]
pub type APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR2_PATT_P_CLEAR`"]
pub struct APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_SAR1_PATT_P_CLEAR`"]
pub type APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR1_PATT_P_CLEAR`"]
pub struct APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_SAR2_PATT_LEN`"]
pub type APB_CTRL_SARADC_SAR2_PATT_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR2_PATT_LEN`"]
pub struct APB_CTRL_SARADC_SAR2_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR2_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR1_PATT_LEN`"]
pub type APB_CTRL_SARADC_SAR1_PATT_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR1_PATT_LEN`"]
pub struct APB_CTRL_SARADC_SAR1_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR1_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | (((value as u32) & 0x0f) << 15);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR_CLK_DIV`"]
pub type APB_CTRL_SARADC_SAR_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR_CLK_DIV`"]
pub struct APB_CTRL_SARADC_SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | (((value as u32) & 0xff) << 7);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR_CLK_GATED`"]
pub type APB_CTRL_SARADC_SAR_CLK_GATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR_CLK_GATED`"]
pub struct APB_CTRL_SARADC_SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR_CLK_GATED_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_SAR_SEL`"]
pub type APB_CTRL_SARADC_SAR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR_SEL`"]
pub struct APB_CTRL_SARADC_SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR_SEL_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_WORK_MODE`"]
pub type APB_CTRL_SARADC_WORK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_WORK_MODE`"]
pub struct APB_CTRL_SARADC_WORK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_WORK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR2_MUX`"]
pub type APB_CTRL_SARADC_SAR2_MUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR2_MUX`"]
pub struct APB_CTRL_SARADC_SAR2_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR2_MUX_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_START`"]
pub type APB_CTRL_SARADC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_START`"]
pub struct APB_CTRL_SARADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_START_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SARADC_START_FORCE`"]
pub type APB_CTRL_SARADC_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_START_FORCE`"]
pub struct APB_CTRL_SARADC_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_START_FORCE_W<'a> {
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
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_data_to_i2s(&self) -> APB_CTRL_SARADC_DATA_TO_I2S_R {
        APB_CTRL_SARADC_DATA_TO_I2S_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn apb_ctrl_saradc_data_sar_sel(&self) -> APB_CTRL_SARADC_DATA_SAR_SEL_R {
        APB_CTRL_SARADC_DATA_SAR_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_p_clear(&self) -> APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_R {
        APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar1_patt_p_clear(&self) -> APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_R {
        APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_len(&self) -> APB_CTRL_SARADC_SAR2_PATT_LEN_R {
        APB_CTRL_SARADC_SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar1_patt_len(&self) -> APB_CTRL_SARADC_SAR1_PATT_LEN_R {
        APB_CTRL_SARADC_SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_clk_div(&self) -> APB_CTRL_SARADC_SAR_CLK_DIV_R {
        APB_CTRL_SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_clk_gated(&self) -> APB_CTRL_SARADC_SAR_CLK_GATED_R {
        APB_CTRL_SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_sel(&self) -> APB_CTRL_SARADC_SAR_SEL_R {
        APB_CTRL_SARADC_SAR_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_work_mode(&self) -> APB_CTRL_SARADC_WORK_MODE_R {
        APB_CTRL_SARADC_WORK_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_mux(&self) -> APB_CTRL_SARADC_SAR2_MUX_R {
        APB_CTRL_SARADC_SAR2_MUX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start(&self) -> APB_CTRL_SARADC_START_R {
        APB_CTRL_SARADC_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start_force(&self) -> APB_CTRL_SARADC_START_FORCE_R {
        APB_CTRL_SARADC_START_FORCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_data_to_i2s(&mut self) -> APB_CTRL_SARADC_DATA_TO_I2S_W {
        APB_CTRL_SARADC_DATA_TO_I2S_W { w: self }
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn apb_ctrl_saradc_data_sar_sel(&mut self) -> APB_CTRL_SARADC_DATA_SAR_SEL_W {
        APB_CTRL_SARADC_DATA_SAR_SEL_W { w: self }
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_p_clear(&mut self) -> APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_W {
        APB_CTRL_SARADC_SAR2_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar1_patt_p_clear(&mut self) -> APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_W {
        APB_CTRL_SARADC_SAR1_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_len(&mut self) -> APB_CTRL_SARADC_SAR2_PATT_LEN_W {
        APB_CTRL_SARADC_SAR2_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar1_patt_len(&mut self) -> APB_CTRL_SARADC_SAR1_PATT_LEN_W {
        APB_CTRL_SARADC_SAR1_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_clk_div(&mut self) -> APB_CTRL_SARADC_SAR_CLK_DIV_W {
        APB_CTRL_SARADC_SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_clk_gated(&mut self) -> APB_CTRL_SARADC_SAR_CLK_GATED_W {
        APB_CTRL_SARADC_SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar_sel(&mut self) -> APB_CTRL_SARADC_SAR_SEL_W {
        APB_CTRL_SARADC_SAR_SEL_W { w: self }
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_work_mode(&mut self) -> APB_CTRL_SARADC_WORK_MODE_W {
        APB_CTRL_SARADC_WORK_MODE_W { w: self }
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_mux(&mut self) -> APB_CTRL_SARADC_SAR2_MUX_W {
        APB_CTRL_SARADC_SAR2_MUX_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start(&mut self) -> APB_CTRL_SARADC_START_W {
        APB_CTRL_SARADC_START_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_start_force(&mut self) -> APB_CTRL_SARADC_START_FORCE_W {
        APB_CTRL_SARADC_START_FORCE_W { w: self }
    }
}
