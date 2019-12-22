#[doc = "Reader of register I2C_FIFO_CONF_REG"]
pub type R = crate::R<u32, super::I2C_FIFO_CONF_REG>;
#[doc = "Writer for register I2C_FIFO_CONF_REG"]
pub type W = crate::W<u32, super::I2C_FIFO_CONF_REG>;
#[doc = "Register I2C_FIFO_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_FIFO_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_NONFIFO_TX_THRES`"]
pub type I2C_NONFIFO_TX_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_NONFIFO_TX_THRES`"]
pub struct I2C_NONFIFO_TX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NONFIFO_TX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2C_NONFIFO_RX_THRES`"]
pub type I2C_NONFIFO_RX_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_NONFIFO_RX_THRES`"]
pub struct I2C_NONFIFO_RX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NONFIFO_RX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `I2C_TX_FIFO_RST`"]
pub type I2C_TX_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TX_FIFO_RST`"]
pub struct I2C_TX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TX_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2C_RX_FIFO_RST`"]
pub type I2C_RX_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RX_FIFO_RST`"]
pub struct I2C_RX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RX_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2C_FIFO_ADDR_CFG_EN`"]
pub type I2C_FIFO_ADDR_CFG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_FIFO_ADDR_CFG_EN`"]
pub struct I2C_FIFO_ADDR_CFG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_FIFO_ADDR_CFG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2C_NONFIFO_EN`"]
pub type I2C_NONFIFO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_NONFIFO_EN`"]
pub struct I2C_NONFIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NONFIFO_EN_W<'a> {
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
#[doc = "Reader of field `I2C_TXFIFO_EMPTY_THRHD`"]
pub type I2C_TXFIFO_EMPTY_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_TXFIFO_EMPTY_THRHD`"]
pub struct I2C_TXFIFO_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2C_RXFIFO_FULL_THRHD`"]
pub type I2C_RXFIFO_FULL_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_RXFIFO_FULL_THRHD`"]
pub struct I2C_RXFIFO_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn i2c_nonfifo_tx_thres(&self) -> I2C_NONFIFO_TX_THRES_R {
        I2C_NONFIFO_TX_THRES_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn i2c_nonfifo_rx_thres(&self) -> I2C_NONFIFO_RX_THRES_R {
        I2C_NONFIFO_RX_THRES_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn i2c_tx_fifo_rst(&self) -> I2C_TX_FIFO_RST_R {
        I2C_TX_FIFO_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn i2c_rx_fifo_rst(&self) -> I2C_RX_FIFO_RST_R {
        I2C_RX_FIFO_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn i2c_fifo_addr_cfg_en(&self) -> I2C_FIFO_ADDR_CFG_EN_R {
        I2C_FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn i2c_nonfifo_en(&self) -> I2C_NONFIFO_EN_R {
        I2C_NONFIFO_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn i2c_txfifo_empty_thrhd(&self) -> I2C_TXFIFO_EMPTY_THRHD_R {
        I2C_TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2c_rxfifo_full_thrhd(&self) -> I2C_RXFIFO_FULL_THRHD_R {
        I2C_RXFIFO_FULL_THRHD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn i2c_nonfifo_tx_thres(&mut self) -> I2C_NONFIFO_TX_THRES_W {
        I2C_NONFIFO_TX_THRES_W { w: self }
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn i2c_nonfifo_rx_thres(&mut self) -> I2C_NONFIFO_RX_THRES_W {
        I2C_NONFIFO_RX_THRES_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn i2c_tx_fifo_rst(&mut self) -> I2C_TX_FIFO_RST_W {
        I2C_TX_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn i2c_rx_fifo_rst(&mut self) -> I2C_RX_FIFO_RST_W {
        I2C_RX_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn i2c_fifo_addr_cfg_en(&mut self) -> I2C_FIFO_ADDR_CFG_EN_W {
        I2C_FIFO_ADDR_CFG_EN_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn i2c_nonfifo_en(&mut self) -> I2C_NONFIFO_EN_W {
        I2C_NONFIFO_EN_W { w: self }
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn i2c_txfifo_empty_thrhd(&mut self) -> I2C_TXFIFO_EMPTY_THRHD_W {
        I2C_TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2c_rxfifo_full_thrhd(&mut self) -> I2C_RXFIFO_FULL_THRHD_W {
        I2C_RXFIFO_FULL_THRHD_W { w: self }
    }
}
