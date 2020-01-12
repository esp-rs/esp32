#[doc = "Reader of register TX_DSCR_CONF"]
pub type R = crate::R<u32, super::TX_DSCR_CONF>;
#[doc = "Writer for register TX_DSCR_CONF"]
pub type W = crate::W<u32, super::TX_DSCR_CONF>;
#[doc = "Register TX_DSCR_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_DSCR_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WR_RETRY_THRESHOLD`"]
pub type WR_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WR_RETRY_THRESHOLD`"]
pub struct WR_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RETRY_THRESHOLD_W<'a> {
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
    pub fn wr_retry_threshold(&self) -> WR_RETRY_THRESHOLD_R {
        WR_RETRY_THRESHOLD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn wr_retry_threshold(&mut self) -> WR_RETRY_THRESHOLD_W {
        WR_RETRY_THRESHOLD_W { w: self }
    }
}
