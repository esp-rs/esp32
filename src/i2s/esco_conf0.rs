#[doc = "Reader of register ESCO_CONF0"]
pub type R = crate::R<u32, super::ESCO_CONF0>;
#[doc = "Writer for register ESCO_CONF0"]
pub type W = crate::W<u32, super::ESCO_CONF0>;
#[doc = "Register ESCO_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESCO_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLC2DMA_EN`"]
pub type PLC2DMA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLC2DMA_EN`"]
pub struct PLC2DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC2DMA_EN_W<'a> {
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
#[doc = "Reader of field `PLC_EN`"]
pub type PLC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLC_EN`"]
pub struct PLC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_EN_W<'a> {
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
#[doc = "Reader of field `CVSD_DEC_RESET`"]
pub type CVSD_DEC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVSD_DEC_RESET`"]
pub struct CVSD_DEC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_DEC_RESET_W<'a> {
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
#[doc = "Reader of field `CVSD_DEC_START`"]
pub type CVSD_DEC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVSD_DEC_START`"]
pub struct CVSD_DEC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_DEC_START_W<'a> {
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
#[doc = "Reader of field `ESCO_CVSD_INF_EN`"]
pub type ESCO_CVSD_INF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCO_CVSD_INF_EN`"]
pub struct ESCO_CVSD_INF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_INF_EN_W<'a> {
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
#[doc = "Reader of field `ESCO_CVSD_PACK_LEN_8K`"]
pub type ESCO_CVSD_PACK_LEN_8K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ESCO_CVSD_PACK_LEN_8K`"]
pub struct ESCO_CVSD_PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `ESCO_CVSD_DEC_PACK_ERR`"]
pub type ESCO_CVSD_DEC_PACK_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCO_CVSD_DEC_PACK_ERR`"]
pub struct ESCO_CVSD_DEC_PACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_DEC_PACK_ERR_W<'a> {
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
#[doc = "Reader of field `ESCO_CHAN_MOD`"]
pub type ESCO_CHAN_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCO_CHAN_MOD`"]
pub struct ESCO_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CHAN_MOD_W<'a> {
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
#[doc = "Reader of field `ESCO_EN`"]
pub type ESCO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCO_EN`"]
pub struct ESCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_EN_W<'a> {
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
    pub fn plc2dma_en(&self) -> PLC2DMA_EN_R {
        PLC2DMA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn plc_en(&self) -> PLC_EN_R {
        PLC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cvsd_dec_reset(&self) -> CVSD_DEC_RESET_R {
        CVSD_DEC_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cvsd_dec_start(&self) -> CVSD_DEC_START_R {
        CVSD_DEC_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn esco_cvsd_inf_en(&self) -> ESCO_CVSD_INF_EN_R {
        ESCO_CVSD_INF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn esco_cvsd_pack_len_8k(&self) -> ESCO_CVSD_PACK_LEN_8K_R {
        ESCO_CVSD_PACK_LEN_8K_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn esco_cvsd_dec_pack_err(&self) -> ESCO_CVSD_DEC_PACK_ERR_R {
        ESCO_CVSD_DEC_PACK_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn esco_chan_mod(&self) -> ESCO_CHAN_MOD_R {
        ESCO_CHAN_MOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn esco_en(&self) -> ESCO_EN_R {
        ESCO_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn plc2dma_en(&mut self) -> PLC2DMA_EN_W {
        PLC2DMA_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn plc_en(&mut self) -> PLC_EN_W {
        PLC_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cvsd_dec_reset(&mut self) -> CVSD_DEC_RESET_W {
        CVSD_DEC_RESET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cvsd_dec_start(&mut self) -> CVSD_DEC_START_W {
        CVSD_DEC_START_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn esco_cvsd_inf_en(&mut self) -> ESCO_CVSD_INF_EN_W {
        ESCO_CVSD_INF_EN_W { w: self }
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn esco_cvsd_pack_len_8k(&mut self) -> ESCO_CVSD_PACK_LEN_8K_W {
        ESCO_CVSD_PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn esco_cvsd_dec_pack_err(&mut self) -> ESCO_CVSD_DEC_PACK_ERR_W {
        ESCO_CVSD_DEC_PACK_ERR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn esco_chan_mod(&mut self) -> ESCO_CHAN_MOD_W {
        ESCO_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn esco_en(&mut self) -> ESCO_EN_W {
        ESCO_EN_W { w: self }
    }
}
