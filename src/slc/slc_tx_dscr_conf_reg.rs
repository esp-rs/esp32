#[doc = "Reader of register SLC_TX_DSCR_CONF_REG"]
pub type R = crate::R<u32, super::SLC_TX_DSCR_CONF_REG>;
#[doc = "Writer for register SLC_TX_DSCR_CONF_REG"]
pub type W = crate::W<u32, super::SLC_TX_DSCR_CONF_REG>;
#[doc = "Register SLC_TX_DSCR_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_TX_DSCR_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_WR_RETRY_THRESHOLD`"]
pub type SLC_WR_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_WR_RETRY_THRESHOLD`"]
pub struct SLC_WR_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_WR_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc_wr_retry_threshold(&self) -> SLC_WR_RETRY_THRESHOLD_R {
        SLC_WR_RETRY_THRESHOLD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc_wr_retry_threshold(&mut self) -> SLC_WR_RETRY_THRESHOLD_W {
        SLC_WR_RETRY_THRESHOLD_W { w: self }
    }
}
