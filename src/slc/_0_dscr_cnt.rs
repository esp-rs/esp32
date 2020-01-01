#[doc = "Reader of register 0_DSCR_CNT"]
pub type R = crate::R<u32, super::_0_DSCR_CNT>;
#[doc = "Writer for register 0_DSCR_CNT"]
pub type W = crate::W<u32, super::_0_DSCR_CNT>;
#[doc = "Register 0_DSCR_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_DSCR_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_RX_GET_EOF_OCC`"]
pub type SLC_SLC0_RX_GET_EOF_OCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_GET_EOF_OCC`"]
pub struct SLC_SLC0_RX_GET_EOF_OCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_GET_EOF_OCC_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_DSCR_CNT_LAT`"]
pub type SLC_SLC0_RX_DSCR_CNT_LAT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC0_RX_DSCR_CNT_LAT`"]
pub struct SLC_SLC0_RX_DSCR_CNT_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_DSCR_CNT_LAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc0_rx_get_eof_occ(&self) -> SLC_SLC0_RX_GET_EOF_OCC_R {
        SLC_SLC0_RX_GET_EOF_OCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_cnt_lat(&self) -> SLC_SLC0_RX_DSCR_CNT_LAT_R {
        SLC_SLC0_RX_DSCR_CNT_LAT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc0_rx_get_eof_occ(&mut self) -> SLC_SLC0_RX_GET_EOF_OCC_W {
        SLC_SLC0_RX_GET_EOF_OCC_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_cnt_lat(&mut self) -> SLC_SLC0_RX_DSCR_CNT_LAT_W {
        SLC_SLC0_RX_DSCR_CNT_LAT_W { w: self }
    }
}
