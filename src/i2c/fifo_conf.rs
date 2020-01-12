#[doc = "Reader of register FIFO_CONF"]
pub type R = crate::R<u32, super::FIFO_CONF>;
#[doc = "Writer for register FIFO_CONF"]
pub type W = crate::W<u32, super::FIFO_CONF>;
#[doc = "Register FIFO_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NONFIFO_TX_THRES`"]
pub type NONFIFO_TX_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NONFIFO_TX_THRES`"]
pub struct NONFIFO_TX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_TX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `NONFIFO_RX_THRES`"]
pub type NONFIFO_RX_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NONFIFO_RX_THRES`"]
pub struct NONFIFO_RX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_RX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `TX_FIFO_RST`"]
pub type TX_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FIFO_RST`"]
pub struct TX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RST_W<'a> {
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
#[doc = "Reader of field `RX_FIFO_RST`"]
pub type RX_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FIFO_RST`"]
pub struct RX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RST_W<'a> {
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
#[doc = "Reader of field `FIFO_ADDR_CFG_EN`"]
pub type FIFO_ADDR_CFG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_ADDR_CFG_EN`"]
pub struct FIFO_ADDR_CFG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_ADDR_CFG_EN_W<'a> {
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
#[doc = "Reader of field `NONFIFO_EN`"]
pub type NONFIFO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NONFIFO_EN`"]
pub struct NONFIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_EN_W<'a> {
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
#[doc = "Reader of field `TXFIFO_EMPTY_THRHD`"]
pub type TXFIFO_EMPTY_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFIFO_EMPTY_THRHD`"]
pub struct TXFIFO_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_FULL_THRHD`"]
pub type RXFIFO_FULL_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFIFO_FULL_THRHD`"]
pub struct RXFIFO_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_THRHD_W<'a> {
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
    pub fn nonfifo_tx_thres(&self) -> NONFIFO_TX_THRES_R {
        NONFIFO_TX_THRES_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&self) -> NONFIFO_RX_THRES_R {
        NONFIFO_RX_THRES_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn nonfifo_tx_thres(&mut self) -> NONFIFO_TX_THRES_W {
        NONFIFO_TX_THRES_W { w: self }
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&mut self) -> NONFIFO_RX_THRES_W {
        NONFIFO_RX_THRES_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W {
        TX_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W {
        RX_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W {
        FIFO_ADDR_CFG_EN_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W {
        NONFIFO_EN_W { w: self }
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W {
        TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W {
        RXFIFO_FULL_THRHD_W { w: self }
    }
}
