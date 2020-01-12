#[doc = "Reader of register CACHE_MUX_MODE"]
pub type R = crate::R<u32, super::CACHE_MUX_MODE>;
#[doc = "Writer for register CACHE_MUX_MODE"]
pub type W = crate::W<u32, super::CACHE_MUX_MODE>;
#[doc = "Register CACHE_MUX_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHE_MUX_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CACHE_MUX_MODE`"]
pub type CACHE_MUX_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CACHE_MUX_MODE`"]
pub struct CACHE_MUX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MUX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&self) -> CACHE_MUX_MODE_R {
        CACHE_MUX_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&mut self) -> CACHE_MUX_MODE_W {
        CACHE_MUX_MODE_W { w: self }
    }
}
