#[doc = "Reader of register WDTWPROTECT"]
pub type R = crate::R<u32, super::WDTWPROTECT>;
#[doc = "Writer for register WDTWPROTECT"]
pub type W = crate::W<u32, super::WDTWPROTECT>;
#[doc = "Register WDTWPROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTWPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_WKEY`"]
pub type TIMG_WDT_WKEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_WDT_WKEY`"]
pub struct TIMG_WDT_WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_WKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - If change its value from default then write protection is on."]
    #[inline(always)]
    pub fn timg_wdt_wkey(&self) -> TIMG_WDT_WKEY_R {
        TIMG_WDT_WKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - If change its value from default then write protection is on."]
    #[inline(always)]
    pub fn timg_wdt_wkey(&mut self) -> TIMG_WDT_WKEY_W {
        TIMG_WDT_WKEY_W { w: self }
    }
}
