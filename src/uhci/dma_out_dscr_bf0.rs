#[doc = "Reader of register DMA_OUT_DSCR_BF0"]
pub type R = crate::R<u32, super::DMA_OUT_DSCR_BF0>;
#[doc = "Writer for register DMA_OUT_DSCR_BF0"]
pub type W = crate::W<u32, super::DMA_OUT_DSCR_BF0>;
#[doc = "Register DMA_OUT_DSCR_BF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_OUT_DSCR_BF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_OUTLINK_DSCR_BF0`"]
pub type UHCI_OUTLINK_DSCR_BF0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_OUTLINK_DSCR_BF0`"]
pub struct UHCI_OUTLINK_DSCR_BF0_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_DSCR_BF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current out link descriptor's first dword"]
    #[inline(always)]
    pub fn uhci_outlink_dscr_bf0(&self) -> UHCI_OUTLINK_DSCR_BF0_R {
        UHCI_OUTLINK_DSCR_BF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of current out link descriptor's first dword"]
    #[inline(always)]
    pub fn uhci_outlink_dscr_bf0(&mut self) -> UHCI_OUTLINK_DSCR_BF0_W {
        UHCI_OUTLINK_DSCR_BF0_W { w: self }
    }
}
