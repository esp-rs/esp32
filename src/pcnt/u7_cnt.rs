#[doc = "Reader of register U7_CNT"]
pub type R = crate::R<u32, super::U7_CNT>;
#[doc = "Writer for register U7_CNT"]
pub type W = crate::W<u32, super::U7_CNT>;
#[doc = "Register U7_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::U7_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLUS_CNT_U7`"]
pub type PLUS_CNT_U7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLUS_CNT_U7`"]
pub struct PLUS_CNT_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> PLUS_CNT_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit7."]
    #[inline(always)]
    pub fn plus_cnt_u7(&self) -> PLUS_CNT_U7_R {
        PLUS_CNT_U7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit7."]
    #[inline(always)]
    pub fn plus_cnt_u7(&mut self) -> PLUS_CNT_U7_W {
        PLUS_CNT_U7_W { w: self }
    }
}
