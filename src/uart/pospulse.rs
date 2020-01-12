#[doc = "Reader of register POSPULSE"]
pub type R = crate::R<u32, super::POSPULSE>;
#[doc = "Writer for register POSPULSE"]
pub type W = crate::W<u32, super::POSPULSE>;
#[doc = "Register POSPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::POSPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POSEDGE_MIN_CNT`"]
pub type POSEDGE_MIN_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POSEDGE_MIN_CNT`"]
pub struct POSEDGE_MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POSEDGE_MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the count of rxd posedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> POSEDGE_MIN_CNT_R {
        POSEDGE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the count of rxd posedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn posedge_min_cnt(&mut self) -> POSEDGE_MIN_CNT_W {
        POSEDGE_MIN_CNT_W { w: self }
    }
}
