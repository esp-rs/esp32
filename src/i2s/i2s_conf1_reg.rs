#[doc = "Reader of register I2S_CONF1_REG"]
pub type R = crate::R<u32, super::I2S_CONF1_REG>;
#[doc = "Writer for register I2S_CONF1_REG"]
pub type W = crate::W<u32, super::I2S_CONF1_REG>;
#[doc = "Register I2S_CONF1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CONF1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_ZEROS_RM_EN`"]
pub type I2S_TX_ZEROS_RM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_ZEROS_RM_EN`"]
pub struct I2S_TX_ZEROS_RM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_ZEROS_RM_EN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_STOP_EN`"]
pub type I2S_TX_STOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_STOP_EN`"]
pub struct I2S_TX_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_STOP_EN_W<'a> {
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
#[doc = "Reader of field `I2S_RX_PCM_BYPASS`"]
pub type I2S_RX_PCM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_PCM_BYPASS`"]
pub struct I2S_RX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_PCM_BYPASS_W<'a> {
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
#[doc = "Reader of field `I2S_RX_PCM_CONF`"]
pub type I2S_RX_PCM_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_RX_PCM_CONF`"]
pub struct I2S_RX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PCM_BYPASS`"]
pub type I2S_TX_PCM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_PCM_BYPASS`"]
pub struct I2S_TX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PCM_BYPASS_W<'a> {
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
#[doc = "Reader of field `I2S_TX_PCM_CONF`"]
pub type I2S_TX_PCM_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PCM_CONF`"]
pub struct I2S_TX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_tx_zeros_rm_en(&self) -> I2S_TX_ZEROS_RM_EN_R {
        I2S_TX_ZEROS_RM_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_stop_en(&self) -> I2S_TX_STOP_EN_R {
        I2S_TX_STOP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_rx_pcm_bypass(&self) -> I2S_RX_PCM_BYPASS_R {
        I2S_RX_PCM_BYPASS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn i2s_rx_pcm_conf(&self) -> I2S_RX_PCM_CONF_R {
        I2S_RX_PCM_CONF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_pcm_bypass(&self) -> I2S_TX_PCM_BYPASS_R {
        I2S_TX_PCM_BYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_tx_pcm_conf(&self) -> I2S_TX_PCM_CONF_R {
        I2S_TX_PCM_CONF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_tx_zeros_rm_en(&mut self) -> I2S_TX_ZEROS_RM_EN_W {
        I2S_TX_ZEROS_RM_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_stop_en(&mut self) -> I2S_TX_STOP_EN_W {
        I2S_TX_STOP_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_rx_pcm_bypass(&mut self) -> I2S_RX_PCM_BYPASS_W {
        I2S_RX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn i2s_rx_pcm_conf(&mut self) -> I2S_RX_PCM_CONF_W {
        I2S_RX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_pcm_bypass(&mut self) -> I2S_TX_PCM_BYPASS_W {
        I2S_TX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_tx_pcm_conf(&mut self) -> I2S_TX_PCM_CONF_W {
        I2S_TX_PCM_CONF_W { w: self }
    }
}
