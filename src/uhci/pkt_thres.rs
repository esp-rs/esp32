#[doc = "Reader of register PKT_THRES"]
pub type R = crate::R<u32, super::PKT_THRES>;
#[doc = "Writer for register PKT_THRES"]
pub type W = crate::W<u32, super::PKT_THRES>;
#[doc = "Register PKT_THRES `reset()`'s with value 0"]
impl crate::ResetValue for super::PKT_THRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_PKT_THRS`"]
pub type UHCI_PKT_THRS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UHCI_PKT_THRS`"]
pub struct UHCI_PKT_THRS_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_PKT_THRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done."]
    #[inline(always)]
    pub fn uhci_pkt_thrs(&self) -> UHCI_PKT_THRS_R {
        UHCI_PKT_THRS_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done."]
    #[inline(always)]
    pub fn uhci_pkt_thrs(&mut self) -> UHCI_PKT_THRS_W {
        UHCI_PKT_THRS_W { w: self }
    }
}
