#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXD`"]
pub type TXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
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
#[doc = "Reader of field `RTSN`"]
pub type RTSN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSN`"]
pub struct RTSN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DTRN`"]
pub type DTRN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTRN`"]
pub struct DTRN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ST_UTX_OUT`"]
pub type ST_UTX_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST_UTX_OUT`"]
pub struct ST_UTX_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_UTX_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TXFIFO_CNT`"]
pub type TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFIFO_CNT`"]
pub struct TXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXD`"]
pub struct RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CTSN`"]
pub type CTSN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSN`"]
pub struct CTSN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DSRN`"]
pub type DSRN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRN`"]
pub struct DSRN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRN_W<'a> {
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
#[doc = "Reader of field `ST_URX_OUT`"]
pub type ST_URX_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST_URX_OUT`"]
pub struct ST_URX_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_URX_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_CNT`"]
pub type RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFIFO_CNT`"]
pub struct RXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
    #[inline(always)]
    pub fn st_utx_out(&self) -> ST_UTX_OUT_R {
        ST_UTX_OUT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - This register stores the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This register stores the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This register stores the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
    #[inline(always)]
    pub fn st_urx_out(&self) -> ST_URX_OUT_R {
        ST_URX_OUT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
    #[doc = "Bit 30 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn rtsn(&mut self) -> RTSN_W {
        RTSN_W { w: self }
    }
    #[doc = "Bit 29 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dtrn(&mut self) -> DTRN_W {
        DTRN_W { w: self }
    }
    #[doc = "Bits 24:27 - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
    #[inline(always)]
    pub fn st_utx_out(&mut self) -> ST_UTX_OUT_W {
        ST_UTX_OUT_W { w: self }
    }
    #[doc = "Bits 16:23 - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn txfifo_cnt(&mut self) -> TXFIFO_CNT_W {
        TXFIFO_CNT_W { w: self }
    }
    #[doc = "Bit 15 - This register stores the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn rxd(&mut self) -> RXD_W {
        RXD_W { w: self }
    }
    #[doc = "Bit 14 - This register stores the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn ctsn(&mut self) -> CTSN_W {
        CTSN_W { w: self }
    }
    #[doc = "Bit 13 - This register stores the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dsrn(&mut self) -> DSRN_W {
        DSRN_W { w: self }
    }
    #[doc = "Bits 8:11 - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
    #[inline(always)]
    pub fn st_urx_out(&mut self) -> ST_URX_OUT_W {
        ST_URX_OUT_W { w: self }
    }
    #[doc = "Bits 0:7 - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn rxfifo_cnt(&mut self) -> RXFIFO_CNT_W {
        RXFIFO_CNT_W { w: self }
    }
}
