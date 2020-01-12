#[doc = "Reader of register T1UPDATE"]
pub type R = crate::R<u32, super::T1UPDATE>;
#[doc = "Writer for register T1UPDATE"]
pub type W = crate::W<u32, super::T1UPDATE>;
#[doc = "Register T1UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::T1UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T1_UPDATE`"]
pub type T1_UPDATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T1_UPDATE`"]
pub struct T1_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_UPDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 1 time-base counter value update (timer 1 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn t1_update(&self) -> T1_UPDATE_R {
        T1_UPDATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 1 time-base counter value update (timer 1 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn t1_update(&mut self) -> T1_UPDATE_W {
        T1_UPDATE_W { w: self }
    }
}
