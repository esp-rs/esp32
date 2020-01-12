#[doc = "Reader of register 0_LEN_LIM_CONF"]
pub type R = crate::R<u32, super::_0_LEN_LIM_CONF>;
#[doc = "Writer for register 0_LEN_LIM_CONF"]
pub type W = crate::W<u32, super::_0_LEN_LIM_CONF>;
#[doc = "Register 0_LEN_LIM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_LEN_LIM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC0_LEN_LIM`"]
pub type SLC0_LEN_LIM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC0_LEN_LIM`"]
pub struct SLC0_LEN_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_LIM_W<'a> {
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
    pub fn slc0_len_lim(&self) -> SLC0_LEN_LIM_R {
        SLC0_LEN_LIM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_lim(&mut self) -> SLC0_LEN_LIM_W {
        SLC0_LEN_LIM_W { w: self }
    }
}
