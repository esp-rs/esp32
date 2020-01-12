#[doc = "Reader of register PCPU_NMI_INT1"]
pub type R = crate::R<u32, super::PCPU_NMI_INT1>;
#[doc = "Writer for register PCPU_NMI_INT1"]
pub type W = crate::W<u32, super::PCPU_NMI_INT1>;
#[doc = "Register PCPU_NMI_INT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCPU_NMI_INT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROCPU_NMI_INT_H`"]
pub type PROCPU_NMI_INT_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROCPU_NMI_INT_H`"]
pub struct PROCPU_NMI_INT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCPU_NMI_INT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 PRO CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn procpu_nmi_int_h(&self) -> PROCPU_NMI_INT_H_R {
        PROCPU_NMI_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 PRO CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn procpu_nmi_int_h(&mut self) -> PROCPU_NMI_INT_H_W {
        PROCPU_NMI_INT_H_W { w: self }
    }
}
