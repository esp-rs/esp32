#[doc = "Reader of register I2S_PD_CONF_REG(i)"]
pub type R = crate::R<u32, super::I2S_PD_CONF_REGI>;
#[doc = "Writer for register I2S_PD_CONF_REG(i)"]
pub type W = crate::W<u32, super::I2S_PD_CONF_REGI>;
#[doc = "Register I2S_PD_CONF_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_PD_CONF_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_PLC_MEM_FORCE_PU`"]
pub type I2S_PLC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PLC_MEM_FORCE_PU`"]
pub struct I2S_PLC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PLC_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `I2S_PLC_MEM_FORCE_PD`"]
pub type I2S_PLC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PLC_MEM_FORCE_PD`"]
pub struct I2S_PLC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PLC_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `I2S_FIFO_FORCE_PU`"]
pub type I2S_FIFO_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_FIFO_FORCE_PU`"]
pub struct I2S_FIFO_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_FIFO_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `I2S_FIFO_FORCE_PD`"]
pub type I2S_FIFO_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_FIFO_FORCE_PD`"]
pub struct I2S_FIFO_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_FIFO_FORCE_PD_W<'a> {
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
    pub fn i2s_plc_mem_force_pu(&self) -> I2S_PLC_MEM_FORCE_PU_R {
        I2S_PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_plc_mem_force_pd(&self) -> I2S_PLC_MEM_FORCE_PD_R {
        I2S_PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_fifo_force_pu(&self) -> I2S_FIFO_FORCE_PU_R {
        I2S_FIFO_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_fifo_force_pd(&self) -> I2S_FIFO_FORCE_PD_R {
        I2S_FIFO_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_plc_mem_force_pu(&mut self) -> I2S_PLC_MEM_FORCE_PU_W {
        I2S_PLC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_plc_mem_force_pd(&mut self) -> I2S_PLC_MEM_FORCE_PD_W {
        I2S_PLC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_fifo_force_pu(&mut self) -> I2S_FIFO_FORCE_PU_W {
        I2S_FIFO_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_fifo_force_pd(&mut self) -> I2S_FIFO_FORCE_PD_W {
        I2S_FIFO_FORCE_PD_W { w: self }
    }
}
