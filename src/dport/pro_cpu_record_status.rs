#[doc = "Reader of register PRO_CPU_RECORD_STATUS"]
pub type R = crate::R<u32, super::PRO_CPU_RECORD_STATUS>;
#[doc = "Writer for register PRO_CPU_RECORD_STATUS"]
pub type W = crate::W<u32, super::PRO_CPU_RECORD_STATUS>;
#[doc = "Register PRO_CPU_RECORD_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_CPU_RECORD_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_CPU_RECORDING`"]
pub type PRO_CPU_RECORDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_CPU_RECORDING`"]
pub struct PRO_CPU_RECORDING_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_RECORDING_W<'a> {
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
    pub fn pro_cpu_recording(&self) -> PRO_CPU_RECORDING_R {
        PRO_CPU_RECORDING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cpu_recording(&mut self) -> PRO_CPU_RECORDING_W {
        PRO_CPU_RECORDING_W { w: self }
    }
}
