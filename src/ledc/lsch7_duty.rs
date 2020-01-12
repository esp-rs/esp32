#[doc = "Reader of register LSCH7_DUTY"]
pub type R = crate::R<u32, super::LSCH7_DUTY>;
#[doc = "Writer for register LSCH7_DUTY"]
pub type W = crate::W<u32, super::LSCH7_DUTY>;
#[doc = "Register LSCH7_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH7_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_LSCH7`"]
pub type DUTY_LSCH7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_LSCH7`"]
pub struct DUTY_LSCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_LSCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When lstimerx(x=\\[0 3\\]) choosed by low speed channel7 has reached reg_lpoint_lsch7 the output signal changes to low. reg_lpoint_lsch7=(reg_hpoint_lsch7\\[19:0\\]+reg_duty_lsch7\\[24:4\\]) (1) reg_lpoint_lsch7=(reg_hpoint_lsch7\\[19:0\\]+reg_duty_lsch7\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_lsch7(&self) -> DUTY_LSCH7_R {
        DUTY_LSCH7_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When lstimerx(x=\\[0 3\\]) choosed by low speed channel7 has reached reg_lpoint_lsch7 the output signal changes to low. reg_lpoint_lsch7=(reg_hpoint_lsch7\\[19:0\\]+reg_duty_lsch7\\[24:4\\]) (1) reg_lpoint_lsch7=(reg_hpoint_lsch7\\[19:0\\]+reg_duty_lsch7\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_lsch7(&mut self) -> DUTY_LSCH7_W {
        DUTY_LSCH7_W { w: self }
    }
}