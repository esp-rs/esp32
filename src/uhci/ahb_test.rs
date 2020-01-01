#[doc = "Reader of register AHB_TEST"]
pub type R = crate::R<u32, super::AHB_TEST>;
#[doc = "Writer for register AHB_TEST"]
pub type W = crate::W<u32, super::AHB_TEST>;
#[doc = "Register AHB_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_AHB_TESTADDR`"]
pub type UHCI_AHB_TESTADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_AHB_TESTADDR`"]
pub struct UHCI_AHB_TESTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_AHB_TESTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_AHB_TESTMODE`"]
pub type UHCI_AHB_TESTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_AHB_TESTMODE`"]
pub struct UHCI_AHB_TESTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_AHB_TESTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - The two bits represent ahb bus address bit\\[20:19\\]"]
    #[inline(always)]
    pub fn uhci_ahb_testaddr(&self) -> UHCI_AHB_TESTADDR_R {
        UHCI_AHB_TESTADDR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - bit2 is ahb bus test enable ,bit1 is used to choose wrtie(1) or read(0) mode. bit0 is used to choose test only once(1) or continue(0)"]
    #[inline(always)]
    pub fn uhci_ahb_testmode(&self) -> UHCI_AHB_TESTMODE_R {
        UHCI_AHB_TESTMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - The two bits represent ahb bus address bit\\[20:19\\]"]
    #[inline(always)]
    pub fn uhci_ahb_testaddr(&mut self) -> UHCI_AHB_TESTADDR_W {
        UHCI_AHB_TESTADDR_W { w: self }
    }
    #[doc = "Bits 0:2 - bit2 is ahb bus test enable ,bit1 is used to choose wrtie(1) or read(0) mode. bit0 is used to choose test only once(1) or continue(0)"]
    #[inline(always)]
    pub fn uhci_ahb_testmode(&mut self) -> UHCI_AHB_TESTMODE_W {
        UHCI_AHB_TESTMODE_W { w: self }
    }
}
