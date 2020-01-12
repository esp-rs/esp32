#[doc = "Reader of register INT_CLR"]
pub type R = crate::R<u32, super::INT_CLR>;
#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGM_DONE_INT_CLR`"]
pub type PGM_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGM_DONE_INT_CLR`"]
pub struct PGM_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_DONE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `READ_DONE_INT_CLR`"]
pub type READ_DONE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_DONE_INT_CLR`"]
pub struct READ_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_DONE_INT_CLR_W<'a> {
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
    #[doc = "Bit 1 - program done interrupt clear"]
    #[inline(always)]
    pub fn pgm_done_int_clr(&self) -> PGM_DONE_INT_CLR_R {
        PGM_DONE_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - read done interrupt clear"]
    #[inline(always)]
    pub fn read_done_int_clr(&self) -> READ_DONE_INT_CLR_R {
        READ_DONE_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - program done interrupt clear"]
    #[inline(always)]
    pub fn pgm_done_int_clr(&mut self) -> PGM_DONE_INT_CLR_W {
        PGM_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0 - read done interrupt clear"]
    #[inline(always)]
    pub fn read_done_int_clr(&mut self) -> READ_DONE_INT_CLR_W {
        READ_DONE_INT_CLR_W { w: self }
    }
}
