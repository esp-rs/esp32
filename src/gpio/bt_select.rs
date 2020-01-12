#[doc = "Reader of register BT_SELECT"]
pub type R = crate::R<u32, super::BT_SELECT>;
#[doc = "Writer for register BT_SELECT"]
pub type W = crate::W<u32, super::BT_SELECT>;
#[doc = "Register BT_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::BT_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BT_SEL`"]
pub type BT_SEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BT_SEL`"]
pub struct BT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BT_SEL_W {
        BT_SEL_W { w: self }
    }
}
