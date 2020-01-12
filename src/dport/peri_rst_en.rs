#[doc = "Reader of register PERI_RST_EN"]
pub type R = crate::R<u32, super::PERI_RST_EN>;
#[doc = "Writer for register PERI_RST_EN"]
pub type W = crate::W<u32, super::PERI_RST_EN>;
#[doc = "Register PERI_RST_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::PERI_RST_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERI_RST_EN`"]
pub type PERI_RST_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERI_RST_EN`"]
pub struct PERI_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_RST_EN_W<'a> {
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
    pub fn peri_rst_en(&self) -> PERI_RST_EN_R {
        PERI_RST_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_rst_en(&mut self) -> PERI_RST_EN_W {
        PERI_RST_EN_W { w: self }
    }
}
