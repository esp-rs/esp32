#[doc = "Reader of register DT1_RED_CFG"]
pub type R = crate::R<u32, super::DT1_RED_CFG>;
#[doc = "Writer for register DT1_RED_CFG"]
pub type W = crate::W<u32, super::DT1_RED_CFG>;
#[doc = "Register DT1_RED_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DT1_RED_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT1_RED`"]
pub type DT1_RED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT1_RED`"]
pub struct DT1_RED_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow reg for RED"]
    #[inline(always)]
    pub fn dt1_red(&self) -> DT1_RED_R {
        DT1_RED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow reg for RED"]
    #[inline(always)]
    pub fn dt1_red(&mut self) -> DT1_RED_W {
        DT1_RED_W { w: self }
    }
}
