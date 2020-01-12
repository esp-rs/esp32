#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Writer for register STATE"]
pub type W = crate::W<u32, super::STATE>;
#[doc = "Register STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_FIFO_RESET_BACK`"]
pub type RX_FIFO_RESET_BACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FIFO_RESET_BACK`"]
pub struct RX_FIFO_RESET_BACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RESET_BACK_W<'a> {
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
#[doc = "Reader of field `TX_FIFO_RESET_BACK`"]
pub type TX_FIFO_RESET_BACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FIFO_RESET_BACK`"]
pub struct TX_FIFO_RESET_BACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RESET_BACK_W<'a> {
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
#[doc = "Reader of field `TX_IDLE`"]
pub type TX_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_IDLE`"]
pub struct TX_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IDLE_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_reset_back(&self) -> RX_FIFO_RESET_BACK_R {
        RX_FIFO_RESET_BACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset_back(&self) -> TX_FIFO_RESET_BACK_R {
        TX_FIFO_RESET_BACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_reset_back(&mut self) -> RX_FIFO_RESET_BACK_W {
        RX_FIFO_RESET_BACK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset_back(&mut self) -> TX_FIFO_RESET_BACK_W {
        TX_FIFO_RESET_BACK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&mut self) -> TX_IDLE_W {
        TX_IDLE_W { w: self }
    }
}
