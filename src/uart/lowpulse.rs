#[doc = "Reader of register LOWPULSE"]
pub type R = crate::R<u32, super::LOWPULSE>;
#[doc = "Writer for register LOWPULSE"]
pub type W = crate::W<u32, super::LOWPULSE>;
#[doc = "Register LOWPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::LOWPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOWPULSE_MIN_CNT`"]
pub type LOWPULSE_MIN_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOWPULSE_MIN_CNT`"]
pub struct LOWPULSE_MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPULSE_MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the value of the minimum duration time for the low level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn lowpulse_min_cnt(&self) -> LOWPULSE_MIN_CNT_R {
        LOWPULSE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the value of the minimum duration time for the low level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn lowpulse_min_cnt(&mut self) -> LOWPULSE_MIN_CNT_W {
        LOWPULSE_MIN_CNT_W { w: self }
    }
}
