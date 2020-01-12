#[doc = "Reader of register LSCH2_HPOINT"]
pub type R = crate::R<u32, super::LSCH2_HPOINT>;
#[doc = "Writer for register LSCH2_HPOINT"]
pub type W = crate::W<u32, super::LSCH2_HPOINT>;
#[doc = "Register LSCH2_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH2_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_LSCH2`"]
pub type HPOINT_LSCH2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HPOINT_LSCH2`"]
pub struct HPOINT_LSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_LSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel2 has reached reg_hpoint_lsch2\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch2(&self) -> HPOINT_LSCH2_R {
        HPOINT_LSCH2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel2 has reached reg_hpoint_lsch2\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch2(&mut self) -> HPOINT_LSCH2_W {
        HPOINT_LSCH2_W { w: self }
    }
}
