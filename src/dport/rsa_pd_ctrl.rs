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
#[doc = "Reader of field `RSA_PD`"]
pub type RSA_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSA_PD`"]
pub struct RSA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA_PD_W<'a> {
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
    pub fn rsa_pd(&self) -> RSA_PD_R {
        RSA_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsa_pd(&mut self) -> RSA_PD_W {
        RSA_PD_W { w: self }
    }
}
