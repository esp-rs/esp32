#[doc = "Reader of register RSA_PD_CTRL"]
pub type R = crate::R<u32, super::RSA_PD_CTRL>;
#[doc = "Writer for register RSA_PD_CTRL"]
pub type W = crate::W<u32, super::RSA_PD_CTRL>;
#[doc = "Register RSA_PD_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RSA_PD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_RSA_PD`"]
pub type DPORT_RSA_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_RSA_PD`"]
pub struct DPORT_RSA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_RSA_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_rsa_pd(&self) -> DPORT_RSA_PD_R {
        DPORT_RSA_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_rsa_pd(&mut self) -> DPORT_RSA_PD_W {
        DPORT_RSA_PD_W { w: self }
    }
}
