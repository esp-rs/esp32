#[doc = "Reader of register SLC_RX_STATUS_REG"]
pub type R = crate::R<u32, super::SLC_RX_STATUS_REG>;
#[doc = "Writer for register SLC_RX_STATUS_REG"]
pub type W = crate::W<u32, super::SLC_RX_STATUS_REG>;
#[doc = "Register SLC_RX_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_RX_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_RX_EMPTY`"]
pub type SLC_SLC1_RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_EMPTY`"]
pub struct SLC_SLC1_RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_EMPTY_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_FULL`"]
pub type SLC_SLC1_RX_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_FULL`"]
pub struct SLC_SLC1_RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_FULL_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_EMPTY`"]
pub type SLC_SLC0_RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_EMPTY`"]
pub struct SLC_SLC0_RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_EMPTY_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_FULL`"]
pub type SLC_SLC0_RX_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_FULL`"]
pub struct SLC_SLC0_RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_FULL_W<'a> {
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
    pub fn slc_slc1_rx_empty(&self) -> SLC_SLC1_RX_EMPTY_R {
        SLC_SLC1_RX_EMPTY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_rx_full(&self) -> SLC_SLC1_RX_FULL_R {
        SLC_SLC1_RX_FULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_rx_empty(&self) -> SLC_SLC0_RX_EMPTY_R {
        SLC_SLC0_RX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_rx_full(&self) -> SLC_SLC0_RX_FULL_R {
        SLC_SLC0_RX_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_rx_empty(&mut self) -> SLC_SLC1_RX_EMPTY_W {
        SLC_SLC1_RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_rx_full(&mut self) -> SLC_SLC1_RX_FULL_W {
        SLC_SLC1_RX_FULL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_rx_empty(&mut self) -> SLC_SLC0_RX_EMPTY_W {
        SLC_SLC0_RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_rx_full(&mut self) -> SLC_SLC0_RX_FULL_W {
        SLC_SLC0_RX_FULL_W { w: self }
    }
}
