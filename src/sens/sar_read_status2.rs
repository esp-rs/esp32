#[doc = "Reader of register SAR_READ_STATUS2"]
pub type R = crate::R<u32, super::SAR_READ_STATUS2>;
#[doc = "Writer for register SAR_READ_STATUS2"]
pub type W = crate::W<u32, super::SAR_READ_STATUS2>;
#[doc = "Register SAR_READ_STATUS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_READ_STATUS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_READER_STATUS`"]
pub type SAR2_READER_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR2_READER_STATUS`"]
pub struct SAR2_READER_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_READER_STATUS_W<'a> {
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
    pub fn sar2_reader_status(&self) -> SAR2_READER_STATUS_R {
        SAR2_READER_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar2_reader_status(&mut self) -> SAR2_READER_STATUS_W {
        SAR2_READER_STATUS_W { w: self }
    }
}