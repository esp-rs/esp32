#[doc = "Reader of register STATUS1_W1TS"]
pub type R = crate::R<u32, super::STATUS1_W1TS>;
#[doc = "Writer for register STATUS1_W1TS"]
pub type W = crate::W<u32, super::STATUS1_W1TS>;
#[doc = "Register STATUS1_W1TS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS1_W1TS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS1_INT_W1TS`"]
pub type STATUS1_INT_W1TS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATUS1_INT_W1TS`"]
pub struct STATUS1_INT_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS1_INT_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status1_int_w1ts(&self) -> STATUS1_INT_W1TS_R {
        STATUS1_INT_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status1_int_w1ts(&mut self) -> STATUS1_INT_W1TS_W {
        STATUS1_INT_W1TS_W { w: self }
    }
}
