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
#[doc = "Reader of field `CIS_CONF_W3`"]
pub type CIS_CONF_W3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CIS_CONF_W3`"]
pub struct CIS_CONF_W3_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_CONF_W3_W<'a> {
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
    pub fn cis_conf_w3(&self) -> CIS_CONF_W3_R {
        CIS_CONF_W3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w3(&mut self) -> CIS_CONF_W3_W {
        CIS_CONF_W3_W { w: self }
    }
}
