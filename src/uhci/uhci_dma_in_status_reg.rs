#[doc = "Reader of register UHCI_DMA_IN_STATUS_REG"]
pub type R = crate::R<u32, super::UHCI_DMA_IN_STATUS_REG>;
#[doc = "Writer for register UHCI_DMA_IN_STATUS_REG"]
pub type W = crate::W<u32, super::UHCI_DMA_IN_STATUS_REG>;
#[doc = "Register UHCI_DMA_IN_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_DMA_IN_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_RX_ERR_CAUSE`"]
pub type UHCI_RX_ERR_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_RX_ERR_CAUSE`"]
pub struct UHCI_RX_ERR_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_ERR_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_IN_EMPTY`"]
pub type UHCI_IN_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_EMPTY`"]
pub struct UHCI_IN_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_EMPTY_W<'a> {
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
#[doc = "Reader of field `UHCI_IN_FULL`"]
pub type UHCI_IN_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_FULL`"]
pub struct UHCI_IN_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_FULL_W<'a> {
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
    #[doc = "Bits 4:6 - This register stores the errors caused in out link descriptor's data packet."]
    #[inline(always)]
    pub fn uhci_rx_err_cause(&self) -> UHCI_RX_ERR_CAUSE_R {
        UHCI_RX_ERR_CAUSE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_in_empty(&self) -> UHCI_IN_EMPTY_R {
        UHCI_IN_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_in_full(&self) -> UHCI_IN_FULL_R {
        UHCI_IN_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - This register stores the errors caused in out link descriptor's data packet."]
    #[inline(always)]
    pub fn uhci_rx_err_cause(&mut self) -> UHCI_RX_ERR_CAUSE_W {
        UHCI_RX_ERR_CAUSE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_in_empty(&mut self) -> UHCI_IN_EMPTY_W {
        UHCI_IN_EMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_in_full(&mut self) -> UHCI_IN_FULL_W {
        UHCI_IN_FULL_W { w: self }
    }
}
