#[doc = "Reader of register SENS_SARDATE_REG"]
pub type R = crate::R<u32, super::SENS_SARDATE_REG>;
#[doc = "Writer for register SENS_SARDATE_REG"]
pub type W = crate::W<u32, super::SENS_SARDATE_REG>;
#[doc = "Register SENS_SARDATE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SARDATE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR_DATE`"]
pub type SENS_SAR_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SENS_SAR_DATE`"]
pub struct SENS_SAR_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR_DATE_W<'a> {
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
    pub fn sens_sar_date(&self) -> SENS_SAR_DATE_R {
        SENS_SAR_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sens_sar_date(&mut self) -> SENS_SAR_DATE_W {
        SENS_SAR_DATE_W { w: self }
    }
}
