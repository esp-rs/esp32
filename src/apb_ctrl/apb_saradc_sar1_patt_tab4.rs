#[doc = "Reader of register APB_SARADC_SAR1_PATT_TAB4"]
pub type R = crate::R<u32, super::APB_SARADC_SAR1_PATT_TAB4>;
#[doc = "Writer for register APB_SARADC_SAR1_PATT_TAB4"]
pub type W = crate::W<u32, super::APB_SARADC_SAR1_PATT_TAB4>;
#[doc = "Register APB_SARADC_SAR1_PATT_TAB4 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_SAR1_PATT_TAB4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SARADC_SAR1_PATT_TAB4`"]
pub type SARADC_SAR1_PATT_TAB4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SARADC_SAR1_PATT_TAB4`"]
pub struct SARADC_SAR1_PATT_TAB4_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_PATT_TAB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab4(&self) -> SARADC_SAR1_PATT_TAB4_R {
        SARADC_SAR1_PATT_TAB4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab4(&mut self) -> SARADC_SAR1_PATT_TAB4_W {
        SARADC_SAR1_PATT_TAB4_W { w: self }
    }
}
