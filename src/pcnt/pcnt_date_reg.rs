#[doc = "Reader of register PCNT_DATE_REG"]
pub type R = crate::R<u32, super::PCNT_DATE_REG>;
#[doc = "Writer for register PCNT_DATE_REG"]
pub type W = crate::W<u32, super::PCNT_DATE_REG>;
#[doc = "Register PCNT_DATE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNT_DATE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_DATE`"]
pub type PCNT_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCNT_DATE`"]
pub struct PCNT_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_DATE_W<'a> {
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
    pub fn pcnt_date(&self) -> PCNT_DATE_R {
        PCNT_DATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pcnt_date(&mut self) -> PCNT_DATE_W {
        PCNT_DATE_W { w: self }
    }
}
