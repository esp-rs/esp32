#[doc = "Reader of register NTIMERS_DATE"]
pub type R = crate::R<u32, super::NTIMERS_DATE>;
#[doc = "Writer for register NTIMERS_DATE"]
pub type W = crate::W<u32, super::NTIMERS_DATE>;
#[doc = "Register NTIMERS_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::NTIMERS_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_NTIMERS_DATE`"]
pub type TIMG_NTIMERS_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_NTIMERS_DATE`"]
pub struct TIMG_NTIMERS_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_NTIMERS_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Version of this regfile"]
    #[inline(always)]
    pub fn timg_ntimers_date(&self) -> TIMG_NTIMERS_DATE_R {
        TIMG_NTIMERS_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version of this regfile"]
    #[inline(always)]
    pub fn timg_ntimers_date(&mut self) -> TIMG_NTIMERS_DATE_W {
        TIMG_NTIMERS_DATE_W { w: self }
    }
}
