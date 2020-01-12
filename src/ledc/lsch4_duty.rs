#[doc = "Reader of register LSCH4_DUTY"]
pub type R = crate::R<u32, super::LSCH4_DUTY>;
#[doc = "Writer for register LSCH4_DUTY"]
pub type W = crate::W<u32, super::LSCH4_DUTY>;
#[doc = "Register LSCH4_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH4_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_LSCH4`"]
pub type DUTY_LSCH4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_LSCH4`"]
pub struct DUTY_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_LSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When lstimerx(x=\\[0 3\\]) choosed by low speed channel4 has reached reg_lpoint_lsch4 the output signal changes to low. reg_lpoint_lsch4=(reg_hpoint_lsch4\\[19:0\\]+reg_duty_lsch4\\[24:4\\]) (1) reg_lpoint_lsch4=(reg_hpoint_lsch4\\[19:0\\]+reg_duty_lsch4\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_lsch4(&self) -> DUTY_LSCH4_R {
        DUTY_LSCH4_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When lstimerx(x=\\[0 3\\]) choosed by low speed channel4 has reached reg_lpoint_lsch4 the output signal changes to low. reg_lpoint_lsch4=(reg_hpoint_lsch4\\[19:0\\]+reg_duty_lsch4\\[24:4\\]) (1) reg_lpoint_lsch4=(reg_hpoint_lsch4\\[19:0\\]+reg_duty_lsch4\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_lsch4(&mut self) -> DUTY_LSCH4_W {
        DUTY_LSCH4_W { w: self }
    }
}
