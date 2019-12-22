#[doc = "Reader of register UHCI_DMA_OUT_STATUS_REG"]
pub type R = crate::R<u32, super::UHCI_DMA_OUT_STATUS_REG>;
#[doc = "Writer for register UHCI_DMA_OUT_STATUS_REG"]
pub type W = crate::W<u32, super::UHCI_DMA_OUT_STATUS_REG>;
#[doc = "Register UHCI_DMA_OUT_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_DMA_OUT_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_OUT_EMPTY`"]
pub type UHCI_OUT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_EMPTY`"]
pub struct UHCI_OUT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_EMPTY_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_FULL`"]
pub type UHCI_OUT_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_FULL`"]
pub struct UHCI_OUT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_FULL_W<'a> {
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
    #[doc = "Bit 1 - 1:DMA in link descriptor's fifo is empty."]
    #[inline(always)]
    pub fn uhci_out_empty(&self) -> UHCI_OUT_EMPTY_R {
        UHCI_OUT_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1:DMA out link descriptor's fifo is full."]
    #[inline(always)]
    pub fn uhci_out_full(&self) -> UHCI_OUT_FULL_R {
        UHCI_OUT_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:DMA in link descriptor's fifo is empty."]
    #[inline(always)]
    pub fn uhci_out_empty(&mut self) -> UHCI_OUT_EMPTY_W {
        UHCI_OUT_EMPTY_W { w: self }
    }
    #[doc = "Bit 0 - 1:DMA out link descriptor's fifo is full."]
    #[inline(always)]
    pub fn uhci_out_full(&mut self) -> UHCI_OUT_FULL_W {
        UHCI_OUT_FULL_W { w: self }
    }
}
