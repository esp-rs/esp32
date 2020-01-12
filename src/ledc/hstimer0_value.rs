#[doc = "Reader of register HSTIMER0_VALUE"]
pub type R = crate::R<u32, super::HSTIMER0_VALUE>;
#[doc = "Writer for register HSTIMER0_VALUE"]
pub type W = crate::W<u32, super::HSTIMER0_VALUE>;
#[doc = "Register HSTIMER0_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTIMER0_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTIMER0_CNT`"]
pub type HSTIMER0_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HSTIMER0_CNT`"]
pub struct HSTIMER0_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER0_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer0"]
    #[inline(always)]
    pub fn hstimer0_cnt(&self) -> HSTIMER0_CNT_R {
        HSTIMER0_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer0"]
    #[inline(always)]
    pub fn hstimer0_cnt(&mut self) -> HSTIMER0_CNT_W {
        HSTIMER0_CNT_W { w: self }
    }
}
