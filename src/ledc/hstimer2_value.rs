#[doc = "Reader of register HSTIMER2_VALUE"]
pub type R = crate::R<u32, super::HSTIMER2_VALUE>;
#[doc = "Writer for register HSTIMER2_VALUE"]
pub type W = crate::W<u32, super::HSTIMER2_VALUE>;
#[doc = "Register HSTIMER2_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTIMER2_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTIMER2_CNT`"]
pub type HSTIMER2_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HSTIMER2_CNT`"]
pub struct HSTIMER2_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer2"]
    #[inline(always)]
    pub fn hstimer2_cnt(&self) -> HSTIMER2_CNT_R {
        HSTIMER2_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer2"]
    #[inline(always)]
    pub fn hstimer2_cnt(&mut self) -> HSTIMER2_CNT_W {
        HSTIMER2_CNT_W { w: self }
    }
}
