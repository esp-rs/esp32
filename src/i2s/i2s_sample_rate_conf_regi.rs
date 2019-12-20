#[doc = "Reader of register I2S_SAMPLE_RATE_CONF_REG(i)"]
pub type R = crate::R<u32, super::I2S_SAMPLE_RATE_CONF_REGI>;
#[doc = "Writer for register I2S_SAMPLE_RATE_CONF_REG(i)"]
pub type W = crate::W<u32, super::I2S_SAMPLE_RATE_CONF_REGI>;
#[doc = "Register I2S_SAMPLE_RATE_CONF_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_SAMPLE_RATE_CONF_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_RX_BITS_MOD`"]
pub type I2S_RX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_RX_BITS_MOD`"]
pub struct I2S_RX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BITS_MOD`"]
pub type I2S_TX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BITS_MOD`"]
pub struct I2S_TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2S_RX_BCK_DIV_NUM`"]
pub type I2S_RX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_RX_BCK_DIV_NUM`"]
pub struct I2S_RX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BCK_DIV_NUM`"]
pub type I2S_TX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BCK_DIV_NUM`"]
pub struct I2S_TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn i2s_rx_bits_mod(&self) -> I2S_RX_BITS_MOD_R {
        I2S_RX_BITS_MOD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn i2s_tx_bits_mod(&self) -> I2S_TX_BITS_MOD_R {
        I2S_TX_BITS_MOD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn i2s_rx_bck_div_num(&self) -> I2S_RX_BCK_DIV_NUM_R {
        I2S_RX_BCK_DIV_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_tx_bck_div_num(&self) -> I2S_TX_BCK_DIV_NUM_R {
        I2S_TX_BCK_DIV_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn i2s_rx_bits_mod(&mut self) -> I2S_RX_BITS_MOD_W {
        I2S_RX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn i2s_tx_bits_mod(&mut self) -> I2S_TX_BITS_MOD_W {
        I2S_TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn i2s_rx_bck_div_num(&mut self) -> I2S_RX_BCK_DIV_NUM_W {
        I2S_RX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_tx_bck_div_num(&mut self) -> I2S_TX_BCK_DIV_NUM_W {
        I2S_TX_BCK_DIV_NUM_W { w: self }
    }
}
