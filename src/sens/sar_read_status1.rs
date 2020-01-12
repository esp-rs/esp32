#[doc = "Reader of register SAR_READ_STATUS1"]
pub type R = crate::R<u32, super::SAR_READ_STATUS1>;
#[doc = "Writer for register SAR_READ_STATUS1"]
pub type W = crate::W<u32, super::SAR_READ_STATUS1>;
#[doc = "Register SAR_READ_STATUS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_READ_STATUS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR1_READER_STATUS`"]
pub type SAR1_READER_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR1_READER_STATUS`"]
pub struct SAR1_READER_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_READER_STATUS_W<'a> {
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
    pub fn sar1_reader_status(&self) -> SAR1_READER_STATUS_R {
        SAR1_READER_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar1_reader_status(&mut self) -> SAR1_READER_STATUS_W {
        SAR1_READER_STATUS_W { w: self }
    }
}
