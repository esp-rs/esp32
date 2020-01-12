#[doc = "Reader of register T0HI"]
pub type R = crate::R<u32, super::T0HI>;
#[doc = "Writer for register T0HI"]
pub type W = crate::W<u32, super::T0HI>;
#[doc = "Register T0HI `reset()`'s with value 0"]
impl crate::ResetValue for super::T0HI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T0_HI`"]
pub type T0_HI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T0_HI`"]
pub struct T0_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register to store timer 0 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn t0_hi(&self) -> T0_HI_R {
        T0_HI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to store timer 0 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn t0_hi(&mut self) -> T0_HI_W {
        T0_HI_W { w: self }
    }
}