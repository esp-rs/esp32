#[doc = "Reader of register STATUS_W1TS"]
pub type R = crate::R<u32, super::STATUS_W1TS>;
#[doc = "Writer for register STATUS_W1TS"]
pub type W = crate::W<u32, super::STATUS_W1TS>;
#[doc = "Register STATUS_W1TS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS_W1TS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS_INT_W1TS`"]
pub type STATUS_INT_W1TS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STATUS_INT_W1TS`"]
pub struct STATUS_INT_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_INT_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&self) -> STATUS_INT_W1TS_R {
        STATUS_INT_W1TS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&mut self) -> STATUS_INT_W1TS_W {
        STATUS_INT_W1TS_W { w: self }
    }
}
