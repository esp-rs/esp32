#[doc = "Reader of register LSTIMER3_VALUE"]
pub type R = crate::R<u32, super::LSTIMER3_VALUE>;
#[doc = "Writer for register LSTIMER3_VALUE"]
pub type W = crate::W<u32, super::LSTIMER3_VALUE>;
#[doc = "Register LSTIMER3_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTIMER3_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSTIMER3_CNT`"]
pub type LSTIMER3_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LSTIMER3_CNT`"]
pub struct LSTIMER3_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer3."]
    #[inline(always)]
    pub fn lstimer3_cnt(&self) -> LSTIMER3_CNT_R {
        LSTIMER3_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer3."]
    #[inline(always)]
    pub fn lstimer3_cnt(&mut self) -> LSTIMER3_CNT_W {
        LSTIMER3_CNT_W { w: self }
    }
}
