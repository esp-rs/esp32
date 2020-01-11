#[doc = "Reader of register RS485_CONF"]
pub type R = crate::R<u32, super::RS485_CONF>;
#[doc = "Writer for register RS485_CONF"]
pub type W = crate::W<u32, super::RS485_CONF>;
#[doc = "Register RS485_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RS485_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RS485_TX_DLY_NUM`"]
pub type UART_RS485_TX_DLY_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_RS485_TX_DLY_NUM`"]
pub struct UART_RS485_TX_DLY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_TX_DLY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `UART_RS485_RX_DLY_NUM`"]
pub type UART_RS485_RX_DLY_NUM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_RX_DLY_NUM`"]
pub struct UART_RS485_RX_DLY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_RX_DLY_NUM_W<'a> {
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
#[doc = "Reader of field `UART_RS485RXBY_TX_EN`"]
pub type UART_RS485RXBY_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485RXBY_TX_EN`"]
pub struct UART_RS485RXBY_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485RXBY_TX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART_RS485TX_RX_EN`"]
pub type UART_RS485TX_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485TX_RX_EN`"]
pub struct UART_RS485TX_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485TX_RX_EN_W<'a> {
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
#[doc = "Reader of field `UART_DL1_EN`"]
pub type UART_DL1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DL1_EN`"]
pub struct UART_DL1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DL1_EN_W<'a> {
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
#[doc = "Reader of field `UART_DL0_EN`"]
pub type UART_DL0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DL0_EN`"]
pub struct UART_DL0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DL0_EN_W<'a> {
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
#[doc = "Reader of field `UART_RS485_EN`"]
pub type UART_RS485_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RS485_EN`"]
pub struct UART_RS485_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_EN_W<'a> {
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
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn uart_rs485_tx_dly_num(&self) -> UART_RS485_TX_DLY_NUM_R {
        UART_RS485_TX_DLY_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn uart_rs485_rx_dly_num(&self) -> UART_RS485_RX_DLY_NUM_R {
        UART_RS485_RX_DLY_NUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    pub fn uart_rs485rxby_tx_en(&self) -> UART_RS485RXBY_TX_EN_R {
        UART_RS485RXBY_TX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    pub fn uart_rs485tx_rx_en(&self) -> UART_RS485TX_RX_EN_R {
        UART_RS485TX_RX_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn uart_dl1_en(&self) -> UART_DL1_EN_R {
        UART_DL1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn uart_dl0_en(&self) -> UART_DL0_EN_R {
        UART_DL0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    pub fn uart_rs485_en(&self) -> UART_RS485_EN_R {
        UART_RS485_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn uart_rs485_tx_dly_num(&mut self) -> UART_RS485_TX_DLY_NUM_W {
        UART_RS485_TX_DLY_NUM_W { w: self }
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn uart_rs485_rx_dly_num(&mut self) -> UART_RS485_RX_DLY_NUM_W {
        UART_RS485_RX_DLY_NUM_W { w: self }
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    pub fn uart_rs485rxby_tx_en(&mut self) -> UART_RS485RXBY_TX_EN_W {
        UART_RS485RXBY_TX_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    pub fn uart_rs485tx_rx_en(&mut self) -> UART_RS485TX_RX_EN_W {
        UART_RS485TX_RX_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn uart_dl1_en(&mut self) -> UART_DL1_EN_W {
        UART_DL1_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn uart_dl0_en(&mut self) -> UART_DL0_EN_W {
        UART_DL0_EN_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    pub fn uart_rs485_en(&mut self) -> UART_RS485_EN_W {
        UART_RS485_EN_W { w: self }
    }
}
