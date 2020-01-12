#[doc = "Reader of register RESET_STATE"]
pub type R = crate::R<u32, super::RESET_STATE>;
#[doc = "Writer for register RESET_STATE"]
pub type W = crate::W<u32, super::RESET_STATE>;
#[doc = "Register RESET_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RESET_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROCPU_STAT_VECTOR_SEL`"]
pub type PROCPU_STAT_VECTOR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROCPU_STAT_VECTOR_SEL`"]
pub struct PROCPU_STAT_VECTOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCPU_STAT_VECTOR_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `APPCPU_STAT_VECTOR_SEL`"]
pub type APPCPU_STAT_VECTOR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APPCPU_STAT_VECTOR_SEL`"]
pub struct APPCPU_STAT_VECTOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_STAT_VECTOR_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RESET_CAUSE_APPCPU`"]
pub type RESET_CAUSE_APPCPU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESET_CAUSE_APPCPU`"]
pub struct RESET_CAUSE_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CAUSE_APPCPU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESET_CAUSE_PROCPU`"]
pub type RESET_CAUSE_PROCPU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESET_CAUSE_PROCPU`"]
pub struct RESET_CAUSE_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CAUSE_PROCPU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&self) -> PROCPU_STAT_VECTOR_SEL_R {
        PROCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&self) -> APPCPU_STAT_VECTOR_SEL_R {
        APPCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 6:11 - reset cause of APP CPU"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&mut self) -> PROCPU_STAT_VECTOR_SEL_W {
        PROCPU_STAT_VECTOR_SEL_W { w: self }
    }
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&mut self) -> APPCPU_STAT_VECTOR_SEL_W {
        APPCPU_STAT_VECTOR_SEL_W { w: self }
    }
    #[doc = "Bits 6:11 - reset cause of APP CPU"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&mut self) -> RESET_CAUSE_APPCPU_W {
        RESET_CAUSE_APPCPU_W { w: self }
    }
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&mut self) -> RESET_CAUSE_PROCPU_W {
        RESET_CAUSE_PROCPU_W { w: self }
    }
}
