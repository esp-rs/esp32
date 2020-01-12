#[doc = "Reader of register CONF1"]
pub type R = crate::R<u32, super::CONF1>;
#[doc = "Writer for register CONF1"]
pub type W = crate::W<u32, super::CONF1>;
#[doc = "Register CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_TOUT_EN`"]
pub type RX_TOUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TOUT_EN`"]
pub struct RX_TOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_EN_W<'a> {
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
#[doc = "Reader of field `RX_TOUT_THRHD`"]
pub type RX_TOUT_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TOUT_THRHD`"]
pub struct RX_TOUT_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RX_FLOW_EN`"]
pub type RX_FLOW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FLOW_EN`"]
pub struct RX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_EN_W<'a> {
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
#[doc = "Reader of field `RX_FLOW_THRHD`"]
pub type RX_FLOW_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FLOW_THRHD`"]
pub struct RX_FLOW_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
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
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
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
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W {
        RX_TOUT_EN_W { w: self }
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W {
        RX_TOUT_THRHD_W { w: self }
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W {
        RX_FLOW_EN_W { w: self }
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W {
        RX_FLOW_THRHD_W { w: self }
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W {
        TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W {
        RXFIFO_FULL_THRHD_W { w: self }
    }
}
