#[doc = "Reader of register HSCH1_DUTY"]
pub type R = crate::R<u32, super::HSCH1_DUTY>;
#[doc = "Writer for register HSCH1_DUTY"]
pub type W = crate::W<u32, super::HSCH1_DUTY>;
#[doc = "Register HSCH1_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH1_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_HSCH1`"]
pub type DUTY_HSCH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_HSCH1`"]
pub struct DUTY_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_HSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel1 has reached reg_lpoint_hsch1 the output signal changes to low. reg_lpoint_hsch1=(reg_hpoint_hsch1\\[19:0\\]+reg_duty_hsch1\\[24:4\\]) (1) reg_lpoint_hsch1=(reg_hpoint_hsch1\\[19:0\\]+reg_duty_hsch1\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch1(&self) -> DUTY_HSCH1_R {
        DUTY_HSCH1_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel1 has reached reg_lpoint_hsch1 the output signal changes to low. reg_lpoint_hsch1=(reg_hpoint_hsch1\\[19:0\\]+reg_duty_hsch1\\[24:4\\]) (1) reg_lpoint_hsch1=(reg_hpoint_hsch1\\[19:0\\]+reg_duty_hsch1\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch1(&mut self) -> DUTY_HSCH1_W {
        DUTY_HSCH1_W { w: self }
    }
}
