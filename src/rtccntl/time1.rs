#[doc = "Reader of register TIME1"]
pub type R = crate::R<u32, super::TIME1>;
#[doc = "Writer for register TIME1"]
pub type W = crate::W<u32, super::TIME1>;
#[doc = "Register TIME1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIME1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIME_HI`"]
pub type TIME_HI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIME_HI`"]
pub struct TIME_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn time_hi(&self) -> TIME_HI_R {
        TIME_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn time_hi(&mut self) -> TIME_HI_W {
        TIME_HI_W { w: self }
    }
}
