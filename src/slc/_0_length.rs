#[doc = "Reader of register 0_LENGTH"]
pub type R = crate::R<u32, super::_0_LENGTH>;
#[doc = "Writer for register 0_LENGTH"]
pub type W = crate::W<u32, super::_0_LENGTH>;
#[doc = "Register 0_LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_LEN`"]
pub type SLC_SLC0_LEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_SLC0_LEN`"]
pub struct SLC_SLC0_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_slc0_len(&self) -> SLC_SLC0_LEN_R {
        SLC_SLC0_LEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_slc0_len(&mut self) -> SLC_SLC0_LEN_W {
        SLC_SLC0_LEN_W { w: self }
    }
}
