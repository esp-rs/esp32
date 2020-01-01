#[doc = "Reader of register ACCESS_CHECK"]
pub type R = crate::R<u32, super::ACCESS_CHECK>;
#[doc = "Writer for register ACCESS_CHECK"]
pub type W = crate::W<u32, super::ACCESS_CHECK>;
#[doc = "Register ACCESS_CHECK `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCESS_CHECK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_ACCESS_CHECK_APP`"]
pub type DPORT_ACCESS_CHECK_APP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_ACCESS_CHECK_APP`"]
pub struct DPORT_ACCESS_CHECK_APP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_ACCESS_CHECK_APP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_ACCESS_CHECK_PRO`"]
pub type DPORT_ACCESS_CHECK_PRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_ACCESS_CHECK_PRO`"]
pub struct DPORT_ACCESS_CHECK_PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_ACCESS_CHECK_PRO_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_access_check_app(&self) -> DPORT_ACCESS_CHECK_APP_R {
        DPORT_ACCESS_CHECK_APP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_access_check_pro(&self) -> DPORT_ACCESS_CHECK_PRO_R {
        DPORT_ACCESS_CHECK_PRO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_access_check_app(&mut self) -> DPORT_ACCESS_CHECK_APP_W {
        DPORT_ACCESS_CHECK_APP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_access_check_pro(&mut self) -> DPORT_ACCESS_CHECK_PRO_W {
        DPORT_ACCESS_CHECK_PRO_W { w: self }
    }
}
