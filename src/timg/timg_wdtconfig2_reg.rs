#[doc = "Reader of register TIMG_WDTCONFIG2_REG"]
pub type R = crate::R<u32, super::TIMG_WDTCONFIG2_REG>;
#[doc = "Writer for register TIMG_WDTCONFIG2_REG"]
pub type W = crate::W<u32, super::TIMG_WDTCONFIG2_REG>;
#[doc = "Register TIMG_WDTCONFIG2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_WDTCONFIG2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_STG0_HOLD`"]
pub type TIMG_WDT_STG0_HOLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_WDT_STG0_HOLD`"]
pub struct TIMG_WDT_STG0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG0_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Stage 0 timeout value in SWDT clock cycles"]
    #[inline(always)]
    pub fn timg_wdt_stg0_hold(&self) -> TIMG_WDT_STG0_HOLD_R {
        TIMG_WDT_STG0_HOLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stage 0 timeout value in SWDT clock cycles"]
    #[inline(always)]
    pub fn timg_wdt_stg0_hold(&mut self) -> TIMG_WDT_STG0_HOLD_W {
        TIMG_WDT_STG0_HOLD_W { w: self }
    }
}
