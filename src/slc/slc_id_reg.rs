#[doc = "Reader of register SLC_ID_REG"]
pub type R = crate::R<u32, super::SLC_ID_REG>;
#[doc = "Writer for register SLC_ID_REG"]
pub type W = crate::W<u32, super::SLC_ID_REG>;
#[doc = "Register SLC_ID_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_ID_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_ID`"]
pub type SLC_ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_ID`"]
pub struct SLC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_ID_W<'a> {
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
    pub fn slc_id(&self) -> SLC_ID_R {
        SLC_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_id(&mut self) -> SLC_ID_W {
        SLC_ID_W { w: self }
    }
}
