#[doc = "Reader of register 0_DSCR_REC_CONF"]
pub type R = crate::R<u32, super::_0_DSCR_REC_CONF>;
#[doc = "Writer for register 0_DSCR_REC_CONF"]
pub type W = crate::W<u32, super::_0_DSCR_REC_CONF>;
#[doc = "Register 0_DSCR_REC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_DSCR_REC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_RX_DSCR_REC_LIM`"]
pub type SLC_SLC0_RX_DSCR_REC_LIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC0_RX_DSCR_REC_LIM`"]
pub struct SLC_SLC0_RX_DSCR_REC_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_DSCR_REC_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_rec_lim(&self) -> SLC_SLC0_RX_DSCR_REC_LIM_R {
        SLC_SLC0_RX_DSCR_REC_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_rec_lim(&mut self) -> SLC_SLC0_RX_DSCR_REC_LIM_W {
        SLC_SLC0_RX_DSCR_REC_LIM_W { w: self }
    }
}
