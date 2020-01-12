#[doc = "Reader of register HIGHPULSE"]
pub type R = crate::R<u32, super::HIGHPULSE>;
#[doc = "Writer for register HIGHPULSE"]
pub type W = crate::W<u32, super::HIGHPULSE>;
#[doc = "Register HIGHPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::HIGHPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIGHPULSE_MIN_CNT`"]
pub type HIGHPULSE_MIN_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIGHPULSE_MIN_CNT`"]
pub struct HIGHPULSE_MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHPULSE_MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the value of the maxinum duration time for the high level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn highpulse_min_cnt(&self) -> HIGHPULSE_MIN_CNT_R {
        HIGHPULSE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the value of the maxinum duration time for the high level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn highpulse_min_cnt(&mut self) -> HIGHPULSE_MIN_CNT_W {
        HIGHPULSE_MIN_CNT_W { w: self }
    }
}
