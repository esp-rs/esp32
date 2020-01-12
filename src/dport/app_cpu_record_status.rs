#[doc = "Reader of register APP_CPU_RECORD_STATUS"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_STATUS>;
#[doc = "Writer for register APP_CPU_RECORD_STATUS"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_STATUS>;
#[doc = "Register APP_CPU_RECORD_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_CPU_RECORDING`"]
pub type APP_CPU_RECORDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CPU_RECORDING`"]
pub struct APP_CPU_RECORDING_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_RECORDING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_recording(&self) -> APP_CPU_RECORDING_R {
        APP_CPU_RECORDING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_recording(&mut self) -> APP_CPU_RECORDING_W {
        APP_CPU_RECORDING_W { w: self }
    }
}
