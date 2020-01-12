#[doc = "Reader of register APP_INTR_STATUS_1"]
pub type R = crate::R<u32, super::APP_INTR_STATUS_1>;
#[doc = "Writer for register APP_INTR_STATUS_1"]
pub type W = crate::W<u32, super::APP_INTR_STATUS_1>;
#[doc = "Register APP_INTR_STATUS_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_INTR_STATUS_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_INTR_STATUS_1`"]
pub type APP_INTR_STATUS_1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APP_INTR_STATUS_1`"]
pub struct APP_INTR_STATUS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_INTR_STATUS_1_W<'a> {
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
    pub fn app_intr_status_1(&self) -> APP_INTR_STATUS_1_R {
        APP_INTR_STATUS_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_intr_status_1(&mut self) -> APP_INTR_STATUS_1_W {
        APP_INTR_STATUS_1_W { w: self }
    }
}
