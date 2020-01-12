#[doc = "Reader of register HSCH4_HPOINT"]
pub type R = crate::R<u32, super::HSCH4_HPOINT>;
#[doc = "Writer for register HSCH4_HPOINT"]
pub type W = crate::W<u32, super::HSCH4_HPOINT>;
#[doc = "Register HSCH4_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH4_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_HSCH4`"]
pub type HPOINT_HSCH4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HPOINT_HSCH4`"]
pub struct HPOINT_HSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_HSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel4 has reached reg_hpoint_hsch4\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch4(&self) -> HPOINT_HSCH4_R {
        HPOINT_HSCH4_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel4 has reached reg_hpoint_hsch4\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch4(&mut self) -> HPOINT_HSCH4_W {
        HPOINT_HSCH4_W { w: self }
    }
}
