#[doc = "Reader of register LSTIMER1_VALUE"]
pub type R = crate::R<u32, super::LSTIMER1_VALUE>;
#[doc = "Writer for register LSTIMER1_VALUE"]
pub type W = crate::W<u32, super::LSTIMER1_VALUE>;
#[doc = "Register LSTIMER1_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTIMER1_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSTIMER1_CNT`"]
pub type LSTIMER1_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LSTIMER1_CNT`"]
pub struct LSTIMER1_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer1."]
    #[inline(always)]
    pub fn lstimer1_cnt(&self) -> LSTIMER1_CNT_R {
        LSTIMER1_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer1."]
    #[inline(always)]
    pub fn lstimer1_cnt(&mut self) -> LSTIMER1_CNT_W {
        LSTIMER1_CNT_W { w: self }
    }
}
