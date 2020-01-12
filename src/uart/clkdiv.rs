#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV_FRAG`"]
pub type CLKDIV_FRAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV_FRAG`"]
pub struct CLKDIV_FRAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_FRAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - The register value is the decimal part of the frequency divider's factor."]
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> CLKDIV_FRAG_R {
        CLKDIV_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 0:19 - The register value is the integer part of the frequency divider's factor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:23 - The register value is the decimal part of the frequency divider's factor."]
    #[inline(always)]
    pub fn clkdiv_frag(&mut self) -> CLKDIV_FRAG_W {
        CLKDIV_FRAG_W { w: self }
    }
    #[doc = "Bits 0:19 - The register value is the integer part of the frequency divider's factor."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
}
