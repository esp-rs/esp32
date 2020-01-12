#[doc = "Reader of register HSCH6_HPOINT"]
pub type R = crate::R<u32, super::HSCH6_HPOINT>;
#[doc = "Writer for register HSCH6_HPOINT"]
pub type W = crate::W<u32, super::HSCH6_HPOINT>;
#[doc = "Register HSCH6_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH6_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_HSCH6`"]
pub type HPOINT_HSCH6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HPOINT_HSCH6`"]
pub struct HPOINT_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_HSCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch6(&self) -> HPOINT_HSCH6_R {
        HPOINT_HSCH6_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch6(&mut self) -> HPOINT_HSCH6_W {
        HPOINT_HSCH6_W { w: self }
    }
}
