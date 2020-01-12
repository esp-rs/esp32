#[doc = "Reader of register CAP_CH2"]
pub type R = crate::R<u32, super::CAP_CH2>;
#[doc = "Writer for register CAP_CH2"]
pub type W = crate::W<u32, super::CAP_CH2>;
#[doc = "Register CAP_CH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_CH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP2_VALUE`"]
pub type CAP2_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CAP2_VALUE`"]
pub struct CAP2_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of last capture on channel 2"]
    #[inline(always)]
    pub fn cap2_value(&self) -> CAP2_VALUE_R {
        CAP2_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of last capture on channel 2"]
    #[inline(always)]
    pub fn cap2_value(&mut self) -> CAP2_VALUE_W {
        CAP2_VALUE_W { w: self }
    }
}
