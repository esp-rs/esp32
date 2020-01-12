#[doc = "Reader of register CPU_INTR_FROM_CPU_3"]
pub type R = crate::R<u32, super::CPU_INTR_FROM_CPU_3>;
#[doc = "Writer for register CPU_INTR_FROM_CPU_3"]
pub type W = crate::W<u32, super::CPU_INTR_FROM_CPU_3>;
#[doc = "Register CPU_INTR_FROM_CPU_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_INTR_FROM_CPU_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPU_INTR_FROM_CPU_3`"]
pub type CPU_INTR_FROM_CPU_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_INTR_FROM_CPU_3`"]
pub struct CPU_INTR_FROM_CPU_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INTR_FROM_CPU_3_W<'a> {
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
    pub fn cpu_intr_from_cpu_3(&self) -> CPU_INTR_FROM_CPU_3_R {
        CPU_INTR_FROM_CPU_3_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_3(&mut self) -> CPU_INTR_FROM_CPU_3_W {
        CPU_INTR_FROM_CPU_3_W { w: self }
    }
}
