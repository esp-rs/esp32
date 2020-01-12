#[doc = "Reader of register INLINK_DSCR_BF0"]
pub type R = crate::R<u32, super::INLINK_DSCR_BF0>;
#[doc = "Writer for register INLINK_DSCR_BF0"]
pub type W = crate::W<u32, super::INLINK_DSCR_BF0>;
#[doc = "Register INLINK_DSCR_BF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INLINK_DSCR_BF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INLINK_DSCR_BF0`"]
pub type INLINK_DSCR_BF0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INLINK_DSCR_BF0`"]
pub struct INLINK_DSCR_BF0_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_BF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&mut self) -> INLINK_DSCR_BF0_W {
        INLINK_DSCR_BF0_W { w: self }
    }
}
