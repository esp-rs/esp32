#[doc = "Reader of register I2S_PDM_CONF_REG(i)"]
pub type R = crate::R<u32, super::I2S_PDM_CONF_REGI>;
#[doc = "Writer for register I2S_PDM_CONF_REG(i)"]
pub type W = crate::W<u32, super::I2S_PDM_CONF_REGI>;
#[doc = "Register I2S_PDM_CONF_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_PDM_CONF_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_PDM_HP_BYPASS`"]
pub type I2S_TX_PDM_HP_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_PDM_HP_BYPASS`"]
pub struct I2S_TX_PDM_HP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_HP_BYPASS_W<'a> {
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
#[doc = "Reader of field `I2S_RX_PDM_SINC_DSR_16_EN`"]
pub type I2S_RX_PDM_SINC_DSR_16_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_PDM_SINC_DSR_16_EN`"]
pub struct I2S_RX_PDM_SINC_DSR_16_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_PDM_SINC_DSR_16_EN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_PDM_SIGMADELTA_IN_SHIFT`"]
pub type I2S_TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_SIGMADELTA_IN_SHIFT`"]
pub struct I2S_TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_SINC_IN_SHIFT`"]
pub type I2S_TX_PDM_SINC_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_SINC_IN_SHIFT`"]
pub struct I2S_TX_PDM_SINC_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_SINC_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_LP_IN_SHIFT`"]
pub type I2S_TX_PDM_LP_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_LP_IN_SHIFT`"]
pub struct I2S_TX_PDM_LP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_LP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_HP_IN_SHIFT`"]
pub type I2S_TX_PDM_HP_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_HP_IN_SHIFT`"]
pub struct I2S_TX_PDM_HP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_HP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_PRESCALE`"]
pub type I2S_TX_PDM_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_PRESCALE`"]
pub struct I2S_TX_PDM_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_SINC_OSR2`"]
pub type I2S_TX_PDM_SINC_OSR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PDM_SINC_OSR2`"]
pub struct I2S_TX_PDM_SINC_OSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_SINC_OSR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2S_PDM2PCM_CONV_EN`"]
pub type I2S_PDM2PCM_CONV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PDM2PCM_CONV_EN`"]
pub struct I2S_PDM2PCM_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PDM2PCM_CONV_EN_W<'a> {
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
#[doc = "Reader of field `I2S_PCM2PDM_CONV_EN`"]
pub type I2S_PCM2PDM_CONV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PCM2PDM_CONV_EN`"]
pub struct I2S_PCM2PDM_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PCM2PDM_CONV_EN_W<'a> {
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
#[doc = "Reader of field `I2S_RX_PDM_EN`"]
pub type I2S_RX_PDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_PDM_EN`"]
pub struct I2S_RX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_PDM_EN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_PDM_EN`"]
pub type I2S_TX_PDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_PDM_EN`"]
pub struct I2S_TX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_EN_W<'a> {
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
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2s_tx_pdm_hp_bypass(&self) -> I2S_TX_PDM_HP_BYPASS_R {
        I2S_TX_PDM_HP_BYPASS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2s_rx_pdm_sinc_dsr_16_en(&self) -> I2S_RX_PDM_SINC_DSR_16_EN_R {
        I2S_RX_PDM_SINC_DSR_16_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sigmadelta_in_shift(&self) -> I2S_TX_PDM_SIGMADELTA_IN_SHIFT_R {
        I2S_TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sinc_in_shift(&self) -> I2S_TX_PDM_SINC_IN_SHIFT_R {
        I2S_TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn i2s_tx_pdm_lp_in_shift(&self) -> I2S_TX_PDM_LP_IN_SHIFT_R {
        I2S_TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2s_tx_pdm_hp_in_shift(&self) -> I2S_TX_PDM_HP_IN_SHIFT_R {
        I2S_TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn i2s_tx_pdm_prescale(&self) -> I2S_TX_PDM_PRESCALE_R {
        I2S_TX_PDM_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sinc_osr2(&self) -> I2S_TX_PDM_SINC_OSR2_R {
        I2S_TX_PDM_SINC_OSR2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_pdm2pcm_conv_en(&self) -> I2S_PDM2PCM_CONV_EN_R {
        I2S_PDM2PCM_CONV_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_pcm2pdm_conv_en(&self) -> I2S_PCM2PDM_CONV_EN_R {
        I2S_PCM2PDM_CONV_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rx_pdm_en(&self) -> I2S_RX_PDM_EN_R {
        I2S_RX_PDM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_pdm_en(&self) -> I2S_TX_PDM_EN_R {
        I2S_TX_PDM_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2s_tx_pdm_hp_bypass(&mut self) -> I2S_TX_PDM_HP_BYPASS_W {
        I2S_TX_PDM_HP_BYPASS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2s_rx_pdm_sinc_dsr_16_en(&mut self) -> I2S_RX_PDM_SINC_DSR_16_EN_W {
        I2S_RX_PDM_SINC_DSR_16_EN_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sigmadelta_in_shift(&mut self) -> I2S_TX_PDM_SIGMADELTA_IN_SHIFT_W {
        I2S_TX_PDM_SIGMADELTA_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sinc_in_shift(&mut self) -> I2S_TX_PDM_SINC_IN_SHIFT_W {
        I2S_TX_PDM_SINC_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn i2s_tx_pdm_lp_in_shift(&mut self) -> I2S_TX_PDM_LP_IN_SHIFT_W {
        I2S_TX_PDM_LP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2s_tx_pdm_hp_in_shift(&mut self) -> I2S_TX_PDM_HP_IN_SHIFT_W {
        I2S_TX_PDM_HP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn i2s_tx_pdm_prescale(&mut self) -> I2S_TX_PDM_PRESCALE_W {
        I2S_TX_PDM_PRESCALE_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn i2s_tx_pdm_sinc_osr2(&mut self) -> I2S_TX_PDM_SINC_OSR2_W {
        I2S_TX_PDM_SINC_OSR2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_pdm2pcm_conv_en(&mut self) -> I2S_PDM2PCM_CONV_EN_W {
        I2S_PDM2PCM_CONV_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_pcm2pdm_conv_en(&mut self) -> I2S_PCM2PDM_CONV_EN_W {
        I2S_PCM2PDM_CONV_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rx_pdm_en(&mut self) -> I2S_RX_PDM_EN_W {
        I2S_RX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_pdm_en(&mut self) -> I2S_TX_PDM_EN_W {
        I2S_TX_PDM_EN_W { w: self }
    }
}
