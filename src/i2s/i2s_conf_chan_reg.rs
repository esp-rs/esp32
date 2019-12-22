#[doc = "Reader of register I2S_CONF_CHAN_REG"]
pub type R = crate::R<u32, super::I2S_CONF_CHAN_REG>;
#[doc = "Writer for register I2S_CONF_CHAN_REG"]
pub type W = crate::W<u32, super::I2S_CONF_CHAN_REG>;
#[doc = "Register I2S_CONF_CHAN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CONF_CHAN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_RX_CHAN_MOD`"]
pub type I2S_RX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_RX_CHAN_MOD`"]
pub struct I2S_RX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_CHAN_MOD`"]
pub type I2S_TX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_CHAN_MOD`"]
pub struct I2S_TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn i2s_rx_chan_mod(&self) -> I2S_RX_CHAN_MOD_R {
        I2S_RX_CHAN_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_tx_chan_mod(&self) -> I2S_TX_CHAN_MOD_R {
        I2S_TX_CHAN_MOD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn i2s_rx_chan_mod(&mut self) -> I2S_RX_CHAN_MOD_W {
        I2S_RX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn i2s_tx_chan_mod(&mut self) -> I2S_TX_CHAN_MOD_W {
        I2S_TX_CHAN_MOD_W { w: self }
    }
}
