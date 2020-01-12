#[doc = "Reader of register DMA_IN_STATUS"]
pub type R = crate::R<u32, super::DMA_IN_STATUS>;
#[doc = "Writer for register DMA_IN_STATUS"]
pub type W = crate::W<u32, super::DMA_IN_STATUS>;
#[doc = "Register DMA_IN_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_ERR_CAUSE`"]
pub type RX_ERR_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_ERR_CAUSE`"]
pub struct RX_ERR_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `IN_EMPTY`"]
pub type IN_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EMPTY`"]
pub struct IN_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EMPTY_W<'a> {
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
#[doc = "Reader of field `IN_FULL`"]
pub type IN_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_FULL`"]
pub struct IN_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_FULL_W<'a> {
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
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn in_empty(&self) -> IN_EMPTY_R {
        IN_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn in_full(&self) -> IN_FULL_R {
        IN_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - This register stores the errors caused in out link descriptor's data packet."]
    #[inline(always)]
    pub fn rx_err_cause(&mut self) -> RX_ERR_CAUSE_W {
        RX_ERR_CAUSE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn in_empty(&mut self) -> IN_EMPTY_W {
        IN_EMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn in_full(&mut self) -> IN_FULL_W {
        IN_FULL_W { w: self }
    }
}
