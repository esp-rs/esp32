#[doc = "Reader of register CORE_RST_EN"]
pub type R = crate::R<u32, super::CORE_RST_EN>;
#[doc = "Writer for register CORE_RST_EN"]
pub type W = crate::W<u32, super::CORE_RST_EN>;
#[doc = "Register CORE_RST_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::CORE_RST_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_CORE_RST`"]
pub type DPORT_CORE_RST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_CORE_RST`"]
pub struct DPORT_CORE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CORE_RST_W<'a> {
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
    pub fn dport_core_rst(&self) -> DPORT_CORE_RST_R {
        DPORT_CORE_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_core_rst(&mut self) -> DPORT_CORE_RST_W {
        DPORT_CORE_RST_W { w: self }
    }
}
