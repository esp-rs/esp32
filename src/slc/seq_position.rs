#[doc = "Reader of register SEQ_POSITION"]
pub type R = crate::R<u32, super::SEQ_POSITION>;
#[doc = "Writer for register SEQ_POSITION"]
pub type W = crate::W<u32, super::SEQ_POSITION>;
#[doc = "Register SEQ_POSITION `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ_POSITION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC1_SEQ_POSITION`"]
pub type SLC1_SEQ_POSITION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC1_SEQ_POSITION`"]
pub struct SLC1_SEQ_POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_SEQ_POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLC0_SEQ_POSITION`"]
pub type SLC0_SEQ_POSITION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC0_SEQ_POSITION`"]
pub struct SLC0_SEQ_POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_SEQ_POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc1_seq_position(&self) -> SLC1_SEQ_POSITION_R {
        SLC1_SEQ_POSITION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc0_seq_position(&self) -> SLC0_SEQ_POSITION_R {
        SLC0_SEQ_POSITION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc1_seq_position(&mut self) -> SLC1_SEQ_POSITION_W {
        SLC1_SEQ_POSITION_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc0_seq_position(&mut self) -> SLC0_SEQ_POSITION_W {
        SLC0_SEQ_POSITION_W { w: self }
    }
}
