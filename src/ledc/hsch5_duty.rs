#[doc = "Reader of register HSCH5_DUTY"]
pub type R = crate::R<u32, super::HSCH5_DUTY>;
#[doc = "Writer for register HSCH5_DUTY"]
pub type W = crate::W<u32, super::HSCH5_DUTY>;
#[doc = "Register HSCH5_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH5_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_HSCH5`"]
pub type DUTY_HSCH5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_HSCH5`"]
pub struct DUTY_HSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_HSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel5 has reached reg_lpoint_hsch5 the output signal changes to low. reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\]) (1) reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch5(&self) -> DUTY_HSCH5_R {
        DUTY_HSCH5_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel5 has reached reg_lpoint_hsch5 the output signal changes to low. reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\]) (1) reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch5(&mut self) -> DUTY_HSCH5_W {
        DUTY_HSCH5_W { w: self }
    }
}
