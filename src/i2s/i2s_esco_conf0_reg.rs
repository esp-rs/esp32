#[doc = "Reader of register I2S_ESCO_CONF0_REG"]
pub type R = crate::R<u32, super::I2S_ESCO_CONF0_REG>;
#[doc = "Writer for register I2S_ESCO_CONF0_REG"]
pub type W = crate::W<u32, super::I2S_ESCO_CONF0_REG>;
#[doc = "Register I2S_ESCO_CONF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_ESCO_CONF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_PLC2DMA_EN`"]
pub type I2S_PLC2DMA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PLC2DMA_EN`"]
pub struct I2S_PLC2DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PLC2DMA_EN_W<'a> {
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
#[doc = "Reader of field `I2S_PLC_EN`"]
pub type I2S_PLC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_PLC_EN`"]
pub struct I2S_PLC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PLC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_DEC_RESET`"]
pub type I2S_CVSD_DEC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CVSD_DEC_RESET`"]
pub struct I2S_CVSD_DEC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_DEC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_DEC_START`"]
pub type I2S_CVSD_DEC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CVSD_DEC_START`"]
pub struct I2S_CVSD_DEC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_DEC_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `I2S_ESCO_CVSD_INF_EN`"]
pub type I2S_ESCO_CVSD_INF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_ESCO_CVSD_INF_EN`"]
pub struct I2S_ESCO_CVSD_INF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_ESCO_CVSD_INF_EN_W<'a> {
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
#[doc = "Reader of field `I2S_ESCO_CVSD_PACK_LEN_8K`"]
pub type I2S_ESCO_CVSD_PACK_LEN_8K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_ESCO_CVSD_PACK_LEN_8K`"]
pub struct I2S_ESCO_CVSD_PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_ESCO_CVSD_PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `I2S_ESCO_CVSD_DEC_PACK_ERR`"]
pub type I2S_ESCO_CVSD_DEC_PACK_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_ESCO_CVSD_DEC_PACK_ERR`"]
pub struct I2S_ESCO_CVSD_DEC_PACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_ESCO_CVSD_DEC_PACK_ERR_W<'a> {
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
#[doc = "Reader of field `I2S_ESCO_CHAN_MOD`"]
pub type I2S_ESCO_CHAN_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_ESCO_CHAN_MOD`"]
pub struct I2S_ESCO_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_ESCO_CHAN_MOD_W<'a> {
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
#[doc = "Reader of field `I2S_ESCO_EN`"]
pub type I2S_ESCO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_ESCO_EN`"]
pub struct I2S_ESCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_ESCO_EN_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_plc2dma_en(&self) -> I2S_PLC2DMA_EN_R {
        I2S_PLC2DMA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_plc_en(&self) -> I2S_PLC_EN_R {
        I2S_PLC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_cvsd_dec_reset(&self) -> I2S_CVSD_DEC_RESET_R {
        I2S_CVSD_DEC_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_cvsd_dec_start(&self) -> I2S_CVSD_DEC_START_R {
        I2S_CVSD_DEC_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_inf_en(&self) -> I2S_ESCO_CVSD_INF_EN_R {
        I2S_ESCO_CVSD_INF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_pack_len_8k(&self) -> I2S_ESCO_CVSD_PACK_LEN_8K_R {
        I2S_ESCO_CVSD_PACK_LEN_8K_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_dec_pack_err(&self) -> I2S_ESCO_CVSD_DEC_PACK_ERR_R {
        I2S_ESCO_CVSD_DEC_PACK_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_esco_chan_mod(&self) -> I2S_ESCO_CHAN_MOD_R {
        I2S_ESCO_CHAN_MOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_esco_en(&self) -> I2S_ESCO_EN_R {
        I2S_ESCO_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_plc2dma_en(&mut self) -> I2S_PLC2DMA_EN_W {
        I2S_PLC2DMA_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_plc_en(&mut self) -> I2S_PLC_EN_W {
        I2S_PLC_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_cvsd_dec_reset(&mut self) -> I2S_CVSD_DEC_RESET_W {
        I2S_CVSD_DEC_RESET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_cvsd_dec_start(&mut self) -> I2S_CVSD_DEC_START_W {
        I2S_CVSD_DEC_START_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_inf_en(&mut self) -> I2S_ESCO_CVSD_INF_EN_W {
        I2S_ESCO_CVSD_INF_EN_W { w: self }
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_pack_len_8k(&mut self) -> I2S_ESCO_CVSD_PACK_LEN_8K_W {
        I2S_ESCO_CVSD_PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_esco_cvsd_dec_pack_err(&mut self) -> I2S_ESCO_CVSD_DEC_PACK_ERR_W {
        I2S_ESCO_CVSD_DEC_PACK_ERR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_esco_chan_mod(&mut self) -> I2S_ESCO_CHAN_MOD_W {
        I2S_ESCO_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_esco_en(&mut self) -> I2S_ESCO_EN_W {
        I2S_ESCO_EN_W { w: self }
    }
}
