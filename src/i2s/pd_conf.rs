#[doc = "Reader of register PD_CONF"]
pub type R = crate::R<u32, super::PD_CONF>;
#[doc = "Writer for register PD_CONF"]
pub type W = crate::W<u32, super::PD_CONF>;
#[doc = "Register PD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::PD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLC_MEM_FORCE_PU`"]
pub type PLC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLC_MEM_FORCE_PU`"]
pub struct PLC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PLC_MEM_FORCE_PD`"]
pub type PLC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLC_MEM_FORCE_PD`"]
pub struct PLC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FIFO_FORCE_PU`"]
pub type FIFO_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_FORCE_PU`"]
pub struct FIFO_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `FIFO_FORCE_PD`"]
pub type FIFO_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_FORCE_PD`"]
pub struct FIFO_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FORCE_PD_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W {
        PLC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W {
        PLC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W {
        FIFO_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W {
        FIFO_FORCE_PD_W { w: self }
    }
}
