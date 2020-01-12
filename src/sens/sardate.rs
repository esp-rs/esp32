#[doc = "Reader of register SARDATE"]
pub type R = crate::R<u32, super::SARDATE>;
#[doc = "Writer for register SARDATE"]
pub type W = crate::W<u32, super::SARDATE>;
#[doc = "Register SARDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SARDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR_DATE`"]
pub type SAR_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR_DATE`"]
pub struct SAR_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sar_date(&self) -> SAR_DATE_R {
        SAR_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sar_date(&mut self) -> SAR_DATE_W {
        SAR_DATE_W { w: self }
    }
}
