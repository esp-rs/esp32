#[doc = "Reader of register SLC_TOKEN_LAT_REG"]
pub type R = crate::R<u32, super::SLC_TOKEN_LAT_REG>;
#[doc = "Writer for register SLC_TOKEN_LAT_REG"]
pub type W = crate::W<u32, super::SLC_TOKEN_LAT_REG>;
#[doc = "Register SLC_TOKEN_LAT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_TOKEN_LAT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN`"]
pub type SLC_SLC1_TOKEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN`"]
pub struct SLC_SLC1_TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC0_TOKEN`"]
pub type SLC_SLC0_TOKEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN`"]
pub struct SLC_SLC0_TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_slc1_token(&self) -> SLC_SLC1_TOKEN_R {
        SLC_SLC1_TOKEN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_slc0_token(&self) -> SLC_SLC0_TOKEN_R {
        SLC_SLC0_TOKEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_slc1_token(&mut self) -> SLC_SLC1_TOKEN_W {
        SLC_SLC1_TOKEN_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_slc0_token(&mut self) -> SLC_SLC0_TOKEN_W {
        SLC_SLC0_TOKEN_W { w: self }
    }
}
