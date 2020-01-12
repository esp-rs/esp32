#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS_INT`"]
pub type STATUS_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STATUS_INT`"]
pub struct STATUS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status"]
    #[inline(always)]
    pub fn status_int(&self) -> STATUS_INT_R {
        STATUS_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status"]
    #[inline(always)]
    pub fn status_int(&mut self) -> STATUS_INT_W {
        STATUS_INT_W { w: self }
    }
}
