#[doc = "Reader of register TIMG_T0HI_REG"]
pub type R = crate::R<u32, super::TIMG_T0HI_REG>;
#[doc = "Writer for register TIMG_T0HI_REG"]
pub type W = crate::W<u32, super::TIMG_T0HI_REG>;
#[doc = "Register TIMG_T0HI_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_T0HI_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T0_HI`"]
pub type TIMG_T0_HI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_T0_HI`"]
pub struct TIMG_T0_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_HI_W<'a> {
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
    pub fn timg_t0_hi(&self) -> TIMG_T0_HI_R {
        TIMG_T0_HI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to store timer 0 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn timg_t0_hi(&mut self) -> TIMG_T0_HI_W {
        TIMG_T0_HI_W { w: self }
    }
}
