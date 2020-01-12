#[doc = "Reader of register 0_RXLINK_DSCR"]
pub type R = crate::R<u32, super::_0_RXLINK_DSCR>;
#[doc = "Writer for register 0_RXLINK_DSCR"]
pub type W = crate::W<u32, super::_0_RXLINK_DSCR>;
#[doc = "Register 0_RXLINK_DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_RXLINK_DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC0_RXLINK_DSCR`"]
pub type SLC0_RXLINK_DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC0_RXLINK_DSCR`"]
pub struct SLC0_RXLINK_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rxlink_dscr(&self) -> SLC0_RXLINK_DSCR_R {
        SLC0_RXLINK_DSCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rxlink_dscr(&mut self) -> SLC0_RXLINK_DSCR_W {
        SLC0_RXLINK_DSCR_W { w: self }
    }
}