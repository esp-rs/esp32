#[doc = "Reader of register DEC_STATUS"]
pub type R = crate::R<u32, super::DEC_STATUS>;
#[doc = "Writer for register DEC_STATUS"]
pub type W = crate::W<u32, super::DEC_STATUS>;
#[doc = "Register DEC_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DEC_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEC_WARNINGS`"]
pub type DEC_WARNINGS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEC_WARNINGS`"]
pub struct DEC_WARNINGS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_WARNINGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - the decode result of 3/4 coding scheme has warning"]
    #[inline(always)]
    pub fn dec_warnings(&self) -> DEC_WARNINGS_R {
        DEC_WARNINGS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - the decode result of 3/4 coding scheme has warning"]
    #[inline(always)]
    pub fn dec_warnings(&mut self) -> DEC_WARNINGS_W {
        DEC_WARNINGS_W { w: self }
    }
}
