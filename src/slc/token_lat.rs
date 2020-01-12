#[doc = "Reader of register TOKEN_LAT"]
pub type R = crate::R<u32, super::TOKEN_LAT>;
#[doc = "Writer for register TOKEN_LAT"]
pub type W = crate::W<u32, super::TOKEN_LAT>;
#[doc = "Register TOKEN_LAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TOKEN_LAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC1_TOKEN`"]
pub type SLC1_TOKEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC1_TOKEN`"]
pub struct SLC1_TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC0_TOKEN`"]
pub type SLC0_TOKEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC0_TOKEN`"]
pub struct SLC0_TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TOKEN_W<'a> {
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
    pub fn slc1_token(&self) -> SLC1_TOKEN_R {
        SLC1_TOKEN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc0_token(&self) -> SLC0_TOKEN_R {
        SLC0_TOKEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc1_token(&mut self) -> SLC1_TOKEN_W {
        SLC1_TOKEN_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc0_token(&mut self) -> SLC0_TOKEN_W {
        SLC0_TOKEN_W { w: self }
    }
}
