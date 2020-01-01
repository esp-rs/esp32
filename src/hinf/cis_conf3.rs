#[doc = "Reader of register CIS_CONF3"]
pub type R = crate::R<u32, super::CIS_CONF3>;
#[doc = "Writer for register CIS_CONF3"]
pub type W = crate::W<u32, super::CIS_CONF3>;
#[doc = "Register CIS_CONF3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CIS_CONF3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HINF_CIS_CONF_W3`"]
pub type HINF_CIS_CONF_W3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HINF_CIS_CONF_W3`"]
pub struct HINF_CIS_CONF_W3_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_CIS_CONF_W3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hinf_cis_conf_w3(&self) -> HINF_CIS_CONF_W3_R {
        HINF_CIS_CONF_W3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hinf_cis_conf_w3(&mut self) -> HINF_CIS_CONF_W3_W {
        HINF_CIS_CONF_W3_W { w: self }
    }
}
