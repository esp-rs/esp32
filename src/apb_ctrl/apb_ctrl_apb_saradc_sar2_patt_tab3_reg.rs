#[doc = "Reader of register APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG"]
pub type R = crate::R<u32, super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG>;
#[doc = "Writer for register APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG"]
pub type W = crate::W<u32, super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG>;
#[doc = "Register APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR2_PATT_TAB3`"]
pub type APB_CTRL_SARADC_SAR2_PATT_TAB3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR2_PATT_TAB3`"]
pub struct APB_CTRL_SARADC_SAR2_PATT_TAB3_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR2_PATT_TAB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_tab3(&self) -> APB_CTRL_SARADC_SAR2_PATT_TAB3_R {
        APB_CTRL_SARADC_SAR2_PATT_TAB3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_tab3(&mut self) -> APB_CTRL_SARADC_SAR2_PATT_TAB3_W {
        APB_CTRL_SARADC_SAR2_PATT_TAB3_W { w: self }
    }
}
