#[doc = "Reader of register PCPU_INT"]
pub type R = crate::R<u32, super::PCPU_INT>;
#[doc = "Writer for register PCPU_INT"]
pub type W = crate::W<u32, super::PCPU_INT>;
#[doc = "Register PCPU_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::PCPU_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_PROCPU_INT`"]
pub type GPIO_PROCPU_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_PROCPU_INT`"]
pub struct GPIO_PROCPU_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PROCPU_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 PRO CPU interrupt status"]
    #[inline(always)]
    pub fn gpio_procpu_int(&self) -> GPIO_PROCPU_INT_R {
        GPIO_PROCPU_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 PRO CPU interrupt status"]
    #[inline(always)]
    pub fn gpio_procpu_int(&mut self) -> GPIO_PROCPU_INT_W {
        GPIO_PROCPU_INT_W { w: self }
    }
}
