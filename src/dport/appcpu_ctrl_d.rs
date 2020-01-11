#[doc = "Reader of register APPCPU_CTRL_D"]
pub type R = crate::R<u32, super::APPCPU_CTRL_D>;
#[doc = "Writer for register APPCPU_CTRL_D"]
pub type W = crate::W<u32, super::APPCPU_CTRL_D>;
#[doc = "Register APPCPU_CTRL_D `reset()`'s with value 0"]
impl crate::ResetValue for super::APPCPU_CTRL_D {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APPCPU_BOOT_ADDR`"]
pub type DPORT_APPCPU_BOOT_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_APPCPU_BOOT_ADDR`"]
pub struct DPORT_APPCPU_BOOT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APPCPU_BOOT_ADDR_W<'a> {
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
    pub fn dport_appcpu_boot_addr(&self) -> DPORT_APPCPU_BOOT_ADDR_R {
        DPORT_APPCPU_BOOT_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_appcpu_boot_addr(&mut self) -> DPORT_APPCPU_BOOT_ADDR_W {
        DPORT_APPCPU_BOOT_ADDR_W { w: self }
    }
}
