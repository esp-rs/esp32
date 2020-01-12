#[doc = "Reader of register DT0_RED_CFG"]
pub type R = crate::R<u32, super::DT0_RED_CFG>;
#[doc = "Writer for register DT0_RED_CFG"]
pub type W = crate::W<u32, super::DT0_RED_CFG>;
#[doc = "Register DT0_RED_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DT0_RED_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT0_RED`"]
pub type DT0_RED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT0_RED`"]
pub struct DT0_RED_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_RED_W<'a> {
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
    pub fn dt0_red(&self) -> DT0_RED_R {
        DT0_RED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow reg for RED"]
    #[inline(always)]
    pub fn dt0_red(&mut self) -> DT0_RED_W {
        DT0_RED_W { w: self }
    }
}