#[doc = "Reader of register cali_data"]
pub type R = crate::R<u32, super::CALI_DATA>;
#[doc = "Writer for register cali_data"]
pub type W = crate::W<u32, super::CALI_DATA>;
#[doc = "Register cali_data `reset()`'s with value 0"]
impl crate::ResetValue for super::CALI_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALI_RDY_SYNC2`"]
pub type CALI_RDY_SYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALI_RDY_SYNC2`"]
pub struct CALI_RDY_SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CALI_RDY_SYNC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `CALI_RDY_REAL`"]
pub type CALI_RDY_REAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALI_RDY_REAL`"]
pub struct CALI_RDY_REAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALI_RDY_REAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CALI_VALUE_SYNC2`"]
pub type CALI_VALUE_SYNC2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CALI_VALUE_SYNC2`"]
pub struct CALI_VALUE_SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CALI_VALUE_SYNC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_rdy_sync2(&self) -> CALI_RDY_SYNC2_R {
        CALI_RDY_SYNC2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cali_rdy_real(&self) -> CALI_RDY_REAL_R {
        CALI_RDY_REAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn cali_value_sync2(&self) -> CALI_VALUE_SYNC2_R {
        CALI_VALUE_SYNC2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_rdy_sync2(&mut self) -> CALI_RDY_SYNC2_W {
        CALI_RDY_SYNC2_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cali_rdy_real(&mut self) -> CALI_RDY_REAL_W {
        CALI_RDY_REAL_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn cali_value_sync2(&mut self) -> CALI_VALUE_SYNC2_W {
        CALI_VALUE_SYNC2_W { w: self }
    }
}
