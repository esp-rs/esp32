#[doc = "Reader of register ULP_CP_SLEEP_CYC0"]
pub type R = crate::R<u32, super::ULP_CP_SLEEP_CYC0>;
#[doc = "Writer for register ULP_CP_SLEEP_CYC0"]
pub type W = crate::W<u32, super::ULP_CP_SLEEP_CYC0>;
#[doc = "Register ULP_CP_SLEEP_CYC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ULP_CP_SLEEP_CYC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLEEP_CYCLES_S0`"]
pub type SLEEP_CYCLES_S0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLEEP_CYCLES_S0`"]
pub struct SLEEP_CYCLES_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_CYCLES_S0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sleep_cycles_s0(&self) -> SLEEP_CYCLES_S0_R {
        SLEEP_CYCLES_S0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sleep_cycles_s0(&mut self) -> SLEEP_CYCLES_S0_W {
        SLEEP_CYCLES_S0_W { w: self }
    }
}
