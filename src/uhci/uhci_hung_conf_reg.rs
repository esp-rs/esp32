#[doc = "Reader of register UHCI_HUNG_CONF_REG"]
pub type R = crate::R<u32, super::UHCI_HUNG_CONF_REG>;
#[doc = "Writer for register UHCI_HUNG_CONF_REG"]
pub type W = crate::W<u32, super::UHCI_HUNG_CONF_REG>;
#[doc = "Register UHCI_HUNG_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_HUNG_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_RXFIFO_TIMEOUT_ENA`"]
pub type UHCI_RXFIFO_TIMEOUT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RXFIFO_TIMEOUT_ENA`"]
pub struct UHCI_RXFIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RXFIFO_TIMEOUT_ENA_W<'a> {
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
#[doc = "Reader of field `UHCI_RXFIFO_TIMEOUT_SHIFT`"]
pub type UHCI_RXFIFO_TIMEOUT_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_RXFIFO_TIMEOUT_SHIFT`"]
pub struct UHCI_RXFIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RXFIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `UHCI_RXFIFO_TIMEOUT`"]
pub type UHCI_RXFIFO_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_RXFIFO_TIMEOUT`"]
pub struct UHCI_RXFIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RXFIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TXFIFO_TIMEOUT_ENA`"]
pub type UHCI_TXFIFO_TIMEOUT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TXFIFO_TIMEOUT_ENA`"]
pub struct UHCI_TXFIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TXFIFO_TIMEOUT_ENA_W<'a> {
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
#[doc = "Reader of field `UHCI_TXFIFO_TIMEOUT_SHIFT`"]
pub type UHCI_TXFIFO_TIMEOUT_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_TXFIFO_TIMEOUT_SHIFT`"]
pub struct UHCI_TXFIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TXFIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TXFIFO_TIMEOUT`"]
pub type UHCI_TXFIFO_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_TXFIFO_TIMEOUT`"]
pub struct UHCI_TXFIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TXFIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout_ena(&self) -> UHCI_RXFIFO_TIMEOUT_ENA_R {
        UHCI_RXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout_shift(&self) -> UHCI_RXFIFO_TIMEOUT_SHIFT_R {
        UHCI_RXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout(&self) -> UHCI_RXFIFO_TIMEOUT_R {
        UHCI_RXFIFO_TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    pub fn uhci_txfifo_timeout_ena(&self) -> UHCI_TXFIFO_TIMEOUT_ENA_R {
        UHCI_TXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    pub fn uhci_txfifo_timeout_shift(&self) -> UHCI_TXFIFO_TIMEOUT_SHIFT_R {
        UHCI_TXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn uhci_txfifo_timeout(&self) -> UHCI_TXFIFO_TIMEOUT_R {
        UHCI_TXFIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout_ena(&mut self) -> UHCI_RXFIFO_TIMEOUT_ENA_W {
        UHCI_RXFIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout_shift(&mut self) -> UHCI_RXFIFO_TIMEOUT_SHIFT_W {
        UHCI_RXFIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn uhci_rxfifo_timeout(&mut self) -> UHCI_RXFIFO_TIMEOUT_W {
        UHCI_RXFIFO_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    pub fn uhci_txfifo_timeout_ena(&mut self) -> UHCI_TXFIFO_TIMEOUT_ENA_W {
        UHCI_TXFIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    pub fn uhci_txfifo_timeout_shift(&mut self) -> UHCI_TXFIFO_TIMEOUT_SHIFT_W {
        UHCI_TXFIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn uhci_txfifo_timeout(&mut self) -> UHCI_TXFIFO_TIMEOUT_W {
        UHCI_TXFIFO_TIMEOUT_W { w: self }
    }
}
