#[doc = "Reader of register DATE"]
pub type R = crate::R<u32, super::DATE>;
#[doc = "Writer for register DATE"]
pub type W = crate::W<u32, super::DATE>;
#[doc = "Register DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTL_DATE`"]
pub type CNTL_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CNTL_DATE`"]
pub struct CNTL_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn cntl_date(&self) -> CNTL_DATE_R {
        CNTL_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn cntl_date(&mut self) -> CNTL_DATE_W {
        CNTL_DATE_W { w: self }
    }
}
