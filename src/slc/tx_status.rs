#[doc = "Reader of register TX_STATUS"]
pub type R = crate::R<u32, super::TX_STATUS>;
#[doc = "Writer for register TX_STATUS"]
pub type W = crate::W<u32, super::TX_STATUS>;
#[doc = "Register TX_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC1_TX_EMPTY`"]
pub type SLC1_TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TX_EMPTY`"]
pub struct SLC1_TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SLC1_TX_FULL`"]
pub type SLC1_TX_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TX_FULL`"]
pub struct SLC1_TX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC0_TX_EMPTY`"]
pub type SLC0_TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_EMPTY`"]
pub struct SLC0_TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_EMPTY_W<'a> {
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
#[doc = "Reader of field `SLC0_TX_FULL`"]
pub type SLC0_TX_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_FULL`"]
pub struct SLC0_TX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_FULL_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_empty(&self) -> SLC1_TX_EMPTY_R {
        SLC1_TX_EMPTY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_full(&self) -> SLC1_TX_FULL_R {
        SLC1_TX_FULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_empty(&self) -> SLC0_TX_EMPTY_R {
        SLC0_TX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_full(&self) -> SLC0_TX_FULL_R {
        SLC0_TX_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_empty(&mut self) -> SLC1_TX_EMPTY_W {
        SLC1_TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_full(&mut self) -> SLC1_TX_FULL_W {
        SLC1_TX_FULL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_empty(&mut self) -> SLC0_TX_EMPTY_W {
        SLC0_TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_full(&mut self) -> SLC0_TX_FULL_W {
        SLC0_TX_FULL_W { w: self }
    }
}
