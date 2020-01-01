#[doc = "Reader of register T0UPDATE"]
pub type R = crate::R<u32, super::T0UPDATE>;
#[doc = "Writer for register T0UPDATE"]
pub type W = crate::W<u32, super::T0UPDATE>;
#[doc = "Register T0UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::T0UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T0_UPDATE`"]
pub type TIMG_T0_UPDATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_T0_UPDATE`"]
pub struct TIMG_T0_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_UPDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn timg_t0_update(&self) -> TIMG_T0_UPDATE_R {
        TIMG_T0_UPDATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn timg_t0_update(&mut self) -> TIMG_T0_UPDATE_W {
        TIMG_T0_UPDATE_W { w: self }
    }
}
