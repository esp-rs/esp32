#[doc = "Reader of register APP_INTRUSION_STATUS"]
pub type R = crate::R<u32, super::APP_INTRUSION_STATUS>;
#[doc = "Writer for register APP_INTRUSION_STATUS"]
pub type W = crate::W<u32, super::APP_INTRUSION_STATUS>;
#[doc = "Register APP_INTRUSION_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_INTRUSION_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_INTRUSION_RECORD`"]
pub type APP_INTRUSION_RECORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_INTRUSION_RECORD`"]
pub struct APP_INTRUSION_RECORD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_INTRUSION_RECORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn app_intrusion_record(&self) -> APP_INTRUSION_RECORD_R {
        APP_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn app_intrusion_record(&mut self) -> APP_INTRUSION_RECORD_W {
        APP_INTRUSION_RECORD_W { w: self }
    }
}
