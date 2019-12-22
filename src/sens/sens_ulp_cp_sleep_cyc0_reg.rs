#[doc = "Reader of register SENS_ULP_CP_SLEEP_CYC0_REG"]
pub type R = crate::R<u32, super::SENS_ULP_CP_SLEEP_CYC0_REG>;
#[doc = "Writer for register SENS_ULP_CP_SLEEP_CYC0_REG"]
pub type W = crate::W<u32, super::SENS_ULP_CP_SLEEP_CYC0_REG>;
#[doc = "Register SENS_ULP_CP_SLEEP_CYC0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_ULP_CP_SLEEP_CYC0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SLEEP_CYCLES_S0`"]
pub type SENS_SLEEP_CYCLES_S0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SENS_SLEEP_CYCLES_S0`"]
pub struct SENS_SLEEP_CYCLES_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SLEEP_CYCLES_S0_W<'a> {
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
    pub fn sens_sleep_cycles_s0(&self) -> SENS_SLEEP_CYCLES_S0_R {
        SENS_SLEEP_CYCLES_S0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sens_sleep_cycles_s0(&mut self) -> SENS_SLEEP_CYCLES_S0_W {
        SENS_SLEEP_CYCLES_S0_W { w: self }
    }
}