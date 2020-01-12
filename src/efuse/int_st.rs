#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Writer for register INT_ST"]
pub type W = crate::W<u32, super::INT_ST>;
#[doc = "Register INT_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGM_DONE_INT_ST`"]
pub type PGM_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGM_DONE_INT_ST`"]
pub struct PGM_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_DONE_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `READ_DONE_INT_ST`"]
pub type READ_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_DONE_INT_ST`"]
pub struct READ_DONE_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_DONE_INT_ST_W<'a> {
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
    #[doc = "Bit 1 - program done interrupt status"]
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> PGM_DONE_INT_ST_R {
        PGM_DONE_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - read done interrupt status"]
    #[inline(always)]
    pub fn read_done_int_st(&self) -> READ_DONE_INT_ST_R {
        READ_DONE_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - program done interrupt status"]
    #[inline(always)]
    pub fn pgm_done_int_st(&mut self) -> PGM_DONE_INT_ST_W {
        PGM_DONE_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - read done interrupt status"]
    #[inline(always)]
    pub fn read_done_int_st(&mut self) -> READ_DONE_INT_ST_W {
        READ_DONE_INT_ST_W { w: self }
    }
}